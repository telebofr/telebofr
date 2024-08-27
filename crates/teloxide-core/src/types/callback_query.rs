use serde::{Deserialize, Serialize};

use crate::types::{MaybeInaccessibleMessage, Message, User};

TelegramStringId!(
    #[doc = "An unique identifier for this query."]
    CallbackQueryId
);

/// This object represents an incoming callback query from a callback button in
/// an [inline keyboard].
///
/// If the button that originated the query was attached to a message sent by
/// the bot, the field message will be present. If the button was attached to a
/// message sent via the bot (in [inline mode]), the field `inline_message_id`
/// will be present. Exactly one of the fields data or `game_short_name` will be
/// present.
///
/// [The official docs](https://core.telegram.org/bots/api#callbackquery).
///
/// [inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
/// [inline mode]: https://core.telegram.org/bots/api#inline-mode
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CallbackQuery {
    /// An unique identifier for this query.
    pub id: Box<CallbackQueryId>,

    /// A sender.
    pub from: User,

    /// Message sent by the bot with the callback button that originated the
    /// query.
    ///
    /// Note: if the message is too old, it will be
    /// [`MaybeInaccessibleMessage::Inaccessible`].
    pub message: Option<MaybeInaccessibleMessage>,

    /// An identifier of the message sent via the bot in inline mode, that
    /// originated the query.
    pub inline_message_id: Option<String>,

    /// A global identifier, uniquely corresponding to the chat to which the
    /// message with the callback button was sent. Useful for high scores in
    /// [games].
    ///
    /// [games]: https://core.telegram.org/bots/api#games
    pub chat_instance: String,

    /// A data associated with the callback button.
    pub data: Option<String>,

    /// A short name of a Game to be returned, serves as the unique identifier
    /// for the game.
    pub game_short_name: Option<String>,
}

impl CallbackQuery {
    /// Returns all users that are "contained" in this `CallbackQuery`
    /// structure.
    ///
    /// This might be useful to track information about users.
    /// Note that this function can return duplicate users.
    pub fn mentioned_users(&self) -> impl Iterator<Item = &User> {
        use crate::util::flatten;
        use std::iter::once;

        once(&self.from).chain(flatten(self.regular_message().map(Message::mentioned_users)))
    }

    #[must_use]
    pub fn regular_message(&self) -> Option<&Message> {
        self.message
            .as_ref()
            // If we can access the message
            .and_then(|maybe| maybe.regular_message())
    }
}

#[cfg(test)]
mod tests {
    use crate::types::UserId;

    use super::*;

    #[test]
    fn deserialize() {
        let json = r#"{
            "id":"id",
            "from":{
                "id":12345,
                "is_bot":false,
                "first_name":"firstName"
            },
            "inline_message_id":"i_m_id",
            "chat_instance":"123456",
            "data":"some_data",
            "game_short_name":"game_name"
        }"#;
        let expected = CallbackQuery {
            id: CallbackQueryId::from_str("id"),
            from: User {
                id: UserId(12345),
                is_bot: false,
                first_name: "firstName".to_string(),
                last_name: None,
                username: None,
                language_code: None,
                is_premium: false,
                added_to_attachment_menu: false,
            },
            chat_instance: "123456".to_string(),
            message: None,
            inline_message_id: Some("i_m_id".to_string()),
            data: Some("some_data".to_string()),
            game_short_name: Some("game_name".to_string()),
        };
        let actual = serde_json::from_str::<CallbackQuery>(json).unwrap();
        assert_eq!(actual, expected);
    }
}
