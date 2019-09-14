//! Crate `ruma_events` contains serializable types for the events in the [Matrix](https://matrix.org)
//! specification that can be shared by client and server code.
//!
//! All data exchanged over Matrix is expressed as an event.
//! Different event types represent different actions, such as joining a room or sending a message.
//! Events are stored and transmitted as simple JSON structures.
//! While anyone can create a new event type for their own purposes, the Matrix specification
//! defines a number of event types which are considered core to the protocol, and Matrix clients
//! and servers must understand their semantics.
//! ruma-events contains Rust types for each of the event types defined by the specification and
//! facilities for extending the event system for custom event types.
//!
//! # Event types
//!
//! ruma-events includes a Rust enum called `EventType`, which provides a simple enumeration of
//! all the event types defined by the Matrix specification. Matrix event types are serialized to
//! JSON strings in [reverse domain name
//! notation](https://en.wikipedia.org/wiki/Reverse_domain_name_notation), although the core event
//! types all use the special "m" TLD, e.g. *m.room.message*.
//! `EventType` also includes a variant called `Custom`, which is a catch-all that stores a string
//! containing the name of any event type that isn't part of the specification.
//! `EventType` is used throughout ruma-events to identify and differentiate between events of
//! different types.
//!
//! # Event kinds
//!
//! Matrix defines three "kinds" of events:
//!
//! 1.  **Events**, which are arbitrary JSON structures that have two required keys:
//!     *   `type`, which specifies the event's type
//!     *   `content`, which is a JSON object containing the "payload" of the event
//! 2.  **Room events**, which are a superset of events and represent actions that occurred within
//!     the context of a Matrix room.
//!     They have at least the following additional keys:
//!     *   `event_id`, which is a unique identifier for the event
//!     *   `room_id`, which is a unique identifier for the room in which the event occurred
//!     *   `sender`, which is the unique identifier of the Matrix user who created the event
//!     *   Optionally, `unsigned`, which is a JSON object containing arbitrary additional metadata
//!     that is not digitally signed by Matrix homeservers.
//! 3.  **State events**, which are a superset of room events and represent persistent state
//!     specific to a room, such as the room's member list or topic.
//!     Within a single room, state events of the same type and with the same "state key" will
//!     effectively "replace" the previous one, updating the room's state.
//!     They have at least the following additional keys:
//!     *   `state_key`, a string which serves as a sort of "sub-type."
//!         The state key allows a room to persist multiple state events of the same type.
//!         You can think of a room's state events as being a `HashMap` where the keys are the tuple
//!         `(event_type, state_key)`.
//!     *   Optionally, `prev_content`, a JSON object containing the `content` object from the
//!     previous event of the given `(event_type, state_key)` tuple in the given room.
//!
//! ruma-events represents these three event kinds as traits, allowing any Rust type to serve as a
//! Matrix event so long as it upholds the contract expected of its kind.
//!
//! # Core event types
//!
//! ruma-events includes Rust types for every one of the event types in the Matrix specification.
//! To better organize the crate, these types live in separate modules with a hierarchy that
//! matches the reverse domain name notation of the event type.
//! For example, the *m.room.message* event lives at `ruma_events::room::message::MessageEvent`.
//! Each type's module also contains a Rust type for that event type's `content` field, and any
//! other supporting types required by the event's other fields.
//!
//! # Custom event types
//!
//! Although any Rust type that implements `Event`, `RoomEvent`, or `StateEvent` can serve as a
//! Matrix event type, ruma-events also includes a few convenience types for representing events
//! that are not covered by the spec and not otherwise known by the application.
//! `CustomEvent`, `CustomRoomEvent`, and `CustomStateEvent` are simple implementations of their
//! respective event traits whose `content` field is simply a `serde_json::Value` value, which
//! represents arbitrary JSON.
//!
//! # Serialization and deserialization
//!
//! All concrete event types in ruma-events can be serialized via the `Serialize` trait from
//! [serde](https://serde.rs/) and can be deserialized from a `&str` of JSON data via the `FromStr`
//! trait from the standard library. (`TryFrom<&str>` is also implemented and can be used in place
//! of `FromStr` if preferred.) Supporting types for each event generally implement serde's
//! `Deserialize` trait directly. In order to handle incoming data that may not conform to
//! ruma-events's strict definitions of event structures, deserializing from JSON will return an
//! `InvalidEvent` on error. This error covers both invalid JSON data as well as valid JSON that
//! doesn't match the structure expected by ruma-events's event types. In the latter case, the error
//! exposes the deserialized `serde_json::Value` so that developers can still work with the received
//! event data. This also makes it possible to deserialize a collection of events without the entire
//! collection failing to deserialize due to a single invalid event. The "content" type for each
//! event also implements `Serialize` and either `FromStr` (for dedicated content types) or
//! `Deserialize` (when the content is a type alias), allowing content to be converted to and from
//! JSON indepedently of the surrounding event structure, if needed.
//!
//! # Collections
//!
//! With the trait-based approach to events, it's easy to write generic collection types like
//! `Vec<Box<R: RoomEvent>>`.
//! However, there are APIs in the Matrix specification that involve heterogeneous collections of
//! events, i.e. a list of events of different event types.
//! Because Rust does not have a facility for arrays, vectors, or slices containing multiple
//! concrete types, ruma-events provides special collection types for this purpose.
//! The collection types are enums which effectively "wrap" each possible event type of a
//! particular event "kind."
//!
//! Because of the hierarchical nature of event kinds in Matrix, these collection types are divied
//! into two modules, `ruma_events::collections::all` and `ruma_events::collections::only`.
//! The "all" versions include every event type that implements the relevant event trait as well as
//! more specific event traits.
//! The "only" versions include only the event types that implement "at most" the relevant event
//! trait.
//!
//! For example, the `ruma_events::collections::all::Event` enum includes *m.room.message*, because
//! that event type is both an event and a room event.
//! However, the `ruma_events::collections::only::Event` enum does *not* include *m.room.message*,
//! because *m.room.message* implements a *more specific* event trait than `Event`.

