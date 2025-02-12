#![allow(clippy::large_enum_variant)]
use serde::{de::MapAccess, Deserialize, Serialize, Serializer};
use serde_json::Value;

use crate::types::{
    BusinessConnection, BusinessMessagesDeleted, CallbackQuery, Chat, ChatBoostRemoved,
    ChatBoostUpdated, ChatJoinRequest, ChatMemberUpdated, ChosenInlineResult, InlineQuery, Message,
    MessageReactionCountUpdated, MessageReactionUpdated, Poll, PollAnswer, PreCheckoutQuery,
    ShippingQuery, User,
};

/// This [object] represents an incoming update.
///
/// [The official docs](https://core.telegram.org/bots/api#update).
///
/// [object]: https://core.telegram.org/bots/api#available-types
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Update {
    /// The update‘s unique identifier. Update identifiers start from a certain
    /// positive number and increase sequentially. This ID becomes especially
    /// handy if you’re using webhooks, since it allows you to ignore
    /// repeated updates or to restore the correct update sequence, should
    /// they get out of order. If there are no new updates for at least a
    /// week, then identifier of the next update will be chosen randomly
    /// instead of sequentially.
    #[serde(rename = "update_id")]
    pub id: UpdateId,

    #[serde(flatten)]
    pub kind: UpdateKind,
}

/// An identifier of a telegram update.
///
/// See [`Update::id`] for more information.
#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct UpdateId(pub u32);

#[derive(Clone, Debug, PartialEq)]
pub enum UpdateKind {
    // NB: When adding new variants, don't forget to update
    //     - `AllowedUpdate`
    //     - `Update::user`
    //     - `Update::chat`
    //     - `DpHandlerDescription::full_set`
    //     - `dispatching/filter_ext.rs`
    /// New incoming message of any kind — text, photo, sticker, etc.
    Message(Message),

    /// New version of a message that is known to the bot and was edited.
    EditedMessage(Message),

    /// New incoming channel post of any kind — text, photo, sticker, etc.
    ChannelPost(Message),

    /// New version of a channel post that is known to the bot and was edited.
    EditedChannelPost(Message),

    /// The bot was connected to or disconnected from a business account, or a
    /// user edited an existing connection with the bot
    BusinessConnection(BusinessConnection),

    /// New non-service message from a connected business account
    BusinessMessage(Message),

    /// New version of a message from a connected business account
    EditedBusinessMessage(Message),

    /// Messages were deleted from a connected business account
    DeletedBusinessMessages(BusinessMessagesDeleted),

    /// A reaction to a message was changed by a user. The bot must be an
    /// administrator in the chat and must explicitly specify
    /// [`AllowedUpdate::MessageReaction`] in the list of `allowed_updates`
    /// to receive these updates. The update isn't received for reactions
    /// set by bots.
    ///
    /// [`AllowedUpdate::MessageReaction`]: crate::types::AllowedUpdate::MessageReaction
    MessageReaction(MessageReactionUpdated),

    /// Reactions to a message with anonymous reactions were changed. The bot
    /// must be an administrator in the chat and must explicitly specify
    /// [`AllowedUpdate::MessageReactionCount`] in the list of `allowed_updates`
    /// to receive these updates. The updates are grouped and can be sent
    /// with delay up to a few minutes.
    ///
    /// [`AllowedUpdate::MessageReactionCount`]: crate::types::AllowedUpdate::MessageReactionCount
    MessageReactionCount(MessageReactionCountUpdated),

    /// New incoming [inline] query.
    ///
    /// [inline]: https://core.telegram.org/bots/api#inline-mode
    InlineQuery(InlineQuery),

    /// The result of an [inline] query that was chosen by a user and sent to
    /// their chat partner. Please see our documentation on the [feedback
    /// collecting] for details on how to enable these updates for your bot.
    ///
    /// [inline]: https://core.telegram.org/bots/api#inline-mode
    /// [feedback collecting]: https://core.telegram.org/bots/inline#collecting-feedback
    ChosenInlineResult(ChosenInlineResult),

    /// New incoming callback query.
    CallbackQuery(CallbackQuery),

    /// New incoming shipping query. Only for invoices with flexible price.
    ShippingQuery(ShippingQuery),

    /// New incoming pre-checkout query. Contains full information about
    /// checkout.
    PreCheckoutQuery(PreCheckoutQuery),

    /// New poll state. Bots receive only updates about stopped polls and
    /// polls, which are sent by the bot.
    Poll(Poll),

    /// A user changed their answer in a non-anonymous poll. Bots receive new
    /// votes only in polls that were sent by the bot itself.
    PollAnswer(PollAnswer),

