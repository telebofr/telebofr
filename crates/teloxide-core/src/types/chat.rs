use serde::{Deserialize, Serialize};

use crate::types::ChatId;

/// This object represents a chat.
///
/// [The official docs](https://core.telegram.org/bots/api#chat).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Chat {
    /// A unique identifier for this chat.
    pub id: ChatId,

    #[serde(flatten)]
    pub kind: ChatKind,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatKind {
    Public(ChatPublic),
    Private(ChatPrivate),
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatPublic {
    /// A title, for supergroups, channels and group chats.
    pub title: Option<String>,

    #[serde(flatten)]
    pub kind: PublicChatKind,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(from = "serde_helper::ChatPrivate", into = "serde_helper::ChatPrivate")]
pub struct ChatPrivate {
    /// A username, for private chats, supergroups and channels if
    /// available.
    pub username: Option<String>,

    /// A first name of the other party in a private chat.
    pub first_name: Option<String>,

    /// A last name of the other party in a private chat.
    pub last_name: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum PublicChatKind {
    Channel(PublicChatChannel),
    Group,
    Supergroup(PublicChatSupergroup),
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct PublicChatChannel {
    /// A username, for private chats, supergroups and channels if available.
    pub username: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PublicChatSupergroup {
    /// A username, for private chats, supergroups and channels if
    /// available.
    pub username: Option<String>,

    /// `true`, if the supergroup chat is a forum (has topics enabled).
    #[serde(default)]
    pub is_forum: bool,
}

impl Chat {
    #[must_use]
    pub fn is_private(&self) -> bool {
        matches!(self.kind, ChatKind::Private(_))
    }

    #[must_use]
    pub fn is_group(&self) -> bool {
        if let ChatKind::Public(chat_pub) = &self.kind {
            matches!(*chat_pub, ChatPublic { kind: PublicChatKind::Group, .. })
        } else {
            false
        }
    }

    #[must_use]
    pub fn is_supergroup(&self) -> bool {
        if let ChatKind::Public(chat_pub) = &self.kind {
            matches!(*chat_pub, ChatPublic { kind: PublicChatKind::Supergroup(_), .. })
        } else {
            false
        }
    }

    #[must_use]
    pub fn is_channel(&self) -> bool {
        if let ChatKind::Public(chat_pub) = &self.kind {
            matches!(*chat_pub, ChatPublic { kind: PublicChatKind::Channel(_), .. })
        } else {
            false
        }
    }

    #[must_use]
    pub fn is_chat(&self) -> bool {
        self.is_private() || self.is_group() || self.is_supergroup()
    }
}

/// Getters
impl Chat {
    /// A title, for supergroups, channels and group chats.
    #[must_use]
    pub fn title(&self) -> Option<&str> {
        match &self.kind {
            ChatKind::Public(this) => this.title.as_deref(),
            _ => None,
        }
    }

    /// A username, for private chats, supergroups and channels if available.
    #[must_use]
    pub fn username(&self) -> Option<&str> {
        match &self.kind {
            ChatKind::Public(this) => match &this.kind {
                PublicChatKind::Channel(PublicChatChannel { username, .. })
                | PublicChatKind::Supergroup(PublicChatSupergroup { username, .. }) => {
                    username.as_deref()
                }
                PublicChatKind::Group => None,
            },
            ChatKind::Private(this) => this.username.as_deref(),
        }
    }

    /// A first name of the other party in a private chat.
    #[must_use]
    pub fn first_name(&self) -> Option<&str> {
        match &self.kind {
            ChatKind::Private(this) => this.first_name.as_deref(),
            _ => None,
        }
    }

    /// A last name of the other party in a private chat.
    #[must_use]
    pub fn last_name(&self) -> Option<&str> {
        match &self.kind {
            ChatKind::Private(this) => this.last_name.as_deref(),
            _ => None,
        }
    }
}

mod serde_helper {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    enum Type {
        #[allow(non_camel_case_types)]
        private,
    }

    #[derive(Serialize, Deserialize)]
    pub(super) struct ChatPrivate {
        /// A dummy field. Used to ensure that the `type` field is equal to
        /// `private`.
        r#type: Type,

        username: Option<String>,
        first_name: Option<String>,
        last_name: Option<String>,
    }

    impl From<ChatPrivate> for super::ChatPrivate {
        fn from(ChatPrivate { r#type: _, username, first_name, last_name }: ChatPrivate) -> Self {
            Self { username, first_name, last_name }
        }
    }

    impl From<super::ChatPrivate> for ChatPrivate {
        fn from(
            super::ChatPrivate { username, first_name, last_name }: super::ChatPrivate,
        ) -> Self {
            Self { r#type: Type::private, username, first_name, last_name }
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{from_str, to_string};

    use crate::types::*;

    #[test]
    fn channel_de() {
        let expected = Chat {
            id: ChatId(-1),
            kind: ChatKind::Public(ChatPublic {
                title: None,
                kind: PublicChatKind::Channel(PublicChatChannel {
                    username: Some("channel_name".into()),
                }),
            }),
        };
        let actual = from_str(
            r#"{
                "id": -1,
                "type": "channel",
                "username": "channel_name"
            }"#,
        )
        .unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn private_chat_de() {
        assert_eq!(
            Chat {
                id: ChatId(0),
                kind: ChatKind::Private(ChatPrivate {
                    username: Some("username".into()),
                    first_name: Some("Anon".into()),
                    last_name: None,
                }),
            },
            from_str(
                r#"{
                    "id": 0,
                    "type": "private",
                    "username": "username",
                    "first_name": "Anon"
                }"#
            )
            .unwrap()
        );
    }

    #[test]
    fn private_roundtrip() {
        let chat = Chat {
            id: ChatId(0),
            kind: ChatKind::Private(ChatPrivate {
                username: Some("username".into()),
                first_name: Some("Anon".into()),
                last_name: None,
            }),
        };

        let json = to_string(&chat).unwrap();
        let chat2 = from_str::<Chat>(&json).unwrap();

        assert_eq!(chat, chat2);
    }

    #[test]
    fn private_chat_de_wrong_type_field() {
        assert!(from_str::<Chat>(r#"{"id":0,"type":"WRONG"}"#).is_err());
    }
}
