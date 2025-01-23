//! Command parsers.
//!
//! You can either create an `enum` with derived [`BotCommands`], containing
//! commands of your bot, or use functions, which split input text into a string
//! command with its arguments.
//!
//! # Using BotCommands
//!
//! ```
//! # #[cfg(feature = "macros")] {
//! use teloxide::utils::command::BotCommands;
//!
//! type UnitOfTime = u8;
//!
//! #[derive(BotCommands, PartialEq, Debug)]
//! #[command(rename_rule = "lowercase", parse_with = "split")]
//! enum AdminCommand {
//!     Mute(UnitOfTime, char),
//!     Ban(UnitOfTime, char),
//! }
//!
//! let command = AdminCommand::parse("/ban 5 h", "bot_name").unwrap();
//! assert_eq!(command, AdminCommand::Ban(5, 'h'));
//! # }
//! ```
//!
//! # Using parse_command
//!
//! ```
//! use teloxide::utils::command::parse_command;
//!
//! let (command, args) = parse_command("/ban@MyBotName 3 hours", "MyBotName").unwrap();
//! assert_eq!(command, "ban");
//! assert_eq!(args, vec!["3", "hours"]);
//! ```
//!
//! # Using parse_command_with_prefix
//!
//! ```
//! use teloxide::utils::command::parse_command_with_prefix;
//!
//! let text = "!ban 3 hours";
//! let (command, args) = parse_command_with_prefix("!", text, "").unwrap();
//! assert_eq!(command, "ban");
//! assert_eq!(args, vec!["3", "hours"]);
//! ```
//!
//! See [examples/admin] as a more complicated examples.
//!
//! [examples/admin]: https://github.com/teloxide/teloxide/blob/master/crates/teloxide/examples/admin.rs

use core::fmt;
use std::{
    error::Error,
    fmt::{Display, Formatter, Write},
};

use teloxide_core::types::{BotCommand, Me};
#[cfg(feature = "macros")]
pub use teloxide_macros::BotCommands;

