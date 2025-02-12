#![allow(clippy::large_enum_variant)]

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::types::{
    Animation, Audio, BareChatId, BusinessConnectionId, Chat, ChatBackground, ChatBoostAdded,
    ChatId, ChatShared, Contact, Dice, Document, ExternalReplyInfo, ForumTopicClosed,
    ForumTopicCreated, ForumTopicEdited, ForumTopicReopened, Game, GeneralForumTopicHidden,
    GeneralForumTopicUnhidden, Giveaway, GiveawayCompleted, GiveawayCreated, GiveawayWinners,
    InlineKeyboardMarkup, Invoice, LinkPreviewOptions, Location, MaybeInaccessibleMessage,
    MessageAutoDeleteTimerChanged, MessageEntity, MessageEntityRef, MessageId, MessageOrigin,
    PassportData, PhotoSize, Poll, ProximityAlertTriggered, Sticker, Story, SuccessfulPayment,
    TextQuote, ThreadId, True, User, UsersShared, Venue, Video, VideoChatEnded,
    VideoChatParticipantsInvited, VideoChatScheduled, VideoChatStarted, VideoNote, Voice,
    WebAppData, WriteAccessAllowed,
};

/// This object represents a message.
///
/// [The official docs](https://core.telegram.org/bots/api#message).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Message {
    /// Unique message identifier inside this chat.
    #[serde(flatten)]
    pub id: MessageId,

    /// Unique identifier of a message thread to which the message belongs; for
    /// supergroups only.
    #[serde(rename = "message_thread_id")]
    pub thread_id: Option<ThreadId>,

    /// Sender, empty for messages sent to channels.
    pub from: Option<User>,

    /// Sender of the message, sent on behalf of a chat. The channel itself for
    /// channel messages. The supergroup itself for messages from anonymous
    /// group administrators. The linked channel for messages automatically
    /// forwarded to the discussion group
    pub sender_chat: Option<Chat>,

    /// Date the message was sent in Unix time.
    #[serde(with = "crate::types::serde_date_from_unix_timestamp")]
    pub date: DateTime<Utc>,

    /// Conversation the message belongs to.
    pub chat: Chat,

    /// `true`, if the message is sent to a forum topic.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_topic_message: bool,

    /// Bot through which the message was sent.
    pub via_bot: Option<User>,

    /// The bot that actually sent the message on behalf of the business
    /// account. Available only for outgoing messages sent on behalf of the
    /// connected business account.
    pub sender_business_bot: Option<User>,

    #[serde(flatten)]
    pub kind: MessageKind,
}

