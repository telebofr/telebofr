//! Possible error types.

use std::io;

use thiserror::Error;

use crate::types::{ChatId, ResponseParameters, Seconds};

/// An error caused by sending a request to Telegram.
#[derive(Debug, Error)]
pub enum RequestError {
    /// A Telegram API error.
    #[error("A Telegram's error: {0}")]
    Api(#[from] ApiError),

    /// The group has been migrated to a supergroup with the specified
    /// identifier.
    #[error("The group has been migrated to a supergroup with ID #{0}")]
    MigrateToChatId(ChatId),

    /// In case of exceeding flood control, the number of seconds left to wait
    /// before the request can be repeated.
    #[error("Retry after {0}")]
    RetryAfter(Seconds),

    /// Network error while sending a request to Telegram.
    #[error("A network error: {0}")]
    // NOTE: this variant must not be created by anything except the explicit From impl
    Network(#[source] reqwest::Error),

    /// Error while parsing a response from Telegram.
    ///
    /// If you've received this error, please, [open an issue] with the
    /// description of the error.
    ///
    /// [open an issue]: https://github.com/teloxide/teloxide/issues/new
    #[error("An error while parsing JSON: {source} (raw: {raw:?})")]
    InvalidJson {
        #[source]
        source: serde_json::Error,
        /// The raw string JSON that couldn't been parsed
        raw: Box<str>,
    },

    /// Occurs when trying to send a file to Telegram.
    #[error("An I/O error: {0}")]
    Io(#[from] io::Error),
}

/// An error caused by downloading a file.
#[derive(Debug, Error)]
pub enum DownloadError {
    /// A network error while downloading a file from Telegram.
    #[error("A network error: {0}")]
    // NOTE: this variant must not be created by anything except the explicit From impl
    Network(#[source] reqwest::Error),

    /// An I/O error while writing a file to destination.
    #[error("An I/O error: {0}")]
    Io(#[from] std::io::Error),
}

pub trait AsResponseParameters {
    fn response_parameters(&self) -> Option<ResponseParameters>;

    fn retry_after(&self) -> Option<Seconds> {
        self.response_parameters().and_then(|rp| match rp {
            ResponseParameters::RetryAfter(n) => Some(n),
            _ => None,
        })
    }

    fn migrate_to_chat_id(&self) -> Option<ChatId> {
        self.response_parameters().and_then(|rp| match rp {
            ResponseParameters::MigrateToChatId(id) => Some(id),
            _ => None,
        })
    }
}

impl AsResponseParameters for crate::RequestError {
    fn response_parameters(&self) -> Option<ResponseParameters> {
        match *self {
            Self::RetryAfter(n) => Some(ResponseParameters::RetryAfter(n)),
            Self::MigrateToChatId(id) => Some(ResponseParameters::MigrateToChatId(id)),
            _ => None,
        }
    }
}

macro_rules! impl_api_error {
    (
        $( #[$meta:meta] )*
        $vis:vis enum $ident:ident {
            $(
                $( #[$var_meta:meta] )*
                $var_name:ident $( ($var_inner:ty) )? = $var_string:literal $(with $var_parser:expr)?
             ),*
         }
    ) => {

        $(#[$meta])*
        #[derive(Error)]
        $vis enum $ident {
            $(
            $(#[$var_meta])*
            #[error($var_string)]
            $var_name $(($var_inner))*,
            )*
        }

        const _: () = {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = $ident;

                fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                    formatter.write_str("telegram api error string")
                }

                fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: ::serde::de::Error,
                {
                    $(impl_api_error!(@de v, $var_name $( ($var_inner) )?, $var_string $(, $var_parser)*);)*
                    Err(E::unknown_variant(v, &[]))
                }
            }

            impl<'de> ::serde::de::Deserialize<'de> for $ident {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    deserializer.deserialize_str(Visitor)
                }
            }
        };
    };
    (@de $value:ident, $variant:ident, $val:literal) => {
        if $value == $val {
            return Ok(Self::Value::$variant)
        }
    };
    (@de $value:ident, $variant:ident ($var_inner:ty), $val:literal, $block:expr) => {
        #[allow(clippy::redundant_closure_call)]
        match $block($value) {
            Some(data) => return Ok(Self::Value::$variant(data)),
            _ => {}
        }
    };
    (@de $value:ident, $variant:ident, $val:literal, $block:expr) => {
        #[allow(clippy::redundant_closure_call)]
        if $block($value) {
            return Ok(Self::Value::$variant);
        }
    };
}

impl_api_error! {
    /// A kind of an API error.
    #[derive(Debug, PartialEq, Hash, Eq, Clone)]
    #[non_exhaustive]
    pub enum ApiError {
        /// Occurs when the bot tries to send message to user who blocked the bot.
        BotBlocked = "Forbidden: bot was blocked by the user",

        /// Occurs when the bot token is invalid.
        // N.B. These errors are actually slightly different, "Unauthorized" is when the bot token
        //      is formatted mostly right, but is incorrect, whereas "Not Found" is when the url is
        //      not handled by TBA at all. From user POV both of those are "token is invalid", but
        //      there might be some cases where this is not right...
        InvalidToken = "Invalid bot token" with |text: &str| text == "Unauthorized" || text == "Not Found",

        /// Occurs when bot tries to modify a message without modification content.
        ///
        /// May happen in methods:
        /// 1. [`EditMessageText`]
        ///
        /// [`EditMessageText`]: crate::payloads::EditMessageText
        MessageNotModified = "Bad Request: message is not modified: specified new message content and reply markup are \
            exactly the same as a current content and reply markup of the message",

        /// Occurs when bot tries to forward or delete a message which was deleted.
        ///
        /// May happen in methods:
        /// 1. [`ForwardMessage`]
        /// 2. [`DeleteMessage`]
        ///
        /// [`ForwardMessage`]: crate::payloads::ForwardMessage
        /// [`DeleteMessage`]: crate::payloads::DeleteMessage
        MessageIdInvalid = "Bad Request: MESSAGE_ID_INVALID",

        /// Occurs when bot tries to forward a message which does not exists.
        ///
        /// May happen in methods:
        /// 1. [`ForwardMessage`]
        ///
        /// [`ForwardMessage`]: crate::payloads::ForwardMessage
        MessageToForwardNotFound = "Bad Request: message to forward not found",

        /// Occurs when bot tries to delete a message which does not exists.
        ///
        /// May happen in methods:
        /// 1. [`DeleteMessage`]
        ///
        /// [`DeleteMessage`]: crate::payloads::DeleteMessage
        MessageToDeleteNotFound = "Bad Request: message to delete not found",

        /// Occurs when bot tries to copy a message which does not exists.
        /// May happen in methods:
        /// 1. [`CopyMessage`]
        ///
        /// [`CopyMessage`]: crate::payloads::CopyMessage
        MessageToCopyNotFound = "Bad Request: message to copy not found",

        /// Occurs when bot tries to send a text message without text.
        ///
        /// May happen in methods:
        /// 1. [`SendMessage`]
        ///
        /// [`SendMessage`]: crate::payloads::SendMessage
        MessageTextIsEmpty = "Bad Request: message text is empty",

        /// Occurs when bot tries to edit a message after long time.
        ///
        /// May happen in methods:
        /// 1. [`EditMessageText`]
        ///
        /// [`EditMessageText`]: crate::payloads::EditMessageText
        MessageCantBeEdited = "Bad Request: message can't be edited",

        /// Occurs when bot tries to delete a someone else's message in group where
        /// it does not have enough rights.
        ///
        /// May happen in methods:
        /// 1. [`DeleteMessage`]
        ///
        /// [`DeleteMessage`]: crate::payloads::DeleteMessage
        MessageCantBeDeleted = "Bad Request: message can't be deleted",

        /// Occurs when a bot tries to delete a message created by the bot
        /// in a group they belong to but have no rights to delete the message
        /// for all in the group.
        ///
        /// May happen in methods:
        /// 1. [`DeleteMessage`]
        ///
        /// [`DeleteMessage`]: crate::payloads::DeleteMessage
        MessageCantBeDeletedForEveryone = "Bad Request: message can't be deleted for everyone",

        /// Occurs when bot tries to edit a message which does not exists.
        ///
        /// May happen in methods:
        /// 1. [`EditMessageText`]
        ///
        /// [`EditMessageText`]: crate::payloads::EditMessageText
        MessageToEditNotFound = "Bad Request: message to edit not found",

        /// Occurs when bot tries to reply to a message which does not exists.
        ///
        /// May happen in methods:
        /// 1. [`SendMessage`]
        ///
        /// [`SendMessage`]: crate::payloads::SendMessage
        MessageToReplyNotFound = "Bad Request: reply message not found",

        /// Occurs when bot tries to
        MessageIdentifierNotSpecified = "Bad Request: message identifier is not specified",

        /// Occurs when bot tries to send a message with text size greater then
        /// 4096 symbols.
        ///
        /// May happen in methods:
        /// 1. [`SendMessage`]
        ///
        /// [`SendMessage`]: crate::payloads::SendMessage
        MessageIsTooLong = "Bad Request: message is too long",

        /// Occurs when bot tries to edit a message with text size greater then
        /// 4096 symbols.
        ///
        /// May happen in methods:
        /// 1. [`EditMessageText`]
        /// 2. [`EditMessageTextInline`]
        /// 3. [`EditMessageCaption`]
        /// 4. [`EditMessageCaptionInline`]
        ///
        /// [`EditMessageText`]: crate::payloads::EditMessageText
        /// [`EditMessageTextInline`]: crate::payloads::EditMessageTextInline
        /// [`EditMessageCaption`]: crate::payloads::EditMessageCaption
        /// [`EditMessageCaptionInline`]: crate::payloads::EditMessageCaptionInline
        EditedMessageIsTooLong = "Bad Request: MESSAGE_TOO_LONG",

        /// Occurs when bot tries to send media group with more than 10 items.
        ///
        /// May happen in methods:
        /// 1. [`SendMediaGroup`]
        ///
        /// [`SendMediaGroup`]: crate::payloads::SendMediaGroup
        TooMuchMessages = "Bad Request: Too much messages to send as an album",

        /// Occurs when bot tries to answer an inline query with more than 50
        /// results.
        ///
        /// Consider using offsets to paginate results.
        ///
        /// May happen in methods:
        /// 1. [`AnswerInlineQuery`]
        ///
        /// [`AnswerInlineQuery`]: crate::payloads::AnswerInlineQuery
        TooMuchInlineQueryResults = "Bad Request: RESULTS_TOO_MUCH",

        /// Occurs when bot tries to stop poll that has already been stopped.
        ///
        /// May happen in methods:
        /// 1. [`SendPoll`]
        ///
        /// [`SendPoll`]: crate::payloads::SendPoll
        PollHasAlreadyClosed = "Bad Request: poll has already been closed",

        /// Occurs when bot tries to send poll with less than 2 options.
        ///
        /// May happen in methods:
        /// 1. [`SendPoll`]
        ///
        /// [`SendPoll`]: crate::payloads::SendPoll
        PollMustHaveMoreOptions = "Bad Request: poll must have at least 2 option",

        /// Occurs when bot tries to send poll with more than 10 options.
        ///
        /// May happen in methods:
        /// 1. [`SendPoll`]
        ///
        /// [`SendPoll`]: crate::payloads::SendPoll
        PollCantHaveMoreOptions = "Bad Request: poll can't have more than 10 options",

        /// Occurs when bot tries to send poll with empty option (without text).
        ///
        /// May happen in methods:
        /// 1. [`SendPoll`]
        ///
        /// [`SendPoll`]: crate::payloads::SendPoll
        PollOptionsMustBeNonEmpty = "Bad Request: poll options must be non-empty",

        /// Occurs when bot tries to send poll with empty question (without text).
        ///
        /// May happen in methods:
        /// 1. [`SendPoll`]
        ///
        /// [`SendPoll`]: crate::payloads::SendPoll
        PollQuestionMustBeNonEmpty = "Bad Request: poll question must be non-empty",

        /// Occurs when bot tries to send poll with total size of options more than
        /// 100 symbols.
        ///
        /// May happen in methods:
        /// 1. [`SendPoll`]
        ///
        /// [`SendPoll`]: crate::payloads::SendPoll
        PollOptionsLengthTooLong = "Bad Request: poll options length must not exceed 100",

        /// Occurs when bot tries to send poll with question size more than 255
        /// symbols.
        ///
        /// May happen in methods:
        /// 1. [`SendPoll`]
        ///
        /// [`SendPoll`]: crate::payloads::SendPoll
        PollQuestionLengthTooLong = "Bad Request: poll question length must not exceed 255",

        /// Occurs when bot tries to stop poll with message without poll.
        ///
        /// May happen in methods:
        /// 1. [`StopPoll`]
        ///
        /// [`StopPoll`]: crate::payloads::StopPoll
        MessageWithPollNotFound = "Bad Request: message with poll to stop not found",

        /// Occurs when bot tries to stop poll with message without poll.
        ///
        /// May happen in methods:
        /// 1. [`StopPoll`]
        ///
        /// [`StopPoll`]: crate::payloads::StopPoll
        MessageIsNotAPoll = "Bad Request: message is not a poll",

        /// Occurs when bot tries to send a message to chat in which it is not a
        /// member.
        ///
        /// May happen in methods:
        /// 1. [`SendMessage`]
        ///
        /// [`SendMessage`]: crate::payloads::SendMessage
        ChatNotFound = "Bad Request: chat not found",

        /// Occurs when bot tries to send method with unknown user_id.
        ///
        /// May happen in methods:
        /// 1. [`getUserProfilePhotos`]
        ///
        /// [`getUserProfilePhotos`]:
        /// crate::payloads::GetUserProfilePhotos
        UserNotFound = "Bad Request: user not found",

        /// Occurs when bot tries to send [`SetChatDescription`] with same text as
        /// in the current description.
        ///
        /// May happen in methods:
        /// 1. [`SetChatDescription`]
        ///
        /// [`SetChatDescription`]: crate::payloads::SetChatDescription
        ChatDescriptionIsNotModified = "Bad Request: chat description is not modified",

        /// Occurs when bot tries to answer to query after timeout expire.
        ///
        /// May happen in methods:
        /// 1. [`AnswerCallbackQuery`]
        ///
        /// [`AnswerCallbackQuery`]: crate::payloads::AnswerCallbackQuery
        InvalidQueryId = "Bad Request: query is too old and response timeout expired or query id is invalid",

        /// Occurs when bot tries to send InlineKeyboardMarkup with invalid button
        /// url.
        ///
        /// May happen in methods:
        /// 1. [`SendMessage`]
        ///
        /// [`SendMessage`]: crate::payloads::SendMessage
        ButtonUrlInvalid = "Bad Request: BUTTON_URL_INVALID",

        /// Occurs when bot tries to send button with data size more than 64 bytes.
        ///
        /// May happen in methods:
        /// 1. [`SendMessage`]
        ///
        /// [`SendMessage`]: crate::payloads::SendMessage
        ButtonDataInvalid = "Bad Request: BUTTON_DATA_INVALID",

        /// Occurs when bot tries to send button with data size == 0.
        ///
        /// May happen in methods:
        /// 1. [`SendMessage`]
        ///
        /// [`SendMessage`]: crate::payloads::SendMessage
        TextButtonsAreUnallowed = "Bad Request: can't parse inline keyboard button: Text buttons are unallowed in the \
            inline keyboard",

        /// Occurs when bot tries to get file by wrong file id.
        ///
        /// May happen in methods:
        /// 1. [`GetFile`]
        ///
        /// [`GetFile`]: crate::payloads::GetFile
        WrongFileId = "Bad Request: wrong file id",

        /// Occurs when bot tries to send files with wrong file identifier or HTTP
        /// url
        WrongFileIdOrUrl = "Bad Request: wrong file identifier/HTTP URL specified",

        /// Occurs when When sending files with an url to a site that doesn't
        /// respond.
        FailedToGetUrlContent = "Bad Request: failed to get HTTP URL content",

        /// Occurs when bot tries to do some with group which was deactivated.
        GroupDeactivated = "Bad Request: group is deactivated",

        /// Occurs when image processing fails on telegram's side.
        ///
        /// This is likely caused by an incorrectly encoded image, make sure that
        /// the image is correctly encoded in a format telegram accepts.
        ImageProcessFailed = "Bad Request: IMAGE_PROCESS_FAILED",

        /// Occurs when bot tries to set chat photo from file ID
        ///
        /// May happen in methods:
        /// 1. [`SetChatPhoto`]
        ///
        /// [`SetChatPhoto`]: crate::payloads::SetChatPhoto
        PhotoAsInputFileRequired = "Bad Request: Photo should be uploaded as an InputFile",

        /// Occurs when bot tries to add sticker to stickerset by invalid name.
        ///
        /// May happen in methods:
        /// 1. [`AddStickerToSet`]
        ///
        /// [`AddStickerToSet`]: crate::payloads::AddStickerToSet
        InvalidStickersSet = "Bad Request: STICKERSET_INVALID",

        /// Occurs when bot tries to create a sticker set with a name that is
        /// already used by another sticker set.
        ///
        /// May happen in methods:
        /// 1. [`CreateNewStickerSet`]
        ///
        /// [`CreateNewStickerSet`]: crate::payloads::CreateNewStickerSet
        StickerSetNameOccupied = "Bad Request: sticker set name is already occupied",

        /// Occurs when bot tries to create a sticker set with user id of a bot.
        ///
        /// May happen in methods:
        /// 1. [`CreateNewStickerSet`]
        ///
        /// [`CreateNewStickerSet`]: crate::payloads::CreateNewStickerSet
        StickerSetOwnerIsBot = "Bad Request: USER_IS_BOT",

        /// Occurs when bot tries to create a sticker set with invalid name.
        ///
        /// From documentation of [`CreateNewStickerSet`]:
        /// > Short name of sticker set, to be used in `t.me/addstickers/` URLs
        /// (e.g., _animals_). Can contain only english letters, digits and
        /// underscores. Must begin with a letter, can't contain consecutive
        /// underscores and must end in “\_by\_<bot\_username>”. <bot\_username>
        /// is case insensitive. 1-64 characters.
        ///
        /// May happen in methods:
        /// 1. [`CreateNewStickerSet`]
        ///
        /// [`CreateNewStickerSet`]: crate::payloads::CreateNewStickerSet
        InvalidStickerName = "Bad Request: invalid sticker set name is specified",

        /// Occurs when bot tries to pin a message without rights to pin in this
        /// chat.
        ///
        /// May happen in methods:
        /// 1. [`PinChatMessage`]
        ///
        /// [`PinChatMessage`]: crate::payloads::PinChatMessage
        NotEnoughRightsToPinMessage = "Bad Request: not enough rights to pin a message",

        /// Occurs when bot tries to pin or unpin a message without rights to pin
        /// in this chat.
        ///
        /// May happen in methods:
        /// 1. [`PinChatMessage`]
        /// 2. [`UnpinChatMessage`]
        ///
        /// [`PinChatMessage`]: crate::payloads::PinChatMessage
        /// [`UnpinChatMessage`]: crate::payloads::UnpinChatMessage
        NotEnoughRightsToManagePins = "Bad Request: not enough rights to manage pinned messages in the chat",

        /// Occurs when bot tries change default chat permissions without "Ban
        /// Users" permission in this chat.
        ///
        /// May happen in methods:
        /// 1. [`SetChatPermissions`]
        ///
        /// [`SetChatPermissions`]: crate::payloads::SetChatPermissions
        NotEnoughRightsToChangeChatPermissions = "Bad Request: not enough rights to change chat permissions",

        /// Occurs when bot tries to use method in group which is allowed only in a
        /// supergroup or channel.
        MethodNotAvailableInPrivateChats = "Bad Request: method is available only for supergroups and channel",

        /// Occurs when bot tries to demote chat creator.
        ///
        /// May happen in methods:
        /// 1. [`PromoteChatMember`]
        ///
        /// [`PromoteChatMember`]: crate::payloads::PromoteChatMember
        CantDemoteChatCreator = "Bad Request: can't demote chat creator",

        /// Occurs when bot tries to restrict self in group chats.
        ///
        /// May happen in methods:
        /// 1. [`RestrictChatMember`]
        ///
        /// [`RestrictChatMember`]: crate::payloads::RestrictChatMember
        CantRestrictSelf = "Bad Request: can't restrict self",

        /// Occurs when bot tries to restrict chat member without rights to
        /// restrict in this chat.
        ///
        /// May happen in methods:
        /// 1. [`RestrictChatMember`]
        ///
        /// [`RestrictChatMember`]: crate::payloads::RestrictChatMember
        NotEnoughRightsToRestrict = "Bad Request: not enough rights to restrict/unrestrict chat member",

        /// Occurs when bot tries to post a message in a channel without "Post
        /// Messages" admin right.
        NotEnoughRightsToPostMessages = "Bad Request: need administrator rights in the channel chat",

        /// Occurs when bot tries set webhook to protocol other than HTTPS.
        ///
        /// May happen in methods:
        /// 1. [`SetWebhook`]
        ///
        /// [`SetWebhook`]: crate::payloads::SetWebhook
        WebhookRequireHttps = "Bad Request: bad webhook: HTTPS url must be provided for webhook",

        /// Occurs when bot tries to set webhook to port other than 80, 88, 443 or
        /// 8443.
        ///
        /// May happen in methods:
        /// 1. [`SetWebhook`]
        ///
        /// [`SetWebhook`]: crate::payloads::SetWebhook
        BadWebhookPort = "Bad Request: bad webhook: Webhook can be set up only on ports 80, 88, 443 or 8443",

        /// Occurs when bot tries to set webhook to unknown host.
        ///
        /// May happen in methods:
        /// 1. [`SetWebhook`]
        ///
        /// [`SetWebhook`]: crate::payloads::SetWebhook
        UnknownHost = "Bad Request: bad webhook: Failed to resolve host: Name or service not known",

        /// Occurs when bot tries to set webhook to invalid URL.
        ///
        /// May happen in methods:
        /// 1. [`SetWebhook`]
        ///
        /// [`SetWebhook`]: crate::payloads::SetWebhook
        CantParseUrl = "Bad Request: can't parse URL",

        /// Occurs when bot tries to send message with unfinished entities.
        ///
        /// May happen in methods:
        /// 1. [`SendMessage`]
        ///
        /// [`SendMessage`]: crate::payloads::SendMessage
        CantParseEntities(String) = "{0}" with |text: &str| {
            if text.starts_with("Bad Request: can't parse entities") {
                Some(text.to_owned())
            } else {
                None
            }
        },

        /// Occurs when bot tries to use getUpdates while webhook is active.
        ///
        /// May happen in methods:
        /// 1. [`GetUpdates`]
        ///
        /// [`GetUpdates`]: crate::payloads::GetUpdates
        CantGetUpdates = "can't use getUpdates method while webhook is active",

        /// Occurs when bot tries to do some in group where bot was kicked.
        ///
        /// May happen in methods:
        /// 1. [`SendMessage`]
        ///
        /// [`SendMessage`]: crate::payloads::SendMessage
        BotKicked = "Unauthorized: bot was kicked from a chat",

        /// Occurs when bot tries to do something in a supergroup the bot was
        /// kicked from.
        ///
        /// May happen in methods:
        /// 1. [`SendMessage`]
        ///
        /// [`SendMessage`]: crate::payloads::SendMessage
        BotKickedFromSupergroup = "Forbidden: bot was kicked from the supergroup chat",

        /// Occurs when bot tries to do something in a channel the bot was
        /// kicked from.
        ///
        /// May happen in methods:
        /// 1. [`SendMessage`]
        ///
        /// [`SendMessage`]: crate::payloads::SendMessage
        BotKickedFromChannel = "Forbidden: bot was kicked from the channel chat",

        /// Occurs when bot tries to send a message to a deactivated user (i.e. a
        /// user that was banned by telegram).
        ///
        /// May happen in methods:
        /// 1. [`SendMessage`]
        ///
        /// [`SendMessage`]: crate::payloads::SendMessage
        UserDeactivated = "Forbidden: user is deactivated",

        /// Occurs when you tries to initiate conversation with a user.
        ///
        /// May happen in methods:
        /// 1. [`SendMessage`]
        ///
        /// [`SendMessage`]: crate::payloads::SendMessage
        CantInitiateConversation = "Unauthorized: bot can't initiate conversation with a user",

        /// Occurs when you tries to send message to bot.
        ///
        /// May happen in methods:
        /// 1. [`SendMessage`]
        ///
        /// [`SendMessage`]: crate::payloads::SendMessage
        CantTalkWithBots = "Unauthorized: bot can't send messages to bots",

        /// Occurs when bot tries to send button with invalid http url.
        ///
        /// May happen in methods:
        /// 1. [`SendMessage`]
        ///
        /// [`SendMessage`]: crate::payloads::SendMessage
        WrongHttpUrl = "Bad Request: wrong HTTP URL",

        /// Occurs when multiple [`GetUpdates`] calls happen at the same time.
        ///
        /// This can happen if
        /// 1. You are running multiple bot instances
        /// 2. You are running multiple update consumers (like `Dispatcher` or `repl`)
        /// 3. You are calling [`GetUpdates`] yourself and the second call is done before the first one finishes
        ///
        /// May happen in methods:
        /// 1. [`GetUpdates`]
        ///
        /// [`GetUpdates`]: crate::payloads::GetUpdates
        TerminatedByOtherGetUpdates = "Conflict: terminated by other getUpdates request; make sure that only one bot instance \
            is running",

        /// Occurs when bot tries to get file by invalid file id.
        ///
        /// May happen in methods:
        /// 1. [`GetFile`]
        ///
        /// [`GetFile`]: crate::payloads::GetFile
        FileIdInvalid = "Bad Request: invalid file id",

        /// Occurs when bot tries to upload a file which is larger than 50 MB using
        /// multipart/form-data.
        ///
        /// May happen in methods:
        /// 1. [`SendVideo`]
        /// 2. [`SendDocument`]
        ///
        /// [`SendVideo`]: crate::payloads::SendVideo
        /// [`SendDocument`]: crate::payloads::SendDocument
        RequestEntityTooLarge = "Request Entity Too Large",


        /// Error which is not known to `teloxide`.
        ///
        /// If you've received this error, please [open an issue] with the
        /// description of the error.
        ///
        /// [open an issue]: https://github.com/teloxide/teloxide/issues/new
        Unknown(String) = "Unknown error: {0:?}" with |text: &str| Some(text.to_owned())
    }
}

/// This impl allows to use `?` to propagate [`DownloadError`]s in function
/// returning [`RequestError`]s. For example:
///
/// ```rust
/// # use teloxide_core::errors::{DownloadError, RequestError};
///
/// async fn handler() -> Result<(), RequestError> {
///     download_file().await?; // `?` just works
///
///     Ok(())
/// }
///
/// async fn download_file() -> Result<(), DownloadError> {
///     /* download file here */
///     Ok(())
/// }
/// ```
impl From<DownloadError> for RequestError {
    fn from(download_err: DownloadError) -> Self {
        match download_err {
            DownloadError::Network(err) => RequestError::Network(err),
            DownloadError::Io(err) => RequestError::Io(err),
        }
    }
}

impl From<reqwest::Error> for DownloadError {
    fn from(error: reqwest::Error) -> Self {
        DownloadError::Network(hide_token(error))
    }
}

impl From<reqwest::Error> for RequestError {
    fn from(error: reqwest::Error) -> Self {
        RequestError::Network(hide_token(error))
    }
}

/// Replaces token in the url in the error with `token:redacted` string.
pub(crate) fn hide_token(mut error: reqwest::Error) -> reqwest::Error {
    let url = match error.url_mut() {
        Some(url) => url,
        None => return error,
    };

    if let Some(mut segments) = url.path_segments() {
        // Usually the url looks like "bot<token>/..." or "file/bot<token>/...".
        let (beginning, segment) = match segments.next() {
            Some("file") => ("file/", segments.next()),
            segment => ("", segment),
        };

        if let Some(token) = segment.and_then(|s| s.strip_prefix("bot")) {
            // make sure that what we are about to delete looks like a bot token
            if let Some((id, secret)) = token.split_once(':') {
                // The part before the : in the token is the id of the bot.
                let id_character = |c: char| c.is_ascii_digit();

                // The part after the : in the token is the secret.
                //
                // In all bot tokens we could find the secret is 35 characters long and is
                // 0-9a-zA-Z_- only.
                //
                // It would be nice to research if TBA always has 35 character secrets or if it
                // is just a coincidence.
                const SECRET_LENGTH: usize = 35;
                let secret_character = |c: char| c.is_ascii_alphanumeric() || c == '-' || c == '_';

                if secret.len() >= SECRET_LENGTH
                    && id.chars().all(id_character)
                    && secret.chars().all(secret_character)
                {
                    // found token, hide only the token
                    let without_token =
                        &url.path()[(beginning.len() + "/bot".len() + token.len())..];
                    let redacted = format!("{beginning}token:redacted{without_token}");

                    url.set_path(&redacted);
                    return error;
                }
            }
        }
    }

    // couldn't find token in the url, hide the whole url
    error.without_url()
}

#[cfg(test)]
mod tests {
    #[test]
    fn custom_result() {
        use super::ApiError;
        use serde::Deserialize;

        let cases = &[
            ("{\"data\": \"Forbidden: bot was blocked by the user\"}", ApiError::BotBlocked),
            ("{\"data\": \"Unauthorized\"}", ApiError::InvalidToken),
            ("{\"data\": \"Not Found\"}", ApiError::InvalidToken),
            (
                "{\"data\": \"Bad Request: message is not modified: specified new message content \
                 and reply markup are exactly the same as a current content and reply markup of \
                 the message\"}",
                ApiError::MessageNotModified,
            ),
            ("{\"data\": \"Bad Request: MESSAGE_ID_INVALID\"}", ApiError::MessageIdInvalid),
            (
                "{\"data\": \"Bad Request: message to forward not found\"}",
                ApiError::MessageToForwardNotFound,
            ),
            (
                "{\"data\": \"Bad Request: message to delete not found\"}",
                ApiError::MessageToDeleteNotFound,
            ),
            (
                "{\"data\": \"Bad Request: message to copy not found\"}",
                ApiError::MessageToCopyNotFound,
            ),
            ("{\"data\": \"Bad Request: message text is empty\"}", ApiError::MessageTextIsEmpty),
            ("{\"data\": \"Bad Request: message can't be edited\"}", ApiError::MessageCantBeEdited),
            (
                "{\"data\": \"Bad Request: message can't be deleted\"}",
                ApiError::MessageCantBeDeleted,
            ),
            (
                "{\"data\": \"Bad Request: message can't be deleted for everyone\"}",
                ApiError::MessageCantBeDeletedForEveryone,
            ),
            (
                "{\"data\": \"Bad Request: message to edit not found\"}",
                ApiError::MessageToEditNotFound,
            ),
            (
                "{\"data\": \"Bad Request: reply message not found\"}",
                ApiError::MessageToReplyNotFound,
            ),
            (
                "{\"data\": \"Bad Request: message identifier is not specified\"}",
                ApiError::MessageIdentifierNotSpecified,
            ),
            ("{\"data\": \"Bad Request: message is too long\"}", ApiError::MessageIsTooLong),
            ("{\"data\": \"Bad Request: MESSAGE_TOO_LONG\"}", ApiError::EditedMessageIsTooLong),
            (
                "{\"data\": \"Bad Request: Too much messages to send as an album\"}",
                ApiError::TooMuchMessages,
            ),
            ("{\"data\": \"Bad Request: RESULTS_TOO_MUCH\"}", ApiError::TooMuchInlineQueryResults),
            (
                "{\"data\": \"Bad Request: poll has already been closed\"}",
                ApiError::PollHasAlreadyClosed,
            ),
            (
                "{\"data\": \"Bad Request: poll must have at least 2 option\"}",
                ApiError::PollMustHaveMoreOptions,
            ),
            (
                "{\"data\": \"Bad Request: poll can't have more than 10 options\"}",
                ApiError::PollCantHaveMoreOptions,
            ),
            (
                "{\"data\": \"Bad Request: poll options must be non-empty\"}",
                ApiError::PollOptionsMustBeNonEmpty,
            ),
            (
                "{\"data\": \"Bad Request: poll question must be non-empty\"}",
                ApiError::PollQuestionMustBeNonEmpty,
            ),
            (
                "{\"data\": \"Bad Request: poll options length must not exceed 100\"}",
                ApiError::PollOptionsLengthTooLong,
            ),
            (
                "{\"data\": \"Bad Request: poll question length must not exceed 255\"}",
                ApiError::PollQuestionLengthTooLong,
            ),
            (
                "{\"data\": \"Bad Request: message with poll to stop not found\"}",
                ApiError::MessageWithPollNotFound,
            ),
            ("{\"data\": \"Bad Request: message is not a poll\"}", ApiError::MessageIsNotAPoll),
            ("{\"data\": \"Bad Request: chat not found\"}", ApiError::ChatNotFound),
            ("{\"data\": \"Bad Request: user not found\"}", ApiError::UserNotFound),
            (
                "{\"data\": \"Bad Request: chat description is not modified\"}",
                ApiError::ChatDescriptionIsNotModified,
            ),
            (
                "{\"data\": \"Bad Request: query is too old and response timeout expired or query \
                 id is invalid\"}",
                ApiError::InvalidQueryId,
            ),
            ("{\"data\": \"Bad Request: BUTTON_URL_INVALID\"}", ApiError::ButtonUrlInvalid),
            ("{\"data\": \"Bad Request: BUTTON_DATA_INVALID\"}", ApiError::ButtonDataInvalid),
            (
                "{\"data\": \"Bad Request: can't parse inline keyboard button: Text buttons are \
                 unallowed in the inline keyboard\"}",
                ApiError::TextButtonsAreUnallowed,
            ),
            ("{\"data\": \"Bad Request: wrong file id\"}", ApiError::WrongFileId),
            (
                "{\"data\": \"Bad Request: wrong file identifier/HTTP URL specified\"}",
                ApiError::WrongFileIdOrUrl,
            ),
            (
                "{\"data\": \"Bad Request: failed to get HTTP URL content\"}",
                ApiError::FailedToGetUrlContent,
            ),
            ("{\"data\": \"Bad Request: group is deactivated\"}", ApiError::GroupDeactivated),
            ("{\"data\": \"Bad Request: IMAGE_PROCESS_FAILED\"}", ApiError::ImageProcessFailed),
            (
                "{\"data\": \"Bad Request: Photo should be uploaded as an InputFile\"}",
                ApiError::PhotoAsInputFileRequired,
            ),
            ("{\"data\": \"Bad Request: STICKERSET_INVALID\"}", ApiError::InvalidStickersSet),
            (
                "{\"data\": \"Bad Request: sticker set name is already occupied\"}",
                ApiError::StickerSetNameOccupied,
            ),
            ("{\"data\": \"Bad Request: USER_IS_BOT\"}", ApiError::StickerSetOwnerIsBot),
            (
                "{\"data\": \"Bad Request: invalid sticker set name is specified\"}",
                ApiError::InvalidStickerName,
            ),
            (
                "{\"data\": \"Bad Request: not enough rights to pin a message\"}",
                ApiError::NotEnoughRightsToPinMessage,
            ),
            (
                "{\"data\": \"Bad Request: not enough rights to manage pinned messages in the \
                 chat\"}",
                ApiError::NotEnoughRightsToManagePins,
            ),
            (
                "{\"data\": \"Bad Request: not enough rights to change chat permissions\"}",
                ApiError::NotEnoughRightsToChangeChatPermissions,
            ),
            (
                "{\"data\": \"Bad Request: method is available only for supergroups and channel\"}",
                ApiError::MethodNotAvailableInPrivateChats,
            ),
            (
                "{\"data\": \"Bad Request: can't demote chat creator\"}",
                ApiError::CantDemoteChatCreator,
            ),
            ("{\"data\": \"Bad Request: can't restrict self\"}", ApiError::CantRestrictSelf),
            (
                "{\"data\": \"Bad Request: not enough rights to restrict/unrestrict chat member\"}",
                ApiError::NotEnoughRightsToRestrict,
            ),
            (
                "{\"data\": \"Bad Request: need administrator rights in the channel chat\"}",
                ApiError::NotEnoughRightsToPostMessages,
            ),
            (
                "{\"data\": \"Bad Request: bad webhook: HTTPS url must be provided for webhook\"}",
                ApiError::WebhookRequireHttps,
            ),
            (
                "{\"data\": \"Bad Request: bad webhook: Webhook can be set up only on ports 80, \
                 88, 443 or 8443\"}",
                ApiError::BadWebhookPort,
            ),
            (
                "{\"data\": \"Bad Request: bad webhook: Failed to resolve host: Name or service \
                 not known\"}",
                ApiError::UnknownHost,
            ),
            ("{\"data\": \"Bad Request: can't parse URL\"}", ApiError::CantParseUrl),
            (
                "{\"data\": \"Bad Request: can't parse entities: SomeRandomString\"}",
                ApiError::CantParseEntities(
                    "Bad Request: can't parse entities: SomeRandomString".to_owned(),
                ),
            ),
            (
                "{\"data\": \"can't use getUpdates method while webhook is active\"}",
                ApiError::CantGetUpdates,
            ),
            ("{\"data\": \"Unauthorized: bot was kicked from a chat\"}", ApiError::BotKicked),
            (
                "{\"data\": \"Forbidden: bot was kicked from the supergroup chat\"}",
                ApiError::BotKickedFromSupergroup,
            ),
            ("{\"data\": \"Forbidden: user is deactivated\"}", ApiError::UserDeactivated),
            (
                "{\"data\": \"Unauthorized: bot can't initiate conversation with a user\"}",
                ApiError::CantInitiateConversation,
            ),
            (
                "{\"data\": \"Unauthorized: bot can't send messages to bots\"}",
                ApiError::CantTalkWithBots,
            ),
            ("{\"data\": \"Bad Request: wrong HTTP URL\"}", ApiError::WrongHttpUrl),
            (
                "{\"data\": \"Conflict: terminated by other getUpdates request; make sure that \
                 only one bot instance is running\"}",
                ApiError::TerminatedByOtherGetUpdates,
            ),
            ("{\"data\": \"Bad Request: invalid file id\"}", ApiError::FileIdInvalid),
            ("{\"data\": \"Request Entity Too Large\"}", ApiError::RequestEntityTooLarge),
            ("{\"data\": \"RandomError\"}", ApiError::Unknown("RandomError".to_string())),
        ];

        #[derive(Deserialize, Debug)]
        struct Res<T> {
            data: T,
        }

        for (data, expected) in cases {
            let raw = serde_json::from_str::<Res<String>>(data).unwrap().data;
            let parsed = serde_json::from_str::<Res<ApiError>>(data).unwrap().data;
            assert_eq!(&parsed, expected);

            let expected_error_message = match parsed {
                ApiError::Unknown(_) => {
                    format!("Unknown error: \"{raw}\"")
                }
                ApiError::InvalidToken => "Invalid bot token".to_owned(),
                _ => raw,
            };
            assert_eq!(parsed.to_string(), expected_error_message);
        }
    }
}