/// An enumeration of bot's commands.
///
/// # Example
/// ```
/// # #[cfg(feature = "macros")] {
/// use teloxide::utils::command::BotCommands;
///
/// type UnitOfTime = u8;
///
/// #[derive(BotCommands, PartialEq, Debug)]
/// #[command(rename_rule = "lowercase", parse_with = "split")]
/// enum AdminCommand {
///     Mute(UnitOfTime, char),
///     Ban(UnitOfTime, char),
/// }
///
/// let command = AdminCommand::parse("/ban 5 h", "bot_name").unwrap();
/// assert_eq!(command, AdminCommand::Ban(5, 'h'));
/// # }
/// ```
///
/// # Enum attributes
///  1. `#[command(rename_rule = "rule")]` Rename all commands by `rule`.
///     Allowed rules are `lowercase`, `UPPERCASE`, `PascalCase`, `camelCase`,
///     `snake_case`, `SCREAMING_SNAKE_CASE`, `kebab-case`, and
///     `SCREAMING-KEBAB-CASE`.
///
///  2. `#[command(prefix = "prefix")]` Change a prefix for all commands (the
///     default is `/`).
///
///  3. `#[command(description = "description")]` and `/// description` Add a
///     summary description of commands before all commands.
///
///  4. `#[command(parse_with = "parser")]` Change the parser of arguments.
///     Possible values:
///       - `default` - the same as the unspecified parser. It only puts all
///         text after the first space into the first argument, which must
///         implement [`FromStr`].
///
/// ## Example
/// ```
/// # #[cfg(feature = "macros")] {
/// use teloxide::utils::command::BotCommands;
///
/// #[derive(BotCommands, PartialEq, Debug)]
/// #[command(rename_rule = "lowercase")]
/// enum Command {
///     Text(String),
/// }
///
/// let command = Command::parse("/text hello my dear friend!", "").unwrap();
/// assert_eq!(command, Command::Text("hello my dear friend!".to_string()));
/// # }
/// ```
///
///  - `split` - separates a message by a given separator (the default is the
///    space character) and parses each part into the corresponding arguments,
///    which must implement [`FromStr`].
///
/// ## Example
/// ```
/// # #[cfg(feature = "macros")] {
/// use teloxide::utils::command::BotCommands;
///
/// #[derive(BotCommands, PartialEq, Debug)]
/// #[command(rename_rule = "lowercase", parse_with = "split")]
/// enum Command {
///     Nums(u8, u16, i32),
/// }
///
/// let command = Command::parse("/nums 1 32 -5", "").unwrap();
/// assert_eq!(command, Command::Nums(1, 32, -5));
/// # }
/// ```
///
/// 5. `#[command(separator = "sep")]` Specify separator used by the `split`
///    parser. It will be ignored when accompanied by another type of parsers.
///
/// ## Example
/// ```
/// # #[cfg(feature = "macros")] {
/// use teloxide::utils::command::BotCommands;
///
/// #[derive(BotCommands, PartialEq, Debug)]
/// #[command(rename_rule = "lowercase", parse_with = "split", separator = "|")]
/// enum Command {
///     Nums(u8, u16, i32),
/// }
///
/// let command = Command::parse("/nums 1|32|5", "").unwrap();
/// assert_eq!(command, Command::Nums(1, 32, 5));
/// # }
/// ```
///
/// 6. `#[command(command_separator = "sep")]` Specify separator between command
///    and args. Default is a space character.
///
/// ## Example
/// ```
/// # #[cfg(feature = "macros")] {
/// use teloxide::utils::command::BotCommands;
///
/// #[derive(BotCommands, PartialEq, Debug)]
/// #[command(
///     rename_rule = "lowercase",
///     parse_with = "split",
///     separator = "_",
///     command_separator = "_"
/// )]
/// enum Command {
///     Nums(u8, u16, i32),
/// }
///
/// let command = Command::parse("/nums_1_32_5", "").unwrap();
/// assert_eq!(command, Command::Nums(1, 32, 5));
/// # }
/// ```
///
/// # Variant attributes
/// All variant attributes override the corresponding `enum` attributes.
///
///  1. `#[command(rename_rule = "rule")]` Rename one command by a rule. Allowed
///     rules are `lowercase`, `UPPERCASE`, `PascalCase`, `camelCase`,
///     `snake_case`, `SCREAMING_SNAKE_CASE`, `kebab-case`,
///     `SCREAMING-KEBAB-CASE`.
///
///  2. `#[command(rename = "name")]` Rename one command to `name` (literal
///     renaming; do not confuse with `rename_rule`).
///
///  3. `#[command(description = "description")]` and `/// description` Give
///     your command a description. It will be shown in the help message.
///
///  4. `#[command(parse_with = "parser")]` Parse arguments of one command with
///     a given parser. `parser` must be a function of the signature `fn(String)
///     -> Result<Tuple, ParseError>`, where `Tuple` corresponds to the
///     variant's arguments.
///
///  5. `#[command(hide)]` Hide a command from the help message. It will still
///     be parsed.
///
/// 6. `#[command(alias = "alias")]` Add an alias to a command. It will be shown
///    in the help message.
///
/// 7. `#[command(aliases = ["alias1", "alias2"])]` Add multiple aliases to a
///    command. They will be shown in the help message.
///
/// 8. `#[command(hide_aliases)]` Hide all aliases of a command from the help
///    message.
///
/// ## Example
/// ```
/// # #[cfg(feature = "macros")] {
/// use teloxide::utils::command::{BotCommands, ParseError};
///
/// fn accept_two_digits(input: String) -> Result<(u8,), ParseError> {
///     match input.len() {
///         2 => {
///             let num = input.parse::<u8>().map_err(|e| ParseError::IncorrectFormat(e.into()))?;
///             Ok((num,))
///         }
///         len => Err(ParseError::Custom(format!("Only 2 digits allowed, not {}", len).into())),
///     }
/// }
///
/// #[derive(BotCommands, PartialEq, Debug)]
/// #[command(rename_rule = "lowercase")]
/// enum Command {
///     #[command(parse_with = accept_two_digits)]
///     Num(u8),
/// }
///
/// let command = Command::parse("/num 12", "").unwrap();
/// assert_eq!(command, Command::Num(12));
/// let command = Command::parse("/num 333", "");
/// assert!(command.is_err());
/// # }
/// ```
///
///  5. `#[command(prefix = "prefix")]`
///  6. `#[command(separator = "sep")]`
///
/// These attributes just override the corresponding `enum` attributes for a
/// specific variant.
///
/// [`FromStr`]: https://doc.rust-lang.org/std/str/trait.FromStr.html
/// [`BotCommands`]: crate::utils::command::BotCommands
pub trait BotCommands: Sized {
    /// Parses a command.
    ///
    /// `bot_username` is required to parse commands like
    /// `/cmd@username_of_the_bot`.
    fn parse(s: &str, bot_username: &str) -> Result<Self, ParseError>;

    /// Returns descriptions of the commands suitable to be shown to the user
    /// (for example when `/help` command is used).
    fn descriptions() -> CommandDescriptions<'static>;

