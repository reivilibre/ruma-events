//! Enums for heterogeneous collections of events, inclusive for every event type that implements
//! the trait of the same name.

use std::str::FromStr;

use serde::{Serialize, Serializer};
use serde_json::{from_value, Value};

use crate::{
    call::{
        answer::AnswerEvent, candidates::CandidatesEvent, hangup::HangupEvent, invite::InviteEvent,
    },
    direct::DirectEvent,
    dummy::DummyEvent,
    forwarded_room_key::ForwardedRoomKeyEvent,
    fully_read::FullyReadEvent,
    ignored_user_list::IgnoredUserListEvent,
    key::verification::{
        accept::AcceptEvent, cancel::CancelEvent, key::KeyEvent, mac::MacEvent,
        request::RequestEvent, start::StartEvent,
    },
    presence::PresenceEvent,
    push_rules::PushRulesEvent,
    receipt::ReceiptEvent,
    room::{
        aliases::AliasesEvent,
        avatar::AvatarEvent,
        canonical_alias::CanonicalAliasEvent,
        create::CreateEvent,
        encrypted::EncryptedEvent,
        encryption::EncryptionEvent,
        guest_access::GuestAccessEvent,
        history_visibility::HistoryVisibilityEvent,
        join_rules::JoinRulesEvent,
        member::MemberEvent,
        message::{feedback::FeedbackEvent, MessageEvent},
        name::NameEvent,
        pinned_events::PinnedEventsEvent,
        power_levels::PowerLevelsEvent,
        redaction::RedactionEvent,
        server_acl::ServerAclEvent,
        third_party_invite::ThirdPartyInviteEvent,
        tombstone::TombstoneEvent,
        topic::TopicEvent,
    },
    room_key::RoomKeyEvent,
    room_key_request::RoomKeyRequestEvent,
    sticker::StickerEvent,
    tag::TagEvent,
    typing::TypingEvent,
    CustomEvent, CustomRoomEvent, CustomStateEvent, EventType, InnerInvalidEvent, InvalidEvent,
};
use js_int::UInt;
use ruma_identifiers::{EventId, RoomId, UserId};

/// A basic event, room event, or state event.
#[derive(Clone, Debug)]
#[allow(clippy::large_enum_variant)]
pub enum Event {
    /// m.call.answer
    CallAnswer(AnswerEvent),

    /// m.call.candidates
    CallCandidates(CandidatesEvent),

    /// m.call.hangup
    CallHangup(HangupEvent),

    /// m.call.invite
    CallInvite(InviteEvent),

    /// m.direct
    Direct(DirectEvent),

    /// m.dummy
    Dummy(DummyEvent),

    /// m.forwarded_room_key
    ForwardedRoomKey(ForwardedRoomKeyEvent),

    /// m.fully_read
    FullyRead(FullyReadEvent),

    /// m.ignored_user_list
    IgnoredUserList(IgnoredUserListEvent),

    /// m.key.verification.accept
    KeyVerificationAccept(AcceptEvent),

    /// m.key.verification.cancel
    KeyVerificationCancel(CancelEvent),

    /// m.key.verification.key
    KeyVerificationKey(KeyEvent),

    /// m.key.verification.mac
    KeyVerificationMac(MacEvent),

    /// m.key.verification.request
    KeyVerificationRequest(RequestEvent),

    /// m.key.verification.start
    KeyVerificationStart(StartEvent),

    /// m.presence
    Presence(PresenceEvent),

    /// m.push_rules
    PushRules(PushRulesEvent),

    /// m.receipt
    Receipt(ReceiptEvent),

    /// m.room.aliases
    RoomAliases(AliasesEvent),

    /// m.room.avatar
    RoomAvatar(AvatarEvent),

    /// m.room.canonical_alias
    RoomCanonicalAlias(CanonicalAliasEvent),

    /// m.room.create
    RoomCreate(CreateEvent),

    /// m.room.encrypted
    RoomEncrypted(EncryptedEvent),

    /// m.room.encryption
    RoomEncryption(EncryptionEvent),

    /// m.room.guest_access
    RoomGuestAccess(GuestAccessEvent),

    /// m.room.history_visibility
    RoomHistoryVisibility(HistoryVisibilityEvent),

    /// m.room.join_rules
    RoomJoinRules(JoinRulesEvent),

    /// m.room.member
    RoomMember(MemberEvent),

    /// m.room.message
    RoomMessage(MessageEvent),

    /// m.room.message.feedback
    RoomMessageFeedback(FeedbackEvent),

    /// m.room.name
    RoomName(NameEvent),

    /// m.room.pinned_events
    RoomPinnedEvents(PinnedEventsEvent),

    /// m.room.power_levels
    RoomPowerLevels(PowerLevelsEvent),

    /// m.room.redaction
    RoomRedaction(RedactionEvent),

    /// m.room.server_acl,
    RoomServerAcl(ServerAclEvent),

    /// m.room.third_party_invite
    RoomThirdPartyInvite(ThirdPartyInviteEvent),

    /// m.room.tombstone
    RoomTombstone(TombstoneEvent),

    /// m.room.topic
    RoomTopic(TopicEvent),

    /// m.room_key
    RoomKey(RoomKeyEvent),

    /// m.room_key_request
    RoomKeyRequest(RoomKeyRequestEvent),

    /// m.sticker
    Sticker(StickerEvent),

    /// m.tag
    Tag(TagEvent),

    /// m.typing
    Typing(TypingEvent),

    /// Any basic event that is not part of the specification.
    Custom(CustomEvent),

    /// Any room event that is not part of the specification.
    CustomRoom(CustomRoomEvent),

    /// Any state event that is not part of the specification.
    CustomState(CustomStateEvent),
}

/// A room event or state event.
#[derive(Clone, Debug)]
#[allow(clippy::large_enum_variant)]
pub enum RoomEvent {
    /// m.call.answer
    CallAnswer(AnswerEvent),

    /// m.call.candidates
    CallCandidates(CandidatesEvent),

    /// m.call.hangup
    CallHangup(HangupEvent),

    /// m.call.invite
    CallInvite(InviteEvent),

    /// m.room.aliases
    RoomAliases(AliasesEvent),

    /// m.room.avatar
    RoomAvatar(AvatarEvent),

    /// m.room.canonical_alias
    RoomCanonicalAlias(CanonicalAliasEvent),

    /// m.room.create
    RoomCreate(CreateEvent),

    /// m.room.encrypted
    RoomEncrypted(EncryptedEvent),

    /// m.room.encryption
    RoomEncryption(EncryptionEvent),

    /// m.room.guest_access
    RoomGuestAccess(GuestAccessEvent),

    /// m.room.history_visibility
    RoomHistoryVisibility(HistoryVisibilityEvent),

    /// m.room.join_rules
    RoomJoinRules(JoinRulesEvent),

    /// m.room.member
    RoomMember(MemberEvent),

    /// m.room.message
    RoomMessage(MessageEvent),

    /// m.room.message.feedback
    RoomMessageFeedback(FeedbackEvent),

    /// m.room.name
    RoomName(NameEvent),

    /// m.room.pinned_events
    RoomPinnedEvents(PinnedEventsEvent),

    /// m.room.power_levels
    RoomPowerLevels(PowerLevelsEvent),

    /// m.room.redaction
    RoomRedaction(RedactionEvent),

    /// m.room.server_acl,
    RoomServerAcl(ServerAclEvent),