#![deny(missing_debug_implementations)]
//#![deny(missing_docs)]
//#![deny(warnings)]

use std::{
    collections::HashMap,
    convert::TryInto,
    error::Error,
    fmt::{Debug, Display, Error as FmtError, Formatter, Result as FmtResult},
    hash::Hash,
};

use js_int::UInt;
use ruma_identifiers::{EventId, RoomId, UserId};
use serde::{
    de::{DeserializeOwned, Error as SerdeError, IntoDeserializer, MapAccess, Visitor},
    ser::SerializeMap,
    Deserialize, Deserializer, Serialize, Serializer,
};
use serde_json::Value;

// pub use custom::CustomEvent;
// pub use custom_room::CustomRoomEvent;
// pub use custom_state::CustomStateEvent;

#[macro_use]
mod macros;

pub mod call;
// /// Enums for heterogeneous collections of events.
// pub mod collections {
//     pub mod all;
//     pub mod only;
// }
pub mod direct;
pub mod dummy;
pub mod forwarded_room_key;
pub mod fully_read;
pub mod ignored_user_list;
pub mod key;
pub mod presence;
pub mod push_rules;
pub mod receipt;
pub mod room;
pub mod room_key;
pub mod room_key_request;
pub mod sticker;
pub mod stripped;
pub mod tag;
pub mod typing;

/// An event that is malformed or otherwise invalid.
///
/// When attempting to create an event from a string of JSON data, an error in the input data may
/// cause deserialization to fail, or the JSON structure may not corresponded to ruma-events's
/// strict definition of the event's schema. If deserialization completely fails, this type will
/// provide a message with details about the deserialization error. If deserialization succeeds but
/// the event is otherwise invalid, a similar message will be provided, as well as a
/// `serde_json::Value` containing the raw JSON data as it was deserialized.
#[derive(Debug)]
pub struct InvalidEvent<T>(InnerInvalidEvent<T>);

