//! Docs later
use std::fmt::{Display, Formatter};

use super::command::ParseError;
use teloxide_core::types::InlineKeyboardButton;
#[cfg(feature = "macros")]
pub use teloxide_macros::InlineButtons;

/// Docs later
pub trait InlineButtons: Sized {
    /// Parses the callback data.
    fn parse(s: &str) -> Result<Self, ParseError>;

    /// Stringifies the callback data.
    fn stringify(self) -> Result<String, StringifyError>;

    /// Builds an [`InlineKeyboardButton`] from the enum variant
    ///
    /// [`InlineKeyboardButton`]: crate::types::InlineKeyboardButton
    fn build_button<T>(self, text: T) -> Result<InlineKeyboardButton, StringifyError>
    where
        T: Into<String>,
    {
        let callback_data = self.stringify()?;
        Ok(InlineKeyboardButton::callback(text.into(), callback_data))
    }
}

/// Errors returned from [`InlineButtons::stringify`].
///
/// [`InlineButtons::stringify`]: InlineButtons::stringify
#[derive(Debug)]
pub enum StringifyError {
    SeparatorInNamedArgument {
        enum_variant: String,
        stringified_data: String,
        separator: String,
        argument: String,
    },
    SeparatorInUnnamedArgument {
        enum_variant: String,
        stringified_data: String,
        separator: String,
        field: usize,
    },
}

fn make_pointers_string(prefix: String, text: String, point_to: String) -> String {
    let all_indexes: Vec<_> = text.match_indices(&point_to).collect();
    let prefix_len = prefix.chars().count();
    let mut pointers_vec = vec![" "; prefix_len + text.chars().count()];

    for (start, matched) in &all_indexes {
        for (i, _) in matched.chars().enumerate() {
            pointers_vec[prefix_len + start + i] = "^";
        }
    }
    pointers_vec.join("")
}

impl Display for StringifyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::SeparatorInNamedArgument {
                enum_variant,
                stringified_data,
                separator,
                argument,
            } => {
                let prefix = format!("self.{argument} == \"");
                // Makes ^^^ pointers to where the separator is
                let separator_pointers_string = make_pointers_string(
                    prefix.clone(),
                    stringified_data.to_string(),
                    separator.to_string(),
                );
                write!(
                    f,
                    "There is a separator \"{separator}\" in `{enum_variant}`: \
                     \n{prefix}{stringified_data}\"\n{separator_pointers_string}\n\nPlease \
                     consider changing the separator with \
                     `#[button(fields_separator=\"NEW_SEPARATOR\")]`"
                )
            }
            Self::SeparatorInUnnamedArgument {
                enum_variant,
                stringified_data,
                separator,
                field,
            } => {
                let prefix = format!("self.{field} == \"");
                let separator_pointers_string = make_pointers_string(
                    prefix.clone(),
                    stringified_data.to_string(),
                    separator.to_string(),
                );
                write!(
                    f,
                    "There is a separator \"{separator}\" in `{enum_variant}`: \
                     \n{prefix}{stringified_data}\"\n{separator_pointers_string}\n\nPlease \
                     consider changing the separator with \
                     `#[button(fields_separator=\"NEW_SEPARATOR\")]`"
                )
            }
        }
    }
}

impl std::error::Error for StringifyError {}