    /// The bot's chat member status was updated in a chat. For private chats,
    /// this update is received only when the bot is blocked or unblocked by the
    /// user.
    MyChatMember(ChatMemberUpdated),

    /// A chat member's status was updated in a chat. The bot must be an
    /// administrator in the chat and must explicitly specify
    /// [`AllowedUpdate::ChatMember`] in the list of `allowed_updates` to
    /// receive these updates.
    ///
    /// [`AllowedUpdate::ChatMember`]: crate::types::AllowedUpdate::ChatMember
    ChatMember(ChatMemberUpdated),

    /// A request to join the chat has been sent. The bot must have the
    /// can_invite_users administrator right in the chat to receive these
    /// updates.
    ChatJoinRequest(ChatJoinRequest),

    /// A chat boost was added or changed. The bot must be an administrator in
    /// the chat to receive these updates.
    ChatBoost(ChatBoostUpdated),

    /// A boost was removed from a chat. The bot must be an administrator in the
    /// chat to receive these updates.
    RemovedChatBoost(ChatBoostRemoved),

    /// An error that happened during deserialization.
    ///
    /// This allows `teloxide` to continue working even if telegram adds a new
    /// kinds of updates.
    ///
    /// **Note that deserialize implementation always returns an empty value**,
    /// teloxide fills in the data when doing deserialization.
    Error(Value),
}

impl Update {
    /// Returns the user that performed the action that caused this update, if
    /// known.
    ///
    /// This is generally the `from` field (except for `PollAnswer` where it's
    /// `user` and `Poll` with `Error` which don't have such field at all).
    #[must_use]
    pub fn from(&self) -> Option<&User> {
        use UpdateKind::*;

        let from = match &self.kind {
            Message(m)
            | EditedMessage(m)
            | ChannelPost(m)
            | EditedChannelPost(m)
            | BusinessMessage(m)
            | EditedBusinessMessage(m) => m.from.as_ref()?,

            BusinessConnection(conn) => &conn.user,

            CallbackQuery(query) => &query.from,
            ChosenInlineResult(chosen) => &chosen.from,
            MessageReaction(reaction) => return reaction.user(),
            InlineQuery(query) => &query.from,
            ShippingQuery(query) => &query.from,
            PreCheckoutQuery(query) => &query.from,
            PollAnswer(answer) => return answer.voter.user(),

            MyChatMember(m) | ChatMember(m) => &m.from,
            ChatJoinRequest(r) => &r.from,
            ChatBoost(b) => return b.boost.source.user(),
            RemovedChatBoost(b) => return b.source.user(),

            MessageReactionCount(_) | DeletedBusinessMessages(_) | Poll(_) | Error(_) => {
                return None
            }
        };

        Some(from)
    }

    /// Returns all users that are "contained" in this `Update` structure.
    ///
    /// This might be useful to track information about users.
    ///
    /// Note that this function may return quite a few users as it scans
    /// replies, pinned messages, message entities, "via bot" fields and more.
    /// Also note that this function can return duplicate users.
    pub fn mentioned_users(&self) -> impl Iterator<Item = &User> {
        use either::Either::{Left as L, Right as R};
        use std::iter::{empty, once};

        //          [root]
        //         /      \
        // left - /        \ - right
        //       /          \
        //      /\          /\
        //     /  \        /  \
        //    /    \      /    \
        //   0     /\    /\    /\
        //        /  \  /  \  /  \
        //       1    2 3  4  5  6
        //
        // 0 = LL
        // 1 = LRL
        // 2 = LRR
        // 3 = RLL
        // 4 = RLR
        // 5 = RRL
        // 6 = RRR

        let i0 = |x| L(L(x));
        let i1 = |x| L(R(L(x)));
        let i2 = |x| L(R(R(x)));
        let i3 = |x| R(L(L(x)));
        let i4 = |x| R(L(R(x)));
        let i5 = |x| R(R(L(x)));
        let i6 = |x| R(R(R(x)));

        match &self.kind {
            UpdateKind::Message(message)
            | UpdateKind::EditedMessage(message)
            | UpdateKind::ChannelPost(message)
            | UpdateKind::EditedChannelPost(message)
            | UpdateKind::BusinessMessage(message)
            | UpdateKind::EditedBusinessMessage(message) => i0(message.mentioned_users()),

            UpdateKind::MessageReaction(answer) => {
                if let Some(user) = answer.user() {
                    return i1(once(user));
                }
                i6(empty())
            }

            UpdateKind::InlineQuery(query) => i1(once(&query.from)),
            UpdateKind::ChosenInlineResult(query) => i1(once(&query.from)),
            UpdateKind::CallbackQuery(query) => i2(query.mentioned_users()),
            UpdateKind::ShippingQuery(query) => i1(once(&query.from)),
            UpdateKind::PreCheckoutQuery(query) => i1(once(&query.from)),
            UpdateKind::Poll(poll) => i3(poll.mentioned_users()),

            UpdateKind::PollAnswer(answer) => {
                if let Some(user) = answer.voter.user() {
                    return i1(once(user));
                }
                i6(empty())
            }

            UpdateKind::MyChatMember(member) | UpdateKind::ChatMember(member) => {
                i4(member.mentioned_users())
            }
            UpdateKind::ChatJoinRequest(request) => i5(request.mentioned_users()),

            UpdateKind::ChatBoost(b) => {
                if let Some(user) = b.boost.source.user() {
                    return i1(once(user));
                }
                i6(empty())
            }
            UpdateKind::RemovedChatBoost(b) => {
                if let Some(user) = b.source.user() {
                    return i1(once(user));
                }
                i6(empty())
            }

            UpdateKind::MessageReactionCount(_)
            | UpdateKind::BusinessConnection(_)
            | UpdateKind::DeletedBusinessMessages(_)
            | UpdateKind::Error(_) => i6(empty()),
        }
    }