impl<T> InvalidEvent<T> {
    /// A message describing why the event is invalid.
    pub fn message(&self) -> String {
        match self.0 {
            InnerInvalidEvent::Deserialization { ref error, .. } => error.to_string(),
            InnerInvalidEvent::Validation { ref message, .. } => message.to_string(),
        }
    }

    /// The raw event data, if deserialization succeeded but validation failed.
    pub fn raw_data(&self) -> Option<&T> {
        match self.0 {
            InnerInvalidEvent::Validation { ref raw_data, .. } => Some(raw_data),
            _ => None,
        }
    }

    /// The `serde_json::Value` representation of the invalid event, if deserialization failed.
    pub fn json(&self) -> Option<&Value> {
        match self.0 {
            InnerInvalidEvent::Deserialization { ref json, .. } => Some(json),
            _ => None,
        }
    }
}

impl<T> Display for InvalidEvent<T> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl<T: Debug> Error for InvalidEvent<T> {}

/// An event that is malformed or otherwise invalid.
#[derive(Debug)]
enum InnerInvalidEvent<T> {
    /// An event that failed to deserialize from JSON.
    Deserialization {
        /// The raw `serde_json::Value` representation of the invalid event.
        json: Value,

        /// The deserialization error returned by serde.
        error: serde_json::Error,
    },

    /// An event that deserialized but failed validation.
    Validation {
        /// The event data that failed validation.
        raw_data: T,

        /// An message describing why the event was invalid.
        message: String,
    },
}

/// An error returned when attempting to create an event with data that would make it invalid.
///
/// This type is similar to `InvalidEvent`, but used during the construction of a new event, as
/// opposed to deserialization of an existing event from JSON.
#[derive(Clone, Debug, PartialEq)]
pub struct InvalidInput(String);

impl Display for InvalidInput {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.0)
    }
}

impl Error for InvalidInput {}

pub trait EventResultCompatible {
    /// Whether this type needs to be validated after deserialization. (type-level boolean)
    ///
    /// Only true for actual event types
    type NeedsValidation;

    /// The 'raw', non-validated form of this event.
    ///
    /// Always `Void` if `NeedsValidation` = `False`.
    type Raw;

    /// The 'raw', non-validated form of this event's content.
    ///
    /// Always `Void` if `NeedsValidation` = `False`.
    type RawContent; // TODO: Required??
}

/// An empty type. Used for `Raw` and `RawContent` in `Event` when validation cannot fail.
#[derive(Debug)]
pub enum Void {}

/// Type equivalent to `true`
#[derive(Debug)]
pub enum True {}

/// Type equivalent to `false`
#[derive(Debug)]
pub enum False {}

impl EventResultCompatible for Empty {
    type NeedsValidation = False;
    type Raw = Void;
    type RawContent = Void;
}

impl<K, V> EventResultCompatible for HashMap<K, V> {
    type NeedsValidation = False;
    type Raw = Void;
    type RawContent = Void;
}

/// The result of deserializing an event, which may or may not be valid.
///
/// When data is successfully deserialized and validated, this structure will contain the
/// deserialized value `T`. When deserialization succeeds, but the event is invalid for any reason,
/// this structure will contain an `InvalidEvent`. See the documentation for `InvalidEvent` for
/// more details.
#[derive(Debug)]
pub enum EventResult<T: EventResultCompatible> {
    /// `T` deserialized and validated successfully.
    Ok(T),

    /// `T` failed either deserialization or validation.
    ///
    /// `InvalidEvent` contains the error message and the raw data.
    Err(InvalidEvent<T::Raw>),
}

