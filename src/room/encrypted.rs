//! Types for the *m.room.encrypted* event.

use std::{convert::TryFrom, str::FromStr};

use js_int::UInt;
use ruma_identifiers::{DeviceId, EventId, RoomId, UserId};
use serde::{de::Error, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer};
use serde_json::{from_value, Value};

use crate::{Algorithm, Event, EventType, InnerInvalidEvent, InvalidEvent, RoomEvent};

/// This event type is used when sending encrypted events.
///
/// This type is to be used within a room. For a to-device event, use `EncryptedEventContent`
/// directly.
#[derive(Clone, Debug, PartialEq)]
pub struct EncryptedEvent {
    /// The event's content.
    pub content: EncryptedEventContent,

    /// The unique identifier for the event.
    pub event_id: EventId,

    /// Timestamp (milliseconds since the UNIX epoch) on originating homeserver when this
    /// event was sent.
    pub origin_server_ts: UInt,

    /// The unique identifier for the room associated with this event.
    pub room_id: Option<RoomId>,

    /// The unique identifier for the user who sent this event.
    pub sender: UserId,

    /// Additional key-value pairs not signed by the homeserver.
    pub unsigned: Option<Value>,
}

/// The payload for `EncryptedEvent`.
#[derive(Clone, Debug, PartialEq)]
pub enum EncryptedEventContent {
    /// An event encrypted with *m.olm.v1.curve25519-aes-sha2*.
    OlmV1Curve25519AesSha2(OlmV1Curve25519AesSha2Content),

    /// An event encrypted with *m.megolm.v1.aes-sha2*.
    MegolmV1AesSha2(MegolmV1AesSha2Content),

    /// Additional variants may be added in the future and will not be considered breaking changes
    /// to ruma-events.
    #[doc(hidden)]
    __Nonexhaustive,
}

impl FromStr for EncryptedEvent {
    type Err = InvalidEvent;

    /// Attempt to create `Self` from parsing a string of JSON data.
    fn from_str(json: &str) -> Result<Self, Self::Err> {
        let raw = match serde_json::from_str::<raw::EncryptedEvent>(json) {
            Ok(raw) => raw,
            Err(error) => match serde_json::from_str::<serde_json::Value>(json) {
                Ok(value) => {
                    return Err(InvalidEvent(InnerInvalidEvent::Validation {
                        json: value,
                        message: error.to_string(),
                    }));
                }
                Err(error) => {
                    return Err(InvalidEvent(InnerInvalidEvent::Deserialization { error }));
                }
            },
        };

        let content = match raw.content {
            raw::EncryptedEventContent::OlmV1Curve25519AesSha2(content) => {
                EncryptedEventContent::OlmV1Curve25519AesSha2(content)
            }
            raw::EncryptedEventContent::MegolmV1AesSha2(content) => {
                EncryptedEventContent::MegolmV1AesSha2(content)
            }
            raw::EncryptedEventContent::__Nonexhaustive => {
                panic!("__Nonexhaustive enum variant is not intended for use.");
            }
        };

        Ok(Self {
            content,
            event_id: raw.event_id,
            origin_server_ts: raw.origin_server_ts,
            room_id: raw.room_id,
            sender: raw.sender,
            unsigned: raw.unsigned,
        })
    }
}

impl<'a> TryFrom<&'a str> for EncryptedEvent {
    type Error = InvalidEvent;

    /// Attempt to create `Self` from parsing a string of JSON data.
    fn try_from(json: &'a str) -> Result<Self, Self::Error> {
        FromStr::from_str(json)
    }
}

impl Serialize for EncryptedEvent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut len = 6;

        if self.room_id.is_some() {
            len += 1;
        }

        if self.unsigned.is_some() {
            len += 1;
        }

        let mut state = serializer.serialize_struct("EncryptedEvent", len)?;

        state.serialize_field("content", &self.content)?;
        state.serialize_field("event_id", &self.event_id)?;
        state.serialize_field("origin_server_ts", &self.origin_server_ts)?;

        if self.room_id.is_some() {
            state.serialize_field("room_id", &self.room_id)?;
        }

        state.serialize_field("sender", &self.sender)?;
        state.serialize_field("type", &self.event_type())?;

        if self.unsigned.is_some() {
            state.serialize_field("unsigned", &self.unsigned)?;
        }

        state.end()
    }
}