    /// Returns the chat in which is update has happened, if any.
    #[must_use]
    pub fn chat(&self) -> Option<&Chat> {
        use UpdateKind::*;

        let chat = match &self.kind {
            Message(m)
            | EditedMessage(m)
            | ChannelPost(m)
            | EditedChannelPost(m)
            | BusinessMessage(m)
            | EditedBusinessMessage(m) => &m.chat,
            CallbackQuery(q) => q.message.as_ref()?.chat(),
            ChatMember(m) => &m.chat,
            MyChatMember(m) => &m.chat,
            ChatJoinRequest(c) => &c.chat,
            MessageReaction(r) => &r.chat,
            MessageReactionCount(r) => &r.chat,
            ChatBoost(b) => &b.chat,
            RemovedChatBoost(b) => &b.chat,
            DeletedBusinessMessages(m) => &m.chat,

            InlineQuery(_)
            | BusinessConnection(_)
            | ChosenInlineResult(_)
            | ShippingQuery(_)
            | PreCheckoutQuery(_)
            | Poll(_)
            | PollAnswer(_)
            | Error(_) => return None,
        };

        Some(chat)
    }
}

impl UpdateId {
    /// Returns the offset for the **next** update that can be used for polling.
    ///
    /// I.e. `self.0 + 1`.
    #[must_use]
    pub fn as_offset(self) -> i32 {
        debug_assert!(self.0 < i32::MAX as u32);

        self.0 as i32 + 1
    }
}