impl<T: EventResultCompatible> EventResult<T> {
    /// Convert `EventResult<T>` into the equivalent `std::result::Result<T, InvalidEvent>`.
    pub fn into_result(self) -> Result<T, InvalidEvent<T::Raw>> {
        match self {
            EventResult::Ok(t) => Ok(t),
            EventResult::Err(invalid_event) => Err(invalid_event),
        }
    }
}

/// Workaround for `T: EventResultDeserialize<NeedsValidation = True>` and
/// `T: EventResultDeserialize<NeedsValidation = False>` being considered
/// potentially overlapping type ranges, even though they're clearly not.
pub trait EventResultDeserialize: Sized {
    type Inner: EventResultCompatible;

    fn inner_deserialize(json: serde_json::Value) -> EventResult<Self::Inner>;
}

impl<'de, T> Deserialize<'de> for EventResult<T>
where
    T: EventResultCompatible,
    (Self, T::NeedsValidation): EventResultDeserialize<Inner = T>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let json = serde_json::Value::deserialize(deserializer)?;
        Ok(<(Self, T::NeedsValidation)>::inner_deserialize(json))
    }
}

impl<T> EventResultDeserialize for (EventResult<T>, True)
where
    T: Event<NeedsValidation = True>,
    T::Raw: DeserializeOwned + TryInto<T, Error = (T::Raw, String)>,
{
    type Inner = T;

    fn inner_deserialize(json: serde_json::Value) -> EventResult<T> {
        let raw_data: T::Raw = match serde_json::from_value(json.clone()) {
            Ok(raw) => raw,
            Err(error) => {
                return EventResult::Err(InvalidEvent(InnerInvalidEvent::Deserialization {
                    json,
                    error,
                }));
            }
        };

        match raw_data.try_into() {
            Ok(value) => EventResult::Ok(value),
            Err((raw_data, message)) => {
                EventResult::Err(InvalidEvent(InnerInvalidEvent::Validation {
                    message,
                    raw_data,
                }))
            }
        }
    }
}

impl<T> EventResultDeserialize for (EventResult<T>, False)
where
    T: EventResultCompatible<NeedsValidation = False> + DeserializeOwned,
{
    type Inner = T;

    fn inner_deserialize(json: serde_json::Value) -> EventResult<T> {
        match serde_json::from_value(json.clone()) {
            Ok(value) => EventResult::Ok(value),
            Err(error) => EventResult::Err(InvalidEvent(InnerInvalidEvent::Deserialization {
                json,
                error,
            })),
        }
    }
}

/// An error when attempting to create a value from a string via the `FromStr` trait.
///
/// This error type is only used for simple enums with unit variants. Event deserialization through
/// the `FromStr` trait returns an `InvalidEvent` on error.
#[derive(Clone, Copy, Eq, Debug, Hash, PartialEq)]
pub struct FromStrError;

impl Display for FromStrError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "failed to parse type from string")
    }
}

impl Error for FromStrError {}

/// A meaningless value that serializes to an empty JSON object.
///
/// This type is used in a few places where the Matrix specification requires an empty JSON object,
/// but it's wasteful to represent it as a `HashMap` in Rust code.
#[derive(Clone, Debug, PartialEq)]
pub struct Empty;

impl Serialize for Empty {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_map(Some(0))?.end()
    }
}

impl<'de> Deserialize<'de> for Empty {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EmptyMapVisitor;

        impl<'de> Visitor<'de> for EmptyMapVisitor {
            type Value = Empty;

            fn expecting(&self, f: &mut Formatter) -> FmtResult {
                write!(f, "an object/map")
            }

            fn visit_map<A>(self, _map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                Ok(Empty)
            }
        }

        deserializer.deserialize_map(EmptyMapVisitor)
    }
}