impl_room_event!(
    EncryptedEvent,
    EncryptedEventContent,
    EventType::RoomEncrypted
);

impl FromStr for EncryptedEventContent {
    type Err = InvalidEvent;

    /// Attempt to create `Self` from parsing a string of JSON data.
    fn from_str(json: &str) -> Result<Self, Self::Err> {
        let raw = match serde_json::from_str::<raw::EncryptedEventContent>(json) {
            Ok(raw) => raw,
            Err(error) => match serde_json::from_str::<serde_json::Value>(json) {
                Ok(value) => {
                    return Err(InvalidEvent(InnerInvalidEvent::Validation {
                        json: value,
                        message: error.to_string(),
                    }));
                }
                Err(error) => {
                    return Err(InvalidEvent(InnerInvalidEvent::Deserialization { error }));
                }
            },
        };

        match raw {
            raw::EncryptedEventContent::OlmV1Curve25519AesSha2(content) => {
                Ok(EncryptedEventContent::OlmV1Curve25519AesSha2(content))
            }
            raw::EncryptedEventContent::MegolmV1AesSha2(content) => {
                Ok(EncryptedEventContent::MegolmV1AesSha2(content))
            }
            raw::EncryptedEventContent::__Nonexhaustive => {
                panic!("__Nonexhaustive enum variant is not intended for use.");
            }
        }
    }
}

impl<'a> TryFrom<&'a str> for EncryptedEventContent {
    type Error = InvalidEvent;

    /// Attempt to create `Self` from parsing a string of JSON data.
    fn try_from(json: &'a str) -> Result<Self, Self::Error> {
        FromStr::from_str(json)
    }
}

impl Serialize for EncryptedEventContent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            EncryptedEventContent::OlmV1Curve25519AesSha2(ref content) => {
                content.serialize(serializer)
            }
            EncryptedEventContent::MegolmV1AesSha2(ref content) => content.serialize(serializer),
            _ => panic!("Attempted to serialize __Nonexhaustive variant."),
        }
    }
}

mod raw {
    use super::*;

    /// This event type is used when sending encrypted events.
    ///
    /// This type is to be used within a room. For a to-device event, use `EncryptedEventContent`
    /// directly.
    #[derive(Clone, Debug, Deserialize, PartialEq)]
    pub struct EncryptedEvent {
        /// The event's content.
        pub content: EncryptedEventContent,

        /// The unique identifier for the event.
        pub event_id: EventId,

        /// Timestamp (milliseconds since the UNIX epoch) on originating homeserver when this
        /// event was sent.
        pub origin_server_ts: UInt,

        /// The unique identifier for the room associated with this event.
        pub room_id: Option<RoomId>,

        /// The unique identifier for the user who sent this event.
        pub sender: UserId,

        /// Additional key-value pairs not signed by the homeserver.
        pub unsigned: Option<Value>,
    }

    /// The payload for `EncryptedEvent`.
    #[derive(Clone, Debug, PartialEq)]
    pub enum EncryptedEventContent {
        /// An event encrypted with *m.olm.v1.curve25519-aes-sha2*.
        OlmV1Curve25519AesSha2(OlmV1Curve25519AesSha2Content),

        /// An event encrypted with *m.megolm.v1.aes-sha2*.
        MegolmV1AesSha2(MegolmV1AesSha2Content),

        /// Additional variants may be added in the future and will not be considered breaking
        /// changes to ruma-events.
        #[doc(hidden)]
        __Nonexhaustive,
    }

    impl<'de> Deserialize<'de> for EncryptedEventContent {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let value: Value = Deserialize::deserialize(deserializer)?;

            let method_value = match value.get("algorithm") {
                Some(value) => value.clone(),
                None => return Err(D::Error::missing_field("algorithm")),
            };

            let method = match from_value::<Algorithm>(method_value.clone()) {
                Ok(method) => method,
                Err(error) => return Err(D::Error::custom(error.to_string())),
            };

