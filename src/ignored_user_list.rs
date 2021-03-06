//! Types for the *m.ignored_user_list* event.

use std::{collections::HashMap, convert::TryFrom, str::FromStr};

use ruma_identifiers::UserId;
use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};

use crate::{Empty, Event, EventType, InnerInvalidEvent, InvalidEvent};

/// A list of users to ignore.
#[derive(Clone, Debug, PartialEq)]
pub struct IgnoredUserListEvent {
    /// The event's content.
    pub content: IgnoredUserListEventContent,
}

/// The payload for `IgnoredUserListEvent`.
#[derive(Clone, Debug, PartialEq)]
pub struct IgnoredUserListEventContent {
    /// A list of users to ignore.
    pub ignored_users: Vec<UserId>,
}

impl FromStr for IgnoredUserListEvent {
    type Err = InvalidEvent;

    /// Attempt to create `Self` from parsing a string of JSON data.
    fn from_str(json: &str) -> Result<Self, InvalidEvent> {
        let raw = match serde_json::from_str::<raw::IgnoredUserListEvent>(json) {
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

        Ok(Self {
            content: IgnoredUserListEventContent {
                ignored_users: raw.content.ignored_users.keys().cloned().collect(),
            },
        })
    }
}

impl<'a> TryFrom<&'a str> for IgnoredUserListEvent {
    type Error = InvalidEvent;

    /// Attempt to create `Self` from parsing a string of JSON data.
    fn try_from(json: &'a str) -> Result<Self, Self::Error> {
        FromStr::from_str(json)
    }
}

impl Serialize for IgnoredUserListEvent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("IgnoredUserListEvent", 2)?;

        state.serialize_field("content", &self.content)?;
        state.serialize_field("type", &self.event_type())?;

        state.end()
    }
}

impl_event!(
    IgnoredUserListEvent,
    IgnoredUserListEventContent,
    EventType::IgnoredUserList
);

impl FromStr for IgnoredUserListEventContent {
    type Err = InvalidEvent;

    /// Attempt to create `Self` from parsing a string of JSON data.
    fn from_str(json: &str) -> Result<Self, Self::Err> {
        let raw = match serde_json::from_str::<raw::IgnoredUserListEventContent>(json) {
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

        Ok(Self {
            ignored_users: raw.ignored_users.keys().cloned().collect(),
        })
    }
}

impl<'a> TryFrom<&'a str> for IgnoredUserListEventContent {
    type Error = InvalidEvent;

    /// Attempt to create `Self` from parsing a string of JSON data.
    fn try_from(json: &'a str) -> Result<Self, Self::Error> {
        FromStr::from_str(json)
    }
}

impl Serialize for IgnoredUserListEventContent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = HashMap::new();

        for user_id in &self.ignored_users {
            map.insert(user_id.clone(), Empty);
        }

        let raw = raw::IgnoredUserListEventContent { ignored_users: map };

        raw.serialize(serializer)
    }
}

mod raw {
    use super::*;
    use crate::Empty;

    /// A list of users to ignore.
    #[derive(Clone, Debug, Deserialize)]
    pub struct IgnoredUserListEvent {
        /// The event's content.
        pub content: IgnoredUserListEventContent,
    }

    /// The payload for `IgnoredUserListEvent`.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct IgnoredUserListEventContent {
        /// A list of users to ignore.
        pub ignored_users: HashMap<UserId, Empty>,
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use ruma_identifiers::UserId;

    use super::{IgnoredUserListEvent, IgnoredUserListEventContent};

    #[test]
    fn serialization() {
        let ignored_user_list_event = IgnoredUserListEvent {
            content: IgnoredUserListEventContent {
                ignored_users: vec![UserId::try_from("@carl:example.com").unwrap()],
            },
        };

        let json = serde_json::to_string(&ignored_user_list_event).unwrap();

        assert_eq!(json, r#"{"content":{"ignored_users":{"@carl:example.com":{}}},"type":"m.ignored_user_list"}"#);
    }

    #[test]
    fn deserialization() {
        let json = r#"{"content":{"ignored_users":{"@carl:example.com":{}}},"type":"m.ignored_user_list"}"#;

        let actual: IgnoredUserListEvent = json.parse().unwrap();

        let expected = IgnoredUserListEvent {
            content: IgnoredUserListEventContent {
                ignored_users: vec![UserId::try_from("@carl:example.com").unwrap()],
            },
        };

        assert_eq!(actual, expected);
    }
}