/// The type of an event.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum EventType {
    /// m.call.answer
    CallAnswer,

    /// m.call.candidates
    CallCandidates,

    /// m.call.hangup
    CallHangup,

    /// m.call.invite
    CallInvite,

    /// m.direct
    Direct,

    /// m.dummy
    Dummy,

    /// m.forwarded_room_key
    ForwardedRoomKey,

    /// m.fully_read
    FullyRead,

    /// m.key.verification.accept
    KeyVerificationAccept,

    /// m.key.verification.cancel
    KeyVerificationCancel,

    /// m.key.verification.key
    KeyVerificationKey,

    /// m.key.verification.mac
    KeyVerificationMac,

    /// m.key.verification.request
    KeyVerificationRequest,

    /// m.key.verification.start
    KeyVerificationStart,

    /// m.ignored_user_list
    IgnoredUserList,

    /// m.presence
    Presence,

    /// m.push_rules
    PushRules,

    /// m.receipt
    Receipt,

    /// m.room.aliases
    RoomAliases,

    /// m.room.avatar
    RoomAvatar,

    /// m.room.canonical_alias
    RoomCanonicalAlias,

    /// m.room.create
    RoomCreate,

    /// m.room.encrypted
    RoomEncrypted,

    /// m.room.encryption
    RoomEncryption,

    /// m.room.guest_access
    RoomGuestAccess,

    /// m.room.history_visibility
    RoomHistoryVisibility,

    /// m.room.join_rules
    RoomJoinRules,

    /// m.room.member
    RoomMember,

    /// m.room.message
    RoomMessage,

    /// m.room.message.feedback
    RoomMessageFeedback,

    /// m.room.name
    RoomName,

    /// m.room.pinned_events
    RoomPinnedEvents,

    /// m.room.power_levels
    RoomPowerLevels,

    /// m.room.redaction
    RoomRedaction,

    /// m.room.server_acl
    RoomServerAcl,

    /// m.room.third_party_invite
    RoomThirdPartyInvite,

    /// m.room.tombstone
    RoomTombstone,

    /// m.room.topic
    RoomTopic,

    /// m.room_key
    RoomKey,

    /// m.room_key_request
    RoomKeyRequest,

    /// m.sticker
    Sticker,

    /// m.tag
    Tag,

    /// m.typing
    Typing,

    /// Any event that is not part of the specification.
    Custom(String),

    /// Additional variants may be added in the future and will not be considered breaking changes
    /// to ruma-events.
    #[doc(hidden)]
    __Nonexhaustive,
}

/// A basic event.
pub trait Event: Debug + Serialize + Sized + EventResultCompatible {
    /// The type of this event's `content` field.
    type Content: Debug + Serialize;

    /// The event's content.
    fn content(&self) -> &Self::Content;

    /// The type of the event.
    fn event_type(&self) -> EventType;
}

/// An event within the context of a room.
pub trait RoomEvent: Event {
    /// The unique identifier for the event.
    fn event_id(&self) -> &EventId;

    /// Timestamp (milliseconds since the UNIX epoch) on originating homeserver when this event was
    /// sent.
    fn origin_server_ts(&self) -> UInt;

    /// The unique identifier for the room associated with this event.
    ///
    /// This can be `None` if the event came from a context where there is
    /// no ambiguity which room it belongs to, like a `/sync` response for example.
    fn room_id(&self) -> Option<&RoomId>;

    /// The unique identifier for the user who sent this event.
    fn sender(&self) -> &UserId;

    /// Additional key-value pairs not signed by the homeserver.
    fn unsigned(&self) -> Option<&Value>;
}

/// An event that describes persistent state about a room.
pub trait StateEvent: RoomEvent {
    /// The previous content for this state key, if any.
    fn prev_content(&self) -> Option<&Self::Content>;

    /// A key that determines which piece of room state the event represents.
    fn state_key(&self) -> &str;
}

// mod custom {
//     use ruma_events_macros::ruma_event;
//     use serde_json::Value;

//     ruma_event! {
//         /// A custom basic event not covered by the Matrix specification.
//         CustomEvent {
//             kind: Event,
//             event_type: Custom,
//             content_type_alias: {
//                 /// The payload for `CustomEvent`.
//                 Value
//             },
//         }
//     }
// }

