use serde::{Deserialize, Serialize};

use crate::types::{MessageEntity, ParseMode};

/// This object contains information about one answer option in a poll to send.
///
/// [The official docs](https://core.telegram.org/bots/api#inputpolloption).
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
pub struct InputPollOption {
    /// Option text, 1-100 characters.
    pub text: String,

    /// Mode for parsing entities in the text. See [formatting options] for more
    /// details. Currently, only custom emoji entities are allowed.
    ///
    /// [formatting options]: https://core.telegram.org/bots/api#formatting-options
    pub text_parse_mode: Option<ParseMode>,

    /// A JSON-serialized list of special entities that appear in the poll
    /// option text. It can be specified instead of _text\_parse\_mode_.
    pub text_entities: Option<Vec<MessageEntity>>,
}

impl InputPollOption {
    pub fn new(text: String) -> Self {
        Self { text, text_parse_mode: None, text_entities: None }
    }

    pub fn text_parse_mode(self, text_parse_mode: ParseMode) -> Self {
        Self { text_parse_mode: Some(text_parse_mode), ..self }
    }

    pub fn text_entities(self, text_entities: Vec<MessageEntity>) -> Self {
        Self { text_entities: Some(text_entities), ..self }
    }
}

impl From<String> for InputPollOption {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&str> for InputPollOption {
    fn from(value: &str) -> Self {
        Self::new(value.to_owned())
    }
}