    /// m.room.third_party_invite
    RoomThirdPartyInvite(ThirdPartyInviteEvent),

    /// m.room.tombstone
    RoomTombstone(TombstoneEvent),

    /// m.room.topic
    RoomTopic(TopicEvent),

    /// m.sticker
    Sticker(StickerEvent),

    /// Any room event that is not part of the specification.
    CustomRoom(CustomRoomEvent),

    /// Any state event that is not part of the specification.
    CustomState(CustomStateEvent),
}

/// A state event.
#[derive(Clone, Debug)]
#[allow(clippy::large_enum_variant)]
pub enum StateEvent {
    /// m.room.aliases
    RoomAliases(AliasesEvent),

    /// m.room.avatar
    RoomAvatar(AvatarEvent),

    /// m.room.canonical_alias
    RoomCanonicalAlias(CanonicalAliasEvent),

    /// m.room.create
    RoomCreate(CreateEvent),

    /// m.room.encryption
    RoomEncryption(EncryptionEvent),

    /// m.room.guest_access
    RoomGuestAccess(GuestAccessEvent),

    /// m.room.history_visibility
    RoomHistoryVisibility(HistoryVisibilityEvent),

    /// m.room.join_rules
    RoomJoinRules(JoinRulesEvent),

    /// m.room.member
    RoomMember(MemberEvent),

    /// m.room.name
    RoomName(NameEvent),

    /// m.room.pinned_events
    RoomPinnedEvents(PinnedEventsEvent),

    /// m.room.power_levels
    RoomPowerLevels(PowerLevelsEvent),

    /// m.room.server_acl,
    RoomServerAcl(ServerAclEvent),

    /// m.room.third_party_invite
    RoomThirdPartyInvite(ThirdPartyInviteEvent),

    /// m.room.tombstone
    RoomTombstone(TombstoneEvent),

    /// m.room.topic
    RoomTopic(TopicEvent),

    /// Any state event that is not part of the specification.
    CustomState(CustomStateEvent),
}

impl Serialize for Event {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Event::CallAnswer(ref event) => event.serialize(serializer),
            Event::CallCandidates(ref event) => event.serialize(serializer),
            Event::CallHangup(ref event) => event.serialize(serializer),
            Event::CallInvite(ref event) => event.serialize(serializer),
            Event::Direct(ref event) => event.serialize(serializer),
            Event::Dummy(ref event) => event.serialize(serializer),
            Event::ForwardedRoomKey(ref event) => event.serialize(serializer),
            Event::FullyRead(ref event) => event.serialize(serializer),
            Event::KeyVerificationAccept(ref event) => event.serialize(serializer),
            Event::KeyVerificationCancel(ref event) => event.serialize(serializer),
            Event::KeyVerificationKey(ref event) => event.serialize(serializer),
            Event::KeyVerificationMac(ref event) => event.serialize(serializer),
            Event::KeyVerificationRequest(ref event) => event.serialize(serializer),
            Event::KeyVerificationStart(ref event) => event.serialize(serializer),
            Event::IgnoredUserList(ref event) => event.serialize(serializer),
            Event::Presence(ref event) => event.serialize(serializer),
            Event::PushRules(ref event) => event.serialize(serializer),
            Event::Receipt(ref event) => event.serialize(serializer),
            Event::RoomAliases(ref event) => event.serialize(serializer),
            Event::RoomAvatar(ref event) => event.serialize(serializer),
            Event::RoomCanonicalAlias(ref event) => event.serialize(serializer),
            Event::RoomCreate(ref event) => event.serialize(serializer),
            Event::RoomEncrypted(ref event) => event.serialize(serializer),
            Event::RoomEncryption(ref event) => event.serialize(serializer),
            Event::RoomGuestAccess(ref event) => event.serialize(serializer),
            Event::RoomHistoryVisibility(ref event) => event.serialize(serializer),
            Event::RoomJoinRules(ref event) => event.serialize(serializer),
            Event::RoomMember(ref event) => event.serialize(serializer),
            Event::RoomMessage(ref event) => event.serialize(serializer),
            Event::RoomMessageFeedback(ref event) => event.serialize(serializer),
            Event::RoomName(ref event) => event.serialize(serializer),
            Event::RoomPinnedEvents(ref event) => event.serialize(serializer),
            Event::RoomPowerLevels(ref event) => event.serialize(serializer),
            Event::RoomRedaction(ref event) => event.serialize(serializer),
            Event::RoomServerAcl(ref event) => event.serialize(serializer),
            Event::RoomThirdPartyInvite(ref event) => event.serialize(serializer),
            Event::RoomTombstone(ref event) => event.serialize(serializer),
            Event::RoomTopic(ref event) => event.serialize(serializer),
            Event::RoomKey(ref event) => event.serialize(serializer),
            Event::RoomKeyRequest(ref event) => event.serialize(serializer),
            Event::Sticker(ref event) => event.serialize(serializer),
            Event::Tag(ref event) => event.serialize(serializer),
            Event::Typing(ref event) => event.serialize(serializer),
            Event::Custom(ref event) => event.serialize(serializer),
            Event::CustomRoom(ref event) => event.serialize(serializer),
            Event::CustomState(ref event) => event.serialize(serializer),
        }
    }
}

impl FromStr for Event {
    type Err = InvalidEvent;