// mod custom_room {
//     use ruma_events_macros::ruma_event;
//     use serde_json::Value;

//     ruma_event! {
//         /// A custom room event not covered by the Matrix specification.
//         CustomRoomEvent {
//             kind: RoomEvent,
//             event_type: Custom,
//             content_type_alias: {
//                 /// The payload for `CustomRoomEvent`.
//                 Value
//             },
//         }
//     }
// }

// mod custom_state {
//     use ruma_events_macros::ruma_event;
//     use serde_json::Value;

//     ruma_event! {
//         /// A custom state event not covered by the Matrix specification.
//         CustomStateEvent {
//             kind: StateEvent,
//             event_type: Custom,
//             content_type_alias: {
//                 /// The payload for `CustomStateEvent`.
//                 Value
//             },
//         }
//     }
// }

impl Display for EventType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        let event_type_str = match *self {
            EventType::CallAnswer => "m.call.answer",
            EventType::CallCandidates => "m.call.candidates",
            EventType::CallHangup => "m.call.hangup",
            EventType::CallInvite => "m.call.invite",
            EventType::Direct => "m.direct",
            EventType::Dummy => "m.dummy",
            EventType::ForwardedRoomKey => "m.forwarded_room_key",
            EventType::FullyRead => "m.fully_read",
            EventType::KeyVerificationAccept => "m.key.verification.accept",
            EventType::KeyVerificationCancel => "m.key.verification.cancel",
            EventType::KeyVerificationKey => "m.key.verification.key",
            EventType::KeyVerificationMac => "m.key.verification.mac",
            EventType::KeyVerificationRequest => "m.key.verification.request",
            EventType::KeyVerificationStart => "m.key.verification.start",
            EventType::IgnoredUserList => "m.ignored_user_list",
            EventType::Presence => "m.presence",
            EventType::PushRules => "m.push_rules",
            EventType::Receipt => "m.receipt",
            EventType::RoomAliases => "m.room.aliases",
            EventType::RoomAvatar => "m.room.avatar",
            EventType::RoomCanonicalAlias => "m.room.canonical_alias",
            EventType::RoomCreate => "m.room.create",
            EventType::RoomEncrypted => "m.room.encrypted",
            EventType::RoomEncryption => "m.room.encryption",
            EventType::RoomGuestAccess => "m.room.guest_access",
            EventType::RoomHistoryVisibility => "m.room.history_visibility",
            EventType::RoomJoinRules => "m.room.join_rules",
            EventType::RoomMember => "m.room.member",
            EventType::RoomMessage => "m.room.message",
            EventType::RoomMessageFeedback => "m.room.message.feedback",
            EventType::RoomName => "m.room.name",
            EventType::RoomPinnedEvents => "m.room.pinned_events",
            EventType::RoomPowerLevels => "m.room.power_levels",
            EventType::RoomRedaction => "m.room.redaction",
            EventType::RoomServerAcl => "m.room.server_acl",
            EventType::RoomThirdPartyInvite => "m.room.third_party_invite",
            EventType::RoomTombstone => "m.room.tombstone",
            EventType::RoomTopic => "m.room.topic",
            EventType::RoomKey => "m.room_key",
            EventType::RoomKeyRequest => "m.room_key_request",
            EventType::Sticker => "m.sticker",
            EventType::Tag => "m.tag",
            EventType::Typing => "m.typing",
            EventType::Custom(ref event_type) => event_type,
            EventType::__Nonexhaustive => {
                panic!("__Nonexhaustive enum variant is not intended for use.")
            }
        };

        write!(f, "{}", event_type_str)
    }
}