    /// Returns a vector of [`BotCommand`] that can be used with
    /// [`set_my_commands`].
    ///
    /// [`BotCommand`]: crate::types::BotCommand
    /// [`set_my_commands`]: crate::requests::Requester::set_my_commands
    fn bot_commands() -> Vec<BotCommand>;
}

pub type PrefixedBotCommand = String;
pub type BotName = String;
pub type CallbackDataVariant = String;

/// Errors returned from [`BotCommands::parse`].
///
/// [`BotCommands::parse`]: BotCommands::parse
#[derive(Debug)]
pub enum ParseError {
    TooFewArguments {
        expected: usize,
        found: usize,
        message: String,
    },
    TooManyArguments {
        expected: usize,
        found: usize,
        message: String,
    },

    /// Redirected from [`FromStr::from_str`].
    ///
    /// [`FromStr::from_str`]: https://doc.rust-lang.org/std/str/trait.FromStr.html#tymethod.from_str
    IncorrectFormat(Box<dyn Error + Send + Sync + 'static>),

    UnknownCommand(PrefixedBotCommand),
    WrongBotName(BotName),

    /// Error for [`InlineButtons`].
    ///
    /// [`InlineButtons`]: crate::utils::button::InlineButtons
    UnknownCallbackDataVariant(CallbackDataVariant),

    /// A custom error which you can return from your custom parser.
    Custom(Box<dyn Error + Send + Sync + 'static>),
}

/// Command descriptions that can be shown to the user (e.g. as a part of
/// `/help` message)
///
/// Most of the time you don't need to create this struct yourself as it's
/// returned from [`BotCommands::descriptions`].
#[derive(Debug, Clone)]
pub struct CommandDescriptions<'a> {
    global_description: Option<&'a str>,
    descriptions: &'a [CommandDescription<'a>],
    bot_username: Option<&'a str>,
}

/// Description of a particular command, used in [`CommandDescriptions`].
#[derive(Debug, Clone)]
pub struct CommandDescription<'a> {
    /// Prefix of the command, usually `/`.
    pub prefix: &'a str,
    /// The command itself, e.g. `start`.
    pub command: &'a str,
    /// The command aliases, e.g. `["help", "h"]`.
    pub aliases: &'a [&'a str],
    /// Human-readable description of the command.
    pub description: &'a str,
}

impl<'a> CommandDescriptions<'a> {
    /// Creates new [`CommandDescriptions`] from a list of command descriptions.
    #[must_use]
    pub const fn new(descriptions: &'a [CommandDescription<'a>]) -> Self {
        Self { global_description: None, descriptions, bot_username: None }
    }

    /// Sets the global description of these commands.
    #[must_use]
    pub fn global_description(self, global_description: &'a str) -> Self {
        Self { global_description: Some(global_description), ..self }
    }

    /// Sets the username of the bot.
    ///
    /// After this method is called, returned instance of
    /// [`CommandDescriptions`] will append `@bot_username` to all commands.
    /// This is useful in groups, to disambiguate commands for different bots.
    ///
    /// ## Examples
    ///
    /// ```
    /// use teloxide::utils::command::{CommandDescription, CommandDescriptions};
    ///
    /// let descriptions = CommandDescriptions::new(&[
    ///     CommandDescription {
    ///         prefix: "/",
    ///         command: "start",
    ///         description: "start this bot",
    ///         aliases: &[],
    ///     },
    ///     CommandDescription {
    ///         prefix: "/",
    ///         command: "help",
    ///         description: "show this message",
    ///         aliases: &[],
    ///     },
    /// ]);
    ///
    /// assert_eq!(descriptions.to_string(), "/start — start this bot\n/help — show this message");
    /// assert_eq!(
    ///     descriptions.username("username_of_the_bot").to_string(),
    ///     "/start@username_of_the_bot — start this bot\n/help@username_of_the_bot — show this \
    ///      message"
    /// );
    /// ```
    #[must_use]
    pub fn username(self, bot_username: &'a str) -> Self {
        Self { bot_username: Some(bot_username), ..self }
    }

    /// Sets the username of the bot.
    ///
    /// This is the same as [`username`], but uses value returned from `get_me`
    /// method to get the username.
    ///
    /// [`username`]: self::CommandDescriptions::username
    #[must_use]
    pub fn username_from_me(self, me: &'a Me) -> CommandDescriptions<'a> {
        self.username(me.user.username.as_deref().expect("Bots must have usernames"))
    }
}