impl<'de> Deserialize<'de> for UpdateKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = UpdateKind;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a map")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut tmp = None;

                // Try to deserialize a borrowed-str key, or else try deserializing an owned
                // string key
                let key = map.next_key::<&str>().or_else(|_| {
                    map.next_key::<String>().map(|k| {
                        tmp = k;
                        tmp.as_deref()
                    })
                });

                let this = key
                    .ok()
                    .flatten()
                    .and_then(|key| match key {
                        "message" => map.next_value::<Message>().ok().map(UpdateKind::Message),
                        "edited_message" => {
                            map.next_value::<Message>().ok().map(UpdateKind::EditedMessage)
                        }
                        "channel_post" => {
                            map.next_value::<Message>().ok().map(UpdateKind::ChannelPost)
                        }
                        "edited_channel_post" => {
                            map.next_value::<Message>().ok().map(UpdateKind::EditedChannelPost)
                        }
                        "business_connection" => map
                            .next_value::<BusinessConnection>()
                            .ok()
                            .map(UpdateKind::BusinessConnection),
                        "business_message" => {
                            map.next_value::<Message>().ok().map(UpdateKind::BusinessMessage)
                        }
                        "edited_business_message" => {
                            map.next_value::<Message>().ok().map(UpdateKind::EditedBusinessMessage)
                        }
                        "deleted_business_messages" => map
                            .next_value::<BusinessMessagesDeleted>()
                            .ok()
                            .map(UpdateKind::DeletedBusinessMessages),
                        "message_reaction" => map
                            .next_value::<MessageReactionUpdated>()
                            .ok()
                            .map(UpdateKind::MessageReaction),
                        "message_reaction_count" => map
                            .next_value::<MessageReactionCountUpdated>()
                            .ok()
                            .map(UpdateKind::MessageReactionCount),
                        "inline_query" => {
                            map.next_value::<InlineQuery>().ok().map(UpdateKind::InlineQuery)
                        }
                        "chosen_inline_result" => map
                            .next_value::<ChosenInlineResult>()
                            .ok()
                            .map(UpdateKind::ChosenInlineResult),
                        "callback_query" => {
                            map.next_value::<CallbackQuery>().ok().map(UpdateKind::CallbackQuery)
                        }
                        "shipping_query" => {
                            map.next_value::<ShippingQuery>().ok().map(UpdateKind::ShippingQuery)
                        }
                        "pre_checkout_query" => map
                            .next_value::<PreCheckoutQuery>()
                            .ok()
                            .map(UpdateKind::PreCheckoutQuery),
                        "poll" => map.next_value::<Poll>().ok().map(UpdateKind::Poll),
                        "poll_answer" => {
                            map.next_value::<PollAnswer>().ok().map(UpdateKind::PollAnswer)
                        }
                        "my_chat_member" => {
                            map.next_value::<ChatMemberUpdated>().ok().map(UpdateKind::MyChatMember)
                        }
                        "chat_member" => {
                            map.next_value::<ChatMemberUpdated>().ok().map(UpdateKind::ChatMember)
                        }
                        "chat_join_request" => map
                            .next_value::<ChatJoinRequest>()
                            .ok()
                            .map(UpdateKind::ChatJoinRequest),
                        "chat_boost" => {
                            map.next_value::<ChatBoostUpdated>().ok().map(UpdateKind::ChatBoost)
                        }
                        "removed_chat_boost" => map
                            .next_value::<ChatBoostRemoved>()
                            .ok()
                            .map(UpdateKind::RemovedChatBoost),
                        _ => Some(empty_error()),
                    })
                    .unwrap_or_else(empty_error);

                Ok(this)
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

impl Serialize for UpdateKind {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let name = "UpdateKind";
        match self {
            UpdateKind::Message(v) => s.serialize_newtype_variant(name, 0, "message", v),
            UpdateKind::EditedMessage(v) => {
                s.serialize_newtype_variant(name, 1, "edited_message", v)
            }
            UpdateKind::ChannelPost(v) => s.serialize_newtype_variant(name, 2, "channel_post", v),
            UpdateKind::EditedChannelPost(v) => {
                s.serialize_newtype_variant(name, 3, "edited_channel_post", v)
            }
            UpdateKind::BusinessConnection(v) => {
                s.serialize_newtype_variant(name, 4, "business_connection", v)
            }
            UpdateKind::BusinessMessage(v) => {
                s.serialize_newtype_variant(name, 5, "business_message", v)
            }
            UpdateKind::EditedBusinessMessage(v) => {
                s.serialize_newtype_variant(name, 6, "edited_business_message", v)
            }
            UpdateKind::DeletedBusinessMessages(v) => {
                s.serialize_newtype_variant(name, 7, "deleted_business_messages", v)
            }
            UpdateKind::MessageReaction(v) => {
                s.serialize_newtype_variant(name, 8, "message_reaction", v)
            }
            UpdateKind::MessageReactionCount(v) => {
                s.serialize_newtype_variant(name, 9, "message_reaction_count", v)
            }
            UpdateKind::InlineQuery(v) => s.serialize_newtype_variant(name, 10, "inline_query", v),
            UpdateKind::ChosenInlineResult(v) => {
                s.serialize_newtype_variant(name, 11, "chosen_inline_result", v)
            }
            UpdateKind::CallbackQuery(v) => {
                s.serialize_newtype_variant(name, 12, "callback_query", v)
            }
            UpdateKind::ShippingQuery(v) => {
                s.serialize_newtype_variant(name, 13, "shipping_query", v)
            }
            UpdateKind::PreCheckoutQuery(v) => {
                s.serialize_newtype_variant(name, 14, "pre_checkout_query", v)
            }
            UpdateKind::Poll(v) => s.serialize_newtype_variant(name, 15, "poll", v),
            UpdateKind::PollAnswer(v) => s.serialize_newtype_variant(name, 16, "poll_answer", v),
            UpdateKind::MyChatMember(v) => {
                s.serialize_newtype_variant(name, 17, "my_chat_member", v)
            }
            UpdateKind::ChatMember(v) => s.serialize_newtype_variant(name, 18, "chat_member", v),
            UpdateKind::ChatJoinRequest(v) => {
                s.serialize_newtype_variant(name, 19, "chat_join_request", v)
            }
            UpdateKind::ChatBoost(v) => s.serialize_newtype_variant(name, 20, "chat_boost", v),
            UpdateKind::RemovedChatBoost(v) => {
                s.serialize_newtype_variant(name, 21, "removed_chat_boost", v)
            }
            UpdateKind::Error(v) => v.serialize(s),
        }
    }
}