// FIXME: this could be a use-case for serde mixed-tags, some variants need to
//        untagged (`MessageCommon` as an example), while other need to be
//        tagged (e.g.: Forum*)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageKind {
    Common(MessageCommon),
    NewChatMembers(MessageNewChatMembers),
    LeftChatMember(MessageLeftChatMember),
    NewChatTitle(MessageNewChatTitle),
    NewChatPhoto(MessageNewChatPhoto),
    DeleteChatPhoto(MessageDeleteChatPhoto),
    GroupChatCreated(MessageGroupChatCreated),
    SupergroupChatCreated(MessageSupergroupChatCreated),
    ChannelChatCreated(MessageChannelChatCreated),
    MessageAutoDeleteTimerChanged(MessageMessageAutoDeleteTimerChanged),
    Pinned(MessagePinned),
    ChatShared(MessageChatShared),
    UsersShared(MessageUsersShared),
    Invoice(MessageInvoice),
    SuccessfulPayment(MessageSuccessfulPayment),
    ConnectedWebsite(MessageConnectedWebsite),
    WriteAccessAllowed(MessageWriteAccessAllowed),
    PassportData(MessagePassportData),
    Dice(MessageDice),
    ProximityAlertTriggered(MessageProximityAlertTriggered),
    ChatBoostAdded(MessageChatBoostAdded),
    ChatBackground(MessageChatBackground),
    ForumTopicCreated(MessageForumTopicCreated),
    ForumTopicEdited(MessageForumTopicEdited),
    ForumTopicClosed(MessageForumTopicClosed),
    ForumTopicReopened(MessageForumTopicReopened),
    GeneralForumTopicHidden(MessageGeneralForumTopicHidden),
    GeneralForumTopicUnhidden(MessageGeneralForumTopicUnhidden),
    Giveaway(MessageGiveaway),
    GiveawayCompleted(MessageGiveawayCompleted),
    GiveawayCreated(MessageGiveawayCreated),
    GiveawayWinners(MessageGiveawayWinners),
    VideoChatScheduled(MessageVideoChatScheduled),
    VideoChatStarted(MessageVideoChatStarted),
    VideoChatEnded(MessageVideoChatEnded),
    VideoChatParticipantsInvited(MessageVideoChatParticipantsInvited),
    WebAppData(MessageWebAppData),
    /// An empty, content-less message, that can appear in callback queries
    /// attached to old messages.
    Empty {},
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageCommon {
    /// Signature of the post author for messages in channels, or the custom
    /// title of an anonymous group administrator.
    pub author_signature: Option<String>,

    /// Information about the original message for forwarded messages
    pub forward_origin: Option<MessageOrigin>,

    /// For replies, the original message. Note that the Message object in this
    /// field will not contain further `reply_to_message` fields even if it
    /// itself is a reply.
    pub reply_to_message: Option<Box<Message>>,

    /// Information about the message that is being replied to, which may come
    /// from another chat or forum topic
    pub external_reply: Option<ExternalReplyInfo>,

    /// For replies that quote part of the original message, the quoted part of
    /// the message
    pub quote: Option<TextQuote>,

    /// For replies to a story, the original story
    pub reply_to_story: Option<Story>,

    /// If the sender of the message boosted the chat, the number of boosts
    /// added by the user
    pub sender_boost_count: Option<u16>,

    /// Date the message was last edited in Unix time.
    #[serde(default, with = "crate::types::serde_opt_date_from_unix_timestamp")]
    pub edit_date: Option<DateTime<Utc>>,

    #[serde(flatten)]
    pub media_kind: MediaKind,

    /// Inline keyboard attached to the message. `login_url` buttons are
    /// represented as ordinary `url` buttons.
    pub reply_markup: Option<InlineKeyboardMarkup>,

    /// `true`, if the message is a channel post that was automatically
    /// forwarded to the connected discussion group.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_automatic_forward: bool,

    /// `true`, if the message can't be forwarded.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub has_protected_content: bool,

    /// `true`, if the message was sent by an implicit action, for example, as
    /// an away or a greeting business message, or as a scheduled message
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_from_offline: bool,

    /// Unique identifier of the business connection from which the message was
    /// received. If non-empty, the message belongs to a chat of the
    /// corresponding business account that is independent from any potential
    /// bot chat which might share the same identifier.
    pub business_connection_id: Option<BusinessConnectionId>,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageNewChatMembers {
    /// New members that were added to the group or supergroup and
    /// information about them (the bot itself may be one of these
    /// members).
    pub new_chat_members: Vec<User>,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageLeftChatMember {
    /// A member was removed from the group, information about them (this
    /// member may be the bot itself).
    pub left_chat_member: User,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageNewChatTitle {
    /// A chat title was changed to this value.
    pub new_chat_title: String,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageNewChatPhoto {
    /// A chat photo was change to this value.
    pub new_chat_photo: Vec<PhotoSize>,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MessageDeleteChatPhoto {
    /// Service message: the chat photo was deleted.
    pub delete_chat_photo: True,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MessageGroupChatCreated {
    /// Service message: the group has been created.
    pub group_chat_created: True,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MessageSupergroupChatCreated {
    /// Service message: the supergroup has been created. This field can‘t
    /// be received in a message coming through updates, because bot can’t
    /// be a member of a supergroup when it is created. It can only be
    /// found in `reply_to_message` if someone replies to a very first
    /// message in a directly created supergroup.
    pub supergroup_chat_created: True,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MessageChannelChatCreated {
    /// Service message: the channel has been created. This field can‘t be
    /// received in a message coming through updates, because bot can’t be
    /// a member of a channel when it is created. It can only be found in
    /// `reply_to_message` if someone replies to a very first message in a
    /// channel.
    pub channel_chat_created: True,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageMessageAutoDeleteTimerChanged {
    /// Service message: auto-delete timer settings changed in the chat.
    pub message_auto_delete_timer_changed: MessageAutoDeleteTimerChanged,
}

/// Represents group migration to a supergroup or a supergroup migration from a
/// group.
///
/// Note that bot receives **both** updates. For example: a group with id `0`
/// migrates to a supergroup with id `1` bots in that group will receive 2
/// updates:
/// - `message.chat.id = 0`, `message.chat_migration() = ChatMigration::To {
///   chat_id: 1 }`
/// - `message.chat.id = 1`, `message.chat_migration() = ChatMigration::From {
///   chat_id: 0 }`
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatMigration {
    /// The group has been migrated to a supergroup with the specified
    /// identifier `chat_id`.
    To {
        #[serde(rename = "migrate_to_chat_id")]
        chat_id: ChatId,
    },

    /// The supergroup has been migrated from a group with the specified
    /// identifier `chat_id`.
    From {
        #[serde(rename = "migrate_from_chat_id")]
        chat_id: ChatId,
    },
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessagePinned {
    /// Specified message was pinned. Note that the Message object in this
    /// field will not contain further `reply_to_message` fields even if it
    /// is itself a reply.
    #[serde(rename = "pinned_message")]
    pub pinned: Box<MaybeInaccessibleMessage>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageChatShared {
    /// A chat was shared with the bot.
    pub chat_shared: ChatShared,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageUsersShared {
    /// Users were shared with the bot
    pub users_shared: UsersShared,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageInvoice {
    /// Message is an invoice for a [payment], information about the
    /// invoice. [More about payments »].
    ///
    /// [payment]: https://core.telegram.org/bots/api#payments
    /// [More about payments »]: https://core.telegram.org/bots/api#payments
    pub invoice: Invoice,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageSuccessfulPayment {
    /// Message is a service message about a successful payment,
    /// information about the payment. [More about payments »].
    ///
    /// [More about payments »]: https://core.telegram.org/bots/api#payments
    pub successful_payment: SuccessfulPayment,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageConnectedWebsite {
    /// The domain name of the website on which the user has logged in.
    /// [More about Telegram Login »].
    ///
    /// [More about Telegram Login »]: https://core.telegram.org/widgets/login
    pub connected_website: String,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessagePassportData {
    /// Telegram Passport data.
    pub passport_data: PassportData,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MediaKind {
    // Note:
    // - `Venue` must be in front of `Location`
    // - `Animation` must be in front of `Document`
    //
    // This is needed so serde doesn't parse `Venue` as `Location` or `Animation` as `Document`
    // (for backward compatability telegram duplicates some fields)
    //
    // See <https://github.com/teloxide/teloxide/issues/481>
    Animation(MediaAnimation),
    Audio(MediaAudio),
    Contact(MediaContact),
    Document(MediaDocument),
    Game(MediaGame),
    Venue(MediaVenue),
    Location(MediaLocation),
    Photo(MediaPhoto),
    Poll(MediaPoll),
    Sticker(MediaSticker),
    Story(MediaStory),
    Text(MediaText),
    Video(MediaVideo),
    VideoNote(MediaVideoNote),
    Voice(MediaVoice),
    Migration(ChatMigration),
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaAnimation {
    /// Message is an animation, information about the animation. For
    /// backward compatibility, when this field is set, the document field
    /// will also be set.
    pub animation: Animation,

    /// Caption for the animation, 0-1024 characters.
    pub caption: Option<String>,

    /// For messages with a caption, special entities like usernames, URLs,
    /// bot commands, etc. that appear in the caption.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub caption_entities: Vec<MessageEntity>,

    /// `true`, if the message media is covered by a spoiler animation.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub has_media_spoiler: bool,
    // Note: for backward compatibility telegram also sends `document` field, but we ignore it
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaAudio {
    /// Message is an audio file, information about the file.
    pub audio: Audio,

    /// Caption for the audio, 0-1024 characters.
    pub caption: Option<String>,

    /// For messages with a caption, special entities like usernames, URLs,
    /// bot commands, etc. that appear in the caption.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub caption_entities: Vec<MessageEntity>,

    /// The unique identifier of a media message group this message belongs
    /// to.
    pub media_group_id: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaContact {
    /// Message is a shared contact, information about the contact.
    pub contact: Contact,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaDocument {
    /// Message is a general file, information about the file.
    pub document: Document,

    /// Caption for the document, 0-1024 characters.
    pub caption: Option<String>,

    /// For messages with a caption, special entities like usernames, URLs,
    /// bot commands, etc. that appear in the caption.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub caption_entities: Vec<MessageEntity>,

    /// The unique identifier of a media message group this message belongs
    /// to.
    pub media_group_id: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaGame {
    /// Message is a game, information about the game. [More
    /// about games »].
    ///
    /// [More about games »]: https://core.telegram.org/bots/api#games
    pub game: Game,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaLocation {
    /// Message is a shared location, information about the location.
    pub location: Location,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaPhoto {
    /// Message is a photo, available sizes of the photo.
    pub photo: Vec<PhotoSize>,

    /// Caption for the photo, 0-1024 characters.
    pub caption: Option<String>,

    /// For messages with a caption, special entities like usernames, URLs,
    /// bot commands, etc. that appear in the caption.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub caption_entities: Vec<MessageEntity>,

    /// `true`, if the message media is covered by a spoiler animation.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub has_media_spoiler: bool,

    /// The unique identifier of a media message group this message belongs
    /// to.
    pub media_group_id: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaPoll {
    /// Message is a native poll, information about the poll.
    pub poll: Poll,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaSticker {
    /// Message is a sticker, information about the sticker.
    pub sticker: Sticker,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaStory {
    /// Message is a forwarded story
    pub story: Story,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaText {
    /// For text messages, the actual UTF-8 text of the message, 0-4096
    /// characters.
    pub text: String,

    /// For text messages, special entities like usernames, URLs, bot
    /// commands, etc. that appear in the text.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entities: Vec<MessageEntity>,

    /// Options used for link preview generation for the message, if it is a
    /// text message and link preview options were changed
    pub link_preview_options: Option<LinkPreviewOptions>,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaVideo {
    /// Message is a video, information about the video.
    pub video: Video,

    /// Caption for the video, 0-1024 characters.
    pub caption: Option<String>,

    /// For messages with a caption, special entities like usernames, URLs,
    /// bot commands, etc. that appear in the caption.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub caption_entities: Vec<MessageEntity>,

    /// `true`, if the message media is covered by a spoiler animation.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub has_media_spoiler: bool,

    /// The unique identifier of a media message group this message belongs
    /// to.
    pub media_group_id: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaVideoNote {
    /// Message is a [video note], information about the video message.
    ///
    /// [video note]: https://telegram.org/blog/video-messages-and-telescope
    pub video_note: VideoNote,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaVoice {
    /// Message is a voice message, information about the file.
    pub voice: Voice,

    /// Caption for the voice, 0-1024 characters.
    pub caption: Option<String>,

    /// For messages with a caption, special entities like usernames, URLs,
    /// bot commands, etc. that appear in the caption.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub caption_entities: Vec<MessageEntity>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaVenue {
    /// Message is a venue, information about the venue.
    pub venue: Venue,
    // Note: for backward compatibility telegram also sends `location` field, but we ignore it
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageDice {
    /// Message is a dice with random value from 1 to 6.
    pub dice: Dice,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageProximityAlertTriggered {
    /// Service message. A user in the chat triggered another user's proximity
    /// alert while sharing Live Location.
    pub proximity_alert_triggered: ProximityAlertTriggered,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageChatBoostAdded {
    /// Service message. User boosted the chat.
    pub boost_added: ChatBoostAdded,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageChatBackground {
    /// Service message. Chat background set.
    pub chat_background_set: ChatBackground,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageWriteAccessAllowed {
    /// Service message: the user allowed the bot added to the attachment menu
    /// to write messages.
    pub write_access_allowed: WriteAccessAllowed,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageForumTopicCreated {
    /// Service message: forum topic created.
    pub forum_topic_created: ForumTopicCreated,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageForumTopicEdited {
    /// Service message: forum topic edited.
    pub forum_topic_edited: ForumTopicEdited,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageForumTopicClosed {
    /// Service message: forum topic closed.
    pub forum_topic_closed: ForumTopicClosed,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageForumTopicReopened {
    /// Service message: forum topic reopened.
    pub forum_topic_reopened: ForumTopicReopened,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageGeneralForumTopicHidden {
    /// Service message: the 'General' forum topic hidden.
    pub general_forum_topic_hidden: GeneralForumTopicHidden,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageGeneralForumTopicUnhidden {
    /// Service message: the 'General' forum topic unhidden.
    pub general_forum_topic_unhidden: GeneralForumTopicUnhidden,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageGiveaway {
    /// Message is giveaway, information about a scheduled giveaway. [More about
    /// giveaways »]
    ///
    /// [More about giveaways »]: https://core.telegram.org/api#giveaways-amp-gifts
    pub giveaway: Giveaway,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageGiveawayCompleted {
    /// Service message: a 'Giveaway' completed. [More about giveaways
    /// »]
    ///
    /// [More about giveaways »]: https://core.telegram.org/api#giveaways-amp-gifts
    pub giveaway_completed: GiveawayCompleted,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageGiveawayCreated {
    /// Service message: a scheduled 'Giveaway' created. [More about giveaways
    /// »]
    ///
    /// [More about giveaways »]: https://core.telegram.org/api#giveaways-amp-gifts
    pub giveaway_created: GiveawayCreated,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageGiveawayWinners {
    /// Message is giveaway winners, information about the completion of a
    /// giveaway with public winners. [More about giveaways »]
    ///
    /// [More about giveaways »]: https://core.telegram.org/api#giveaways-amp-gifts
    pub giveaway_winners: GiveawayWinners,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageVideoChatScheduled {
    /// Service message: video chat scheduled
    pub video_chat_scheduled: VideoChatScheduled,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageVideoChatStarted {
    /// Service message: video chat started.
    pub video_chat_started: VideoChatStarted,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageVideoChatEnded {
    /// Service message: video chat ended.
    pub video_chat_ended: VideoChatEnded,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageVideoChatParticipantsInvited {
    /// Service message: new participants invited to a video chat.
    pub video_chat_participants_invited: VideoChatParticipantsInvited,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageWebAppData {
    /// Service message: data sent by a Web App.
    pub web_app_data: WebAppData,
}

mod getters {
    use chrono::{DateTime, Utc};
    use std::ops::Deref;

    use crate::types::{
        self, message::MessageKind::*, Chat, ChatId, ChatMigration, LinkPreviewOptions,
        MaybeInaccessibleMessage, MediaAnimation, MediaAudio, MediaContact, MediaDocument,
        MediaGame, MediaKind, MediaLocation, MediaPhoto, MediaPoll, MediaSticker, MediaStory,
        MediaText, MediaVenue, MediaVideo, MediaVideoNote, MediaVoice, Message,
        MessageChannelChatCreated, MessageChatShared, MessageCommon, MessageConnectedWebsite,
        MessageDeleteChatPhoto, MessageDice, MessageEntity, MessageGroupChatCreated, MessageId,
        MessageInvoice, MessageLeftChatMember, MessageNewChatMembers, MessageNewChatPhoto,
        MessageNewChatTitle, MessageOrigin, MessagePassportData, MessagePinned,
        MessageProximityAlertTriggered, MessageSuccessfulPayment, MessageSupergroupChatCreated,
        MessageUsersShared, MessageVideoChatParticipantsInvited, PhotoSize, Story, TextQuote, User,
    };

    use super::{
        MessageChatBackground, MessageChatBoostAdded, MessageForumTopicClosed,
        MessageForumTopicCreated, MessageForumTopicEdited, MessageForumTopicReopened,
        MessageGeneralForumTopicHidden, MessageGeneralForumTopicUnhidden, MessageGiveaway,
        MessageGiveawayCompleted, MessageGiveawayCreated, MessageGiveawayWinners,
        MessageMessageAutoDeleteTimerChanged, MessageVideoChatEnded, MessageVideoChatScheduled,
        MessageVideoChatStarted, MessageWebAppData, MessageWriteAccessAllowed,
    };

    /// Getters for [Message] fields from [telegram docs].
    ///
    /// [Message]: crate::types::Message
    /// [telegram docs]: https://core.telegram.org/bots/api#message
    impl Message {
        /// Returns the user who sent the message.
        #[deprecated(since = "0.13.0", note = "use `.from` field instead")]
        #[must_use]
        pub fn from(&self) -> Option<&User> {
            self.from.as_ref()
        }

        #[must_use]
        pub fn author_signature(&self) -> Option<&str> {
            match &self.kind {
                Common(MessageCommon { author_signature, .. }) => author_signature.as_deref(),
                _ => None,
            }
        }

        #[deprecated(since = "0.13.0", note = "use `.sender_chat` field instead")]
        #[must_use]
        pub fn sender_chat(&self) -> Option<&Chat> {
            self.sender_chat.as_ref()
        }

        #[must_use]
        pub fn forward_origin(&self) -> Option<&MessageOrigin> {
            match &self.kind {
                Common(MessageCommon { forward_origin, .. }) => forward_origin.as_ref(),
                _ => None,
            }
        }

        #[must_use]
        pub fn quote(&self) -> Option<&TextQuote> {
            match &self.kind {
                Common(MessageCommon { quote, .. }) => quote.as_ref(),
                _ => None,
            }
        }

        #[must_use]
        pub fn reply_to_story(&self) -> Option<&Story> {
            match &self.kind {
                Common(MessageCommon { reply_to_story, .. }) => reply_to_story.as_ref(),
                _ => None,
            }
        }

        #[must_use]
        pub fn sender_boost_count(&self) -> Option<u16> {
            match &self.kind {
                Common(MessageCommon { sender_boost_count, .. }) => *sender_boost_count,
                _ => None,
            }
        }

        #[must_use]
        pub fn forward_date(&self) -> Option<DateTime<Utc>> {
            self.forward_origin().map(|f| f.date())
        }

        #[must_use]
        pub fn forward_from_user(&self) -> Option<&User> {
            self.forward_origin().and_then(|origin| match origin {
                MessageOrigin::User { sender_user, .. } => Some(sender_user),
                _ => None,
            })
        }

        #[must_use]
        pub fn forward_from_chat(&self) -> Option<&Chat> {
            self.forward_origin().and_then(|origin| match origin {
                MessageOrigin::Chat { sender_chat, .. } => Some(sender_chat),
                _ => None,
            })
        }

        #[must_use]
        pub fn forward_from_sender_name(&self) -> Option<&str> {
            self.forward_origin().and_then(|origin| match origin {
                MessageOrigin::HiddenUser { sender_user_name, .. } => {
                    Some(sender_user_name.as_str())
                }
                _ => None,
            })
        }

        #[must_use]
        pub fn forward_from_message_id(&self) -> Option<MessageId> {
            self.forward_origin().and_then(|origin| match origin {
                MessageOrigin::Channel { message_id, .. } => Some(*message_id),
                _ => None,
            })
        }

        #[must_use]
        pub fn forward_author_signature(&self) -> Option<&str> {
            self.forward_origin().and_then(|origin| match origin {
                MessageOrigin::Chat { author_signature, .. } => {
                    author_signature.as_ref().map(|a| a.as_str())
                }
                MessageOrigin::Channel { author_signature, .. } => {
                    author_signature.as_ref().map(|a| a.as_str())
                }
                _ => None,
            })
        }

        #[must_use]
        pub fn reply_to_message(&self) -> Option<&Message> {
            self.common().and_then(|m| m.reply_to_message.as_deref())
        }

        #[must_use]
        pub fn edit_date(&self) -> Option<&DateTime<Utc>> {
            match &self.kind {
                Common(MessageCommon { edit_date, .. }) => edit_date.as_ref(),
                _ => None,
            }
        }

        #[must_use]
        pub fn media_group_id(&self) -> Option<&str> {
            match &self.kind {
                Common(MessageCommon {
                    media_kind: MediaKind::Video(MediaVideo { media_group_id, .. }),
                    ..
                })
                | Common(MessageCommon {
                    media_kind: MediaKind::Photo(MediaPhoto { media_group_id, .. }),
                    ..
                })
                | Common(MessageCommon {
                    media_kind: MediaKind::Document(MediaDocument { media_group_id, .. }),
                    ..
                })
                | Common(MessageCommon {
                    media_kind: MediaKind::Audio(MediaAudio { media_group_id, .. }),
                    ..
                }) => media_group_id.as_ref().map(Deref::deref),
                _ => None,
            }
        }

        #[must_use]
        pub fn text(&self) -> Option<&str> {
            match &self.kind {
                Common(MessageCommon {
                    media_kind: MediaKind::Text(MediaText { text, .. }),
                    ..
                }) => Some(text),
                _ => None,
            }
        }

        /// Returns message entities that represent text formatting.
        ///
        /// **Note:** you probably want to use [`parse_entities`] instead.
        ///
        /// This function returns `Some(entities)` for **text messages** and
        /// `None` for all other kinds of messages (including photos with
        /// captions).
        ///
        /// See also: [`caption_entities`].
        ///
        /// [`parse_entities`]: Message::parse_entities
        /// [`caption_entities`]: Message::caption_entities
        #[must_use]
        pub fn entities(&self) -> Option<&[MessageEntity]> {
            match &self.kind {
                Common(MessageCommon {
                    media_kind: MediaKind::Text(MediaText { entities, .. }),
                    ..
                }) => Some(entities),
                _ => None,
            }
        }

        #[must_use]
        pub fn link_preview_options(&self) -> Option<&LinkPreviewOptions> {
            match &self.kind {
                Common(MessageCommon {
                    media_kind: MediaKind::Text(MediaText { link_preview_options, .. }),
                    ..
                }) => link_preview_options.as_ref(),
                _ => None,
            }
        }

        /// Returns message entities that represent text formatting.
        ///
        /// **Note:** you probably want to use [`parse_caption_entities`]
        /// instead.
        ///
        /// This function returns `Some(entities)` for **media messages** and
        /// `None` for all other kinds of messages (including text messages).
        ///
        /// See also: [`entities`].
        ///
        /// [`parse_caption_entities`]: Message::parse_caption_entities
        /// [`entities`]: Message::entities
        #[must_use]
        pub fn caption_entities(&self) -> Option<&[MessageEntity]> {
            match &self.kind {
                Common(MessageCommon {
                    media_kind: MediaKind::Animation(MediaAnimation { caption_entities, .. }),
                    ..
                })
                | Common(MessageCommon {
                    media_kind: MediaKind::Audio(MediaAudio { caption_entities, .. }),
                    ..
                })
                | Common(MessageCommon {
                    media_kind: MediaKind::Document(MediaDocument { caption_entities, .. }),
                    ..
                })
                | Common(MessageCommon {
                    media_kind: MediaKind::Photo(MediaPhoto { caption_entities, .. }),
                    ..
                })
                | Common(MessageCommon {
                    media_kind: MediaKind::Video(MediaVideo { caption_entities, .. }),
                    ..
                })
                | Common(MessageCommon {
                    media_kind: MediaKind::Voice(MediaVoice { caption_entities, .. }),
                    ..
                }) => Some(caption_entities),
                _ => None,
            }
        }

        /// Returns `true` if the message media is covered by a spoiler
        /// animation.
        ///
        /// Getter for [`MediaPhoto::has_media_spoiler`],
        /// [`MediaVideo::has_media_spoiler`] and
        /// [`MediaAnimation::has_media_spoiler`].
        #[must_use]
        pub fn has_media_spoiler(&self) -> bool {
            self.common()
                .map(|m| match m.media_kind {
                    MediaKind::Animation(MediaAnimation { has_media_spoiler, .. })
                    | MediaKind::Photo(MediaPhoto { has_media_spoiler, .. })
                    | MediaKind::Video(MediaVideo { has_media_spoiler, .. }) => has_media_spoiler,
                    MediaKind::Audio(_)
                    | MediaKind::Contact(_)
                    | MediaKind::Document(_)
                    | MediaKind::Game(_)
                    | MediaKind::Venue(_)
                    | MediaKind::Location(_)
                    | MediaKind::Poll(_)
                    | MediaKind::Sticker(_)
                    | MediaKind::Story(_)
                    | MediaKind::Text(_)
                    | MediaKind::VideoNote(_)
                    | MediaKind::Voice(_)
                    | MediaKind::Migration(_) => false,
                })
                .unwrap_or(false)
        }

        #[must_use]
        pub fn audio(&self) -> Option<&types::Audio> {
            match &self.kind {
                Common(MessageCommon {
                    media_kind: MediaKind::Audio(MediaAudio { audio, .. }),
                    ..
                }) => Some(audio),
                _ => None,
            }
        }

        #[must_use]
        pub fn document(&self) -> Option<&types::Document> {
            match &self.kind {
                Common(MessageCommon {
                    media_kind: MediaKind::Document(MediaDocument { document, .. }),
                    ..
                }) => Some(document),
                _ => None,
            }
        }

        #[must_use]
        pub fn animation(&self) -> Option<&types::Animation> {
            match &self.kind {
                Common(MessageCommon {
                    media_kind: MediaKind::Animation(MediaAnimation { animation, .. }),
                    ..
                }) => Some(animation),
                _ => None,
            }
        }

        #[must_use]
        pub fn game(&self) -> Option<&types::Game> {
            match &self.kind {
                Common(MessageCommon {
                    media_kind: MediaKind::Game(MediaGame { game, .. }),
                    ..
                }) => Some(game),
                _ => None,
            }
        }

        #[must_use]
        pub fn photo(&self) -> Option<&[PhotoSize]> {
            match &self.kind {
                Common(MessageCommon {
                    media_kind: MediaKind::Photo(MediaPhoto { photo, .. }),
                    ..
                }) => Some(photo),
                _ => None,
            }
        }

        #[must_use]
        pub fn sticker(&self) -> Option<&types::Sticker> {
            match &self.kind {
                Common(MessageCommon {
                    media_kind: MediaKind::Sticker(MediaSticker { sticker, .. }),
                    ..
                }) => Some(sticker),
                _ => None,
            }
        }

        #[must_use]
        pub fn story(&self) -> Option<&types::Story> {
            match &self.kind {
                Common(MessageCommon {
                    media_kind: MediaKind::Story(MediaStory { story, .. }),
                    ..
                }) => Some(story),
                _ => None,
            }
        }

        #[must_use]
        pub fn video(&self) -> Option<&types::Video> {
            match &self.kind {
                Common(MessageCommon {
                    media_kind: MediaKind::Video(MediaVideo { video, .. }),
                    ..
                }) => Some(video),
                _ => None,
            }
        }

        #[must_use]
        pub fn voice(&self) -> Option<&types::Voice> {
            match &self.kind {
                Common(MessageCommon {
                    media_kind: MediaKind::Voice(MediaVoice { voice, .. }),
                    ..
                }) => Some(voice),
                _ => None,
            }
        }

        #[must_use]
        pub fn video_note(&self) -> Option<&types::VideoNote> {
            match &self.kind {
                Common(MessageCommon {
                    media_kind: MediaKind::VideoNote(MediaVideoNote { video_note, .. }),
                    ..
                }) => Some(video_note),
                _ => None,
            }
        }

        #[must_use]
        pub fn caption(&self) -> Option<&str> {
            match &self.kind {
                Common(MessageCommon {
                    media_kind:
                        MediaKind::Animation(MediaAnimation { caption, .. })
                        | MediaKind::Audio(MediaAudio { caption, .. })
                        | MediaKind::Document(MediaDocument { caption, .. })
                        | MediaKind::Photo(MediaPhoto { caption, .. })
                        | MediaKind::Video(MediaVideo { caption, .. })
                        | MediaKind::Voice(MediaVoice { caption, .. }),
                    ..
                }) => caption.as_ref().map(Deref::deref),
                _ => None,
            }
        }

        #[must_use]
        pub fn contact(&self) -> Option<&types::Contact> {
            match &self.kind {
                Common(MessageCommon {
                    media_kind: MediaKind::Contact(MediaContact { contact, .. }),
                    ..
                }) => Some(contact),
                _ => None,
            }
        }

        #[must_use]
        pub fn location(&self) -> Option<&types::Location> {
            match &self.kind {
                Common(MessageCommon {
                    media_kind: MediaKind::Location(MediaLocation { location, .. }),
                    ..
                }) => Some(location),
                _ => None,
            }
        }

        #[must_use]
        pub fn venue(&self) -> Option<&types::Venue> {
            match &self.kind {
                Common(MessageCommon {
                    media_kind: MediaKind::Venue(MediaVenue { venue, .. }),
                    ..
                }) => Some(venue),
                _ => None,
            }
        }

        #[must_use]
        pub fn poll(&self) -> Option<&types::Poll> {
            match &self.kind {
                Common(MessageCommon {
                    media_kind: MediaKind::Poll(MediaPoll { poll, .. }),
                    ..
                }) => Some(poll),
                _ => None,
            }
        }

        #[must_use]
        pub fn new_chat_members(&self) -> Option<&[User]> {
            match &self.kind {
                NewChatMembers(MessageNewChatMembers { new_chat_members }) => {
                    Some(new_chat_members.as_ref())
                }
                _ => None,
            }
        }

        #[must_use]
        pub fn left_chat_member(&self) -> Option<&User> {
            match &self.kind {
                LeftChatMember(MessageLeftChatMember { left_chat_member }) => {
                    Some(left_chat_member)
                }
                _ => None,
            }
        }

        #[must_use]
        pub fn new_chat_title(&self) -> Option<&str> {
            match &self.kind {
                NewChatTitle(MessageNewChatTitle { new_chat_title }) => Some(new_chat_title),
                _ => None,
            }
        }

        #[must_use]
        pub fn new_chat_photo(&self) -> Option<&[PhotoSize]> {
            match &self.kind {
                NewChatPhoto(MessageNewChatPhoto { new_chat_photo }) => Some(new_chat_photo),
                _ => None,
            }
        }

        /// Returns `true` if the incoming [`Message`] contains the
        /// `delete_chat_photo` Service message.
        ///
        /// [More on this](https://core.telegram.org/bots/api#message)
        #[must_use]
        pub fn is_delete_chat_photo(&self) -> bool {
            matches!(&self.kind, DeleteChatPhoto(..))
        }

        #[must_use]
        pub fn delete_chat_photo(&self) -> Option<&MessageDeleteChatPhoto> {
            match &self.kind {
                DeleteChatPhoto(message_delete_chat_photo) => Some(message_delete_chat_photo),
                _ => None,
            }
        }

        /// Returns `true` if the incoming [`Message`] contains the
        /// `group_chat_created` Service message.
        ///
        /// [More on this](https://core.telegram.org/bots/api#message)
        #[must_use]
        pub fn is_group_chat_created(&self) -> bool {
            matches!(&self.kind, GroupChatCreated(..))
        }

        #[must_use]
        pub fn group_chat_created(&self) -> Option<&MessageGroupChatCreated> {
            match &self.kind {
                GroupChatCreated(message_group_chat_created) => Some(message_group_chat_created),
                _ => None,
            }
        }

        /// Returns `true` if the incoming [`Message`] contains the
        /// `supergroup_chat_created` Service message.
        ///
        /// [More on this](https://core.telegram.org/bots/api#message)
        #[must_use]
        pub fn is_super_group_chat_created(&self) -> bool {
            matches!(&self.kind, SupergroupChatCreated(..))
        }

        #[must_use]
        pub fn super_group_chat_created(&self) -> Option<&MessageSupergroupChatCreated> {
            match &self.kind {
                SupergroupChatCreated(message_supergroup_chat_created) => {
                    Some(message_supergroup_chat_created)
                }
                _ => None,
            }
        }

        /// Returns `true` if the incoming [`Message`] contains the
        /// `channel_chat_created` Service message.
        ///
        /// [More on this](https://core.telegram.org/bots/api#message)
        #[must_use]
        pub fn is_channel_chat_created(&self) -> bool {
            matches!(&self.kind, ChannelChatCreated(..))
        }

        #[must_use]
        pub fn channel_chat_created(&self) -> Option<&MessageChannelChatCreated> {
            match &self.kind {
                ChannelChatCreated(message_channel_chat_created) => {
                    Some(message_channel_chat_created)
                }
                _ => None,
            }
        }

        #[must_use]
        pub fn message_auto_delete_timer_changed(
            &self,
        ) -> Option<&types::MessageAutoDeleteTimerChanged> {
            match &self.kind {
                MessageAutoDeleteTimerChanged(MessageMessageAutoDeleteTimerChanged {
                    message_auto_delete_timer_changed,
                }) => Some(message_auto_delete_timer_changed),
                _ => None,
            }
        }

        #[must_use]
        pub fn chat_migration(&self) -> Option<&ChatMigration> {
            match &self.kind {
                Common(MessageCommon {
                    media_kind: MediaKind::Migration(chat_migration), ..
                }) => Some(chat_migration),
                _ => None,
            }
        }

        // FIXME: remove references to small values (requires changing
        // `define_message_ext`)
        #[must_use]
        pub fn migrate_to_chat_id(&self) -> Option<&ChatId> {
            match &self.kind {
                Common(MessageCommon {
                    media_kind: MediaKind::Migration(ChatMigration::To { chat_id }),
                    ..
                }) => Some(chat_id),
                _ => None,
            }
        }

        // FIXME: remove references to small values (requires changing
        // `define_message_ext`)
        #[must_use]
        pub fn migrate_from_chat_id(&self) -> Option<&ChatId> {
            match &self.kind {
                Common(MessageCommon {
                    media_kind: MediaKind::Migration(ChatMigration::From { chat_id }),
                    ..
                }) => Some(chat_id),
                _ => None,
            }
        }

        #[must_use]
        pub fn pinned_message(&self) -> Option<&MaybeInaccessibleMessage> {
            match &self.kind {
                Pinned(MessagePinned { pinned }) => Some(pinned),
                _ => None,
            }
        }

        #[must_use]
        pub fn invoice(&self) -> Option<&types::Invoice> {
            match &self.kind {
                Invoice(MessageInvoice { invoice }) => Some(invoice),
                _ => None,
            }
        }

        #[must_use]
        pub fn successful_payment(&self) -> Option<&types::SuccessfulPayment> {
            match &self.kind {
                SuccessfulPayment(MessageSuccessfulPayment { successful_payment }) => {
                    Some(successful_payment)
                }
                _ => None,
            }
        }

        #[must_use]
        pub fn connected_website(&self) -> Option<&str> {
            match &self.kind {
                ConnectedWebsite(MessageConnectedWebsite { connected_website }) => {
                    Some(connected_website)
                }
                _ => None,
            }
        }

        #[must_use]
        pub fn write_access_allowed(&self) -> Option<&types::WriteAccessAllowed> {
            match &self.kind {
                WriteAccessAllowed(MessageWriteAccessAllowed { write_access_allowed }) => {
                    Some(write_access_allowed)
                }
                _ => None,
            }
        }

        #[must_use]
        pub fn passport_data(&self) -> Option<&types::PassportData> {
            match &self.kind {
                PassportData(MessagePassportData { passport_data }) => Some(passport_data),
                _ => None,
            }
        }

        #[must_use]
        pub fn shared_chat(&self) -> Option<&types::ChatShared> {
            match &self.kind {
                ChatShared(MessageChatShared { chat_shared }) => Some(chat_shared),
                _ => None,
            }
        }

        #[must_use]
        pub fn shared_users(&self) -> Option<&types::UsersShared> {
            match &self.kind {
                UsersShared(MessageUsersShared { users_shared }) => Some(users_shared),
                _ => None,
            }
        }

        #[must_use]
        pub fn dice(&self) -> Option<&types::Dice> {
            match &self.kind {
                Dice(MessageDice { dice }) => Some(dice),
                _ => None,
            }
        }

        #[must_use]
        pub fn proximity_alert_triggered(&self) -> Option<&types::ProximityAlertTriggered> {
            match &self.kind {
                ProximityAlertTriggered(MessageProximityAlertTriggered {
                    proximity_alert_triggered,
                }) => Some(proximity_alert_triggered),
                _ => None,
            }
        }

        #[must_use]
        pub fn boost_added(&self) -> Option<&types::ChatBoostAdded> {
            match &self.kind {
                ChatBoostAdded(MessageChatBoostAdded { boost_added }) => Some(boost_added),
                _ => None,
            }
        }

        #[must_use]
        pub fn chat_background_set(&self) -> Option<&types::ChatBackground> {
            match &self.kind {
                ChatBackground(MessageChatBackground { chat_background_set }) => {
                    Some(chat_background_set)
                }
                _ => None,
            }
        }

        #[must_use]
        pub fn forum_topic_created(&self) -> Option<&types::ForumTopicCreated> {
            match &self.kind {
                ForumTopicCreated(MessageForumTopicCreated { forum_topic_created }) => {
                    Some(forum_topic_created)
                }
                _ => None,
            }
        }

        #[must_use]
        pub fn forum_topic_edited(&self) -> Option<&types::ForumTopicEdited> {
            match &self.kind {
                ForumTopicEdited(MessageForumTopicEdited { forum_topic_edited }) => {
                    Some(forum_topic_edited)
                }
                _ => None,
            }
        }

        #[must_use]
        pub fn forum_topic_closed(&self) -> Option<&types::ForumTopicClosed> {
            match &self.kind {
                ForumTopicClosed(MessageForumTopicClosed { forum_topic_closed }) => {
                    Some(forum_topic_closed)
                }
                _ => None,
            }
        }

        #[must_use]
        pub fn forum_topic_reopened(&self) -> Option<&types::ForumTopicReopened> {
            match &self.kind {
                ForumTopicReopened(MessageForumTopicReopened { forum_topic_reopened }) => {
                    Some(forum_topic_reopened)
                }
                _ => None,
            }
        }

        #[must_use]
        pub fn general_forum_topic_hidden(&self) -> Option<&types::GeneralForumTopicHidden> {
            match &self.kind {
                GeneralForumTopicHidden(MessageGeneralForumTopicHidden {
                    general_forum_topic_hidden,
                }) => Some(general_forum_topic_hidden),
                _ => None,
            }
        }

        #[must_use]
        pub fn general_forum_topic_unhidden(&self) -> Option<&types::GeneralForumTopicUnhidden> {
            match &self.kind {
                GeneralForumTopicUnhidden(MessageGeneralForumTopicUnhidden {
                    general_forum_topic_unhidden,
                }) => Some(general_forum_topic_unhidden),
                _ => None,
            }
        }

        #[must_use]
        pub fn giveaway(&self) -> Option<&types::Giveaway> {
            match &self.kind {
                Giveaway(MessageGiveaway { giveaway }) => Some(giveaway),
                _ => None,
            }
        }

        #[must_use]
        pub fn giveaway_completed(&self) -> Option<&types::GiveawayCompleted> {
            match &self.kind {
                GiveawayCompleted(MessageGiveawayCompleted { giveaway_completed }) => {
                    Some(giveaway_completed)
                }
                _ => None,
            }
        }

        #[must_use]
        pub fn giveaway_created(&self) -> Option<&types::GiveawayCreated> {
            match &self.kind {
                GiveawayCreated(MessageGiveawayCreated { giveaway_created }) => {
                    Some(giveaway_created)
                }
                _ => None,
            }
        }

        #[must_use]
        pub fn giveaway_winners(&self) -> Option<&types::GiveawayWinners> {
            match &self.kind {
                GiveawayWinners(MessageGiveawayWinners { giveaway_winners }) => {
                    Some(giveaway_winners)
                }
                _ => None,
            }
        }

        #[must_use]
        pub fn video_chat_scheduled(&self) -> Option<&types::VideoChatScheduled> {
            match &self.kind {
                VideoChatScheduled(MessageVideoChatScheduled { video_chat_scheduled }) => {
                    Some(video_chat_scheduled)
                }
                _ => None,
            }
        }

        #[must_use]
        pub fn video_chat_started(&self) -> Option<&types::VideoChatStarted> {
            match &self.kind {
                VideoChatStarted(MessageVideoChatStarted { video_chat_started }) => {
                    Some(video_chat_started)
                }
                _ => None,
            }
        }

        #[must_use]
        pub fn video_chat_ended(&self) -> Option<&types::VideoChatEnded> {
            match &self.kind {
                VideoChatEnded(MessageVideoChatEnded { video_chat_ended }) => {
                    Some(video_chat_ended)
                }
                _ => None,
            }
        }

        #[must_use]
        pub fn video_chat_participants_invited(
            &self,
        ) -> Option<&types::VideoChatParticipantsInvited> {
            match &self.kind {
                VideoChatParticipantsInvited(MessageVideoChatParticipantsInvited {
                    video_chat_participants_invited,
                }) => Some(video_chat_participants_invited),
                _ => None,
            }
        }

        #[must_use]
        pub fn web_app_data(&self) -> Option<&types::WebAppData> {
            match &self.kind {
                WebAppData(MessageWebAppData { web_app_data }) => Some(web_app_data),
                _ => None,
            }
        }

        #[must_use]
        pub fn reply_markup(&self) -> Option<&types::InlineKeyboardMarkup> {
            match &self.kind {
                Common(MessageCommon { reply_markup, .. }) => reply_markup.as_ref(),
                _ => None,
            }
        }

        #[must_use]
        pub fn is_automatic_forward(&self) -> bool {
            match &self.kind {
                Common(MessageCommon { is_automatic_forward, .. }) => *is_automatic_forward,
                _ => false,
            }
        }

        #[must_use]
        pub fn has_protected_content(&self) -> bool {
            match &self.kind {
                Common(MessageCommon { has_protected_content, .. }) => *has_protected_content,
                _ => false,
            }
        }

        /// Common message (text, image, etc)
        fn common(&self) -> Option<&MessageCommon> {
            match &self.kind {
                Common(message) => Some(message),
                _ => None,
            }
        }

        // FIXME: add more getters for other types of messages
    }
}

impl Message {
    /// Produces a direct link to this message.
    ///
    /// Note that for private groups the link will only be accessible for group
    /// members.
    ///
    /// Returns `None` for private chats (i.e.: DMs) and private groups (not
    /// supergroups).
    #[must_use]
    pub fn url(&self) -> Option<Url> {
        Self::url_of(self.chat.id, self.chat.username(), self.id)
    }

    /// Produces a direct link to a message in a chat.
    ///
    /// If you have a `Message` object, use [`url`] instead.
    /// This function should only be used if you have limited information about
    /// the message (chat id, username of the chat, if any and its id).
    ///
    /// Note that for private groups the link will only be accessible for group
    /// members.
    ///
    /// Returns `None` for private chats (i.e.: DMs) and private groups (not
    /// supergroups).
    ///
    /// [`url`]: Message::url
    #[track_caller]
    #[must_use]
    pub fn url_of(
        chat_id: ChatId,
        chat_username: Option<&str>,
        message_id: MessageId,
    ) -> Option<Url> {
        use BareChatId::*;

        // Note: `t.me` links use bare chat ids
        let chat_id = match chat_id.to_bare() {
            // For private chats (i.e.: DMs) we can't produce "normal" t.me link.
            //
            // There are "tg://openmessage?user_id={0}&message_id={1}" links, which are
            // supposed to open any chat, including private messages, but they
            // are only supported by some telegram clients (e.g. Plus Messenger,
            // Telegram for Android 4.9+).
            User(_) => return None,
            // Similarly to user chats, there is no way to create a link to a message in a normal,
            // private group.
            //
            // (public groups are always supergroup which are in turn channels).
            Group(_) => return None,
            Channel(id) => id,
        };

        let url = match chat_username {
            // If it's public group (i.e. not DM, not private group), we can produce
            // "normal" t.me link (accessible to everyone).
            Some(username) => format!("https://t.me/{0}/{1}", username, message_id.0),
            // For private supergroups and channels we produce "private" t.me/c links. These are
            // only accessible to the group members.
            None => format!("https://t.me/c/{0}/{1}", chat_id, message_id.0),
        };

        // UNWRAP:
        //
        // The `url` produced by formatting is correct since username is
        // /[a-zA-Z0-9_]{5,32}/ and chat/message ids are integers.
        Some(reqwest::Url::parse(&url).unwrap())
    }

    /// Produces a direct link to a comment on this post.
    ///
    /// Note that for private channels the link will only be accessible for
    /// channel members.
    ///
    /// Returns `None` for private chats (i.e.: DMs) and private groups (not
    /// supergroups).
    #[must_use]
    pub fn comment_url(&self, comment_id: MessageId) -> Option<Url> {
        Self::comment_url_of(self.chat.id, self.chat.username(), self.id, comment_id)
    }

    /// Produces a direct link to a comment on a post.
    ///
    /// If you have a `Message` object of the channel post, use [`comment_url`]
    /// instead. This function should only be used if you have limited
    /// information about the message (channel id, username of the channel,
    /// if any, post id and comment id).
    ///
    /// Note that for private channels the link will only be accessible for
    /// channel members.
    ///
    /// Returns `None` for private chats (i.e.: DMs) and private groups (not
    /// supergroups).
    ///
    /// [`comment_url`]: Message::comment_url
    #[must_use]
    pub fn comment_url_of(
        channel_id: ChatId,
        channel_username: Option<&str>,
        post_id: MessageId,
        comment_id: MessageId,
    ) -> Option<Url> {
        Self::url_of(channel_id, channel_username, post_id).map(|mut url| {
            url.set_query(Some(&format!("comment={}", comment_id.0)));
            url
        })
    }

    /// Produces a direct link to this message in a given thread.
    ///
    /// "Thread" is a group of messages that reply to each other in a tree-like
    /// structure. `thread_starter_msg_id` is the id of the first message in
    /// the thread, the root of the tree.
    ///
    /// Note that for private groups the link will only be accessible for group
    /// members.
    ///
    /// Returns `None` for private chats (i.e.: DMs) and private groups (not
    /// supergroups).
    #[must_use]
    pub fn url_in_thread(&self, thread_starter_msg_id: MessageId) -> Option<Url> {
        Self::url_in_thread_of(self.chat.id, self.chat.username(), thread_starter_msg_id, self.id)
    }

    /// Produces a direct link to a message in a given thread.
    ///
    /// If you have a `Message` object of the channel post, use
    /// [`url_in_thread`] instead. This function should only be used if you
    /// have limited information about the message (chat id, username of the
    /// chat, if any, thread starter id and message id).
    ///
    /// "Thread" is a group of messages that reply to each other in a tree-like
    /// structure. `thread_starter_msg_id` is the id of the first message in
    /// the thread, the root of the tree.
    ///
    /// Note that for private groups the link will only be accessible for group
    /// members.
    ///
    /// Returns `None` for private chats (i.e.: DMs) and private groups (not
    /// supergroups).
    ///
    /// [`url_in_thread`]: Message::url_in_thread
    #[must_use]
    pub fn url_in_thread_of(
        chat_id: ChatId,
        chat_username: Option<&str>,
        thread_starter_msg_id: MessageId,
        message_id: MessageId,
    ) -> Option<Url> {
        Self::url_of(chat_id, chat_username, message_id).map(|mut url| {
            url.set_query(Some(&format!("thread={}", thread_starter_msg_id.0)));
            url
        })
    }

    /// Returns message entities that represent text formatting.
    ///
    /// This function returns `Some(entities)` for **text messages** and
    /// `None` for all other kinds of messages (including photos with
    /// captions).
    ///
    /// See also: [`parse_caption_entities`].
    ///
    /// [`parse_caption_entities`]: Message::parse_caption_entities
    #[must_use]
    pub fn parse_entities(&self) -> Option<Vec<MessageEntityRef<'_>>> {
        self.text().zip(self.entities()).map(|(t, e)| MessageEntityRef::parse(t, e))
    }

    /// Returns message entities that represent text formatting.
    ///
    /// This function returns `Some(entities)` for **media messages** and
    /// `None` for all other kinds of messages (including text messages).
    ///
    /// See also: [`parse_entities`].
    ///
    /// [`parse_entities`]: Message::parse_entities
    #[must_use]
    pub fn parse_caption_entities(&self) -> Option<Vec<MessageEntityRef<'_>>> {
        self.caption().zip(self.caption_entities()).map(|(t, e)| MessageEntityRef::parse(t, e))
    }

    /// Returns all users that are "contained" in this `Message` structure.
    ///
    /// This might be useful to track information about users.
    ///
    /// Note that this function may return quite a few users as it scans
    /// replies, message entities and more. Also note that this
    /// function can return duplicate users.
    pub fn mentioned_users(&self) -> impl Iterator<Item = &User> {
        use crate::util::{flatten, mentioned_users_from_entities};

        // Lets just hope we didn't forget something here...

        self.from
            .iter()
            .chain(self.via_bot.as_ref())
            .chain(flatten(self.reply_to_message().map(Self::mentioned_users_rec)))
            .chain(flatten(self.new_chat_members()))
            .chain(self.left_chat_member())
            .chain(self.forward_from_user())
            .chain(flatten(self.game().map(Game::mentioned_users)))
            .chain(flatten(self.entities().map(mentioned_users_from_entities)))
            .chain(flatten(self.caption_entities().map(mentioned_users_from_entities)))
            .chain(flatten(self.poll().map(Poll::mentioned_users)))
            .chain(flatten(self.proximity_alert_triggered().map(|a| [&a.traveler, &a.watcher])))
            .chain(flatten(self.video_chat_participants_invited().and_then(|i| i.users.as_deref())))
    }

    /// `Message::mentioned_users` is recursive (due to replies), as such we
    /// can't use `->impl Iterator` everywhere, as it would make an infinite
    /// type. So we need to box somewhere.
    pub(crate) fn mentioned_users_rec(&self) -> Box<dyn Iterator<Item = &User> + Send + Sync + '_> {
        Box::new(self.mentioned_users())
    }
}

/// Implemented for syntax sugar, see issue <https://github.com/teloxide/teloxide/issues/1143>
impl From<Message> for MessageId {
    fn from(message: Message) -> MessageId {
        message.id
    }
}

impl From<&Message> for MessageId {
    fn from(message: &Message) -> MessageId {
        message.id
    }
}

#[cfg(test)]
mod tests {
    use chrono::DateTime;
    use cool_asserts::assert_matches;
    use serde_json::from_str;

    use crate::types::*;

    #[test]
    fn de_media_forwarded() {
        let json = r#"{
          "message_id": 198283,
          "from": {
            "id": 250918540,
            "is_bot": false,
            "first_name": "Андрей",
            "last_name": "Власов",
            "username": "aka_dude",
            "language_code": "en"
          },
          "chat": {
            "id": 250918540,
            "first_name": "Андрей",
            "last_name": "Власов",
            "username": "aka_dude",
            "type": "private"
          },
          "date": 1567927221,
          "video": {
            "duration": 13,
            "width": 512,
            "height": 640,
            "mime_type": "video/mp4",
            "thumbnail": {
              "file_id": "AAQCAAOmBAACBf2oS53pByA-I4CWWCObDwAEAQAHbQADMWcAAhYE",
              "file_unique_id":"",
              "file_size": 10339,
              "width": 256,
              "height": 320
            },
            "file_id": "BAADAgADpgQAAgX9qEud6QcgPiOAlhYE",
            "file_unique_id":"",
            "file_size": 1381334
          }
        }"#;
        let message = from_str::<Message>(json);
        assert!(message.is_ok());
    }

    #[test]
    fn de_shared_chat() {
        let json = r#"{
            "message_id": 198283,
            "chat": {
              "id": 250918540,
              "first_name": "Андрей",
              "last_name": "Власов",
              "username": "aka_dude",
              "type": "private"
            },
            "date": 1567927221,
            "chat_shared": {
                "request_id": 348349,
                "chat_id": 384939
            }
          }"#;
        let message = from_str::<Message>(json);
        assert!(message.is_ok());
        assert_eq!(
            message.unwrap(),
            Message {
                id: MessageId(198283),
                thread_id: None,
                from: None,
                sender_chat: None,
                date: chrono::DateTime::from_timestamp(1567927221, 0).unwrap(),
                is_topic_message: false,
                chat: Chat {
                    id: ChatId(250918540),
                    kind: ChatKind::Private(ChatPrivate {
                        first_name: Some("Андрей".to_string()),
                        last_name: Some("Власов".to_string()),
                        username: Some("aka_dude".to_string()),
                    }),
                },
                sender_business_bot: None,
                kind: MessageKind::ChatShared(MessageChatShared {
                    chat_shared: ChatShared {
                        request_id: RequestId(348349),
                        chat_id: ChatId(384939),
                        title: None,
                        username: None,
                        photo: None,
                    }
                }),
                via_bot: None
            }
        );
    }

    #[test]
    fn de_media_group_forwarded() {
        let json = r#"{
          "message_id": 198283,
          "from": {
            "id": 250918540,
            "is_bot": false,
            "first_name": "Андрей",
            "last_name": "Власов",
            "username": "aka_dude",
            "language_code": "en"
          },
          "chat": {
            "id": 250918540,
            "first_name": "Андрей",
            "last_name": "Власов",
            "username": "aka_dude",
            "type": "private"
          },
          "date": 1567927221,
          "media_group_id": "12543417770506682",
          "video": {
            "duration": 13,
            "width": 512,
            "height": 640,
            "mime_type": "video/mp4",
            "thumbnail": {
              "file_id": "AAQCAAOmBAACBf2oS53pByA-I4CWWCObDwAEAQAHbQADMWcAAhYE",
              "file_unique_id":"",
              "file_size": 10339,
              "width": 256,
              "height": 320
            },
            "file_id": "BAADAgADpgQAAgX9qEud6QcgPiOAlhYE",
            "file_unique_id":"",
            "file_size": 1381334
          }
        }"#;
        let message = from_str::<Message>(json);
        assert!(message.is_ok());
    }

    #[test]
    fn de_text() {
        let json = r#"{
          "message_id": 199785,
          "from": {
           "id": 250918540,
           "is_bot": false,
           "first_name": "Андрей",
           "last_name": "Власов",
           "username": "aka_dude",
           "language_code": "en"
          },
          "chat": {
           "id": 250918540,
           "first_name": "Андрей",
           "last_name": "Власов",
           "username": "aka_dude",
           "type": "private"
          },
          "date": 1568289890,
          "text": "Лол кек 😂"
         }"#;
        let message = from_str::<Message>(json);
        assert!(message.is_ok());
    }

    #[test]
    fn de_sticker() {
        let json = r#"{
            "message_id": 199787,
            "from": {
                "id": 250918540,
                "is_bot": false,
                "first_name": "Андрей",
                "last_name": "Власов",
                "username": "aka_dude",
                "language_code": "en"
            },
            "chat": {
                "id": 250918540,
                "first_name": "Андрей",
                "last_name": "Власов",
                "username": "aka_dude",
                "type": "private"
            },
            "date": 1568290188,
            "sticker": {
                "width": 512,
                "height": 512,
                "emoji": "😡",
                "set_name": "AdvenTimeAnim",
                "is_animated": true,
                "is_video": false,
                "type": "regular",
                "thumbnail": {
                    "file_id": "AAMCAgADGQEAARIt0GMwiZ6n4nRbxdpM3pL8vPX6PVAhAAIjAAOw0PgMaabKAcaXKCABAAdtAAMpBA",
                    "file_unique_id": "AQADIwADsND4DHI",
                    "file_size": 4118,
                    "width": 128,
                    "height": 128
                },
                "file_id": "CAACAgIAAxkBAAESLdBjMImep-J0W8XaTN6S_Lz1-j1QIQACIwADsND4DGmmygHGlyggKQQ",
                "file_unique_id": "AgADIwADsND4DA",
                "file_size": 16639
            }
        }"#;
        from_str::<Message>(json).unwrap();
    }

    #[test]
    fn de_image() {
        let json = r#"{
          "message_id": 199791,
          "from": {
           "id": 250918540,
           "is_bot": false,
           "first_name": "Андрей",
           "last_name": "Власов",
           "username": "aka_dude",
           "language_code": "en"
          },
          "chat": {
           "id": 250918540,
           "first_name": "Андрей",
           "last_name": "Власов",
           "username": "aka_dude",
           "type": "private"
          },
          "date": 1568290622,
          "photo": [
           {
            "file_id": "AgADAgAD36sxG-PX0UvQSXIn9rccdw-ACA4ABAEAAwIAA20AAybcBAABFgQ",
            "file_unique_id":"",
            "file_size": 18188,
            "width": 320,
            "height": 239
           },
           {
            "file_id": "AgADAgAD36sxG-PX0UvQSXIn9rccdw-ACA4ABAEAAwIAA3gAAyfcBAABFgQ",
            "file_unique_id":"",
            "file_size": 62123,
            "width": 800,
            "height": 598
           },
           {
            "file_id": "AgADAgAD36sxG-PX0UvQSXIn9rccdw-ACA4ABAEAAwIAA3kAAyTcBAABFgQ",
            "file_unique_id":"",
            "file_size": 75245,
            "width": 962,
            "height": 719
           }
          ]
         }"#;
        let message = from_str::<Message>(json);
        assert!(message.is_ok());
    }

    /// Regression test for <https://github.com/teloxide/teloxide/issues/419>
    #[test]
    fn issue_419() {
        let json = r#"{
            "message_id": 1,
            "from": {
                "id": 1087968824,
                "is_bot": true,
                "first_name": "Group",
                "username": "GroupAnonymousBot"
            },
            "author_signature": "TITLE2",
            "sender_chat": {
                "id": -1001160242915,
                "title": "a",
                "type": "supergroup"
            },
            "chat": {
                "id": -1001160242915,
                "title": "a",
                "type": "supergroup"
            },
            "forward_origin": {
                "type": "chat",
                "date": 1640359544,
                "sender_chat": {
                    "id": -1001160242915,
                    "title": "a",
                    "type": "supergroup"
                },
                "author_signature": "TITLE"
            },
            "date": 1640359576,
            "text": "text"
        }"#;

        // Anonymous admin with title "TITLE2" forwards a message from anonymous
        // admin with title "TITLE" with text "a", everything is happening in
        // the same group.
        let message: Message = serde_json::from_str(json).unwrap();

        let group = Chat {
            id: ChatId(-1001160242915),
            kind: ChatKind::Public(ChatPublic {
                title: Some("a".to_owned()),
                kind: PublicChatKind::Supergroup(PublicChatSupergroup {
                    username: None,
                    is_forum: false,
                }),
            }),
        };

        assert!(message.from.as_ref().unwrap().is_anonymous());
        assert_eq!(message.author_signature().unwrap(), "TITLE2");
        assert_eq!(message.sender_chat.as_ref().unwrap(), &group);
        assert_eq!(&message.chat, &group);
        assert_eq!(message.forward_from_chat().unwrap(), &group);
        assert_eq!(message.forward_author_signature().unwrap(), "TITLE");
        assert!(message.forward_date().is_some());
        assert_eq!(message.text().unwrap(), "text");
    }

    /// Regression test for <https://github.com/teloxide/teloxide/issues/427>
    #[test]
    fn issue_427() {
        let old = ChatId(-599075523);
        let new = ChatId(-1001555296434);

        // Migration to a supergroup
        let json = r#"{"chat":{"all_members_are_administrators":false,"id":-599075523,"title":"test","type":"group"},"date":1629404938,"from":{"first_name":"nullptr","id":729497414,"is_bot":false,"language_code":"en","username":"hex0x0000"},"message_id":16,"migrate_to_chat_id":-1001555296434}"#;
        let message: Message = from_str(json).unwrap();

        assert_eq!(message.chat.id, old);
        assert_eq!(message.chat_migration(), Some(&ChatMigration::To { chat_id: new }));
        assert_eq!(message.migrate_to_chat_id(), Some(&new));

        // The user who initialized the migration
        assert!(message.from.is_some());

        // Migration from a common group
        let json = r#"{"chat":{"id":-1001555296434,"title":"test","type":"supergroup"},"date":1629404938,"from":{"first_name":"Group","id":1087968824,"is_bot":true,"username":"GroupAnonymousBot"},"message_id":1,"migrate_from_chat_id":-599075523,"sender_chat":{"id":-1001555296434,"title":"test","type":"supergroup"}}"#;
        let message: Message = from_str(json).unwrap();

        assert_eq!(message.chat.id, new);
        assert_eq!(message.chat_migration(), Some(&ChatMigration::From { chat_id: old }));
        assert_eq!(message.migrate_from_chat_id(), Some(&old));

        // Anonymous bot
        assert!(message.from.is_some());

        // The chat to which the group migrated
        assert!(message.sender_chat.is_some());
    }

    /// Regression test for <https://github.com/teloxide/teloxide/issues/481>
    #[test]
    fn issue_481() {
        let json = r#"
{
  "message_id": 0,
  "date": 0,
  "location": {
   "latitude": 0.0,
   "longitude": 0.0
  },
  "chat": {
   "id": 0,
   "first_name": "f",
   "type": "private"
  },
  "venue": {
   "location": {
    "latitude": 0.0,
    "longitude": 0.0,
    "live_period": 900
   },
   "title": "Title",
   "address": "Address",
   "foursquare_id": "some_foursquare_id"
  }
 }
"#;
        let message: Message = from_str(json).unwrap();
        assert_eq!(
            message.venue().unwrap(),
            &Venue {
                location: Location {
                    longitude: 0.0,
                    latitude: 0.0,
                    horizontal_accuracy: None,
                    live_period: Some(900.into()),
                    heading: None,
                    proximity_alert_radius: None
                },
                title: "Title".to_owned(),
                address: "Address".to_owned(),
                foursquare_id: Some("some_foursquare_id".to_owned()),
                foursquare_type: None,
                google_place_id: None,
                google_place_type: None,
            }
        )
    }

    /// Regression test for <https://github.com/teloxide/teloxide/issues/475>
    #[test]
    fn issue_475() {
        let json = r#"{"message_id":198295,"from":{"id":1087968824,"is_bot":true,"first_name":"Group","username":"GroupAnonymousBot"},"sender_chat":{"id":-1001331354980,"title":"C++ Together 2.0","username":"cpptogether","type":"supergroup"},"chat":{"id":-1001331354980,"title":"C++ Together 2.0","username":"cpptogether","type":"supergroup"},"date":1638236631,"video_chat_started":{}}"#;

        let message: Message = serde_json::from_str(json).unwrap();

        assert!(matches!(message.kind, MessageKind::VideoChatStarted { .. }));

        // FIXME(waffle): it seems like we are losing `sender_chat` in some
        // cases inclusing this
        // assert!(message.sender_chat().is_some());
    }

    #[test]
    fn parse_caption_entities() {
        let json = r#"
        {
            "message_id": 3460,
            "from": {
              "id": 27433968,
              "is_bot": false,
              "first_name": "Crax | rats addict",
              "username": "tacocrasco",
              "language_code": "en"
            },
            "chat": {
              "id": 27433968,
              "first_name": "Crax | rats addict",
              "username": "tacocrasco",
              "type": "private"
            },
            "date": 1655671349,
            "photo": [
              {
                "file_id": "AgACAgQAAxkBAAINhGKvijUVSn2i3980bQIIc1fqWGNCAAJpvDEbEmaBUfuA43fR-BnlAQADAgADcwADJAQ",
                "file_unique_id": "AQADabwxGxJmgVF4",
                "file_size": 2077,
                "width": 90,
                "height": 90
              },
              {
                "file_id": "AgACAgQAAxkBAAINhGKvijUVSn2i3980bQIIc1fqWGNCAAJpvDEbEmaBUfuA43fR-BnlAQADAgADbQADJAQ",
                "file_unique_id": "AQADabwxGxJmgVFy",
                "file_size": 27640,
                "width": 320,
                "height": 320
              },
              {
                "file_id": "AgACAgQAAxkBAAINhGKvijUVSn2i3980bQIIc1fqWGNCAAJpvDEbEmaBUfuA43fR-BnlAQADAgADeAADJAQ",
                "file_unique_id": "AQADabwxGxJmgVF9",
                "file_size": 99248,
                "width": 800,
                "height": 800
              },
              {
                "file_id": "AgACAgQAAxkBAAINhGKvijUVSn2i3980bQIIc1fqWGNCAAJpvDEbEmaBUfuA43fR-BnlAQADAgADeQADJAQ",
                "file_unique_id": "AQADabwxGxJmgVF-",
                "file_size": 162061,
                "width": 1280,
                "height": 1280
              }
            ],
            "caption": "www.example.com",
            "caption_entities": [
              {
                "offset": 0,
                "length": 15,
                "type": "url"
              }
            ]
        }"#;

        let message: Message = serde_json::from_str(json).unwrap();
        let entities = message.parse_caption_entities();
        assert!(entities.is_some());

        let entities = entities.unwrap();
        assert!(!entities.is_empty());
        assert_eq!(entities[0].kind().clone(), MessageEntityKind::Url);
    }

    #[test]
    fn topic_created() {
        let json = r#"{
            "chat":{"id":-1001847508954,"is_forum":true,"title":"twest","type":"supergroup"},
            "date":1675229139,
            "forum_topic_created":{
                "icon_color":9367192,
                "icon_custom_emoji_id":"5312536423851630001",
                "name":"???"
            },
            "from":{
                "first_name":"вафель'",
                "id":1253681278,
                "is_bot":false,
                "language_code":"en",
                "username":"wafflelapkin"
            },
            "is_topic_message":true,
            "message_id":4,
            "message_thread_id":4
        }"#;

        let message: Message = serde_json::from_str(json).unwrap();
        // https://github.com/teloxide/teloxide/issues/945
        assert!(message.from.is_some());
    }

    #[test]
    fn topic_message() {
        let json = r#"{"chat":{"id":-1001847508954,"is_forum":true,"title":"twest","type":"supergroup"},"date":1675229140,"from":{"first_name":"вафель'","id":1253681278,"is_bot":false,"language_code":"en","username":"wafflelapkin"},"is_topic_message":true,"message_id":5,"message_thread_id":4,"reply_to_message":{"chat":{"id":-1001847508954,"is_forum":true,"title":"twest","type":"supergroup"},"date":1675229139,"forum_topic_created":{"icon_color":9367192,"icon_custom_emoji_id":"5312536423851630001","name":"???"},"from":{"first_name":"вафель'","id":1253681278,"is_bot":false,"language_code":"en","username":"wafflelapkin"},"is_topic_message":true,"message_id":4,"message_thread_id":4},"text":"blah"}"#;

        let _: Message = serde_json::from_str(json).unwrap();
    }

    /// Regression test for <https://github.com/teloxide/teloxide/issues/873>
    #[test]
    fn empty_message() {
        let json = r#"{"chat": {"first_name": "FN", "id": 1234567890, "type": "private"}, "date": 0, "message_id": 875400}"#;

        let msg: Message = serde_json::from_str(json).unwrap();
        assert_matches!(msg.kind, MessageKind::Empty {})
    }

    #[test]
    fn issue_874() {
        let json = r#"{
            "chat": {
                "id": -1001840751935,
                "is_forum": true,
                "title": "AI",
                "type": "supergroup"
            },
            "date": 1682191229,
            "forum_topic_closed": {},
            "from": {
                "first_name": "Владислав",
                "id": 112455916,
                "is_bot": false,
                "language_code": "en",
                "username": "scv977"
            },
            "message_id": 62
        }"#;

        let _: Message = serde_json::from_str(json).unwrap();
    }

    #[test]
    fn giveaway() {
        let json = r#"{
            "message_id": 27,
            "sender_chat": {
                "id": -1002236736395,
                "title": "Test",
                "type": "channel"
            },
            "chat": {
                "id": -1002236736395,
                "title": "Test",
                "type": "channel"
            },
            "date": 1721162577,
            "giveaway": {
                "chats": [
                    {
                        "id": -1002236736395,
                        "title": "Test",
                        "type": "channel"
                    }
                ],
                "winners_selection_date": 1721162701,
                "winner_count": 1,
                "has_public_winners": true,
                "premium_subscription_month_count": 6
            }
        }"#;
        let message: Message = from_str(json).unwrap();
        assert_eq!(
            message.giveaway().unwrap(),
            &Giveaway {
                chats: vec![Chat {
                    id: ChatId(-1002236736395),
                    kind: ChatKind::Public(ChatPublic {
                        title: Some("Test".to_owned()),
                        kind: PublicChatKind::Channel(PublicChatChannel { username: None }),
                    }),
                }],
                winners_selection_date: DateTime::from_timestamp(1721162701, 0).unwrap(),
                winner_count: 1,
                only_new_members: false,
                has_public_winners: true,
                prize_description: None,
                country_codes: None,
                premium_subscription_month_count: Some(6)
            }
        )
    }

    #[test]
    fn giveaway_created() {
        let json = r#"{
            "message_id": 27,
            "sender_chat": {
                "id": -1002236736395,
                "title": "Test",
                "type": "channel"
            },
            "chat": {
                "id": -1002236736395,
                "title": "Test",
                "type": "channel"
            },
            "date": 1721162577,
            "giveaway_created": {}
        }"#;
        let message: Message = from_str(json).unwrap();
        assert_eq!(message.giveaway_created().unwrap(), &GiveawayCreated {})
    }

    #[test]
    fn giveaway_completed() {
        let json = r#"{
            "message_id": 27,
            "sender_chat": {
                "id": -1002236736395,
                "title": "Test",
                "type": "channel"
            },
            "chat": {
                "id": -1002236736395,
                "title": "Test",
                "type": "channel"
            },
            "date": 1721162577,
            "giveaway_completed": {
                "winner_count": 0,
                "unclaimed_prize_count": 1,
                "giveaway_message": {
                    "message_id": 24,
                    "sender_chat": {
                        "id": -1002236736395,
                        "title": "Test",
                        "type": "channel"
                    },
                    "chat": {
                        "id": -1002236736395,
                        "title": "Test",
                        "type": "channel"
                    },
                    "date": 1721161230,
                    "giveaway": {
                        "chats": [
                            {
                                "id": -1002236736395,
                                "title": "Test",
                                "type": "channel"
                            }
                        ],
                        "winners_selection_date": 1721162701,
                        "winner_count": 1,
                        "has_public_winners": true,
                        "premium_subscription_month_count": 6
                    }
                }
            }
        }"#;
        let message: Message = from_str(json).unwrap();
        assert_eq!(
            message.giveaway_completed().unwrap(),
            &GiveawayCompleted {
                winner_count: 0,
                unclaimed_prize_count: Some(1),
                giveaway_message: Some(Box::new(Message {
                    id: MessageId(24),
                    thread_id: None,
                    from: None,
                    sender_chat: Some(Chat {
                        id: ChatId(-1002236736395),
                        kind: ChatKind::Public(ChatPublic {
                            title: Some("Test".to_owned()),
                            kind: PublicChatKind::Channel(PublicChatChannel { username: None }),
                        }),
                    }),
                    is_topic_message: false,
                    date: DateTime::from_timestamp(1721161230, 0).unwrap(),
                    chat: Chat {
                        id: ChatId(-1002236736395),
                        kind: ChatKind::Public(ChatPublic {
                            title: Some("Test".to_owned()),
                            kind: PublicChatKind::Channel(PublicChatChannel { username: None }),
                        }),
                    },
                    via_bot: None,
                    sender_business_bot: None,
                    kind: MessageKind::Giveaway(MessageGiveaway {
                        giveaway: Giveaway {
                            chats: vec![Chat {
                                id: ChatId(-1002236736395),
                                kind: ChatKind::Public(ChatPublic {
                                    title: Some("Test".to_owned()),
                                    kind: PublicChatKind::Channel(PublicChatChannel {
                                        username: None,
                                    }),
                                }),
                            }],
                            winners_selection_date: DateTime::from_timestamp(1721162701, 0)
                                .unwrap(),
                            winner_count: 1,
                            only_new_members: false,
                            has_public_winners: true,
                            prize_description: None,
                            country_codes: None,
                            premium_subscription_month_count: Some(6)
                        }
                    })
                }))
            }
        )
    }

    #[test]
    fn giveaway_winners() {
        let json = r#"{
            "message_id": 28,
            "sender_chat": {
                "id": -1002236736395,
                "title": "Test",
                "type": "channel"
            },
            "chat": {
                "id": -1002236736395,
                "title": "Test",
                "type": "channel"
            },
            "date": 1721162702,
            "reply_to_message": {
                "message_id": 27,
                "sender_chat": {
                    "id": -1002236736395,
                    "title": "Test",
                    "type": "channel"
                },
                "chat": {
                    "id": -1002236736395,
                    "title": "Test",
                    "type": "channel"
                },
                "date": 1721162577,
                "giveaway": {
                    "chats": [
                        {
                            "id": -1002236736395,
                            "title": "Test",
                            "type": "channel"
                        }
                    ],
                    "winners_selection_date": 1721162701,
                    "winner_count": 1,
                    "has_public_winners": true,
                    "premium_subscription_month_count": 6
                }
            },
            "giveaway_winners": {
                "chat": {
                    "id": -1002236736395,
                    "title": "Test",
                    "type": "channel"
                },
                "giveaway_message_id": 27,
                "winners_selection_date": 1721162701,
                "premium_subscription_month_count": 6,
                "winner_count": 1,
                "winners": [
                    {
                        "id": 1459074222,
                        "is_bot": false,
                        "first_name": "shadowchain",
                        "username": "shdwchn10"
                    }
                ]
            }
        }"#;
        let message: Message = from_str(json).unwrap();
        assert_eq!(
            message.giveaway_winners().expect("Failed to get GiveawayWinners from Message!"),
            &GiveawayWinners {
                chat: Chat {
                    id: ChatId(-1002236736395),
                    kind: ChatKind::Public(ChatPublic {
                        title: Some("Test".to_owned()),
                        kind: PublicChatKind::Channel(PublicChatChannel { username: None }),
                    }),
                },
                giveaway_message_id: MessageId(27),
                winners_selection_date: DateTime::from_timestamp(1721162701, 0).unwrap(),
                winner_count: 1,
                winners: vec![User {
                    id: UserId(1459074222),
                    is_bot: false,
                    first_name: "shadowchain".to_owned(),
                    last_name: None,
                    username: Some("shdwchn10".to_owned()),
                    language_code: None,
                    is_premium: false,
                    added_to_attachment_menu: false
                }],
                additional_chat_count: None,
                premium_subscription_month_count: Some(6),
                unclaimed_prize_count: None,
                only_new_members: false,
                was_refunded: false,
                prize_description: None
            }
        )
    }

    #[test]
    fn chat_boost_added() {
        let json = r#"{
            "message_id": 28,
            "sender_chat": {
                "id": -1002236736395,
                "title": "Test",
                "type": "channel"
            },
            "chat": {
                "id": -1002236736395,
                "title": "Test",
                "type": "channel"
            },
            "date": 1721162702,
            "boost_added": {
                "boost_count": 4
            }
        }"#;
        let message: Message = from_str(json).unwrap();
        assert_eq!(
            message.boost_added().expect("Failed to get ChatBoostAdded from Message!"),
            &ChatBoostAdded { boost_count: 4 }
        )
    }
}