/// Parses a string into a command with args.
///
/// This function is just a shortcut for calling [`parse_command_with_prefix`]
/// with the default prefix `/`.
///
/// ## Example
/// ```
/// use teloxide::utils::command::parse_command;
///
/// let text = "/mute@my_admin_bot 5 hours";
/// let (command, args) = parse_command(text, "my_admin_bot").unwrap();
/// assert_eq!(command, "mute");
/// assert_eq!(args, vec!["5", "hours"]);
/// ```
///
/// If the name of a bot does not match, it will return `None`:
/// ```
/// use teloxide::utils::command::parse_command;
///
/// let result = parse_command("/ban@MyNameBot1 3 hours", "MyNameBot2");
/// assert!(result.is_none());
/// ```
///
/// [`parse_command_with_prefix`]:
/// crate::utils::command::parse_command_with_prefix
pub fn parse_command<N>(text: &str, bot_name: N) -> Option<(&str, Vec<&str>)>
where
    N: AsRef<str>,
{
    parse_command_with_prefix("/", text, bot_name)
}

/// Parses a string into a command with args (custom prefix).
///
/// `prefix`: symbols, which denote start of a command.
///
/// ## Example
/// ```
/// use teloxide::utils::command::parse_command_with_prefix;
///
/// let text = "!mute 5 hours";
/// let (command, args) = parse_command_with_prefix("!", text, "").unwrap();
/// assert_eq!(command, "mute");
/// assert_eq!(args, vec!["5", "hours"]);
/// ```
///
/// If the name of a bot does not match, it will return `None`:
/// ```
/// use teloxide::utils::command::parse_command_with_prefix;
///
/// let result = parse_command_with_prefix("!", "!ban@MyNameBot1 3 hours", "MyNameBot2");
/// assert!(result.is_none());
/// ```
pub fn parse_command_with_prefix<'a, N>(
    prefix: &str,
    text: &'a str,
    bot_name: N,
) -> Option<(&'a str, Vec<&'a str>)>
where
    N: AsRef<str>,
{
    if !text.starts_with(prefix) {
        return None;
    }
    let mut words = text.split_whitespace();
    let mut split = words.next()?[prefix.len()..].split('@');
    let command = split.next()?;
    let bot = split.next();
    match bot {
        Some(name) if name.eq_ignore_ascii_case(bot_name.as_ref()) => {}
        None => {}
        _ => return None,
    }
    Some((command, words.collect()))
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ParseError::TooFewArguments { expected, found, message } => write!(
                f,
                "Too few arguments (expected {expected}, found {found}, message = '{message}')"
            ),
            ParseError::TooManyArguments { expected, found, message } => write!(
                f,
                "Too many arguments (expected {expected}, found {found}, message = '{message}')"
            ),
            ParseError::IncorrectFormat(e) => write!(f, "Incorrect format of command args: {e}"),
            ParseError::UnknownCommand(e) => write!(f, "Unknown command: {e}"),
            ParseError::UnknownCallbackDataVariant(v) => {
                write!(f, "Unknown callback data variant: {v}")
            }
            ParseError::WrongBotName(n) => write!(f, "Wrong bot name: {n}"),
            ParseError::Custom(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for ParseError {}

impl Display for CommandDescriptions<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(global_description) = self.global_description {
            f.write_str(global_description)?;
            f.write_str("\n\n")?;
        }

        let format_command = |command: &str, prefix: &str, formater: &mut fmt::Formatter<'_>| {
            formater.write_str(prefix)?;
            formater.write_str(command)?;
            if let Some(username) = self.bot_username {
                formater.write_char('@')?;
                formater.write_str(username)?;
            }
            fmt::Result::Ok(())
        };

        let mut write = |&CommandDescription { prefix, command, aliases, description }, nls| {
            if nls {
                f.write_char('\n')?;
            }

            format_command(command, prefix, f)?;
            for alias in aliases {
                f.write_str(", ")?;
                format_command(alias, prefix, f)?;
            }

            if !description.is_empty() {
                f.write_str(" — ")?;
                f.write_str(description)?;
            }

            fmt::Result::Ok(())
        };

        if let Some(descr) = self.descriptions.first() {
            write(descr, false)?;
            for descr in &self.descriptions[1..] {
                write(descr, true)?;
            }
        }

        Ok(())
    }
}

// The rest of tests are integration due to problems with macro expansion in
// unit tests.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_command_with_args_() {
        let data = "/command arg1 arg2";
        let expected = Some(("command", vec!["arg1", "arg2"]));
        let actual = parse_command(data, "");
        assert_eq!(actual, expected)
    }

    #[test]
    fn parse_command_with_args_without_args() {
        let data = "/command";
        let expected = Some(("command", vec![]));
        let actual = parse_command(data, "");
        assert_eq!(actual, expected)
    }
}