fn empty_error() -> UpdateKind {
    UpdateKind::Error(Value::Object(<_>::default()))
}

#[cfg(test)]
mod test {
    use crate::types::{
        Chat, ChatBoost, ChatBoostRemoved, ChatBoostSource, ChatBoostSourcePremium,
        ChatBoostUpdated, ChatId, ChatKind, ChatPrivate, ChatPublic, LinkPreviewOptions,
        MaybeAnonymousUser, MediaKind, MediaText, Message, MessageCommon, MessageId, MessageKind,
        MessageReactionCountUpdated, MessageReactionUpdated, PublicChatChannel, PublicChatKind,
        PublicChatSupergroup, ReactionCount, ReactionType, Update, UpdateId, UpdateKind, User,
        UserId,
    };

    use chrono::DateTime;
    use pretty_assertions::assert_eq;

    // TODO: more tests for deserialization
    #[test]
    fn message() {
        let timestamp = 1_569_518_342;
        let date = DateTime::from_timestamp(timestamp, 0).unwrap();

        let json = r#"{
            "update_id":892252934,
            "message":{
                "message_id":6557,
                "from":{
                    "id":218485655,
                    "is_bot": false,
                    "first_name":"Waffle",
                    "username":"WaffleLapkin",
                    "language_code":"en"
                },
                "chat":{
                    "id":218485655,
                    "first_name":"Waffle",
                    "username":"WaffleLapkin",
                    "type":"private"
                },
               "date":1569518342,
               "text":"hello there",
               "link_preview_options":{"is_disabled":true}
            }
        }"#;

        let expected = Update {
            id: UpdateId(892_252_934),
            kind: UpdateKind::Message(Message {
                via_bot: None,
                id: MessageId(6557),
                thread_id: None,
                from: Some(User {
                    id: UserId(218_485_655),
                    is_bot: false,
                    first_name: String::from("Waffle"),
                    last_name: None,
                    username: Some(String::from("WaffleLapkin")),
                    language_code: Some(String::from("en")),
                    is_premium: false,
                    added_to_attachment_menu: false,
                }),
                sender_chat: None,
                is_topic_message: false,
                date,
                chat: Chat {
                    id: ChatId(218_485_655),
                    kind: ChatKind::Private(ChatPrivate {
                        username: Some(String::from("WaffleLapkin")),
                        first_name: Some(String::from("Waffle")),
                        last_name: None,
                    }),
                },
                sender_business_bot: None,
                kind: MessageKind::Common(MessageCommon {
                    reply_to_message: None,
                    forward_origin: None,
                    external_reply: None,
                    quote: None,
                    reply_to_story: None,
                    sender_boost_count: None,
                    edit_date: None,
                    media_kind: MediaKind::Text(MediaText {
                        text: String::from("hello there"),
                        entities: vec![],
                        link_preview_options: Some(LinkPreviewOptions {
                            is_disabled: true,
                            url: None,
                            prefer_small_media: false,
                            prefer_large_media: false,
                            show_above_text: false,
                        }),
                    }),
                    reply_markup: None,
                    author_signature: None,
                    is_automatic_forward: false,
                    has_protected_content: false,
                    is_from_offline: false,
                    business_connection_id: None,
                }),
            }),
        };

        let actual = serde_json::from_str::<Update>(json).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn issue_1107() {
        let update = r#"{
            "message": {
                "chat": {
                    "id": -1001293752024,
                    "title": "CryptoInside Chat",
                    "type": "supergroup",
                    "username": "cryptoinside_talk"
                },
                "date": 1721592028,
                "from": {
                    "first_name": "Wert",
                    "id": 6962620676,
                    "is_bot": false,
                    "username": "WertCrypto"
                },
                "message_id": 134545,
                "story": {
                    "chat": {
                        "id": -1002149282975,
                        "title": "TON Spin",
                        "type": "channel",
                        "username": "TONSpinChannel"
                    },
                    "id": 2
                }
            },
            "update_id": 439432599
        }"#;

        let Update { kind, .. } = serde_json::from_str::<Update>(update).unwrap();
        match kind {
            UpdateKind::Message(_) => {}
            _ => panic!("Expected `Message`"),
        }

