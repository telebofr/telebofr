# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## unreleased

### Added

- Support for TBA 7.1 ([#1131](pr1131))
  - Updated docs for `can_post_stories`, `can_edit_stories` and `can_delete_stories` admin privileges
  - Add `ChatBoostAdded` and `StoryId` structs
  - Add `ChatBoostAdded` variant to `MessageKind` enum
  - Add `sender_boost_count` and `reply_to_story` fields to `MessageCommon` struct
  - Add `chat` and `id` fields to `Story` struct
  - Add `unrestrict_boost_count` and `custom_emoji_sticker_set_name` fields to `PublicChatSupergroup` struct
  - Add `boost_added` and `reply_to_story` getters to `Message` struct
  - Add `unrestrict_boost_count` and `custom_emoji_sticker_set_name` getters to `Chat` struct

- Support for TBA 7.2 ([#1146](pr1146))
  - Update documentation of `SendSticker` method
  - Add `is_from_offline` field to `MessageCommon` struct
  - Add `can_connect_to_business` field to `Me` struct
  - Add `personal_chat` field to `ChatPrivate` struct
  - Add `ReplaceStickerInSet` and `GetBusinessConnection` methods
  - Add `Birthdate` struct and corresponding field `birthdate` in `ChatPrivate`
  - Add `request_name`, `request_username` and `request_photo` fields to `KeyboardButtonRequestUsers` struct
  - Add `request_title`, `request_username` and `request_photo` fields to `KeyboardButtonRequestChat` struct
  - Add `SharedUser` struct
  - Add `title`, `username` and `photo` fields to `ChatShared` struct
  - Add `format` field to `InputSticker` struct
  - Add `format` parameter to `SetStickerSetThumbnail` method
  - Add `BusinessConnectionId` struct
  - Add `business_connection_id` parameter to `SendMessage`, `SendPhoto`, `SendVideo`, `SendAnimation`, `SendAudio`, `SendDocument`, `SendSticker`, `SendVideoNote`, `SendVoice`, `SendLocation`, `SendVenue`, `SendContact`, `SendPoll`, `SendDice`, `SendGame`, and `SendMediaGroup` methods
  - Add `sender_business_bot` and `business_connection_id` fields to `Message` struct
  - Add `BusinessIntro` struct and corresponding field `business_intro` to `ChatPrivate` struct
  - Add `BusinessLocation` struct and corresponding field `business_location` to `ChatPrivate` struct
  - Add `BusinessOpeningHoursInterval` and `BusinessOpeningHours` structs and corresponding field `business_opening_hours` to `ChatPrivate` struct
  - Add `BusinessConnection` struct
  - Add `BusinessMessagesDeleted` struct
  - Add `BusinessConnection`, `BusinessMessage`, `EditedBusinessMessage` and `DeletedBusinessMessages` variants to `UpdateKind` enum

- `ApiError::BotKickedFromChannel` ([#1157][pr1157])
- `sender_boost_count` method to the `Message` struct ([#1264][pr1264])
- `From<&Message> for MessageId` impl ([#1271][pr1271])
- `protect_content` parameter to the `sendVoice` method ([#1265][pr1265])

- Support for TBA 7.3 ([#1159](pr1159))
  - Add `via_join_request` field to `ChatMemberUpdated` struct
  - Add `LivePeriod` enum and replace `u32` with it in all `live_period` fields
  - Add `live_period` parameter to `editMessageLiveLocation` method
  - Add `question_entities` field to `Poll` struct
  - Add `text_entities` filed to `PollOption` struct
  - Add `question_parse_mode` and `question_entities` parameters to `sendPoll` method
  - Add `InputPollOption` struct
  - Add `BackgroundFill` and `BackgroundType` enum
  - Add `ChatBackground` variant to `MessageKind` enum
  - Add `max_reaction_count` field to `ChatFullInfo` struct
  - Document that .MP3 and .M4A files can be used as voice messages

[pr1157]: https://github.com/teloxide/teloxide/pull/1157
[pr1264]: https://github.com/teloxide/teloxide/pull/1264
[pr1271]: https://github.com/teloxide/teloxide/pull/1271
[pr1265]: https://github.com/teloxide/teloxide/pull/1265

### Changed

- `MaybeAnonymousUser` type introduced, which replaced `PollAnswer::voter: Voter` and `MessageReactionUpdated::{user, actor_chat}` in `MessageReactionUpdated`([#1134][pr1134])
- Environment bumps: ([#1147][pr1147])
  - MSRV (Minimal Supported Rust Version) was bumped from `1.70.0` to `1.80.0`
  - Some dependencies was bumped: `reqwest` to `0.12.7` and `ron` to `0.8.1`
  - `tokio` version was explicitly specified as `1.39` and feature `io-util` was enabled for it
- `[u8; 3]` sometimes used for RGB values was replaced with dedicated `Rgb` struct: ([#1151][pr1151])
  - `serde_rgb` module from `types.rs` file was removed
  - `CreateForumTopic`, `ForumTopicCreated` and `ForumTopic` structs now use `Rgb` instead of `[u8; 3]` for `icon_color` field
  - Added `rgb` crate dependency to Cargo.toml
  - Added `Rgb` struct with `From` implementation for `RGB8` type from popular `rgb` crate

- Support for TBA 7.2 ([#1146](pr1146))
  - Remove `flags` field from `StickerSet` struct
  - Remove `sticker_format` parameter from `CreateNewStickerSet` method
  - Wrap `Public` variant of `ChatKind` in `Box`
  - Replaced `user_ids` with `users` in `UsersShared` struct

- Remove a useless generic type in the `KeyboardMarkup::selective` function ([#1176][pr1176])

- Support for TBA 7.3 ([#1159](pr1159))
  - Change `options` parameter type in method `sendPoll` to `InputPollOption`
  - Move most of the fields and methods from `Chat` to `ChatFullInfo` struct
  - Return `ChatFullInfo` struct from `getChat` method instead of `Chat`

[pr1131]: https://github.com/teloxide/teloxide/pull/1131
[pr1134]: https://github.com/teloxide/teloxide/pull/1134
[pr1146]: https://github.com/teloxide/teloxide/pull/1146
[pr1147]: https://github.com/teloxide/teloxide/pull/1147
[pr1151]: https://github.com/teloxide/teloxide/pull/1151
[pr1176]: https://github.com/teloxide/teloxide/pull/1176

### Removed

- `Currency` enum removed, its usages replaced with `String`.

- Support for TBA 7.3 ([#1159](pr1159))
  - Remove some degraded functionality (because of `Chat` to `ChatFullInfo` migration):
    - `fn mentioned_users` from `chat_member_updated.rs` and `chat_join_request.rs` lost ability to track chat users
    - `fn mentioned_users` from `Message` lost ability to track chat users (e.g. from pins) and chat users from forward
    - `fn mentioned_users_rec` was removed from `Chat` and not included in `ChatFullInfo` struct

[pr1159]: https://github.com/teloxide/teloxide/pull/1159

## 0.10.1 - 2024-08-17

### Fixed
- Issue, when using `ReplyParameters` and multipart-requests involving file-sending it failed with `unimplemented error` ([#1136][pr1136], issue [#1135][issue1135])

[pr1136]: https://github.com/teloxide/teloxide/pull/1136
[issue1135]: https://github.com/teloxide/teloxide/issues/1135

## 0.10.0 - 2024-08-16

### Added

- `ChatPermission::can_*` helper functions ([#851][pr851])
- `mentioned_users` functions for `CallbackQuery`, `Chat`, `ChatJoinRequest`, `ChatMemberUpdated`, `Game`, `Message`, `Poll`, `Update` which return all contained `User` instances ([#850][pr850])
- `Message::video_chat_participants_invited` ([#850][pr850])
- `Update::from`, a replacement for `Update::user` ([#850][pr850])
- `Seconds` type, which represents a duration is seconds ([#859][pr859])
- `VideoChatEnded::duration` field that was previously missed ([#859][pr859])
- `ThreadId` newtype over `MessageId`, used for identifying reply threads ([#887][pr887])
- `ChatId::as_user` ([#905][pr905])
- Implement `PartialEq<ChatId> for UserId` and `PartialEq<UserId> for ChatId` ([#905][pr905])
- `ChatId::{MIN, MAX}` ([#905][pr905])
- Missing `Message` getters ([#982][pr982]):
  - `message_auto_delete_timer_changed`
  - `write_access_allowed`
  - `forum_topic_created`
  - `forum_topic_edited`
  - `forum_topic_closed`
  - `forum_topic_reopened`
  - `general_forum_topic_hidden`
  - `general_forum_topic_unhidden`
  - `video_chat_scheduled`
  - `video_chat_started`
  - `video_chat_ended`
  - `web_app_data`
- `is_delete_chat_photo`, `is_group_chat_created`, `is_super_group_chat_created`, `is_channel_chat_created` functions to `Message` ([#982][pr982])
- Support for TBA 6.5  ([#954][pr954])
  - Add `can_send_audios`, `can_send_documents`, `can_send_photos`, `can_send_videos`, `can_send_video_notes`, and `can_send_voice_notes` to `ChatPermissions` and `Restricted`
  - Add `use_independent_chat_permissions` optional parameter to `restrict_chat_member` and `set_chat_permissions`
  - Add `user_chat_id` field to `ChatJoinRequest`
  - Add `KeyboardButtonRequestChat` and `ChatShared` types
  - Add `RequestChat` variant to `ButtonRequest`
  - Add `ChatShared` variant to `MessageKind`
  - Add `shared_chat` method to `Message`
  - Add `KeyboardButtonRequestUser` and `UserShared` types
  - Add `RequestUser` variant to `ButtonRequest`
  - Add `UserShared` variant to `MessageKind`
  - Add `shared_user` method to `Message`
- Support for TBA 6.6 ([#1040](pr1040))
  - Add methods for working with bot's description:
    - `set_my_description`
    - `get_my_description`
    - `set_my_short_description`
    - `get_my_short_description`
  - Add methods for working with sticker sets:
    - `set_custom_emoji_sticker_set_thumbnail`
    - `set_sticker_set_title`
    - `delete_sticker_set`
    - `set_sticker_emoji_list`
    - `set_sticker_keywords`
    - `set_sticker_mask_position`
  - Add parameter `emoji` to the `send_sticker` method to specify an emoji for just uploaded stickers
  - Add support for the creation of custom emoji sticker sets in `create_new_sticker_set`
  - Add parameter `needs_repainting` to the `create_new_sticker_set` methodto automatically change the color of emoji based on context (e.g., use text color in messages, accent color in statuses, etc.)
  - Add field `needs_repainting` to the `Sticker` struct
  - Add support for the creation of sticker sets with multiple initial stickers in `create_new_sticker_set` by replacing the parameters `sticker`, `emojis` and `mask_position` with the parameters `stickers` and `sticker_format`.
  - Add support for .WEBP files in `create_new_sticker_set` and `add_sticker_to_set`
  - Add support for .WEBP, .TGS, and .WEBM files in `upload_sticker_file` by replacing the parameter `png_sticker` with the parameters `sticker` and `sticker_format`
  - Add the ability to specify search keywords for stickers added to sticker sets
  - Add new type `StickerFormatFlags` with fields `is_animated` and `is_video`
  - Add new method `Sticker::format` which returns the format of the sticker, the format is determined by the `StickerFormatFlags` of the `Sticker`
  - Add missing fields `{InlineQueryResultGif, InlineQueryResultMpeg4Gif}::thumbnail_mime_type`
- Support for TBA 6.7 ([#1086](pr1086))
  - Add support for launching [Web Apps](https://core.telegram.org/bots/webapps) from inline query results by replacing the parameters `switch_pm_text` and `switch_pm_parameter` of the method `answer_inline_query` with the parameter `button` of type `InlineQueryResultsButton`
  - Add new fields:
    - `web_app_name` to `WriteAccessAllowed`
    - `switch_inline_query_chosen_chat` of the type `SwitchInlineQueryChosenChat` to `InlineKeyboardButton`
    - `via_chat_folder_invite_link` to `ChatMemberUpdated`
  - Add methods for working with bot's name:
    - `set_my_name`
    - `get_my_name`
  - Add the ability to specify custom emoji entities using `HTML` and `MarkdownV2` formatting options for bots that purchased additional usernames on [Fragment](https://fragment.com/)
- Support for TBA 6.8 ([#1087](pr1087))
  - Add the `MediaKind::Story`
  - Add new fields
    - `PollAnswer::voter` to support anonymous poll answers in chats
    - `emoji_status_expiration_date` to `Chat` as part of the future `ChatFullInfo` type TBA type
  - Add the `unpin_all_general_forum_topic_messages` method
- Support for TBA 6.9 ([#1095](pr1095))
  - Add `can_post_stories`, `can_edit_stories` and `can_delete_stories` fields to `ChatMemberKind::Administrator`, `ChatAdministratorRights` and `PromoteChatMember`
  - Add `from_request` and `from_attachment_menu` fields to `WriteAccessAllowed`
- Support for TBA 7.0 ([#1101](pr1101))
  - Reactions:
    - Add `ReactionType` enum
    - Add `MessageReactionUpdated` and `MessageReactionCountUpdated` structs
    - Add `MessageReaction` and `MessageReactionCount` variants to `UpdateKind` enum
    - Add `filter_message_reaction_updated` and `filter_message_reaction_count_updated` filters to `UpdateFilterExt` trait
    - Add `set_message_reaction` TBA method to `Requester` trait
    - Add `available_reactions` field to `Chat` struct
  - Replies 2.0
    - Add the fields `MessageCommon::{external_reply, quote}` of types `ExternalReplyInfo` and `TextQuote` respectively
  - Link Preview Customization
    - `disable_web_page_preview` replaced with `link_preview_options`:
      - Remove `disable_web_page_preview` field from `send_message` and `send_message` TBA methods and `InputMessageContentText` struct
      - Add `LinkPreviewOptions` struct
      - Add `link_preview_options` field to `InputMessageContentText` and `Message` structs
      - Add `link_preview_options` field to `send_message` and `send_message` TBA methods
  - Multiple Message Actions
    - Add TBA methods `delete_messages`, `forward_messages` and `copy_messages` to `Requester` trait
  - Chat Boost
    - Add `ChatBoostSource` enum
    - Add `ChatBoost`, `ChatBoostUpdated`, `ChatBoostRemoved` and `UserChatBoosts` structs
    - Add `ChatBoost` and `RemovedChatBoost` variants to `UpdateKind` enum
    - Add `filter_chat_boost` and `filter_removed_chat_boost` filters to `UpdateFilterExt` trait
    - Add `get_user_chat_boosts` TBA method to `Requester` trait
  - Giveaway:
    - Add `Giveaway`, `GiveawayCreated`, `GiveawayWinners` and `GiveawayCompleted` structs
    - Add `Giveaway`, `GiveawayCreated`, `GiveawayWinners` and `GiveawayCompleted` variants to `MessageKind` enum
    - Add `giveaway`, `giveaway_created`, `giveaway_winners` and `giveaway_completed` getters to `Message`
  - Other Changes
    - Add fields `ChafFullInfo::{has_visible_history, accent_color_id, background_custom_emoji_id, profile_accent_color_id, profile_background_custom_emoji_id}`
  - Add `RequestId` type
  - Add `CallbackQuery::regular_message` getter

[pr851]: https://github.com/teloxide/teloxide/pull/851
[pr887]: https://github.com/teloxide/teloxide/pull/887
[pr905]: https://github.com/teloxide/teloxide/pull/905
[pr982]: https://github.com/teloxide/teloxide/pull/982
[pr1040]: https://github.com/teloxide/teloxide/pull/1040
[pr1086]: https://github.com/teloxide/teloxide/pull/1086
[pr1087]: https://github.com/teloxide/teloxide/pull/1087
[pr1095]: https://github.com/teloxide/teloxide/pull/1095
[pr1101]: https://github.com/teloxide/teloxide/pull/1101

### Fixed

- Return types of `edit_message_live_location_inline`, `stop_message_live_location_inline`, and `set_game_score_inline`: `Message` => `True` ([#854][pr854])
- Remove `latitude` and `longitude` parameters from `stop_message_live_location` and `stop_message_live_location_inline` ([#854][pr854])
- Fix the type of `photo_size`,`photo_width` and `photo_height` in the `send_invoice` method ([#936][pr936])
- Fix roundtrip de/serialization of `InlineQueryResult` ([#990][pr990])
- Deserialization of `ApiError::CantParseEntities` ([#839][pr839])
- Deserialization of empty (content-less) messages that can sometimes appear as a part of callback query ([#850][pr850], issue [#873][issue873])
- Type of `chat_id` in `send_game`: `u32` => `ChatId` ([#1066][pr1066])
- `Requester::Err` was bounded to the `AsResponseParameters`. In case of `RetryAfter(..)` errors the polling is paused for the specified delay instead of falling into the backoff strategy ([#1113][pr1113])
- `SendPoll` documentation ([#1101][pr1101], issue [#1058][issue1058])
- `from`, `sender_chat` and `is_topic_message` are moved into the `Message` from the `MessageCommon` ([#1101][pr1101], issue [#945][issue945]). Initially [#946][pr946]
- Add support for `blockquote` entity in received messages + in `MarkdownV2` and `HTML` ([#1101][pr1101], issue [#1062][issue1062])

[pr839]: https://github.com/teloxide/teloxide/pull/839
[pr879]: https://github.com/teloxide/teloxide/pull/879
[issue873]: https://github.com/teloxide/teloxide/issues/873
[pr854]: https://github.com/teloxide/teloxide/pull/854
[pr936]: https://github.com/teloxide/teloxide/pull/936
[pr990]: https://github.com/teloxide/teloxide/pull/990
[pr990]: https://github.com/teloxide/teloxide/pull/990
[pr1066]: https://github.com/teloxide/teloxide/pull/1066
[pr1113]: https://github.com/teloxide/teloxide/pull/1113

### Changed

- Types of `Option<bool>` fields of `KeyboardMarkup`, `KeyboardRemove` and `ForceReply` to `bool` ([#853][pr853])
- Type of `KeyboardMarkup::input_field_placeholder`: `Option<String>` => `String` ([#853][pr853])
- The following fields now use `Seconds` type instead of `u32`, `u16` or `Duration` ([#859][pr859])
  - `Animation::duration`
  - `Audio::duration`
  - `Chat::message_auto_delete_time`
  - `Chat::slow_mode_delay`
  - `InlineQueryResultLocation::live_period`
  - `Location::live_period`
  - `MessageAutoDeleteTimerChanged::message_auto_delete_time`
  - `Poll::open_period`
  - `Video::duration`
  - `VideoNote::duration`
  - `Voice::duration`
- `RequestError::MigrateToChatId` single fields type to `ChatId` ([#859][pr859])
- `RequestError::RetryAfter` single fields type to `Seconds` ([#859][pr859])
- `CallbackGame`, `ForumTopicClosed`, `ForumTopicReopened`, `GeneralForumTopicHidden`, `GeneralForumTopicUnhidden` and `WriteAccessAllowed` structures
  are now defined as named (`struct S {}`) instead of unit (`struct S;`) in order to fix their deserialization ([#876][pr876])
- `Download` now uses GAT feature on the `Fut` and `Err` associated types, instead of a lifetime on the whole trait ([#885][pr885])
- Renamed `ForumTopic::message_thread_id` into `thread_id` ([#887][pr887])
- `ForumTopic::thread_id` and `Message::thread_id` now use `ThreadId` instead of `i32` ([#887][pr887])
- `message_thread_id` method parameters now use `ThreadId` instead of `i32` ([#887][pr887])
- `DiceEmoji` variant order ([#887][pr887])
- `Dice::value` now use `u8`, instead of `i32` ([#887][pr887])
- `Invoice::total_amount`, `LabeledPrice::amount`, `PreCheckoutQuery::total_amount`, `SuccessfulPayment::total_amount` now use `u32`, instead of `i32` ([#887][pr887])
- `Forward::message_id` and `Message::forward_from_message_id` now use `MessageId` instead of `i32` ([#887][pr887])
- `Poll::total_voter_count` and `PollOption::voter_count` now use `u32` instead of `i32` ([#887][pr887])
- `PollAnswer::option_ids` now use `u8` instead of `i32` ([#887][pr887])
- Use `u32` for sizes and `Seconds` for timespans in `InlineQueryResult*` ([#887][pr887])
- `SendGame::reply_to_message_id`, `SendSticker::reply_to_message_id` and `SendInvoice::reply_to_message_id` now use `MessageId` instead of `i32` ([#887][pr887])
- Use `UpdateId` for `Update::id` ([#892][pr892])
- MSRV (Minimal Supported Rust Version) was bumped from `1.64.0` to `1.68.0` ([#950][pr950])
- Add proper support for `edit_message_caption_inline`, `copy_message`, `answer_inline_query`, `answer_web_app_query`, `send_media_group`, `edit_message_media`, and `edit_message_media_inline` to `DefaultParseMode` adaptor ([#961][pr961])
  - Note that now `DefaultParseMode` sets the default on `send`, instead of request creation
  - `DefaultParseMode` now also requires that the supported requests implement `Clone` (as a user you should not notice anything changing)
- Methods of the Message type: `delete_chat_photo`, `group_chat_created`, `super_group_chat_created`, `channel_chat_created`, `chat_migration`, `migrate_to_chat_id`, `migrate_from_chat_id` now return shared reference instead of owned value inside `Option` ([#982][pr982])
- Methods `delete_chat_photo`, `group_chat_created`, `super_group_chat_created`, `channel_chat_created` now return appropriate structs not `Option<True>` ([#982][pr982])
- MSRV (Minimal Supported Rust Version) was bumped from `1.68.0` to `1.70.0` ([#996][pr996])
- Changes in the existing API to support TBA6.6 ([#1040](pr1040))
  - `InputSticker` was changed from enum to the struct
  - `{Animation, Audio, Document, Sticker, Video, VideoNote, InputMediaAnimation, InputMediaAudio, InputMediaDocument, InputMediaVideo, StickerSet}::thumb` has been renamed to `thumbnail`
  - `{SendAnimation, SendAudio, SendDocument, SendSticker,  SendVideo, SendVideoNote}::thumb` has been renamed to`thumbnail`
  - Now `StickerFormat` is defined as the enum with the variants: `Static`, `Video`, `Animated`, so the previous variant `Raster` has been renamed to `Static`
  - Method `StickerFormat::is_raster` has been renamed to `StickerFormat::is_static`
  - Methods `StickerSet::{format, is_static, is_animated, is_video}` are deprecated now, due to the breaking change in TBA 7.2 API (which removes `StickerFormatFlags::{is_video, is_animated}` from the `StickerSet` class)
  - Method `set_sticker_set_thumb` and it's parameter `thumb` have been renamed to `set_sticker_set_thumbnail` and `thumbnail` respectively
  - Fields `{InlineQueryResultArticle, InlineQueryResultContact, InlineQueryResultDocument, InlineQueryResultLocation, InlineQueryResultVenue}::{thumb_url, thumb_width, thumb_height}` have been renamed to `{thumbnail_url, thumbnail_width, thumbnail_height}` respectively
  - Field `{InlineQueryResultPhoto, InlineQueryResultVideo, InlineQueryResultGif, InlineQueryResultMpeg4Gif}::thumb_url` has been renamed to `thumbnail_url`
- Support for TBA 7.0 ([#1101](pr1101))
  - Replies 2.0
    - Parameter `reply_parameters` of type `ReplyParameters` replaces parameters `reply_to_message_id` and `allow_sending_without_reply` in the methods:
      - `copy_message`
      - `send_message`
      - `send_photo`
      - `send_video`
      - `send_animation`
      - `send_audio`
      - `send_document`
      - `send_sticker`
      - `send_video_note`
      - `send_voice`
      - `send_location`
      - `send_venue`
      - `send_contact`
      - `send_poll`
      - `send_dice`
      - `send_invoice`
      - `send_game`
      - `send_media_group`
  - Request for multiple users
    - Struct `KeyboardButtonRequestUser` was renamed to `KeyboardButtonRequestUsers` + added field `max_quantity` to it
    - Field `KeyboardButton::request_user` was renamed to `request_users`
    - `MessageUserShared` was renamed to `MessageUsersShared`
  - Other Changes
    - `Message::pinned_message` and `CallbackQuery::message` now have `MaybeInaccessibleMessage` type
    - Field `emoji_status_custom_emoji_id` is allowed in non-private chats (moved to the `ChatFullInfo`)
    - Struct `Forward` was replaced by `MessageOrigin` in `MessageCommon`
  - `RequestId` replaces `i32` in `ChatShared` and `KeyboardButtonRequestChat` structs


[pr852]: https://github.com/teloxide/teloxide/pull/853
[pr859]: https://github.com/teloxide/teloxide/pull/859
[pr876]: https://github.com/teloxide/teloxide/pull/876
[pr885]: https://github.com/teloxide/teloxide/pull/885
[pr892]: https://github.com/teloxide/teloxide/pull/892
[pr950]: https://github.com/teloxide/teloxide/pull/950
[pr961]: https://github.com/teloxide/teloxide/pull/961
[pr996]: https://github.com/teloxide/teloxide/pull/996

### Deprecated

- `Update::user`, use `Update::from` instead ([#850][pr850])
- `Message::from()` and `Message::sender_chat()` in favour of fields with the same name([initially #946][pr946][#1101][pr1101])

[pr850]: https://github.com/teloxide/teloxide/pull/850
[pr946]: https://github.com/teloxide/teloxide/pull/946

### Removed

- Remove `can_send_media_messages` from `ChatPermissions` ([#954][pr954])
- Remove `can_send_media_messages` field from `Restricted` ([#954][pr954])
- Previously deprecated items ([#1013][pr1013])
  - `AutoSend` bot adaptor
  - `ChatMemberKind::is_kicked` (use `is_banned` instead)
  - `ChatMemberKind::is_creator` (use `is_owner` instead)
  - `ChatMemberKind::{can_change_info, can_pin_messages, can_invite_users, can_manage_topics, can_send_polls, can_add_web_page_previews, can_send_other_messages, can_send_media_messages, can_send_messages}` (match on `ChatMemberKind` yourself)
  - `ChatMemberStatus::is_present` (use `ChatMemberKind::is_present` instead)
  - `InlineKeyboardButton::{text, kind}`
  `teloxide::dispatching::{update_listeners, repls}` (use `reloxide::{update_listeners, repls}` instead)
  - `Dispatcher::setup_ctrlc_handler` (use `enable_ctrlc_handler` on the builder instead)
  - `BotCommands::ty` and `repls::{commands_repl, commands_repl_with_listener}` (use `CommandsRepl::{repl, repl_with_listener}` instead)
  - `Message::chat_id` (use `.chat.id`)
  - `Update::user` (use `Update::from`)
  - `update_listeners::polling` (use `Polling::builder` instead)

[pr954]: https://github.com/teloxide/teloxide/pull/954
[pr1013]: https://github.com/teloxide/teloxide/pull/1013


## 0.9.1 - 2023-02-15

### Fixed

- `Update::user` now handles channel posts, chat member changes and chat join request updates correctly ([#835][pr835])
- In cases when `teloxide` can't deserialize an update, error now includes the full json value ([#826][pr826])
- Deserialization of topic messages ([#830][pr830])

[pr835]: https://github.com/teloxide/teloxide/pull/835
[pr826]: https://github.com/teloxide/teloxide/pull/826
[pr830]: https://github.com/teloxide/teloxide/pull/830

### Added

- `ApiError::ImageProcessFailed` ([#825][pr825])

[pr825]: https://github.com/teloxide/teloxide/pull/825

## 0.9.0 - 2023-01-17

### Changed

- The methods `ChatMember::{can_pin_messages, can_invite_users, can_change_info}` now take into account the permissions of `Restricted` chat member kind ([#764][pr764])
- The method `ChatMemberKind::is_present` now takes into account the value of `Restricted::is_member` field ([#764][pr764])
- The following functions were made `#[must_use]`:
  - `MaskPoint::{new, point}`
  - `StickerKind::{premium_animation, mask_position, custom_emoji_id}`
- `Option<bool>` fields of `Administrator` are now `bool` ([#800][pr800]):
  - `can_post_messages`
  - `can_edit_messages`
  - `can_pin_messages`
  - `can_manage_topics`
- `ApiError::NotFound` is replaced with `ApiError::InvalidToken` which correctly parses all currently known errors caused by invalid bot tokens ([#998][pr998])

[pr998]: https://github.com/teloxide/teloxide/pull/998

### Added

- `Restricted::{is_member, can_change_info, can_invite_users, can_pin_messages, can_send_polls}` fields ([#764][pr764])
- `ChatMember::can_send_polls` method ([#764][pr764])
- Support for Telegram Bot API [version 6.3](https://core.telegram.org/bots/api#november-5-2022) ([#789][pr789])
- Support for Telegram Bot API [version 6.4](https://core.telegram.org/bots/api#december-30-2022) ([#809][pr809])

[pr764]: https://github.com/teloxide/teloxide/pull/764
[pr764]: https://github.com/teloxide/teloxide/pull/789
[pr800]: https://github.com/teloxide/teloxide/pull/800
[pr809]: https://github.com/teloxide/teloxide/pull/809

### Deprecated

- `ChatMemberKind` methods ([#800][pr800]):
  - `can_change_info`
  - `can_invite_users`
  - `can_pin_messages`
  - `can_send_messages`
  - `can_send_media_messages`
  - `can_send_other_messages`
  - `can_send_polls`
  - `can_add_web_page_previews`
- `ChatMemberStatus::is_present` method ([#800][pr800])

### Fixed

- `ChatMemberKind::can_manage_chat` method now correctly returns `false` for non owner/administrator users ([#800][pr800])

## 0.8.0 - 2022-10-03

### Added

- Support for Telegram Bot API [version 6.2](https://core.telegram.org/bots/api#august-12-2022) ([#251][pr251])

[pr251]: https://github.com/teloxide/teloxide-core/pull/251

### Changed

- Removed `file_` prefix from `File` and `FileMeta` fields [#255][pr255]
- `Animation`, `Audio`, `Document`, `PassportFile`, `PhotoSize`, `Video`, `VideoNote` and `Voice` now contain `FileMeta` instead of its fields ([#253][pr253])
  - Combined with `File` fields renaming, instead of `.file_size` you can write `.file.size` and similarly with other fields
- **You can now `.await` any `Request`!** ([#249][pr249])
  - `Request` now requires `Self: IntoFuture`
  - There is no need for `AutoSend` anymore
- MSRV (Minimal Supported Rust Version) was bumped from `1.58.0` to `1.64.0`
- Message id parameters and fields now use `MessageId` type instead of `i32` ([#254][pr254])
- Refactored `Sticker` and related types ([#251][pr251])

[pr253]: https://github.com/teloxide/teloxide-core/pull/253
[pr254]: https://github.com/teloxide/teloxide-core/pull/254
[pr255]: https://github.com/teloxide/teloxide-core/pull/255

### Removed

- Methods for creating `InlineQuery` ([#246][pr244])

[pr244]: https://github.com/teloxide/teloxide-core/pull/246

### Fixed

- `SetWebhook` request can now properly send certificate ([#250][pr250])
- Serialization of `InputSticker::Webm` ([#252][pr252])

[pr250]: https://github.com/teloxide/teloxide-core/pull/250
[pr252]: https://github.com/teloxide/teloxide-core/pull/252

### Deprecated

- `AutoSend` adaptor ([#249][pr249])

[pr249]: https://github.com/teloxide/teloxide-core/pull/249

## 0.7.1 - 2022-08-19

### Fixed

- `ErasedRequester<E>` now implements `Clone` even if `E` is not `Clone` ([#244][pr244])

[pr244]: https://github.com/teloxide/teloxide-core/pull/244

### Added

- `From<ApiError>`, `From<DownloadError>` and `From<std::io::Error>` impls for `RequestError` ([#241][pr241])

[pr241]: https://github.com/teloxide/teloxide-core/pull/241

### Changed

- More functions are now marked with `#[must_use]` ([#242][PR242])

[pr242]: https://github.com/teloxide/teloxide-core/pull/242

## 0.7.0 - 2022-07-19

### Added

- `InlineKeyboardButton::{pay, login, web_app, callback_game, pay}` constructors ([#231][pr231])
- Support for Telegram Bot API [version 6.1](https://core.telegram.org/bots/api#june-20-2022) ([#233][pr233])
- `StickerKind` that is now used instead of `is_animated` and `is_video` fields of `Sticker` and `StickerSet` ([#238][pr238])

[pr238]: https://github.com/teloxide/teloxide-core/pull/238

### Changed

- `InlineKeyboardButtonKind::Pay`'s only field now has type `True` ([#231][pr231])
- `file_size` fields are now always `u32` ([#237][pr237])
- `File` is now split into `File` and `FileMeta`, the latter is used in `UploadStickerFile` and `Sticker::premium_animation` ([#237][pr237])

[pr237]: https://github.com/teloxide/teloxide-core/pull/237

### Deprecated

- `InlineKeyboardButton::{text, kind}` functions ([#231][pr231])

[pr231]: https://github.com/teloxide/teloxide-core/pull/231
[pr233]: https://github.com/teloxide/teloxide-core/pull/233

### Removed

- `ChatPrivate::type_` field ([#232][pr232])

[pr232]: https://github.com/teloxide/teloxide-core/pull/232

## 0.6.3 - 2022-06-21

### Fixed

- Fix `Message::parse_caption_entities` ([#229][pr229])

[pr229]: https://github.com/teloxide/teloxide-core/pull/229

## 0.6.2 - 2022-06-16

### Fixed

- Fix `ChatPrivate` serialization ([#226][pr226])
- Build with particular crates versions (enable `"codec"` feature of `tokio-util`) ([#225][pr225])
- Remove trailing `/` from `Message::url` (on ios it caused problems) ([#223][pr223])
- Fix incorrect panic in `User::is_channel` ([#222][pr222])

[pr226]: https://github.com/teloxide/teloxide-core/pull/226
[pr225]: https://github.com/teloxide/teloxide-core/pull/225
[pr222]: https://github.com/teloxide/teloxide-core/pull/222

### Added

- `Message::{url_of, comment_url, comment_url_of, url_in_thread, url_in_thread_of}` functions ([#223][pr223])
- Utilities to parse message entities (see `Message::parse_entities`) ([#217][pr217])

[pr223]: https://github.com/teloxide/teloxide-core/pull/223
[pr212]: https://github.com/teloxide/teloxide-core/pull/212

## 0.6.1 - 2022-06-02

### Fixed

- Deserialization of `File` when `file_path` or `file_size` are missing ([#220][pr220])
- Correct how `NotFound` and `UserDeactivated` errors are deserialized ([#219][pr219])

[pr220]: https://github.com/teloxide/teloxide-core/pull/220
[pr219]: https://github.com/teloxide/teloxide-core/pull/219

### Added

- `is_*` methods to `ChatMemberStatus` analogous to the `ChatMember{,Kind}` methods ([#216][pr216])
- `ChatId` and `UserId` to the prelude ([#212][pr212])

[pr216]: https://github.com/teloxide/teloxide-core/pull/216
[pr212]: https://github.com/teloxide/teloxide-core/pull/212

## 0.6.0 - 2022-04-25

### Added

- Support for Telegram Bot API [version 6.0](https://core.telegram.org/bots/api#april-16-2022) ([#206][pr206], [#211][pr211])
  - Note that some field were renamed
- Shortcut methods for `MessageEntity` ([#208][pr208], [#210][pr210])

[pr208]: https://github.com/teloxide/teloxide-core/pull/208
[pr206]: https://github.com/teloxide/teloxide-core/pull/206
[pr210]: https://github.com/teloxide/teloxide-core/pull/210
[pr211]: https://github.com/teloxide/teloxide-core/pull/211

### Changed

- Make `KeyboardMarkup` creation more convenient ([#207][pr207])
  - Accept `IntoIterator` in `KeyboardMarkup::append_row`.
  - Accept `Into<String>` instead of `String` in `InlineKeyboardButton::{url, callback, switch_inline_query, switch_inline_query_current_chat}`.

[pr207]: https://github.com/teloxide/teloxide-core/pull/207

## 0.5.1 - 2022-04-18

### Fixed

- Document the `errors` module.

## 0.5.0 - 2022-04-13

### Added

- `errors` module and `errors::AsResponseParameters` trait ([#130][pr130])
- `UserId::{url, is_anonymous, is_channel, is_telegram}` convenience functions ([#197][pr197])
- `User::{tme_url, preferably_tme_url}` convenience functions ([#197][pr197])
- `Me::username` and `Deref<Target = User>` implementation for `Me` ([#197][pr197])
- `Me::{mention, tme_url}` ([#197][pr197])
- `AllowedUpdate::ChatJoinRequest` ([#201][pr201])
- `ChatId::{is_user, is_group, is_channel_or_supergroup}` functions [#198][pr198]

[pr197]: https://github.com/teloxide/teloxide-core/pull/197
[pr198]: https://github.com/teloxide/teloxide-core/pull/198
[pr201]: https://github.com/teloxide/teloxide-core/pull/201

### Changed

- `user.id` now uses `UserId` type, `ChatId` now represents only _chat id_, not channel username, all `chat_id` function parameters now accept `Recipient` [**BC**]
- Improve `Throttling` adaptor ([#130][pr130])
  - Freeze when getting `RetryAfter(_)` error
  - Retry requests that previously returned `RetryAfter(_)` error
- `RequestError::RetryAfter` now has a `Duration` field instead of `i32`

### Fixed

- A bug in `Message::url` implementation ([#198][pr198])
- Fix never ending loop that caused programs that used `Throttling` to never stop, see issue [#535][issue535] ([#130][pr130])

[issue535]: https://github.com/teloxide/teloxide/issues/535
[pr130]: https://github.com/teloxide/teloxide-core/pull/130

## 0.4.5 - 2022-04-03

### Fixed

- Hide bot token in errors ([#200][200])

[200]: https://github.com/teloxide/teloxide-core/pull/200

## 0.4.4 - 2022-04-21

### Added

- `WrongFileIdOrUrl` and `FailedToGetUrlContent` errors ([#188][pr188])
- `NotFound` error ([#190][pr190])
- `HasPayload::with_payload_mut` function ([#189][pr189])

[pr188]: https://github.com/teloxide/teloxide-core/pull/188
[pr189]: https://github.com/teloxide/teloxide-core/pull/189
[pr190]: https://github.com/teloxide/teloxide-core/pull/190

## 0.4.3 - 2022-03-08

### Added

- `User::is_telegram` function ([#186][pr186])

[pr186]: https://github.com/teloxide/teloxide-core/pull/186

### Fixed

- `Update::chat()` now returns `Some(&Chat)` for `UpdateKind::ChatMember`, `UpdateKind::MyChatMember`,
  `UpdateKind::ChatJoinRequest` ([#184][pr184])
- `get_updates` timeouts (partially revert buggy [#180][pr180]) ([#185][pr185])

[pr184]: https://github.com/teloxide/teloxide-core/pull/184
[pr185]: https://github.com/teloxide/teloxide-core/pull/185

## 0.4.2 - 2022-02-17 [yanked]

### Deprecated

- `Message::chat_id` use `.chat.id` field instead ([#182][pr182])

[pr182]: https://github.com/teloxide/teloxide-core/pull/182

### Fixed

- Serialization of `SendPoll::type_` (it's now possible to send quiz polls) ([#181][pr181])

[pr181]: https://github.com/teloxide/teloxide-core/pull/181

### Added

- `Payload::timeout_hint` method to properly handle long running requests like `GetUpdates` ([#180][pr180])

[pr180]: https://github.com/teloxide/teloxide-core/pull/180

## 0.4.1 - 2022-02-13

### Fixed

- Deserialization of `UntilDate` ([#178][pr178])

[pr178]: https://github.com/teloxide/teloxide-core/pull/178

## 0.4.0 - 2022-02-03

### Added

- `ApiError::TooMuchInlineQueryResults` ([#135][pr135])
- `ApiError::NotEnoughRightsToChangeChatPermissions` ([#155][pr155])
- Support for 5.4 telegram bot API ([#133][pr133])
- Support for 5.5 telegram bot API ([#143][pr143], [#164][pr164])
- Support for 5.6 telegram bot API ([#162][pr162])
- Support for 5.7 telegram bot API ([#175][pr175])
- `EditedMessageIsTooLong` error ([#109][pr109])
- `UntilDate` enum and use it for `{Restricted, Banned}::until_date` ([#117][pr117])
- `Limits::messages_per_min_channel` ([#121][pr121])
- `media_group_id` field to `MediaDocument` and `MediaAudio` ([#139][pr139])
- `caption_entities` method to `InputMediaPhoto` ([#140][pr140])
- `User::is_anonymous` and `User::is_channel` functions ([#151][pr151])
- `UpdateKind::Error` ([#156][pr156])

[pr109]: https://github.com/teloxide/teloxide-core/pull/109
[pr117]: https://github.com/teloxide/teloxide-core/pull/117
[pr121]: https://github.com/teloxide/teloxide-core/pull/121
[pr135]: https://github.com/teloxide/teloxide-core/pull/135
[pr139]: https://github.com/teloxide/teloxide-core/pull/139
[pr140]: https://github.com/teloxide/teloxide-core/pull/140
[pr143]: https://github.com/teloxide/teloxide-core/pull/143
[pr151]: https://github.com/teloxide/teloxide-core/pull/151
[pr155]: https://github.com/teloxide/teloxide-core/pull/155
[pr156]: https://github.com/teloxide/teloxide-core/pull/156
[pr162]: https://github.com/teloxide/teloxide-core/pull/162
[pr164]: https://github.com/teloxide/teloxide-core/pull/164
[pr175]: https://github.com/teloxide/teloxide-core/pull/175

### Changed

- Refactor `InputFile` ([#167][pr167])
  - Make it an opaque structure, instead of enum
  - Add `read` constructor, that allows creating `InputFile` from `impl AsyncRead`
  - Internal changes
- Refactor errors ([#134][pr134])
  - Rename `DownloadError::NetworkError` to `Network`
  - Rename `RequestError::ApiError` to `Api`
  - Remove `RequestError::Api::status_code` and rename `RequestError::Api::kind` to `0` (struct to tuple struct)
  - Rename `RequestError::NetworkError` to `Network`
  - Implement `Error` for `ApiError`
- Use `url::Url` for urls, use `chrono::DateTime<Utc>` for dates in types ([#115][pr115])
- Mark `ApiError` as `non_exhaustive` ([#125][pr125])
- `InputFile` and related structures now do **not** implement `PartialEq`, `Eq` and `Hash` ([#133][pr133])
- How forwarded messages are represented ([#151][pr151])
- `RequestError::InvalidJson` now has a `raw` field with raw json for easier debuggability ([#150][pr150])
- `ChatPermissions` is now bitflags ([#157][pr157])
- Type of `WebhookInfo::ip_address` from `Option<String>` to `Option<std::net::IpAddr>` ([#172][pr172])
- Type of `WebhookInfo::allowed_updates` from `Option<Vec<String>>` to `Option<Vec<AllowedUpdate>>` ([#174][pr174])

[pr115]: https://github.com/teloxide/teloxide-core/pull/115
[pr125]: https://github.com/teloxide/teloxide-core/pull/125
[pr134]: https://github.com/teloxide/teloxide-core/pull/134
[pr150]: https://github.com/teloxide/teloxide-core/pull/150
[pr157]: https://github.com/teloxide/teloxide-core/pull/157
[pr167]: https://github.com/teloxide/teloxide-core/pull/167
[pr172]: https://github.com/teloxide/teloxide-core/pull/172
[pr174]: https://github.com/teloxide/teloxide-core/pull/174

### Fixed

- Deserialization of chat migrations, see issue [#427][issue427] ([#143][pr143])
- Type of `BanChatMember::until_date`: `u64` -> `chrono::DateTime<Utc>` ([#117][pr117])
- Type of `Poll::correct_option_id`: `i32` -> `u8` ([#119][pr119])
- Type of `Poll::open_period`: `i32` -> `u16` ([#119][pr119])
- `Throttle` adaptor not honouring chat/min limits ([#121][pr121])
- Make `SendPoll::type_` optional ([#133][pr133])
- Bug with `caption_entities`, see issue [#473][issue473]
- Type of response for `CopyMessage` method ([#141][pr141], [#142][pr142])
- Bad request serialization when the `language` field of `MessageEntityKind::Pre` is `None` ([#145][pr145])
- Deserialization of `MediaKind::Venue` ([#147][pr147])
- Deserialization of `VoiceChat{Started,Ended}` messages ([#153][pr153])
- Serialization of `BotCommandScope::Chat{,Administrators}` ([#154][pr154])

[pr119]: https://github.com/teloxide/teloxide-core/pull/119
[pr133]: https://github.com/teloxide/teloxide-core/pull/133
[pr141]: https://github.com/teloxide/teloxide-core/pull/141
[pr142]: https://github.com/teloxide/teloxide-core/pull/142
[pr143]: https://github.com/teloxide/teloxide-core/pull/143
[pr145]: https://github.com/teloxide/teloxide-core/pull/145
[pr147]: https://github.com/teloxide/teloxide-core/pull/147
[pr153]: https://github.com/teloxide/teloxide-core/pull/153
[pr154]: https://github.com/teloxide/teloxide-core/pull/154
[issue473]: https://github.com/teloxide/teloxide/issues/473
[issue427]: https://github.com/teloxide/teloxide/issues/427

### Removed

- `get_updates_fault_tolerant` method and `SemiparsedVec` ([#156][pr156])

## 0.3.3 - 2021-08-03

### Fixed

- Compilation with `nightly` feature (use `type_alias_impl_trait` instead of `min_type_alias_impl_trait`) ([#108][pr108])

[pr108]: https://github.com/teloxide/teloxide-core/pull/108

## 0.3.2 - 2021-07-27

### Added

- `ErasedRequester` bot adaptor, `ErasedRequest` struct, `{Request, RequesterExt}::erase` functions ([#105][pr105])
- `Trace` bot adaptor ([#104][pr104])
- `HasPayload`, `Request` and `Requester` implementations for `either::Either` ([#103][pr103])

[pr103]: https://github.com/teloxide/teloxide-core/pull/103
[pr104]: https://github.com/teloxide/teloxide-core/pull/104
[pr105]: https://github.com/teloxide/teloxide-core/pull/105

## 0.3.1 - 2021-07-07

- Minor documentation tweaks ([#102][pr102])
- Remove `Self: 'static` bound on `RequesterExt::throttle` ([#102][pr102])

[pr102]: https://github.com/teloxide/teloxide-core/pull/102

## 0.3.0 - 2021-07-05

### Added

- `impl Clone` for {`CacheMe`, `DefaultParseMode`, `Throttle`} ([#76][pr76])
- `DefaultParseMode::parse_mode` which allows to get currently used default parse mode ([#77][pr77])
- `Throttle::{limits,set_limits}` functions ([#77][pr77])
- `Throttle::{with_settings,spawn_with_settings}` and `throttle::Settings` ([#96][pr96])
- Getters for fields nested in `Chat` ([#80][pr80])
- API errors: `ApiError::NotEnoughRightsToManagePins`, `ApiError::BotKickedFromSupergroup` ([#84][pr84])
- Telegram bot API 5.2 support ([#86][pr86])
- Telegram bot API 5.3 support ([#99][pr99])
- `net::default_reqwest_settings` function ([#90][pr90])

[pr75]: https://github.com/teloxide/teloxide-core/pull/75
[pr77]: https://github.com/teloxide/teloxide-core/pull/77
[pr76]: https://github.com/teloxide/teloxide-core/pull/76
[pr80]: https://github.com/teloxide/teloxide-core/pull/80
[pr84]: https://github.com/teloxide/teloxide-core/pull/84
[pr86]: https://github.com/teloxide/teloxide-core/pull/86
[pr90]: https://github.com/teloxide/teloxide-core/pull/90
[pr96]: https://github.com/teloxide/teloxide-core/pull/96
[pr99]: https://github.com/teloxide/teloxide-core/pull/99

### Changed

- `Message::url` now returns links to messages in private groups too ([#80][pr80])
- Refactor `ChatMember` methods ([#74][pr74])
  - impl `Deref<Target = ChatMemberKind>` to make `ChatMemberKind`'s methods callable directly on `ChatMember`
  - Add `ChatMemberKind::is_{creator,administrator,member,restricted,left,kicked}` which check `kind` along with `is_privileged` and `is_in_chat` which combine some of the above.
  - Refactor privilege getters
- Rename `ChatAction::{RecordAudio => RecordVoice, UploadAudio => UploadVoice}` ([#86][pr86])
- Use `url::Url` for urls, use `chrono::DateTime<Utc>` for dates ([#97][pr97])

[pr74]: https://github.com/teloxide/teloxide-core/pull/74
[pr97]: https://github.com/teloxide/teloxide-core/pull/97

### Fixed

- telegram_response: fix issue `retry_after` and `migrate_to_chat_id` handling ([#94][pr94])
- Type of `PublicChatSupergroup::slow_mode_delay` field: `Option<i32>`=> `Option<u32>` ([#80][pr80])
- Add missing `Chat::message_auto_delete_time` field ([#80][pr80])
- Output types of `LeaveChat` `PinChatMessage`, `SetChatDescription`, `SetChatPhoto` `SetChatTitle`, `UnpinAllChatMessages` and `UnpinChatMessage`: `String` => `True` ([#79][pr79])
- `SendChatAction` output type `Message` => `True` ([#75][pr75])
- `GetChatAdministrators` output type `ChatMember` => `Vec<ChatMember>` ([#73][pr73])
- `reqwest` dependency bringing `native-tls` in even when `rustls` was selected ([#71][pr71])
- Type of `{Restricted,Kicked}::until_date` fields: `i32` => `i64` ([#74][pr74])
- Type of `PhotoSize::{width,height}` fields: `i32` => `u32` ([#100][pr100])

[pr71]: https://github.com/teloxide/teloxide-core/pull/71
[pr73]: https://github.com/teloxide/teloxide-core/pull/73
[pr75]: https://github.com/teloxide/teloxide-core/pull/75
[pr79]: https://github.com/teloxide/teloxide-core/pull/79
[pr94]: https://github.com/teloxide/teloxide-core/pull/94
[pr100]: https://github.com/teloxide/teloxide-core/pull/100

## 0.2.2 - 2020-03-22

### Fixed

- Typo: `ReplyMarkup::{keyboad => keyboard}` ([#69][pr69])
  - Note: method with the old name was deprecated and hidden from docs

[pr69]: https://github.com/teloxide/teloxide-core/pull/69

## 0.2.1 - 2020-03-19

### Fixed

- Types fields privacy (make fields of some types public) ([#68][pr68])
  - `Dice::{emoji, value}`
  - `MessageMessageAutoDeleteTimerChanged::message_auto_delete_timer_changed`
  - `PassportElementError::{message, kind}`
  - `StickerSet::thumb`

[pr68]: https://github.com/teloxide/teloxide-core/pull/68

## 0.2.0 - 2020-03-16

### Changed

- Refactor `ReplyMarkup` ([#pr65][pr65]) (**BC**)
  - Rename `ReplyMarkup::{InlineKeyboardMarkup => InlineKeyboard, ReplyKeyboardMarkup => Keyboard, ReplyKeyboardRemove => KeyboardRemove}`
  - Add `inline_kb`, `keyboad`, `kb_remove` and `force_reply` `ReplyMarkup` constructors
  - Rename `ReplyKeyboardMarkup` => `KeyboardMarkup`
  - Rename `ReplyKeyboardRemove` => `KeyboardRemove`
  - Remove useless generic param from `ReplyKeyboardMarkup::new` and `InlineKeyboardMarkup::new`
  - Change parameters order in `ReplyKeyboardMarkup::append_to_row` and `InlineKeyboardMarkup::append_to_row`
- Support telegram bot API version 5.1 (see it's [changelog](https://core.telegram.org/bots/api#march-9-2021)) ([#pr63][pr63]) (**BC**)
- Support telegram bot API version 5.0 (see it's [changelog](https://core.telegram.org/bots/api#november-4-2020)) ([#pr62][pr62]) (**BC**)

[pr62]: https://github.com/teloxide/teloxide-core/pull/62
[pr63]: https://github.com/teloxide/teloxide-core/pull/63
[pr65]: https://github.com/teloxide/teloxide-core/pull/65

### Added

- `GetUpdatesFaultTolerant` - fault tolerant version of `GetUpdates` ([#58][pr58]) (**BC**)
- Derive `Clone` for `AutoSend`.

[pr58]: https://github.com/teloxide/teloxide-core/pull/58

### Fixed

- Make `MediaContact::contact` public ([#pr64][pr64])
- `set_webhook` signature (make `allowed_updates` optional) ([#59][pr59])
- Fix typos in payloads ([#57][pr57]):
  - `get_updates`: `offset` `i64` -> `i32`
  - `send_location`: make `live_period` optional
- `send_contact` signature (`phone_number` and `first_name` `f64` => `String`) ([#56][pr56])

[pr56]: https://github.com/teloxide/teloxide-core/pull/56
[pr57]: https://github.com/teloxide/teloxide-core/pull/57
[pr59]: https://github.com/teloxide/teloxide-core/pull/59
[pr64]: https://github.com/teloxide/teloxide-core/pull/64

### Removed

- `Message::text_owned` ([#pr62][pr62]) (**BC**)

### Changed

- `NonStrictVec` -> `SemiparsedVec`.

## 0.1.1 - 2020-02-17

### Fixed

- Remove `dbg!` call from internals ([#53][pr53])

[pr53]: https://github.com/teloxide/teloxide-core/pull/53

## 0.1.0 - 2020-02-17

### Added

- `#[non_exhaustive]` on `InputFile` since we may want to add new ways to send files in the future ([#49][pr49])
- `MultipartPayload` for future proofing ([#49][pr49])
- Support for `rustls` ([#24][pr24])
- `#[must_use]` attr to payloads implemented by macro ([#22][pr22])
- forward-to-deref `Requester` impls ([#39][pr39])
- `Bot::{set_,}api_url` methods ([#26][pr26], [#35][pr35])
- `payloads` module
- `RequesterExt` trait which is implemented for all `Requester`s and allows easily wrapping them in adaptors
- `adaptors` module ([#14][pr14])
  - `throttle`, `cache_me`, `auto_send` and `full` crate features
  - Request throttling - opt-in feature represented by `Throttle` bot adapter which allows automatically checking telegram limits ([#10][pr10], [#46][pr46], [#50][pr50])
  - Request auto sending - ability to `.await` requests without need to call `.send()` (opt-in feature represented by `AutoSend` bot adapter, [#8][pr8])
  - `get_me` caching (opt-in feature represented by `CacheMe` bot adapter)
- `Requester` trait which represents bot-clients ([#7][pr7], [#12][pr12], [#27][pr27])
- `{Json,Multipart}Request` the `Bot` requests types ([#6][pr6])
- `Output<T>` alias to `<<T as HasPayload>::Payload as Payload>::Output`
- `Payload`, `HasPayload` and `Request` traits which represent different parts of the request ([#5][pr5])
- `GetUpdatesNonStrict` 'telegram' method, that behaves just like `GetUpdates` but doesn't [#2][pr2]
  fail if one of updates fails to be deserialized
- Move core code here from the [`teloxide`] main repo, for older changes see it's [`CHANGELOG.md`].
  - Following modules were moved:
    - `bot`
    - `requests` [except `requests::respond` function]
    - `types`
    - `errors`
    - `net` [private]
  - `client_from_env` was moved from `teloxide::utils` to crate root of `teloxide-core`
  - To simplify `GetUpdates` request it was changed to simply return `Vec<Update>`
    (instead of `Vec<Result<Update, (Value, serde_json::Error)>>`)

[pr2]: https://github.com/teloxide/teloxide-core/pull/2
[pr5]: https://github.com/teloxide/teloxide-core/pull/5
[pr6]: https://github.com/teloxide/teloxide-core/pull/6
[pr7]: https://github.com/teloxide/teloxide-core/pull/7
[pr8]: https://github.com/teloxide/teloxide-core/pull/8
[pr10]: https://github.com/teloxide/teloxide-core/pull/10
[pr12]: https://github.com/teloxide/teloxide-core/pull/12
[pr14]: https://github.com/teloxide/teloxide-core/pull/14
[pr22]: https://github.com/teloxide/teloxide-core/pull/22
[pr24]: https://github.com/teloxide/teloxide-core/pull/24
[pr26]: https://github.com/teloxide/teloxide-core/pull/26
[pr27]: https://github.com/teloxide/teloxide-core/pull/27
[pr35]: https://github.com/teloxide/teloxide-core/pull/35
[pr39]: https://github.com/teloxide/teloxide-core/pull/39
[pr46]: https://github.com/teloxide/teloxide-core/pull/46
[pr49]: https://github.com/teloxide/teloxide-core/pull/49
[pr50]: https://github.com/teloxide/teloxide-core/pull/50

### Changed

- Cleanup setters in `types::*` (remove most of them) ([#44][pr44])
- Refactor `KeyboardButtonPollType` ([#44][pr44])
- Replace `Into<Vec<_>>` by `IntoIterator<Item = _>` in function arguments ([#44][pr44])
- Update dependencies (including tokio 1.0) ([#37][pr37])
- Refactor file downloading ([#30][pr30]):
  - Make `net` module public
  - Move `Bot::download_file{,_stream}` methods to a new `Download` trait
    - Impl `Download` for all bot adaptors & the `Bot` itself
  - Change return type of `download_file_stream` — return ` Stream<Result<Bytes>>``, instead of  `Future<Result<Stream<Result<Bytes>>>>``
  - Add `api_url` param to standalone versions of `download_file{,_stream}`
  - Make `net::{TELEGRAM_API_URL, download_file{,_stream}}` pub
- Refactor `Bot` ([#29][pr29]):
  - Move default parse mode to an adaptor (`DefaultParseMode`)
  - Remove bot builder (it's not useful anymore, since parse_mode is moved away)
  - Undeprecate bot constructors (`Bot::{new, with_client, from_env_with_client}`)
- Rename `StickerType` => `InputSticker`, `{CreateNewStickerSet,AddStickerToSet}::sticker_type}` => `sticker` ([#23][pr23], [#43][pr43])
- Use `_: IntoIterator<Item = T>` bound instead of `_: Into<Vec<T>>` in telegram methods which accept collections ([#21][pr21])
- Make `MessageDice::dice` pub ([#20][pr20])
- Merge `ApiErrorKind` and `KnownApiErrorKind` into `ApiError` ([#13][pr13])
- Refactor ChatMember ([#9][pr9])
  - Replace a bunch of `Option<_>` fields with `ChatMemberKind`
  - Remove setters (users are not expected to create this struct)
  - Add getters
- Changed internal mechanism of sending multipart requests ([#1][pr1])
- Added `RequestError::Io(io::Error)` to wrap I/O error those can happen while sending files to telegram
- Make all fields of all methods `pub` ([#3][pr3])

[pr1]: https://github.com/teloxide/teloxide-core/pull/1
[pr3]: https://github.com/teloxide/teloxide-core/pull/3
[pr9]: https://github.com/teloxide/teloxide-core/pull/9
[pr13]: https://github.com/teloxide/teloxide-core/pull/13
[pr20]: https://github.com/teloxide/teloxide-core/pull/20
[pr21]: https://github.com/teloxide/teloxide-core/pull/21
[pr23]: https://github.com/teloxide/teloxide-core/pull/23
[pr29]: https://github.com/teloxide/teloxide-core/pull/29
[pr30]: https://github.com/teloxide/teloxide-core/pull/30
[pr37]: https://github.com/teloxide/teloxide-core/pull/37
[pr43]: https://github.com/teloxide/teloxide-core/pull/43

### Removed

- `unstable-stream` feature (now `Bot::download_file_stream` is accessible by default)
- old `Request` trait
- `RequestWithFile`, now multipart requests use `Request`
- Remove all `#[non_exhaustive]` annotations ([#4][pr4])
- Remove `MessageEntity::text_from` because it's wrong ([#44][pr44])

[pr4]: https://github.com/teloxide/teloxide-core/pull/4
[pr44]: https://github.com/teloxide/teloxide-core/pull/44
[`teloxide`]: https://github.com/teloxide/teloxide
[`changelog.md`]: https://github.com/teloxide/teloxide/blob/master/CHANGELOG.md