impl<'a> From<&'a str> for EventType {
    fn from(s: &'a str) -> EventType {
        match s {
            "m.call.answer" => EventType::CallAnswer,
            "m.call.candidates" => EventType::CallCandidates,
            "m.call.hangup" => EventType::CallHangup,
            "m.call.invite" => EventType::CallInvite,
            "m.direct" => EventType::Direct,
            "m.dummy" => EventType::Dummy,
            "m.forwarded_room_key" => EventType::ForwardedRoomKey,
            "m.fully_read" => EventType::FullyRead,
            "m.key.verification.accept" => EventType::KeyVerificationAccept,
            "m.key.verification.cancel" => EventType::KeyVerificationCancel,
            "m.key.verification.key" => EventType::KeyVerificationKey,
            "m.key.verification.mac" => EventType::KeyVerificationMac,
            "m.key.verification.request" => EventType::KeyVerificationRequest,
            "m.key.verification.start" => EventType::KeyVerificationStart,
            "m.ignored_user_list" => EventType::IgnoredUserList,
            "m.presence" => EventType::Presence,
            "m.push_rules" => EventType::PushRules,
            "m.receipt" => EventType::Receipt,
            "m.room.aliases" => EventType::RoomAliases,
            "m.room.avatar" => EventType::RoomAvatar,
            "m.room.canonical_alias" => EventType::RoomCanonicalAlias,
            "m.room.create" => EventType::RoomCreate,
            "m.room.encrypted" => EventType::RoomEncrypted,
            "m.room.encryption" => EventType::RoomEncryption,
            "m.room.guest_access" => EventType::RoomGuestAccess,
            "m.room.history_visibility" => EventType::RoomHistoryVisibility,
            "m.room.join_rules" => EventType::RoomJoinRules,
            "m.room.member" => EventType::RoomMember,
            "m.room.message" => EventType::RoomMessage,
            "m.room.message.feedback" => EventType::RoomMessageFeedback,
            "m.room.name" => EventType::RoomName,
            "m.room.pinned_events" => EventType::RoomPinnedEvents,
            "m.room.power_levels" => EventType::RoomPowerLevels,
            "m.room.redaction" => EventType::RoomRedaction,
            "m.room.server_acl" => EventType::RoomServerAcl,
            "m.room.third_party_invite" => EventType::RoomThirdPartyInvite,
            "m.room.tombstone" => EventType::RoomTombstone,
            "m.room.topic" => EventType::RoomTopic,
            "m.room_key" => EventType::RoomKey,
            "m.room_key_request" => EventType::RoomKeyRequest,
            "m.sticker" => EventType::Sticker,
            "m.tag" => EventType::Tag,
            "m.typing" => EventType::Typing,
            event_type => EventType::Custom(event_type.to_string()),
        }
    }
}

impl Serialize for EventType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for EventType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EventTypeVisitor;

        impl<'de> Visitor<'de> for EventTypeVisitor {
            type Value = EventType;

            fn expecting(&self, formatter: &mut Formatter<'_>) -> FmtResult {
                write!(formatter, "a Matrix event type as a string")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: SerdeError,
            {
                Ok(EventType::from(v))
            }
        }

        deserializer.deserialize_str(EventTypeVisitor)
    }
}

/// An encryption algorithm to be used to encrypt messages sent to a room.
#[derive(Clone, Debug, PartialEq)]
pub enum Algorithm {
    /// Olm version 1 using Curve25519, AES-256, and SHA-256.
    OlmV1Curve25519AesSha2,

    /// Megolm version 1 using AES-256 and SHA-256.
    MegolmV1AesSha2,

    /// Any algorithm that is not part of the specification.
    Custom(String),

    /// Additional variants may be added in the future and will not be considered breaking changes
    /// to `ruma-events`.
    #[doc(hidden)]
    __Nonexhaustive,
}

impl Display for Algorithm {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let algorithm_str = match *self {
            Algorithm::OlmV1Curve25519AesSha2 => "m.olm.v1.curve25519-aes-sha2",
            Algorithm::MegolmV1AesSha2 => "m.megolm.v1.aes-sha2",
            Algorithm::Custom(ref algorithm) => algorithm,
            Algorithm::__Nonexhaustive => {
                panic!("__Nonexhaustive enum variant is not intended for use.")
            }
        };