        let update = r#"{
            "message": {
                "chat": {
                    "id": -1001293752024,
                    "title": "CryptoInside Chat",
                    "type": "supergroup",
                    "username": "cryptoinside_talk"
                },
                "date": 1721592580,
                "entities": [
                    {
                        "length": 7,
                        "offset": 0,
                        "type": "bot_command"
                    }
                ],
                "from": {
                    "first_name": "the Cable Guy",
                    "id": 5964236329,
                    "is_bot": false,
                    "language_code":"en",
                    "username": "spacewhaleblues"
                },
                "message_id": 134546,
                "message_thread_id": 134545,
                "reply_to_message": {
                    "chat": {
                        "id": -1001293752024,
                        "title": "CryptoInside Chat",
                        "type": "supergroup",
                        "username": "cryptoinside_talk"
                    },
                    "date": 1721592028,
                    "from": {
                        "first_name": "Wert",
                        "id": 6962620676,
                        "is_bot": false,
                        "username": "WertCrypto"
                    },
                    "message_id": 134545,
                    "story": {
                        "chat": {
                            "id": -1002149282975,
                            "title": "TON Spin",
                            "type": "channel",
                            "username": "TONSpinChannel"
                        },
                        "id": 2
                    }
                },
                "text": "/report"
            },
            "update_id": 439432600
        }"#;

        let Update { kind, .. } = serde_json::from_str::<Update>(update).unwrap();
        match kind {
            UpdateKind::Message(_) => {}
            _ => panic!("Expected `Message`"),
        }
    }

    #[test]
    fn de_private_chat_text_message() {
        let text = r#"
  {
    "message": {
      "chat": {
        "first_name": "Hirrolot",
        "id": 408258968,
        "type": "private",
        "username": "hirrolot"
      },
      "date": 1581448857,
      "from": {
        "first_name": "Hirrolot",
        "id": 408258968,
        "is_bot": false,
        "language_code": "en",
        "username": "hirrolot"
      },
      "message_id": 154,
      "text": "4"
    },
    "update_id": 306197398
  }
"#;

        let Update { kind, .. } = serde_json::from_str::<Update>(text).unwrap();
        match kind {
            UpdateKind::Message(_) => {}
            _ => panic!("Expected `Message`"),
        }
    }

    #[test]
    fn pinned_message_works() {
        let json = r#"{
    "message": {
        "chat": {
            "id": -1001276785818,
            "title": "teloxide dev",
            "type": "supergroup",
            "username": "teloxide_dev"
        },
        "date": 1582134655,
        "from": {
            "first_name": "Hirrolot",
            "id": 408258968,
            "is_bot": false,
            "username": "hirrolot"
        },
        "message_id": 20225,
        "pinned_message": {
            "chat": {
                "id": -1001276785818,
                "title": "teloxide dev",
                "type": "supergroup",
                "username": "teloxide_dev"
            },
            "date": 1582134643,
            "from": {
                "first_name": "Hirrolot",
                "id": 408258968,
                "is_bot": false,
                "username": "hirrolot"
            },
            "message_id": 20224,
            "text": "Faster than a bullet"
        }
    },
    "update_id": 845402291
}"#;

        let Update { kind, .. } = serde_json::from_str(json).unwrap();
        match kind {
            UpdateKind::Message(_) => {}
            _ => panic!("Expected `Message`"),
        }
    }

    #[test]
    fn dice_works() {
        let json = r#"
        {
    "message": {
        "chat": {
            "id": -1001276785818,
            "title": "bla bla bla chat",
            "type": "supergroup",
            "username": "teloxide_dev"
        },
        "date": 1596014550,
        "dice": {
            "emoji": "🎲",
            "value": 2
        },
        "from": {
            "first_name": "Hirrolot",
            "id": 408258968,
            "is_bot": false,
            "language_code": "en",
            "username": "hirrolot"
        },
        "message_id": 35410
    },
    "update_id": 573255266
}
        "#;

        let Update { kind, .. } = serde_json::from_str(json).unwrap();
        match kind {
            UpdateKind::Message(_) => {}
            _ => panic!("Expected `Message`"),
        }
    }

    #[test]
    fn new_update_kind_error() {
        let json = r#"{
            "new_update_kind": {"some_field_idk": 1},
            "update_id": 1
        }"#;

        let Update { kind, .. } = serde_json::from_str(json).unwrap();

        match kind {
            // Deserialization failed successfully
            UpdateKind::Error(_) => {}
            _ => panic!("Expected error"),
        }
    }

    #[test]
    fn issue_523() {
        let json = r#"{
            "update_id":0,
            "my_chat_member": {
                "chat":{"id":0,"first_name":"FN","last_name":"LN","username":"UN","type":"private"},
                "from":{"id":0,"is_bot":false,"first_name":"FN","last_name":"LN","username":"UN"},
                "date":1644677726,
                "old_chat_member":{"user":{"id":1,"is_bot":true,"first_name":"bot","username":"unBot"},"status":"member"},
                "new_chat_member":{"user":{"id":1,"is_bot":true,"first_name":"bot","username":"unBot"},"status":"kicked","until_date":0}
            }
        }"#;

        let Update { kind, .. } = serde_json::from_str(json).unwrap();

        match kind {
            UpdateKind::MyChatMember(_) => {}
            _ => panic!("Expected `MyChatMember`"),
        }
    }

    #[test]
    fn message_reaction_updated() {
        let json = r#"
        {
            "update_id": 71651249,
            "message_reaction": {
                "chat": {
                    "id": -1002184233434,
                    "title": "Test",
                    "type": "supergroup"
                },
                "message_id": 35,
                "user": {
                    "id": 1459074222,
                    "is_bot": false,
                    "first_name": "shadowchain",
                    "username": "shdwchn10",
                    "language_code": "en",
                    "is_premium": true
                },
                "date": 1721306082,
                "old_reaction": [],
                "new_reaction": [
                    {
                        "type": "emoji",
                        "emoji": "🌭"
                    }
                ]
            }
        }
        "#;

        let expected = Update {
            id: UpdateId(71651249),
            kind: UpdateKind::MessageReaction(MessageReactionUpdated {
                chat: Chat {
                    id: ChatId(-1002184233434),
                    kind: ChatKind::Public(ChatPublic {
                        title: Some("Test".to_owned()),
                        kind: PublicChatKind::Supergroup(PublicChatSupergroup {
                            username: None,
                            is_forum: false,
                        }),
                    }),
                },
                message_id: MessageId(35),
                actor: MaybeAnonymousUser::User(User {
                    id: UserId(1459074222),
                    is_bot: false,
                    first_name: "shadowchain".to_owned(),
                    last_name: None,
                    username: Some("shdwchn10".to_owned()),
                    language_code: Some("en".to_owned()),
                    is_premium: true,
                    added_to_attachment_menu: false,
                }),
                date: DateTime::from_timestamp(1721306082, 0).unwrap(),
                old_reaction: vec![],
                new_reaction: vec![ReactionType::Emoji { emoji: "🌭".to_owned() }],
            }),
        };

        let actual = serde_json::from_str::<Update>(json).unwrap();
        assert_eq!(expected, actual);

        let json = r#"
        {
            "update_id": 767844136,
            "message_reaction": {
                "chat": {
                    "id": -1002199793788,
                    "title": "тест",
                    "type": "supergroup"
                },
                "message_id": 2,
                "actor_chat": {
                    "id": -1002199793788,
                    "title": "тест",
                    "type": "supergroup"
                },
                "date": 1723798597,
                "old_reaction": [
                    {
                        "type": "emoji",
                        "emoji": "❤"
                    }
                ],
                "new_reaction": []
            }
        }
        "#;
        let chat = Chat {
            id: ChatId(-1002199793788),
            kind: ChatKind::Public(ChatPublic {
                title: Some("тест".to_owned()),
                kind: PublicChatKind::Supergroup(PublicChatSupergroup {
                    username: None,
                    is_forum: false,
                }),
            }),
        };
        let expected = Update {
            id: UpdateId(767844136),
            kind: UpdateKind::MessageReaction(MessageReactionUpdated {
                chat: chat.clone(),
                message_id: MessageId(2),
                actor: MaybeAnonymousUser::Chat(chat),
                date: DateTime::from_timestamp(1723798597, 0).unwrap(),
                old_reaction: vec![ReactionType::Emoji { emoji: "❤".to_owned() }],
                new_reaction: vec![],
            }),
        };

        let actual = serde_json::from_str::<Update>(json).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn message_reaction_count_updated() {
        let json = r#"
        {
            "update_id": 71651251,
            "message_reaction_count": {
                "chat": {
                    "id": -1002236736395,
                    "title": "Test",
                    "type": "channel"
                },
                "message_id": 36,
                "date": 1721306391,
                "reactions": [
                    {
                        "type": {
                            "type": "emoji",
                            "emoji": "🗿"
                        },
                        "total_count": 2
                    },
                    {
                        "type": {
                            "type": "emoji",
                            "emoji": "🌭"
                        },
                        "total_count": 1
                    }
                ]
            }
        }
        "#;

        let expected = Update {
            id: UpdateId(71651251),
            kind: UpdateKind::MessageReactionCount(MessageReactionCountUpdated {
                chat: Chat {
                    id: ChatId(-1002236736395),
                    kind: ChatKind::Public(ChatPublic {
                        title: Some("Test".to_owned()),
                        kind: PublicChatKind::Channel(PublicChatChannel { username: None }),
                    }),
                },
                message_id: MessageId(36),
                date: DateTime::from_timestamp(1721306391, 0).unwrap(),
                reactions: vec![
                    ReactionCount {
                        r#type: ReactionType::Emoji { emoji: "🗿".to_owned() },
                        total_count: 2,
                    },
                    ReactionCount {
                        r#type: ReactionType::Emoji { emoji: "🌭".to_owned() },
                        total_count: 1,
                    },
                ],
            }),
        };

        let actual = serde_json::from_str::<Update>(json).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn chat_boost_updated() {
        let json = r#"
        {
            "update_id": 71651297,
            "chat_boost": {
                "chat": {
                    "id": -1002236736395,
                    "title": "Test",
                    "type": "channel"
                },
                "boost": {
                    "boost_id": "4506e1b7e866e33fcbde78fe1746ec3a",
                    "add_date": 1721399621,
                    "expiration_date": 1745088963,
                    "source": {
                        "source": "premium",
                        "user": {
                            "id": 1459074222,
                            "is_bot": false,
                            "first_name": "shadowchain",
                            "username": "shdwchn10",
                            "language_code": "en",
                            "is_premium": true
                        }
                    }
                }
            }
        }
        "#;

        let expected = Update {
            id: UpdateId(71651297),
            kind: UpdateKind::ChatBoost(ChatBoostUpdated {
                chat: Chat {
                    id: ChatId(-1002236736395),
                    kind: ChatKind::Public(ChatPublic {
                        title: Some("Test".to_owned()),
                        kind: PublicChatKind::Channel(PublicChatChannel { username: None }),
                    }),
                },
                boost: ChatBoost {
                    boost_id: "4506e1b7e866e33fcbde78fe1746ec3a".to_owned(),
                    add_date: DateTime::from_timestamp(1721399621, 0).unwrap(),
                    expiration_date: DateTime::from_timestamp(1745088963, 0).unwrap(),
                    source: ChatBoostSource::Premium(ChatBoostSourcePremium {
                        user: User {
                            id: UserId(1459074222),
                            is_bot: false,
                            first_name: "shadowchain".to_owned(),
                            last_name: None,
                            username: Some("shdwchn10".to_owned()),
                            language_code: Some("en".to_owned()),
                            is_premium: true,
                            added_to_attachment_menu: false,
                        },
                    }),
                },
            }),
        };

        let actual = serde_json::from_str::<Update>(json).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn chat_boost_removed() {
        let json = r#"
        {
            "update_id": 71651297,
            "removed_chat_boost": {
                "chat": {
                    "id": -1002236736395,
                    "title": "Test",
                    "type": "channel"
                },
                "boost_id": "4506e1b7e866e33fcbde78fe1746ec3a",
                "remove_date": 1721999621,
                "source": {
                    "source": "premium",
                    "user": {
                        "id": 1459074222,
                        "is_bot": false,
                        "first_name": "shadowchain",
                        "username": "shdwchn10",
                        "language_code": "en",
                        "is_premium": true
                    }
                }
            }
        }
        "#;

        let expected = Update {
            id: UpdateId(71651297),
            kind: UpdateKind::RemovedChatBoost(ChatBoostRemoved {
                chat: Chat {
                    id: ChatId(-1002236736395),
                    kind: ChatKind::Public(ChatPublic {
                        title: Some("Test".to_owned()),
                        kind: PublicChatKind::Channel(PublicChatChannel { username: None }),
                    }),
                },
                boost_id: "4506e1b7e866e33fcbde78fe1746ec3a".to_owned(),
                remove_date: DateTime::from_timestamp(1721999621, 0).unwrap(),
                source: ChatBoostSource::Premium(ChatBoostSourcePremium {
                    user: User {
                        id: UserId(1459074222),
                        is_bot: false,
                        first_name: "shadowchain".to_owned(),
                        last_name: None,
                        username: Some("shdwchn10".to_owned()),
                        language_code: Some("en".to_owned()),
                        is_premium: true,
                        added_to_attachment_menu: false,
                    },
                }),
            }),
        };

        let actual = serde_json::from_str::<Update>(json).unwrap();
        assert_eq!(expected, actual);
    }
}