            match method {
                Algorithm::OlmV1Curve25519AesSha2 => {
                    let content = match from_value::<OlmV1Curve25519AesSha2Content>(value) {
                        Ok(content) => content,
                        Err(error) => return Err(D::Error::custom(error.to_string())),
                    };

                    Ok(EncryptedEventContent::OlmV1Curve25519AesSha2(content))
                }
                Algorithm::MegolmV1AesSha2 => {
                    let content = match from_value::<MegolmV1AesSha2Content>(value) {
                        Ok(content) => content,
                        Err(error) => return Err(D::Error::custom(error.to_string())),
                    };

                    Ok(EncryptedEventContent::MegolmV1AesSha2(content))
                }
                Algorithm::Custom(_) => Err(D::Error::custom(
                    "Custom algorithms are not supported by `EncryptedEventContent`.",
                )),
                Algorithm::__Nonexhaustive => Err(D::Error::custom(
                    "Attempted to deserialize __Nonexhaustive variant.",
                )),
            }
        }
    }
}

/// The payload for `EncryptedEvent` using the *m.olm.v1.curve25519-aes-sha2* algorithm.
#[derive(Clone, Debug, Serialize, PartialEq, Deserialize)]
pub struct OlmV1Curve25519AesSha2Content {
    /// The encryption algorithm used to encrypt this event.
    pub algorithm: Algorithm,

    /// The encrypted content of the event.
    pub ciphertext: CiphertextInfo,

    /// The Curve25519 key of the sender.
    pub sender_key: String,
}

/// A map from the recipient Curve25519 identity key to ciphertext information.
///
/// Used for messages encrypted with the *m.olm.v1.curve25519-aes-sha2* algorithm.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct CiphertextInfo {
    /// The encrypted payload.
    pub body: String,

    /// The Olm message type.
    #[serde(rename = "type")]
    pub message_type: UInt,
}

/// The payload for `EncryptedEvent` using the *m.megolm.v1.aes-sha2* algorithm.
#[derive(Clone, Debug, Serialize, PartialEq, Deserialize)]
pub struct MegolmV1AesSha2Content {
    /// The encryption algorithm used to encrypt this event.
    pub algorithm: Algorithm,

    /// The encrypted content of the event.
    pub ciphertext: String,

    /// The Curve25519 key of the sender.
    pub sender_key: String,

    /// The ID of the sending device.
    pub device_id: DeviceId,

    /// The ID of the session used to encrypt the message.
    pub session_id: String,
}

#[cfg(test)]
mod tests {
    use serde_json::to_string;

    use super::{Algorithm, EncryptedEventContent, MegolmV1AesSha2Content};

    #[test]
    fn serializtion() {
        let key_verification_start_content =
            EncryptedEventContent::MegolmV1AesSha2(MegolmV1AesSha2Content {
                algorithm: Algorithm::MegolmV1AesSha2,
                ciphertext: "ciphertext".to_string(),
                sender_key: "sender_key".to_string(),
                device_id: "device_id".to_string(),
                session_id: "session_id".to_string(),
            });

        assert_eq!(
            to_string(&key_verification_start_content).unwrap(),
            r#"{"algorithm":"m.megolm.v1.aes-sha2","ciphertext":"ciphertext","sender_key":"sender_key","device_id":"device_id","session_id":"session_id"}"#
        );
    }

    #[test]
    fn deserialization() {
        let key_verification_start_content =
            EncryptedEventContent::MegolmV1AesSha2(MegolmV1AesSha2Content {
                algorithm: Algorithm::MegolmV1AesSha2,
                ciphertext: "ciphertext".to_string(),
                sender_key: "sender_key".to_string(),
                device_id: "device_id".to_string(),
                session_id: "session_id".to_string(),
            });

        assert_eq!(
            r#"{"algorithm":"m.megolm.v1.aes-sha2","ciphertext":"ciphertext","sender_key":"sender_key","device_id":"device_id","session_id":"session_id"}"#
            .parse::<EncryptedEventContent>()
            .unwrap(),
            key_verification_start_content
        );
    }

    #[test]
    fn deserialization_failure() {
        assert!(
            r#"{"algorithm":"m.megolm.v1.aes-sha2"}"#.parse::<EncryptedEventContent>().is_err()
        );
    }
}