    /// Attempt to create `Self` from parsing a string of JSON data.
    #[allow(clippy::cognitive_complexity)]
    fn from_str(json: &str) -> Result<Self, Self::Err> {
        let value: Value = serde_json::from_str(json)?;

        let event_type_value = match value.get("type") {
            Some(value) => value.clone(),
            None => {
                return Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: "missing field `type`".to_string(),
                }))
            }
        };

        let event_type = match from_value::<EventType>(event_type_value.clone()) {
            Ok(event_type) => event_type,
            Err(error) => {
                return Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                }))
            }
        };

        match event_type {
            EventType::CallAnswer => match json.parse() {
                Ok(event) => Ok(Event::CallAnswer(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::CallCandidates => match json.parse() {
                Ok(event) => Ok(Event::CallCandidates(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::CallHangup => match json.parse() {
                Ok(event) => Ok(Event::CallHangup(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::CallInvite => match json.parse() {
                Ok(event) => Ok(Event::CallInvite(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::Direct => match json.parse() {
                Ok(event) => Ok(Event::Direct(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::Dummy => match json.parse() {
                Ok(event) => Ok(Event::Dummy(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::ForwardedRoomKey => match json.parse() {
                Ok(event) => Ok(Event::ForwardedRoomKey(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::FullyRead => match json.parse() {
                Ok(event) => Ok(Event::FullyRead(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::KeyVerificationAccept => match json.parse() {
                Ok(event) => Ok(Event::KeyVerificationAccept(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::KeyVerificationCancel => match json.parse() {
                Ok(event) => Ok(Event::KeyVerificationCancel(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::KeyVerificationKey => match json.parse() {
                Ok(event) => Ok(Event::KeyVerificationKey(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::KeyVerificationMac => match json.parse() {
                Ok(event) => Ok(Event::KeyVerificationMac(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::KeyVerificationRequest => match json.parse() {
                Ok(event) => Ok(Event::KeyVerificationRequest(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::KeyVerificationStart => match json.parse() {
                Ok(event) => Ok(Event::KeyVerificationStart(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::IgnoredUserList => match json.parse() {
                Ok(event) => Ok(Event::IgnoredUserList(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::Presence => match json.parse() {
                Ok(event) => Ok(Event::Presence(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::PushRules => match json.parse() {
                Ok(event) => Ok(Event::PushRules(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::Receipt => match json.parse() {
                Ok(event) => Ok(Event::Receipt(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::RoomAliases => match json.parse() {
                Ok(event) => Ok(Event::RoomAliases(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::RoomAvatar => match json.parse() {
                Ok(event) => Ok(Event::RoomAvatar(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::RoomCanonicalAlias => match json.parse() {
                Ok(event) => Ok(Event::RoomCanonicalAlias(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::RoomCreate => match json.parse() {
                Ok(event) => Ok(Event::RoomCreate(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::RoomEncrypted => match json.parse() {
                Ok(event) => Ok(Event::RoomEncrypted(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::RoomEncryption => match json.parse() {
                Ok(event) => Ok(Event::RoomEncryption(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::RoomGuestAccess => match json.parse() {
                Ok(event) => Ok(Event::RoomGuestAccess(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::RoomHistoryVisibility => match json.parse() {
                Ok(event) => Ok(Event::RoomHistoryVisibility(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::RoomJoinRules => match json.parse() {
                Ok(event) => Ok(Event::RoomJoinRules(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::RoomMember => match json.parse() {
                Ok(event) => Ok(Event::RoomMember(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::RoomMessage => match json.parse() {
                Ok(event) => Ok(Event::RoomMessage(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::RoomMessageFeedback => match json.parse() {
                Ok(event) => Ok(Event::RoomMessageFeedback(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::RoomName => match json.parse() {
                Ok(event) => Ok(Event::RoomName(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::RoomPinnedEvents => match json.parse() {
                Ok(event) => Ok(Event::RoomPinnedEvents(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::RoomPowerLevels => match json.parse() {
                Ok(event) => Ok(Event::RoomPowerLevels(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::RoomRedaction => match json.parse() {
                Ok(event) => Ok(Event::RoomRedaction(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::RoomServerAcl => match json.parse() {
                Ok(event) => Ok(Event::RoomServerAcl(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::RoomThirdPartyInvite => match json.parse() {
                Ok(event) => Ok(Event::RoomThirdPartyInvite(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::RoomTombstone => match json.parse() {
                Ok(event) => Ok(Event::RoomTombstone(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::RoomTopic => match json.parse() {
                Ok(event) => Ok(Event::RoomTopic(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::RoomKey => match json.parse() {
                Ok(event) => Ok(Event::RoomKey(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::RoomKeyRequest => match json.parse() {
                Ok(event) => Ok(Event::RoomKeyRequest(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::Sticker => match json.parse() {
                Ok(event) => Ok(Event::Sticker(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::Tag => match json.parse() {
                Ok(event) => Ok(Event::Tag(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::Typing => match json.parse() {
                Ok(event) => Ok(Event::Typing(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::Custom(_) => {
                if value.get("state_key").is_some() {
                    match json.parse() {
                        Ok(event) => Ok(Event::CustomState(event)),
                        Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                            json: value,
                            message: error.to_string(),
                        })),
                    }
                } else if value.get("event_id").is_some()
                    && value.get("room_id").is_some()
                    && value.get("sender").is_some()
                {
                    match json.parse() {
                        Ok(event) => Ok(Event::CustomRoom(event)),
                        Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                            json: value,
                            message: error.to_string(),
                        })),
                    }
                } else {
                    match json.parse() {
                        Ok(event) => Ok(Event::Custom(event)),
                        Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                            json: value,
                            message: error.to_string(),
                        })),
                    }
                }
            }
            EventType::__Nonexhaustive => {
                panic!("__Nonexhaustive enum variant is not intended for use.")
            }
        }
    }
}

impl Serialize for RoomEvent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            RoomEvent::CallAnswer(ref event) => event.serialize(serializer),
            RoomEvent::CallCandidates(ref event) => event.serialize(serializer),
            RoomEvent::CallHangup(ref event) => event.serialize(serializer),
            RoomEvent::CallInvite(ref event) => event.serialize(serializer),
            RoomEvent::RoomAliases(ref event) => event.serialize(serializer),
            RoomEvent::RoomAvatar(ref event) => event.serialize(serializer),
            RoomEvent::RoomCanonicalAlias(ref event) => event.serialize(serializer),
            RoomEvent::RoomCreate(ref event) => event.serialize(serializer),
            RoomEvent::RoomEncrypted(ref event) => event.serialize(serializer),
            RoomEvent::RoomEncryption(ref event) => event.serialize(serializer),
            RoomEvent::RoomGuestAccess(ref event) => event.serialize(serializer),
            RoomEvent::RoomHistoryVisibility(ref event) => event.serialize(serializer),
            RoomEvent::RoomJoinRules(ref event) => event.serialize(serializer),
            RoomEvent::RoomMember(ref event) => event.serialize(serializer),
            RoomEvent::RoomMessage(ref event) => event.serialize(serializer),
            RoomEvent::RoomMessageFeedback(ref event) => event.serialize(serializer),
            RoomEvent::RoomName(ref event) => event.serialize(serializer),
            RoomEvent::RoomPinnedEvents(ref event) => event.serialize(serializer),
            RoomEvent::RoomPowerLevels(ref event) => event.serialize(serializer),
            RoomEvent::RoomRedaction(ref event) => event.serialize(serializer),
            RoomEvent::RoomServerAcl(ref event) => event.serialize(serializer),
            RoomEvent::RoomThirdPartyInvite(ref event) => event.serialize(serializer),
            RoomEvent::RoomTombstone(ref event) => event.serialize(serializer),
            RoomEvent::RoomTopic(ref event) => event.serialize(serializer),
            RoomEvent::Sticker(ref event) => event.serialize(serializer),
            RoomEvent::CustomRoom(ref event) => event.serialize(serializer),
            RoomEvent::CustomState(ref event) => event.serialize(serializer),
        }
    }
}

impl FromStr for RoomEvent {
    type Err = InvalidEvent;

    /// Attempt to create `Self` from parsing a string of JSON data.
    #[allow(clippy::cognitive_complexity)]
    fn from_str(json: &str) -> Result<Self, Self::Err> {
        let value: Value = serde_json::from_str(json)?;

        let event_type_value = match value.get("type") {
            Some(value) => value.clone(),
            None => {
                return Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: "missing field `type`".to_string(),
                }))
            }
        };

        let event_type = match from_value::<EventType>(event_type_value.clone()) {
            Ok(event_type) => event_type,
            Err(error) => {
                return Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                }))
            }
        };

        match event_type {
            EventType::CallAnswer => match json.parse() {
                Ok(event) => Ok(RoomEvent::CallAnswer(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::CallCandidates => match json.parse() {
                Ok(event) => Ok(RoomEvent::CallCandidates(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::CallHangup => match json.parse() {
                Ok(event) => Ok(RoomEvent::CallHangup(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::CallInvite => match json.parse() {
                Ok(event) => Ok(RoomEvent::CallInvite(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::RoomAliases => match json.parse() {
                Ok(event) => Ok(RoomEvent::RoomAliases(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::RoomAvatar => match json.parse() {
                Ok(event) => Ok(RoomEvent::RoomAvatar(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::RoomCanonicalAlias => match json.parse() {
                Ok(event) => Ok(RoomEvent::RoomCanonicalAlias(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::RoomCreate => match json.parse() {
                Ok(event) => Ok(RoomEvent::RoomCreate(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::RoomEncrypted => match json.parse() {
                Ok(event) => Ok(RoomEvent::RoomEncrypted(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::RoomEncryption => match json.parse() {
                Ok(event) => Ok(RoomEvent::RoomEncryption(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::RoomGuestAccess => match json.parse() {
                Ok(event) => Ok(RoomEvent::RoomGuestAccess(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::RoomHistoryVisibility => match json.parse() {
                Ok(event) => Ok(RoomEvent::RoomHistoryVisibility(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::RoomJoinRules => match json.parse() {
                Ok(event) => Ok(RoomEvent::RoomJoinRules(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::RoomMember => match json.parse() {
                Ok(event) => Ok(RoomEvent::RoomMember(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::RoomMessage => match json.parse() {
                Ok(event) => Ok(RoomEvent::RoomMessage(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::RoomMessageFeedback => match json.parse() {
                Ok(event) => Ok(RoomEvent::RoomMessageFeedback(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::RoomName => match json.parse() {
                Ok(event) => Ok(RoomEvent::RoomName(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::RoomPinnedEvents => match json.parse() {
                Ok(event) => Ok(RoomEvent::RoomPinnedEvents(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::RoomPowerLevels => match json.parse() {
                Ok(event) => Ok(RoomEvent::RoomPowerLevels(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::RoomRedaction => match json.parse() {
                Ok(event) => Ok(RoomEvent::RoomRedaction(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::RoomServerAcl => match json.parse() {
                Ok(event) => Ok(RoomEvent::RoomServerAcl(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::RoomThirdPartyInvite => match json.parse() {
                Ok(event) => Ok(RoomEvent::RoomThirdPartyInvite(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::RoomTombstone => match json.parse() {
                Ok(event) => Ok(RoomEvent::RoomTombstone(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::RoomTopic => match json.parse() {
                Ok(event) => Ok(RoomEvent::RoomTopic(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::Sticker => match json.parse() {
                Ok(event) => Ok(RoomEvent::Sticker(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::Custom(_) => {
                if value.get("state_key").is_some() {
                    match json.parse() {
                        Ok(event) => Ok(RoomEvent::CustomState(event)),
                        Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                            json: value,
                            message: error.to_string(),
                        })),
                    }
                } else {
                    match json.parse() {
                        Ok(event) => Ok(RoomEvent::CustomRoom(event)),
                        Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                            json: value,
                            message: error.to_string(),
                        })),
                    }
                }
            }
            EventType::Direct
            | EventType::Dummy
            | EventType::ForwardedRoomKey
            | EventType::FullyRead
            | EventType::KeyVerificationAccept
            | EventType::KeyVerificationCancel
            | EventType::KeyVerificationKey
            | EventType::KeyVerificationMac
            | EventType::KeyVerificationRequest
            | EventType::KeyVerificationStart
            | EventType::IgnoredUserList
            | EventType::Presence
            | EventType::PushRules
            | EventType::Receipt
            | EventType::RoomKey
            | EventType::RoomKeyRequest
            | EventType::Tag
            | EventType::Typing => Err(InvalidEvent(InnerInvalidEvent::Validation {
                json: value,
                message: "not a room event".to_string(),
            })),
            EventType::__Nonexhaustive => {
                panic!("__Nonexhaustive enum variant is not intended for use.")
            }
        }
    }
}

impl crate::Event for RoomEvent {
    type Content = (); // TODO

    fn content(&self) -> &Self::Content {
        unimplemented!() // TODO
    }

    fn event_type(&self) -> EventType {
        match &self {
            RoomEvent::CallAnswer(e) => e.event_type(),
            RoomEvent::CallCandidates(e) => e.event_type(),
            RoomEvent::CallHangup(e) => e.event_type(),
            RoomEvent::CallInvite(e) => e.event_type(),
            RoomEvent::RoomAliases(e) => e.event_type(),
            RoomEvent::RoomAvatar(e) => e.event_type(),
            RoomEvent::RoomCanonicalAlias(e) => e.event_type(),
            RoomEvent::RoomCreate(e) => e.event_type(),
            RoomEvent::RoomEncrypted(e) => e.event_type(),
            RoomEvent::RoomEncryption(e) => e.event_type(),
            RoomEvent::RoomGuestAccess(e) => e.event_type(),
            RoomEvent::RoomHistoryVisibility(e) => e.event_type(),
            RoomEvent::RoomJoinRules(e) => e.event_type(),
            RoomEvent::RoomMember(e) => e.event_type(),
            RoomEvent::RoomMessage(e) => e.event_type(),
            RoomEvent::RoomMessageFeedback(e) => e.event_type(),
            RoomEvent::RoomName(e) => e.event_type(),
            RoomEvent::RoomPinnedEvents(e) => e.event_type(),
            RoomEvent::RoomPowerLevels(e) => e.event_type(),
            RoomEvent::RoomRedaction(e) => e.event_type(),
            RoomEvent::RoomServerAcl(e) => e.event_type(),
            RoomEvent::RoomThirdPartyInvite(e) => e.event_type(),
            RoomEvent::RoomTombstone(e) => e.event_type(),
            RoomEvent::RoomTopic(e) => e.event_type(),
            RoomEvent::Sticker(e) => e.event_type(),
            RoomEvent::CustomRoom(e) => e.event_type(),
            RoomEvent::CustomState(e) => e.event_type(),
        }
    }
}

impl crate::RoomEvent for RoomEvent {
    fn event_id(&self) -> &EventId {
        match &self {
            RoomEvent::CallAnswer(e) => e.event_id(),
            RoomEvent::CallCandidates(e) => e.event_id(),
            RoomEvent::CallHangup(e) => e.event_id(),
            RoomEvent::CallInvite(e) => e.event_id(),
            RoomEvent::RoomAliases(e) => e.event_id(),
            RoomEvent::RoomAvatar(e) => e.event_id(),
            RoomEvent::RoomCanonicalAlias(e) => e.event_id(),
            RoomEvent::RoomCreate(e) => e.event_id(),
            RoomEvent::RoomEncrypted(e) => e.event_id(),
            RoomEvent::RoomEncryption(e) => e.event_id(),
            RoomEvent::RoomGuestAccess(e) => e.event_id(),
            RoomEvent::RoomHistoryVisibility(e) => e.event_id(),
            RoomEvent::RoomJoinRules(e) => e.event_id(),
            RoomEvent::RoomMember(e) => e.event_id(),
            RoomEvent::RoomMessage(e) => e.event_id(),
            RoomEvent::RoomMessageFeedback(e) => e.event_id(),
            RoomEvent::RoomName(e) => e.event_id(),
            RoomEvent::RoomPinnedEvents(e) => e.event_id(),
            RoomEvent::RoomPowerLevels(e) => e.event_id(),
            RoomEvent::RoomRedaction(e) => e.event_id(),
            RoomEvent::RoomServerAcl(e) => e.event_id(),
            RoomEvent::RoomThirdPartyInvite(e) => e.event_id(),
            RoomEvent::RoomTombstone(e) => e.event_id(),
            RoomEvent::RoomTopic(e) => e.event_id(),
            RoomEvent::Sticker(e) => e.event_id(),
            RoomEvent::CustomRoom(e) => e.event_id(),
            RoomEvent::CustomState(e) => e.event_id(),
        }
    }

    fn origin_server_ts(&self) -> UInt {
        match &self {
            RoomEvent::CallAnswer(e) => e.origin_server_ts(),
            RoomEvent::CallCandidates(e) => e.origin_server_ts(),
            RoomEvent::CallHangup(e) => e.origin_server_ts(),
            RoomEvent::CallInvite(e) => e.origin_server_ts(),
            RoomEvent::RoomAliases(e) => e.origin_server_ts(),
            RoomEvent::RoomAvatar(e) => e.origin_server_ts(),
            RoomEvent::RoomCanonicalAlias(e) => e.origin_server_ts(),
            RoomEvent::RoomCreate(e) => e.origin_server_ts(),
            RoomEvent::RoomEncrypted(e) => e.origin_server_ts(),
            RoomEvent::RoomEncryption(e) => e.origin_server_ts(),
            RoomEvent::RoomGuestAccess(e) => e.origin_server_ts(),
            RoomEvent::RoomHistoryVisibility(e) => e.origin_server_ts(),
            RoomEvent::RoomJoinRules(e) => e.origin_server_ts(),
            RoomEvent::RoomMember(e) => e.origin_server_ts(),
            RoomEvent::RoomMessage(e) => e.origin_server_ts(),
            RoomEvent::RoomMessageFeedback(e) => e.origin_server_ts(),
            RoomEvent::RoomName(e) => e.origin_server_ts(),
            RoomEvent::RoomPinnedEvents(e) => e.origin_server_ts(),
            RoomEvent::RoomPowerLevels(e) => e.origin_server_ts(),
            RoomEvent::RoomRedaction(e) => e.origin_server_ts(),
            RoomEvent::RoomServerAcl(e) => e.origin_server_ts(),
            RoomEvent::RoomThirdPartyInvite(e) => e.origin_server_ts(),
            RoomEvent::RoomTombstone(e) => e.origin_server_ts(),
            RoomEvent::RoomTopic(e) => e.origin_server_ts(),
            RoomEvent::Sticker(e) => e.origin_server_ts(),
            RoomEvent::CustomRoom(e) => e.origin_server_ts(),
            RoomEvent::CustomState(e) => e.origin_server_ts(),
        }
    }

    fn room_id(&self) -> Option<&RoomId> {
        match &self {
            RoomEvent::CallAnswer(e) => e.room_id(),
            RoomEvent::CallCandidates(e) => e.room_id(),
            RoomEvent::CallHangup(e) => e.room_id(),
            RoomEvent::CallInvite(e) => e.room_id(),
            RoomEvent::RoomAliases(e) => e.room_id(),
            RoomEvent::RoomAvatar(e) => e.room_id(),
            RoomEvent::RoomCanonicalAlias(e) => e.room_id(),
            RoomEvent::RoomCreate(e) => e.room_id(),
            RoomEvent::RoomEncrypted(e) => e.room_id(),
            RoomEvent::RoomEncryption(e) => e.room_id(),
            RoomEvent::RoomGuestAccess(e) => e.room_id(),
            RoomEvent::RoomHistoryVisibility(e) => e.room_id(),
            RoomEvent::RoomJoinRules(e) => e.room_id(),
            RoomEvent::RoomMember(e) => e.room_id(),
            RoomEvent::RoomMessage(e) => e.room_id(),
            RoomEvent::RoomMessageFeedback(e) => e.room_id(),
            RoomEvent::RoomName(e) => e.room_id(),
            RoomEvent::RoomPinnedEvents(e) => e.room_id(),
            RoomEvent::RoomPowerLevels(e) => e.room_id(),
            RoomEvent::RoomRedaction(e) => e.room_id(),
            RoomEvent::RoomServerAcl(e) => e.room_id(),
            RoomEvent::RoomThirdPartyInvite(e) => e.room_id(),
            RoomEvent::RoomTombstone(e) => e.room_id(),
            RoomEvent::RoomTopic(e) => e.room_id(),
            RoomEvent::Sticker(e) => e.room_id(),
            RoomEvent::CustomRoom(e) => e.room_id(),
            RoomEvent::CustomState(e) => e.room_id(),
        }
    }

    fn sender(&self) -> &UserId {
        match &self {
            RoomEvent::CallAnswer(e) => e.sender(),
            RoomEvent::CallCandidates(e) => e.sender(),
            RoomEvent::CallHangup(e) => e.sender(),
            RoomEvent::CallInvite(e) => e.sender(),
            RoomEvent::RoomAliases(e) => e.sender(),
            RoomEvent::RoomAvatar(e) => e.sender(),
            RoomEvent::RoomCanonicalAlias(e) => e.sender(),
            RoomEvent::RoomCreate(e) => e.sender(),
            RoomEvent::RoomEncrypted(e) => e.sender(),
            RoomEvent::RoomEncryption(e) => e.sender(),
            RoomEvent::RoomGuestAccess(e) => e.sender(),
            RoomEvent::RoomHistoryVisibility(e) => e.sender(),
            RoomEvent::RoomJoinRules(e) => e.sender(),
            RoomEvent::RoomMember(e) => e.sender(),
            RoomEvent::RoomMessage(e) => e.sender(),
            RoomEvent::RoomMessageFeedback(e) => e.sender(),
            RoomEvent::RoomName(e) => e.sender(),
            RoomEvent::RoomPinnedEvents(e) => e.sender(),
            RoomEvent::RoomPowerLevels(e) => e.sender(),
            RoomEvent::RoomRedaction(e) => e.sender(),
            RoomEvent::RoomServerAcl(e) => e.sender(),
            RoomEvent::RoomThirdPartyInvite(e) => e.sender(),
            RoomEvent::RoomTombstone(e) => e.sender(),
            RoomEvent::RoomTopic(e) => e.sender(),
            RoomEvent::Sticker(e) => e.sender(),
            RoomEvent::CustomRoom(e) => e.sender(),
            RoomEvent::CustomState(e) => e.sender(),
        }
    }

    fn unsigned(&self) -> Option<&Value> {
        match &self {
            RoomEvent::CallAnswer(e) => e.unsigned(),
            RoomEvent::CallCandidates(e) => e.unsigned(),
            RoomEvent::CallHangup(e) => e.unsigned(),
            RoomEvent::CallInvite(e) => e.unsigned(),
            RoomEvent::RoomAliases(e) => e.unsigned(),
            RoomEvent::RoomAvatar(e) => e.unsigned(),
            RoomEvent::RoomCanonicalAlias(e) => e.unsigned(),
            RoomEvent::RoomCreate(e) => e.unsigned(),
            RoomEvent::RoomEncrypted(e) => e.unsigned(),
            RoomEvent::RoomEncryption(e) => e.unsigned(),
            RoomEvent::RoomGuestAccess(e) => e.unsigned(),
            RoomEvent::RoomHistoryVisibility(e) => e.unsigned(),
            RoomEvent::RoomJoinRules(e) => e.unsigned(),
            RoomEvent::RoomMember(e) => e.unsigned(),
            RoomEvent::RoomMessage(e) => e.unsigned(),
            RoomEvent::RoomMessageFeedback(e) => e.unsigned(),
            RoomEvent::RoomName(e) => e.unsigned(),
            RoomEvent::RoomPinnedEvents(e) => e.unsigned(),
            RoomEvent::RoomPowerLevels(e) => e.unsigned(),
            RoomEvent::RoomRedaction(e) => e.unsigned(),
            RoomEvent::RoomServerAcl(e) => e.unsigned(),
            RoomEvent::RoomThirdPartyInvite(e) => e.unsigned(),
            RoomEvent::RoomTombstone(e) => e.unsigned(),
            RoomEvent::RoomTopic(e) => e.unsigned(),
            RoomEvent::Sticker(e) => e.unsigned(),
            RoomEvent::CustomRoom(e) => e.unsigned(),
            RoomEvent::CustomState(e) => e.unsigned(),
        }
    }
}

impl Serialize for StateEvent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            StateEvent::RoomAliases(ref event) => event.serialize(serializer),
            StateEvent::RoomAvatar(ref event) => event.serialize(serializer),
            StateEvent::RoomCanonicalAlias(ref event) => event.serialize(serializer),
            StateEvent::RoomCreate(ref event) => event.serialize(serializer),
            StateEvent::RoomEncryption(ref event) => event.serialize(serializer),
            StateEvent::RoomGuestAccess(ref event) => event.serialize(serializer),
            StateEvent::RoomHistoryVisibility(ref event) => event.serialize(serializer),
            StateEvent::RoomJoinRules(ref event) => event.serialize(serializer),
            StateEvent::RoomMember(ref event) => event.serialize(serializer),
            StateEvent::RoomName(ref event) => event.serialize(serializer),
            StateEvent::RoomPinnedEvents(ref event) => event.serialize(serializer),
            StateEvent::RoomPowerLevels(ref event) => event.serialize(serializer),
            StateEvent::RoomServerAcl(ref event) => event.serialize(serializer),
            StateEvent::RoomThirdPartyInvite(ref event) => event.serialize(serializer),
            StateEvent::RoomTombstone(ref event) => event.serialize(serializer),
            StateEvent::RoomTopic(ref event) => event.serialize(serializer),
            StateEvent::CustomState(ref event) => event.serialize(serializer),
        }
    }
}

impl FromStr for StateEvent {
    type Err = InvalidEvent;

    /// Attempt to create `Self` from parsing a string of JSON data.
    fn from_str(json: &str) -> Result<Self, Self::Err> {
        let value: Value = serde_json::from_str(json)?;

        let event_type_value = match value.get("type") {
            Some(value) => value.clone(),
            None => {
                return Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: "missing field `type`".to_string(),
                }))
            }
        };

        let event_type = match from_value::<EventType>(event_type_value.clone()) {
            Ok(event_type) => event_type,
            Err(error) => {
                return Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                }))
            }
        };

        match event_type {
            EventType::RoomAliases => match json.parse() {
                Ok(event) => Ok(StateEvent::RoomAliases(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::RoomAvatar => match json.parse() {
                Ok(event) => Ok(StateEvent::RoomAvatar(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::RoomCanonicalAlias => match json.parse() {
                Ok(event) => Ok(StateEvent::RoomCanonicalAlias(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::RoomCreate => match json.parse() {
                Ok(event) => Ok(StateEvent::RoomCreate(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::RoomEncryption => match json.parse() {
                Ok(event) => Ok(StateEvent::RoomEncryption(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::RoomGuestAccess => match json.parse() {
                Ok(event) => Ok(StateEvent::RoomGuestAccess(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::RoomHistoryVisibility => match json.parse() {
                Ok(event) => Ok(StateEvent::RoomHistoryVisibility(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::RoomJoinRules => match json.parse() {
                Ok(event) => Ok(StateEvent::RoomJoinRules(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::RoomMember => match json.parse() {
                Ok(event) => Ok(StateEvent::RoomMember(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::RoomName => match json.parse() {
                Ok(event) => Ok(StateEvent::RoomName(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::RoomPinnedEvents => match json.parse() {
                Ok(event) => Ok(StateEvent::RoomPinnedEvents(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::RoomPowerLevels => match json.parse() {
                Ok(event) => Ok(StateEvent::RoomPowerLevels(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::RoomServerAcl => match json.parse() {
                Ok(event) => Ok(StateEvent::RoomServerAcl(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::RoomThirdPartyInvite => match json.parse() {
                Ok(event) => Ok(StateEvent::RoomThirdPartyInvite(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::RoomTombstone => match json.parse() {
                Ok(event) => Ok(StateEvent::RoomTombstone(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::RoomTopic => match json.parse() {
                Ok(event) => Ok(StateEvent::RoomTopic(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::Custom(_) => match json.parse() {
                Ok(event) => Ok(StateEvent::CustomState(event)),
                Err(error) => Err(InvalidEvent(InnerInvalidEvent::Validation {
                    json: value,
                    message: error.to_string(),
                })),
            },
            EventType::CallAnswer
            | EventType::CallCandidates
            | EventType::CallHangup
            | EventType::CallInvite
            | EventType::Direct
            | EventType::Dummy
            | EventType::ForwardedRoomKey
            | EventType::FullyRead
            | EventType::KeyVerificationAccept
            | EventType::KeyVerificationCancel
            | EventType::KeyVerificationKey
            | EventType::KeyVerificationMac
            | EventType::KeyVerificationRequest
            | EventType::KeyVerificationStart
            | EventType::IgnoredUserList
            | EventType::Presence
            | EventType::PushRules
            | EventType::Receipt
            | EventType::RoomEncrypted
            | EventType::RoomMessage
            | EventType::RoomMessageFeedback
            | EventType::RoomRedaction
            | EventType::RoomKey
            | EventType::RoomKeyRequest
            | EventType::Sticker
            | EventType::Tag
            | EventType::Typing => Err(InvalidEvent(InnerInvalidEvent::Validation {
                json: value,
                message: "not a state event".to_string(),
            })),
            EventType::__Nonexhaustive => {
                panic!("__Nonexhaustive enum variant is not intended for use.")
            }
        }
    }
}

impl crate::Event for StateEvent {
    type Content = (); // TODO

    fn content(&self) -> &Self::Content {
        unimplemented!() // TODO
    }

    fn event_type(&self) -> EventType {
        match &self {
            StateEvent::RoomAliases(e) => e.event_type(),
            StateEvent::RoomAvatar(e) => e.event_type(),
            StateEvent::RoomCanonicalAlias(e) => e.event_type(),
            StateEvent::RoomCreate(e) => e.event_type(),
            StateEvent::RoomEncryption(e) => e.event_type(),
            StateEvent::RoomGuestAccess(e) => e.event_type(),
            StateEvent::RoomHistoryVisibility(e) => e.event_type(),
            StateEvent::RoomJoinRules(e) => e.event_type(),
            StateEvent::RoomMember(e) => e.event_type(),
            StateEvent::RoomName(e) => e.event_type(),
            StateEvent::RoomPinnedEvents(e) => e.event_type(),
            StateEvent::RoomPowerLevels(e) => e.event_type(),
            StateEvent::RoomServerAcl(e) => e.event_type(),
            StateEvent::RoomThirdPartyInvite(e) => e.event_type(),
            StateEvent::RoomTombstone(e) => e.event_type(),
            StateEvent::RoomTopic(e) => e.event_type(),
            StateEvent::CustomState(e) => e.event_type(),
        }
    }
}

impl crate::RoomEvent for StateEvent {
    fn event_id(&self) -> &EventId {
        match &self {
            StateEvent::RoomAliases(e) => e.event_id(),
            StateEvent::RoomAvatar(e) => e.event_id(),
            StateEvent::RoomCanonicalAlias(e) => e.event_id(),
            StateEvent::RoomCreate(e) => e.event_id(),
            StateEvent::RoomEncryption(e) => e.event_id(),
            StateEvent::RoomGuestAccess(e) => e.event_id(),
            StateEvent::RoomHistoryVisibility(e) => e.event_id(),
            StateEvent::RoomJoinRules(e) => e.event_id(),
            StateEvent::RoomMember(e) => e.event_id(),
            StateEvent::RoomName(e) => e.event_id(),
            StateEvent::RoomPinnedEvents(e) => e.event_id(),
            StateEvent::RoomPowerLevels(e) => e.event_id(),
            StateEvent::RoomServerAcl(e) => e.event_id(),
            StateEvent::RoomThirdPartyInvite(e) => e.event_id(),
            StateEvent::RoomTombstone(e) => e.event_id(),
            StateEvent::RoomTopic(e) => e.event_id(),
            StateEvent::CustomState(e) => e.event_id(),
        }
    }

    fn origin_server_ts(&self) -> UInt {
        match &self {
            StateEvent::RoomAliases(e) => e.origin_server_ts(),
            StateEvent::RoomAvatar(e) => e.origin_server_ts(),
            StateEvent::RoomCanonicalAlias(e) => e.origin_server_ts(),
            StateEvent::RoomCreate(e) => e.origin_server_ts(),
            StateEvent::RoomEncryption(e) => e.origin_server_ts(),
            StateEvent::RoomGuestAccess(e) => e.origin_server_ts(),
            StateEvent::RoomHistoryVisibility(e) => e.origin_server_ts(),
            StateEvent::RoomJoinRules(e) => e.origin_server_ts(),
            StateEvent::RoomMember(e) => e.origin_server_ts(),
            StateEvent::RoomName(e) => e.origin_server_ts(),
            StateEvent::RoomPinnedEvents(e) => e.origin_server_ts(),
            StateEvent::RoomPowerLevels(e) => e.origin_server_ts(),
            StateEvent::RoomServerAcl(e) => e.origin_server_ts(),
            StateEvent::RoomThirdPartyInvite(e) => e.origin_server_ts(),
            StateEvent::RoomTombstone(e) => e.origin_server_ts(),
            StateEvent::RoomTopic(e) => e.origin_server_ts(),
            StateEvent::CustomState(e) => e.origin_server_ts(),
        }
    }

    fn room_id(&self) -> Option<&RoomId> {
        match &self {
            StateEvent::RoomAliases(e) => e.room_id(),
            StateEvent::RoomAvatar(e) => e.room_id(),
            StateEvent::RoomCanonicalAlias(e) => e.room_id(),
            StateEvent::RoomCreate(e) => e.room_id(),
            StateEvent::RoomEncryption(e) => e.room_id(),
            StateEvent::RoomGuestAccess(e) => e.room_id(),
            StateEvent::RoomHistoryVisibility(e) => e.room_id(),
            StateEvent::RoomJoinRules(e) => e.room_id(),
            StateEvent::RoomMember(e) => e.room_id(),
            StateEvent::RoomName(e) => e.room_id(),
            StateEvent::RoomPinnedEvents(e) => e.room_id(),
            StateEvent::RoomPowerLevels(e) => e.room_id(),
            StateEvent::RoomServerAcl(e) => e.room_id(),
            StateEvent::RoomThirdPartyInvite(e) => e.room_id(),
            StateEvent::RoomTombstone(e) => e.room_id(),
            StateEvent::RoomTopic(e) => e.room_id(),
            StateEvent::CustomState(e) => e.room_id(),
        }
    }

    fn sender(&self) -> &UserId {
        match &self {
            StateEvent::RoomAliases(e) => e.sender(),
            StateEvent::RoomAvatar(e) => e.sender(),
            StateEvent::RoomCanonicalAlias(e) => e.sender(),
            StateEvent::RoomCreate(e) => e.sender(),
            StateEvent::RoomEncryption(e) => e.sender(),
            StateEvent::RoomGuestAccess(e) => e.sender(),
            StateEvent::RoomHistoryVisibility(e) => e.sender(),
            StateEvent::RoomJoinRules(e) => e.sender(),
            StateEvent::RoomMember(e) => e.sender(),
            StateEvent::RoomName(e) => e.sender(),
            StateEvent::RoomPinnedEvents(e) => e.sender(),
            StateEvent::RoomPowerLevels(e) => e.sender(),
            StateEvent::RoomServerAcl(e) => e.sender(),
            StateEvent::RoomThirdPartyInvite(e) => e.sender(),
            StateEvent::RoomTombstone(e) => e.sender(),
            StateEvent::RoomTopic(e) => e.sender(),
            StateEvent::CustomState(e) => e.sender(),
        }
    }

    fn unsigned(&self) -> Option<&Value> {
        match &self {
            StateEvent::RoomAliases(e) => e.unsigned(),
            StateEvent::RoomAvatar(e) => e.unsigned(),
            StateEvent::RoomCanonicalAlias(e) => e.unsigned(),
            StateEvent::RoomCreate(e) => e.unsigned(),
            StateEvent::RoomEncryption(e) => e.unsigned(),
            StateEvent::RoomGuestAccess(e) => e.unsigned(),
            StateEvent::RoomHistoryVisibility(e) => e.unsigned(),
            StateEvent::RoomJoinRules(e) => e.unsigned(),
            StateEvent::RoomMember(e) => e.unsigned(),
            StateEvent::RoomName(e) => e.unsigned(),
            StateEvent::RoomPinnedEvents(e) => e.unsigned(),
            StateEvent::RoomPowerLevels(e) => e.unsigned(),
            StateEvent::RoomServerAcl(e) => e.unsigned(),
            StateEvent::RoomThirdPartyInvite(e) => e.unsigned(),
            StateEvent::RoomTombstone(e) => e.unsigned(),
            StateEvent::RoomTopic(e) => e.unsigned(),
            StateEvent::CustomState(e) => e.unsigned(),
        }
    }
}

impl crate::StateEvent for StateEvent {
    fn prev_content(&self) -> Option<&Self::Content> {
        unimplemented!() // TODO
    }

    fn state_key(&self) -> &str {
        match &self {
            StateEvent::RoomAliases(e) => e.state_key(),
            StateEvent::RoomAvatar(e) => e.state_key(),
            StateEvent::RoomCanonicalAlias(e) => e.state_key(),
            StateEvent::RoomCreate(e) => e.state_key(),
            StateEvent::RoomEncryption(e) => e.state_key(),
            StateEvent::RoomGuestAccess(e) => e.state_key(),
            StateEvent::RoomHistoryVisibility(e) => e.state_key(),
            StateEvent::RoomJoinRules(e) => e.state_key(),
            StateEvent::RoomMember(e) => e.state_key(),
            StateEvent::RoomName(e) => e.state_key(),
            StateEvent::RoomPinnedEvents(e) => e.state_key(),
            StateEvent::RoomPowerLevels(e) => e.state_key(),
            StateEvent::RoomServerAcl(e) => e.state_key(),
            StateEvent::RoomThirdPartyInvite(e) => e.state_key(),
            StateEvent::RoomTombstone(e) => e.state_key(),
            StateEvent::RoomTopic(e) => e.state_key(),
            StateEvent::CustomState(e) => e.state_key(),
        }
    }
}

macro_rules! impl_from_t_for_event {
    ($ty:ty, $variant:ident) => {
        impl From<$ty> for Event {
            fn from(event: $ty) -> Self {
                Event::$variant(event)
            }
        }
    };
}

impl_from_t_for_event!(AnswerEvent, CallAnswer);
impl_from_t_for_event!(CandidatesEvent, CallCandidates);
impl_from_t_for_event!(HangupEvent, CallHangup);
impl_from_t_for_event!(InviteEvent, CallInvite);
impl_from_t_for_event!(DirectEvent, Direct);
impl_from_t_for_event!(DummyEvent, Dummy);
impl_from_t_for_event!(ForwardedRoomKeyEvent, ForwardedRoomKey);
impl_from_t_for_event!(FullyReadEvent, FullyRead);
impl_from_t_for_event!(AcceptEvent, KeyVerificationAccept);
impl_from_t_for_event!(CancelEvent, KeyVerificationCancel);
impl_from_t_for_event!(KeyEvent, KeyVerificationKey);
impl_from_t_for_event!(MacEvent, KeyVerificationMac);
impl_from_t_for_event!(RequestEvent, KeyVerificationRequest);
impl_from_t_for_event!(StartEvent, KeyVerificationStart);
impl_from_t_for_event!(IgnoredUserListEvent, IgnoredUserList);
impl_from_t_for_event!(PresenceEvent, Presence);
impl_from_t_for_event!(PushRulesEvent, PushRules);
impl_from_t_for_event!(ReceiptEvent, Receipt);
impl_from_t_for_event!(AliasesEvent, RoomAliases);
impl_from_t_for_event!(AvatarEvent, RoomAvatar);
impl_from_t_for_event!(CanonicalAliasEvent, RoomCanonicalAlias);
impl_from_t_for_event!(CreateEvent, RoomCreate);
impl_from_t_for_event!(EncryptedEvent, RoomEncrypted);
impl_from_t_for_event!(EncryptionEvent, RoomEncryption);
impl_from_t_for_event!(GuestAccessEvent, RoomGuestAccess);
impl_from_t_for_event!(HistoryVisibilityEvent, RoomHistoryVisibility);
impl_from_t_for_event!(JoinRulesEvent, RoomJoinRules);
impl_from_t_for_event!(MemberEvent, RoomMember);
impl_from_t_for_event!(MessageEvent, RoomMessage);
impl_from_t_for_event!(FeedbackEvent, RoomMessageFeedback);
impl_from_t_for_event!(NameEvent, RoomName);
impl_from_t_for_event!(PinnedEventsEvent, RoomPinnedEvents);
impl_from_t_for_event!(PowerLevelsEvent, RoomPowerLevels);
impl_from_t_for_event!(RedactionEvent, RoomRedaction);
impl_from_t_for_event!(ServerAclEvent, RoomServerAcl);
impl_from_t_for_event!(ThirdPartyInviteEvent, RoomThirdPartyInvite);
impl_from_t_for_event!(TombstoneEvent, RoomTombstone);
impl_from_t_for_event!(TopicEvent, RoomTopic);
impl_from_t_for_event!(RoomKeyEvent, RoomKey);
impl_from_t_for_event!(RoomKeyRequestEvent, RoomKeyRequest);
impl_from_t_for_event!(StickerEvent, Sticker);
impl_from_t_for_event!(TagEvent, Tag);
impl_from_t_for_event!(TypingEvent, Typing);
impl_from_t_for_event!(CustomEvent, Custom);
impl_from_t_for_event!(CustomRoomEvent, CustomRoom);
impl_from_t_for_event!(CustomStateEvent, CustomState);

macro_rules! impl_from_t_for_room_event {
    ($ty:ty, $variant:ident) => {
        impl From<$ty> for RoomEvent {
            fn from(event: $ty) -> Self {
                RoomEvent::$variant(event)
            }
        }
    };
}

impl_from_t_for_room_event!(AnswerEvent, CallAnswer);
impl_from_t_for_room_event!(CandidatesEvent, CallCandidates);
impl_from_t_for_room_event!(HangupEvent, CallHangup);
impl_from_t_for_room_event!(InviteEvent, CallInvite);
impl_from_t_for_room_event!(AliasesEvent, RoomAliases);
impl_from_t_for_room_event!(AvatarEvent, RoomAvatar);
impl_from_t_for_room_event!(CanonicalAliasEvent, RoomCanonicalAlias);
impl_from_t_for_room_event!(CreateEvent, RoomCreate);
impl_from_t_for_room_event!(EncryptedEvent, RoomEncrypted);
impl_from_t_for_room_event!(EncryptionEvent, RoomEncryption);
impl_from_t_for_room_event!(GuestAccessEvent, RoomGuestAccess);
impl_from_t_for_room_event!(HistoryVisibilityEvent, RoomHistoryVisibility);
impl_from_t_for_room_event!(JoinRulesEvent, RoomJoinRules);
impl_from_t_for_room_event!(MemberEvent, RoomMember);
impl_from_t_for_room_event!(MessageEvent, RoomMessage);
impl_from_t_for_room_event!(FeedbackEvent, RoomMessageFeedback);
impl_from_t_for_room_event!(NameEvent, RoomName);
impl_from_t_for_room_event!(PinnedEventsEvent, RoomPinnedEvents);
impl_from_t_for_room_event!(PowerLevelsEvent, RoomPowerLevels);
impl_from_t_for_room_event!(RedactionEvent, RoomRedaction);
impl_from_t_for_room_event!(ServerAclEvent, RoomServerAcl);
impl_from_t_for_room_event!(StickerEvent, Sticker);
impl_from_t_for_room_event!(ThirdPartyInviteEvent, RoomThirdPartyInvite);
impl_from_t_for_room_event!(TombstoneEvent, RoomTombstone);
impl_from_t_for_room_event!(TopicEvent, RoomTopic);
impl_from_t_for_room_event!(CustomRoomEvent, CustomRoom);
impl_from_t_for_room_event!(CustomStateEvent, CustomState);

macro_rules! impl_from_t_for_state_event {
    ($ty:ty, $variant:ident) => {
        impl From<$ty> for StateEvent {
            fn from(event: $ty) -> Self {
                StateEvent::$variant(event)
            }
        }
    };
}

impl_from_t_for_state_event!(AliasesEvent, RoomAliases);
impl_from_t_for_state_event!(AvatarEvent, RoomAvatar);
impl_from_t_for_state_event!(CanonicalAliasEvent, RoomCanonicalAlias);
impl_from_t_for_state_event!(CreateEvent, RoomCreate);
impl_from_t_for_state_event!(EncryptionEvent, RoomEncryption);
impl_from_t_for_state_event!(GuestAccessEvent, RoomGuestAccess);
impl_from_t_for_state_event!(HistoryVisibilityEvent, RoomHistoryVisibility);
impl_from_t_for_state_event!(JoinRulesEvent, RoomJoinRules);
impl_from_t_for_state_event!(MemberEvent, RoomMember);
impl_from_t_for_state_event!(NameEvent, RoomName);
impl_from_t_for_state_event!(PinnedEventsEvent, RoomPinnedEvents);
impl_from_t_for_state_event!(PowerLevelsEvent, RoomPowerLevels);
impl_from_t_for_state_event!(ServerAclEvent, RoomServerAcl);
impl_from_t_for_state_event!(ThirdPartyInviteEvent, RoomThirdPartyInvite);
impl_from_t_for_state_event!(TombstoneEvent, RoomTombstone);
impl_from_t_for_state_event!(TopicEvent, RoomTopic);
impl_from_t_for_state_event!(CustomStateEvent, CustomState);