        write!(f, "{}", algorithm_str)
    }
}

impl<'a> From<&'a str> for Algorithm {
    fn from(s: &'a str) -> Algorithm {
        match s {
            "m.olm.v1.curve25519-aes-sha2" => Algorithm::OlmV1Curve25519AesSha2,
            "m.megolm.v1.aes-sha2" => Algorithm::MegolmV1AesSha2,
            algorithm => Algorithm::Custom(algorithm.to_string()),
        }
    }
}

impl Serialize for Algorithm {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Algorithm {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CancelCodeVisitor;

        impl<'de> Visitor<'de> for CancelCodeVisitor {
            type Value = Algorithm;

            fn expecting(&self, formatter: &mut Formatter<'_>) -> FmtResult {
                write!(formatter, "an encryption algorithm code as a string")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: SerdeError,
            {
                Ok(Algorithm::from(v))
            }
        }

        deserializer.deserialize_str(CancelCodeVisitor)
    }
}

/// Serde deserialization decorator to map empty Strings to None,
/// and forward non-empty Strings to the Deserialize implementation for T.
/// Useful for the typical
/// "A room with an X event with an absent, null, or empty Y field
/// should be treated the same as a room with no such event."
/// formulation in the spec.
///
/// To be used like this:
/// `#[serde(deserialize_with = "empty_string_as_none"]`
/// Relevant serde issue: https://github.com/serde-rs/serde/issues/1425
fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de>,
{
    let opt = Option::<String>::deserialize(de)?;
    let opt = opt.as_ref().map(String::as_str);
    match opt {
        None | Some("") => Ok(None),
        // If T = String, like in m.room.name, the second deserialize is actually superfluous.
        // TODO: optimize that somehow?
        Some(s) => T::deserialize(s.into_deserializer()).map(Some),
    }
}

/// Used to default the `bool` fields to `true` during deserialization.
fn default_true() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use serde_json::{from_str, to_string};

    use super::{Algorithm, EventType};

    #[test]
    fn event_types_serialize_to_display_form() {
        assert_eq!(
            to_string(&EventType::RoomCreate).unwrap(),
            r#""m.room.create""#
        );
    }

    #[test]
    fn custom_event_types_serialize_to_display_form() {
        assert_eq!(
            to_string(&EventType::Custom("io.ruma.test".to_string())).unwrap(),
            r#""io.ruma.test""#
        );
    }

    #[test]
    fn event_types_deserialize_from_display_form() {
        assert_eq!(
            from_str::<EventType>(r#""m.room.create""#).unwrap(),
            EventType::RoomCreate
        );
    }

    #[test]
    fn custom_event_types_deserialize_from_display_form() {
        assert_eq!(
            from_str::<EventType>(r#""io.ruma.test""#).unwrap(),
            EventType::Custom("io.ruma.test".to_string())
        )
    }

    #[test]
    fn algorithms_serialize_to_display_form() {
        assert_eq!(
            to_string(&Algorithm::MegolmV1AesSha2).unwrap(),
            r#""m.megolm.v1.aes-sha2""#
        );
    }

    #[test]
    fn custom_algorithms_serialize_to_display_form() {
        assert_eq!(
            to_string(&Algorithm::Custom("io.ruma.test".to_string())).unwrap(),
            r#""io.ruma.test""#
        );
    }

    #[test]
    fn algorithms_deserialize_from_display_form() {
        assert_eq!(
            from_str::<Algorithm>(r#""m.megolm.v1.aes-sha2""#).unwrap(),
            Algorithm::MegolmV1AesSha2
        );
    }

    #[test]
    fn custom_algorithms_deserialize_from_display_form() {
        assert_eq!(
            from_str::<Algorithm>(r#""io.ruma.test""#).unwrap(),
            Algorithm::Custom("io.ruma.test".to_string())
        )
    }
}
