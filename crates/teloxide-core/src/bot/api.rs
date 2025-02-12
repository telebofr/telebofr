use url::Url;

use crate::{
    payloads,
    prelude::Requester,
    requests::{JsonRequest, MultipartRequest},
    types::{
        BotCommand, BusinessConnectionId, ChatId, ChatPermissions, InlineQueryResult, InputFile,
        InputMedia, InputPollOption, InputSticker, LabeledPrice, MessageId, Recipient, Rgb,
        StickerFormat, ThreadId, UserId,
    },
    Bot,
};

impl Requester for Bot {
    type Err = crate::errors::RequestError;

    type GetUpdates = JsonRequest<payloads::GetUpdates>;

    fn get_updates(&self) -> Self::GetUpdates {
        Self::GetUpdates::new(self.clone(), payloads::GetUpdates::new())
    }

    type SetWebhook = MultipartRequest<payloads::SetWebhook>;

    fn set_webhook(&self, url: Url) -> Self::SetWebhook {
        Self::SetWebhook::new(self.clone(), payloads::SetWebhook::new(url))
    }

    type DeleteWebhook = JsonRequest<payloads::DeleteWebhook>;

    fn delete_webhook(&self) -> Self::DeleteWebhook {
        Self::DeleteWebhook::new(self.clone(), payloads::DeleteWebhook::new())
    }

    type GetWebhookInfo = JsonRequest<payloads::GetWebhookInfo>;

    fn get_webhook_info(&self) -> Self::GetWebhookInfo {
        Self::GetWebhookInfo::new(self.clone(), payloads::GetWebhookInfo::new())
    }

    type GetMe = JsonRequest<payloads::GetMe>;

    fn get_me(&self) -> Self::GetMe {
        Self::GetMe::new(self.clone(), payloads::GetMe::new())
    }

    type SendMessage = JsonRequest<payloads::SendMessage>;

    fn send_message<C, T>(&self, chat_id: C, text: T) -> Self::SendMessage
    where
        C: Into<Recipient>,
        T: Into<String>,
    {
        Self::SendMessage::new(self.clone(), payloads::SendMessage::new(chat_id, text))
    }

    type ForwardMessage = JsonRequest<payloads::ForwardMessage>;

    fn forward_message<C, F>(
        &self,
        chat_id: C,
        from_chat_id: F,
        message_id: MessageId,
    ) -> Self::ForwardMessage
    where
        C: Into<Recipient>,
        F: Into<Recipient>,
    {
        Self::ForwardMessage::new(
            self.clone(),
            payloads::ForwardMessage::new(chat_id, from_chat_id, message_id),
        )
    }

    type ForwardMessages = JsonRequest<payloads::ForwardMessages>;
    fn forward_messages<C, F, M>(
        &self,
        chat_id: C,
        from_chat_id: F,
        message_ids: M,
    ) -> Self::ForwardMessages
    where
        C: Into<Recipient>,
        F: Into<Recipient>,
        M: IntoIterator<Item = MessageId>,
    {
        Self::ForwardMessages::new(
            self.clone(),
            payloads::ForwardMessages::new(chat_id, from_chat_id, message_ids),
        )
    }

    type SendPhoto = MultipartRequest<payloads::SendPhoto>;

    fn send_photo<C>(&self, chat_id: C, photo: InputFile) -> Self::SendPhoto
    where
        C: Into<Recipient>,
    {
        Self::SendPhoto::new(self.clone(), payloads::SendPhoto::new(chat_id, photo))
    }

    type SendAudio = MultipartRequest<payloads::SendAudio>;

    fn send_audio<C>(&self, chat_id: C, audio: InputFile) -> Self::SendAudio
    where
        C: Into<Recipient>,
    {
        Self::SendAudio::new(self.clone(), payloads::SendAudio::new(chat_id, audio))
    }

    type SendDocument = MultipartRequest<payloads::SendDocument>;

    fn send_document<C>(&self, chat_id: C, document: InputFile) -> Self::SendDocument
    where
        C: Into<Recipient>,
    {
        Self::SendDocument::new(self.clone(), payloads::SendDocument::new(chat_id, document))
    }

    type SendVideo = MultipartRequest<payloads::SendVideo>;

    fn send_video<C>(&self, chat_id: C, video: InputFile) -> Self::SendVideo
    where
        C: Into<Recipient>,
    {
        Self::SendVideo::new(self.clone(), payloads::SendVideo::new(chat_id, video))
    }

    type SendAnimation = MultipartRequest<payloads::SendAnimation>;

    fn send_animation<C>(&self, chat_id: C, animation: InputFile) -> Self::SendAnimation
    where
        C: Into<Recipient>,
    {
        Self::SendAnimation::new(self.clone(), payloads::SendAnimation::new(chat_id, animation))
    }

    type SendVoice = MultipartRequest<payloads::SendVoice>;

    fn send_voice<C>(&self, chat_id: C, voice: InputFile) -> Self::SendVoice
    where
        C: Into<Recipient>,
    {
        Self::SendVoice::new(self.clone(), payloads::SendVoice::new(chat_id, voice))
    }

    type SendVideoNote = MultipartRequest<payloads::SendVideoNote>;

    fn send_video_note<C>(&self, chat_id: C, video_note: InputFile) -> Self::SendVideoNote
    where
        C: Into<Recipient>,
    {
        Self::SendVideoNote::new(self.clone(), payloads::SendVideoNote::new(chat_id, video_note))
    }

    type SendMediaGroup = MultipartRequest<payloads::SendMediaGroup>;

    fn send_media_group<C, M>(&self, chat_id: C, media: M) -> Self::SendMediaGroup
    where
        C: Into<Recipient>,
        M: IntoIterator<Item = InputMedia>,
    {
        Self::SendMediaGroup::new(self.clone(), payloads::SendMediaGroup::new(chat_id, media))
    }

    type SendLocation = JsonRequest<payloads::SendLocation>;

    fn send_location<C>(&self, chat_id: C, latitude: f64, longitude: f64) -> Self::SendLocation
    where
        C: Into<Recipient>,
    {
        Self::SendLocation::new(
            self.clone(),
            payloads::SendLocation::new(chat_id, latitude, longitude),
        )
    }

    type EditMessageLiveLocation = JsonRequest<payloads::EditMessageLiveLocation>;

    fn edit_message_live_location<C>(
        &self,
        chat_id: C,
        message_id: MessageId,
        latitude: f64,
        longitude: f64,
    ) -> Self::EditMessageLiveLocation
    where
        C: Into<Recipient>,
    {
        Self::EditMessageLiveLocation::new(
            self.clone(),
            payloads::EditMessageLiveLocation::new(chat_id, message_id, latitude, longitude),
        )
    }

    type EditMessageLiveLocationInline = JsonRequest<payloads::EditMessageLiveLocationInline>;

    fn edit_message_live_location_inline<I>(
        &self,
        inline_message_id: I,
        latitude: f64,
        longitude: f64,
    ) -> Self::EditMessageLiveLocationInline
    where
        I: Into<String>,
    {
        Self::EditMessageLiveLocationInline::new(
            self.clone(),
            payloads::EditMessageLiveLocationInline::new(inline_message_id, latitude, longitude),
        )
    }

    type StopMessageLiveLocation = JsonRequest<payloads::StopMessageLiveLocation>;

    fn stop_message_live_location<C>(
        &self,
        chat_id: C,
        message_id: MessageId,
    ) -> Self::StopMessageLiveLocation
    where
        C: Into<Recipient>,
    {
        Self::StopMessageLiveLocation::new(
            self.clone(),
            payloads::StopMessageLiveLocation::new(chat_id, message_id),
        )
    }

    type StopMessageLiveLocationInline = JsonRequest<payloads::StopMessageLiveLocationInline>;

    fn stop_message_live_location_inline<I>(
        &self,
        inline_message_id: I,
    ) -> Self::StopMessageLiveLocationInline
    where
        I: Into<String>,
    {
        Self::StopMessageLiveLocationInline::new(
            self.clone(),
            payloads::StopMessageLiveLocationInline::new(inline_message_id),
        )
    }

    type SendVenue = JsonRequest<payloads::SendVenue>;

    fn send_venue<C, T, A>(
        &self,
        chat_id: C,
        latitude: f64,
        longitude: f64,
        title: T,
        address: A,
    ) -> Self::SendVenue
    where
        C: Into<Recipient>,
        T: Into<String>,
        A: Into<String>,
    {
        Self::SendVenue::new(
            self.clone(),
            payloads::SendVenue::new(chat_id, latitude, longitude, title, address),
        )
    }

    type SendContact = JsonRequest<payloads::SendContact>;

    fn send_contact<C, P, F>(&self, chat_id: C, phone_number: P, first_name: F) -> Self::SendContact
    where
        C: Into<Recipient>,
        P: Into<String>,
        F: Into<String>,
    {
        Self::SendContact::new(
            self.clone(),
            payloads::SendContact::new(chat_id, phone_number, first_name),
        )
    }

    type SendPoll = JsonRequest<payloads::SendPoll>;

    fn send_poll<C, Q, O>(&self, chat_id: C, question: Q, options: O) -> Self::SendPoll
    where
        C: Into<Recipient>,
        Q: Into<String>,
        O: IntoIterator<Item = InputPollOption>,
    {
        Self::SendPoll::new(self.clone(), payloads::SendPoll::new(chat_id, question, options))
    }

    type SendDice = JsonRequest<payloads::SendDice>;

    fn send_dice<C>(&self, chat_id: C) -> Self::SendDice
    where
        C: Into<Recipient>,
    {
        Self::SendDice::new(self.clone(), payloads::SendDice::new(chat_id))
    }

    type SendChatAction = JsonRequest<payloads::SendChatAction>;

    fn send_chat_action<C>(
        &self,
        chat_id: C,
        action: crate::types::ChatAction,
    ) -> Self::SendChatAction
    where
        C: Into<Recipient>,
    {
        Self::SendChatAction::new(self.clone(), payloads::SendChatAction::new(chat_id, action))
    }

    type SetMessageReaction = JsonRequest<payloads::SetMessageReaction>;

    fn set_message_reaction<C>(&self, chat_id: C, message_id: MessageId) -> Self::SetMessageReaction
    where
        C: Into<Recipient>,
    {
        Self::SetMessageReaction::new(
            self.clone(),
            payloads::SetMessageReaction::new(chat_id, message_id),
        )
    }

    type GetUserProfilePhotos = JsonRequest<payloads::GetUserProfilePhotos>;

    fn get_user_profile_photos(&self, user_id: UserId) -> Self::GetUserProfilePhotos {
        Self::GetUserProfilePhotos::new(self.clone(), payloads::GetUserProfilePhotos::new(user_id))
    }

    type GetFile = JsonRequest<payloads::GetFile>;

    fn get_file<F>(&self, file_id: F) -> Self::GetFile
    where
        F: Into<String>,
    {
        Self::GetFile::new(self.clone(), payloads::GetFile::new(file_id))
    }

    type KickChatMember = JsonRequest<payloads::KickChatMember>;

    fn kick_chat_member<C>(&self, chat_id: C, user_id: UserId) -> Self::KickChatMember
    where
        C: Into<Recipient>,
    {
        Self::KickChatMember::new(self.clone(), payloads::KickChatMember::new(chat_id, user_id))
    }

    type BanChatMember = JsonRequest<payloads::BanChatMember>;

    fn ban_chat_member<C>(&self, chat_id: C, user_id: UserId) -> Self::BanChatMember
    where
        C: Into<Recipient>,
    {
        Self::BanChatMember::new(self.clone(), payloads::BanChatMember::new(chat_id, user_id))
    }

    type UnbanChatMember = JsonRequest<payloads::UnbanChatMember>;

    fn unban_chat_member<C>(&self, chat_id: C, user_id: UserId) -> Self::UnbanChatMember
    where
        C: Into<Recipient>,
    {
        Self::UnbanChatMember::new(self.clone(), payloads::UnbanChatMember::new(chat_id, user_id))
    }

    type RestrictChatMember = JsonRequest<payloads::RestrictChatMember>;

    fn restrict_chat_member<C>(
        &self,
        chat_id: C,
        user_id: UserId,
        permissions: ChatPermissions,
    ) -> Self::RestrictChatMember
    where
        C: Into<Recipient>,
    {
        Self::RestrictChatMember::new(
            self.clone(),
            payloads::RestrictChatMember::new(chat_id, user_id, permissions),
        )
    }

    type PromoteChatMember = JsonRequest<payloads::PromoteChatMember>;

    fn promote_chat_member<C>(&self, chat_id: C, user_id: UserId) -> Self::PromoteChatMember
    where
        C: Into<Recipient>,
    {
        Self::PromoteChatMember::new(
            self.clone(),
            payloads::PromoteChatMember::new(chat_id, user_id),
        )
    }

    type SetChatAdministratorCustomTitle = JsonRequest<payloads::SetChatAdministratorCustomTitle>;

    fn set_chat_administrator_custom_title<Ch, Cu>(
        &self,
        chat_id: Ch,
        user_id: UserId,
        custom_title: Cu,
    ) -> Self::SetChatAdministratorCustomTitle
    where
        Ch: Into<Recipient>,
        Cu: Into<String>,
    {
        Self::SetChatAdministratorCustomTitle::new(
            self.clone(),
            payloads::SetChatAdministratorCustomTitle::new(chat_id, user_id, custom_title),
        )
    }

    type BanChatSenderChat = JsonRequest<payloads::BanChatSenderChat>;

    fn ban_chat_sender_chat<C, S>(&self, chat_id: C, sender_chat_id: S) -> Self::BanChatSenderChat
    where
        C: Into<Recipient>,
        S: Into<ChatId>,
    {
        Self::BanChatSenderChat::new(
            self.clone(),
            payloads::BanChatSenderChat::new(chat_id, sender_chat_id),
        )
    }

    type UnbanChatSenderChat = JsonRequest<payloads::UnbanChatSenderChat>;

    fn unban_chat_sender_chat<C, S>(
        &self,
        chat_id: C,
        sender_chat_id: S,
    ) -> Self::UnbanChatSenderChat
    where
        C: Into<Recipient>,
        S: Into<ChatId>,
    {
        Self::UnbanChatSenderChat::new(
            self.clone(),
            payloads::UnbanChatSenderChat::new(chat_id, sender_chat_id),
        )
    }

    type SetChatPermissions = JsonRequest<payloads::SetChatPermissions>;

    fn set_chat_permissions<C>(
        &self,
        chat_id: C,
        permissions: ChatPermissions,
    ) -> Self::SetChatPermissions
    where
        C: Into<Recipient>,
    {
        Self::SetChatPermissions::new(
            self.clone(),
            payloads::SetChatPermissions::new(chat_id, permissions),
        )
    }

    type ExportChatInviteLink = JsonRequest<payloads::ExportChatInviteLink>;

    fn export_chat_invite_link<C>(&self, chat_id: C) -> Self::ExportChatInviteLink
    where
        C: Into<Recipient>,
    {
        Self::ExportChatInviteLink::new(self.clone(), payloads::ExportChatInviteLink::new(chat_id))
    }

    type CreateChatInviteLink = JsonRequest<payloads::CreateChatInviteLink>;

    fn create_chat_invite_link<C>(&self, chat_id: C) -> Self::CreateChatInviteLink
    where
        C: Into<Recipient>,
    {
        Self::CreateChatInviteLink::new(self.clone(), payloads::CreateChatInviteLink::new(chat_id))
    }

    type EditChatInviteLink = JsonRequest<payloads::EditChatInviteLink>;

    fn edit_chat_invite_link<C, I>(&self, chat_id: C, invite_link: I) -> Self::EditChatInviteLink
    where
        C: Into<Recipient>,
        I: Into<String>,
    {
        Self::EditChatInviteLink::new(
            self.clone(),
            payloads::EditChatInviteLink::new(chat_id, invite_link),
        )
    }

    type RevokeChatInviteLink = JsonRequest<payloads::RevokeChatInviteLink>;

    fn revoke_chat_invite_link<C, I>(
        &self,
        chat_id: C,
        invite_link: I,
    ) -> Self::RevokeChatInviteLink
    where
        C: Into<Recipient>,
        I: Into<String>,
    {
        Self::RevokeChatInviteLink::new(
            self.clone(),
            payloads::RevokeChatInviteLink::new(chat_id, invite_link),
        )
    }

    type ApproveChatJoinRequest = JsonRequest<payloads::ApproveChatJoinRequest>;

    fn approve_chat_join_request<C>(
        &self,
        chat_id: C,
        user_id: UserId,
    ) -> Self::ApproveChatJoinRequest
    where
        C: Into<Recipient>,
    {
        Self::ApproveChatJoinRequest::new(
            self.clone(),
            payloads::ApproveChatJoinRequest::new(chat_id, user_id),
        )
    }

    type DeclineChatJoinRequest = JsonRequest<payloads::DeclineChatJoinRequest>;

    fn decline_chat_join_request<C>(
        &self,
        chat_id: C,
        user_id: UserId,
    ) -> Self::DeclineChatJoinRequest
    where
        C: Into<Recipient>,
    {
        Self::DeclineChatJoinRequest::new(
            self.clone(),
            payloads::DeclineChatJoinRequest::new(chat_id, user_id),
        )
    }

    type SetChatPhoto = MultipartRequest<payloads::SetChatPhoto>;

    fn set_chat_photo<C>(&self, chat_id: C, photo: InputFile) -> Self::SetChatPhoto
    where
        C: Into<Recipient>,
    {
        Self::SetChatPhoto::new(self.clone(), payloads::SetChatPhoto::new(chat_id, photo))
    }

    type DeleteChatPhoto = JsonRequest<payloads::DeleteChatPhoto>;

    fn delete_chat_photo<C>(&self, chat_id: C) -> Self::DeleteChatPhoto
    where
        C: Into<Recipient>,
    {
        Self::DeleteChatPhoto::new(self.clone(), payloads::DeleteChatPhoto::new(chat_id))
    }

    type SetChatTitle = JsonRequest<payloads::SetChatTitle>;

    fn set_chat_title<C, T>(&self, chat_id: C, title: T) -> Self::SetChatTitle
    where
        C: Into<Recipient>,
        T: Into<String>,
    {
        Self::SetChatTitle::new(self.clone(), payloads::SetChatTitle::new(chat_id, title))
    }

    type SetChatDescription = JsonRequest<payloads::SetChatDescription>;

    fn set_chat_description<C>(&self, chat_id: C) -> Self::SetChatDescription
    where
        C: Into<Recipient>,
    {
        Self::SetChatDescription::new(self.clone(), payloads::SetChatDescription::new(chat_id))
    }

    type PinChatMessage = JsonRequest<payloads::PinChatMessage>;

    fn pin_chat_message<C>(&self, chat_id: C, message_id: MessageId) -> Self::PinChatMessage
    where
        C: Into<Recipient>,
    {
        Self::PinChatMessage::new(self.clone(), payloads::PinChatMessage::new(chat_id, message_id))
    }

    type UnpinChatMessage = JsonRequest<payloads::UnpinChatMessage>;

    fn unpin_chat_message<C>(&self, chat_id: C) -> Self::UnpinChatMessage
    where
        C: Into<Recipient>,
    {
        Self::UnpinChatMessage::new(self.clone(), payloads::UnpinChatMessage::new(chat_id))
    }

    type LeaveChat = JsonRequest<payloads::LeaveChat>;

    fn leave_chat<C>(&self, chat_id: C) -> Self::LeaveChat
    where
        C: Into<Recipient>,
    {
        Self::LeaveChat::new(self.clone(), payloads::LeaveChat::new(chat_id))
    }

    type GetChat = JsonRequest<payloads::GetChat>;

    fn get_chat<C>(&self, chat_id: C) -> Self::GetChat
    where
        C: Into<Recipient>,
    {
        Self::GetChat::new(self.clone(), payloads::GetChat::new(chat_id))
    }

    type GetChatAdministrators = JsonRequest<payloads::GetChatAdministrators>;

    fn get_chat_administrators<C>(&self, chat_id: C) -> Self::GetChatAdministrators
    where
        C: Into<Recipient>,
    {
        Self::GetChatAdministrators::new(
            self.clone(),
            payloads::GetChatAdministrators::new(chat_id),
        )
    }

    type GetChatMembersCount = JsonRequest<payloads::GetChatMembersCount>;

    fn get_chat_members_count<C>(&self, chat_id: C) -> Self::GetChatMembersCount
    where
        C: Into<Recipient>,
    {
        Self::GetChatMembersCount::new(self.clone(), payloads::GetChatMembersCount::new(chat_id))
    }

    type GetChatMemberCount = JsonRequest<payloads::GetChatMemberCount>;

    fn get_chat_member_count<C>(&self, chat_id: C) -> Self::GetChatMemberCount
    where
        C: Into<Recipient>,
    {
        Self::GetChatMemberCount::new(self.clone(), payloads::GetChatMemberCount::new(chat_id))
    }

    type GetChatMember = JsonRequest<payloads::GetChatMember>;

    fn get_chat_member<C>(&self, chat_id: C, user_id: UserId) -> Self::GetChatMember
    where
        C: Into<Recipient>,
    {
        Self::GetChatMember::new(self.clone(), payloads::GetChatMember::new(chat_id, user_id))
    }

    type SetChatStickerSet = JsonRequest<payloads::SetChatStickerSet>;

    fn set_chat_sticker_set<C, S>(&self, chat_id: C, sticker_set_name: S) -> Self::SetChatStickerSet
    where
        C: Into<Recipient>,
        S: Into<String>,
    {
        Self::SetChatStickerSet::new(
            self.clone(),
            payloads::SetChatStickerSet::new(chat_id, sticker_set_name),
        )
    }

    type DeleteChatStickerSet = JsonRequest<payloads::DeleteChatStickerSet>;

    fn delete_chat_sticker_set<C>(&self, chat_id: C) -> Self::DeleteChatStickerSet
    where
        C: Into<Recipient>,
    {
        Self::DeleteChatStickerSet::new(self.clone(), payloads::DeleteChatStickerSet::new(chat_id))
    }

    type GetForumTopicIconStickers = JsonRequest<payloads::GetForumTopicIconStickers>;

    fn get_forum_topic_icon_stickers(&self) -> Self::GetForumTopicIconStickers {
        Self::GetForumTopicIconStickers::new(
            self.clone(),
            payloads::GetForumTopicIconStickers::new(),
        )
    }

    type CreateForumTopic = JsonRequest<payloads::CreateForumTopic>;

    fn create_forum_topic<C, N, I>(
        &self,
        chat_id: C,
        name: N,
        icon_color: Rgb,
        icon_custom_emoji_id: I,
    ) -> Self::CreateForumTopic
    where
        C: Into<Recipient>,
        N: Into<String>,
        I: Into<String>,
    {
        Self::CreateForumTopic::new(
            self.clone(),
            payloads::CreateForumTopic::new(chat_id, name, icon_color, icon_custom_emoji_id),
        )
    }

    type EditForumTopic = JsonRequest<payloads::EditForumTopic>;

    fn edit_forum_topic<C>(&self, chat_id: C, message_thread_id: ThreadId) -> Self::EditForumTopic
    where
        C: Into<Recipient>,
    {
        Self::EditForumTopic::new(
            self.clone(),
            payloads::EditForumTopic::new(chat_id, message_thread_id),
        )
    }

    type CloseForumTopic = JsonRequest<payloads::CloseForumTopic>;

    fn close_forum_topic<C>(&self, chat_id: C, message_thread_id: ThreadId) -> Self::CloseForumTopic
    where
        C: Into<Recipient>,
    {
        Self::CloseForumTopic::new(
            self.clone(),
            payloads::CloseForumTopic::new(chat_id, message_thread_id),
        )
    }

    type ReopenForumTopic = JsonRequest<payloads::ReopenForumTopic>;

    fn reopen_forum_topic<C>(
        &self,
        chat_id: C,
        message_thread_id: ThreadId,
    ) -> Self::ReopenForumTopic
    where
        C: Into<Recipient>,
    {
        Self::ReopenForumTopic::new(
            self.clone(),
            payloads::ReopenForumTopic::new(chat_id, message_thread_id),
        )
    }

    type DeleteForumTopic = JsonRequest<payloads::DeleteForumTopic>;

    fn delete_forum_topic<C>(
        &self,
        chat_id: C,
        message_thread_id: ThreadId,
    ) -> Self::DeleteForumTopic
    where
        C: Into<Recipient>,
    {
        Self::DeleteForumTopic::new(
            self.clone(),
            payloads::DeleteForumTopic::new(chat_id, message_thread_id),
        )
    }

    type UnpinAllForumTopicMessages = JsonRequest<payloads::UnpinAllForumTopicMessages>;

    fn unpin_all_forum_topic_messages<C>(
        &self,
        chat_id: C,
        message_thread_id: ThreadId,
    ) -> Self::UnpinAllForumTopicMessages
    where
        C: Into<Recipient>,
    {
        Self::UnpinAllForumTopicMessages::new(
            self.clone(),
            payloads::UnpinAllForumTopicMessages::new(chat_id, message_thread_id),
        )
    }

    type EditGeneralForumTopic = JsonRequest<payloads::EditGeneralForumTopic>;

    fn edit_general_forum_topic<C, N>(&self, chat_id: C, name: N) -> Self::EditGeneralForumTopic
    where
        C: Into<Recipient>,
        N: Into<String>,
    {
        Self::EditGeneralForumTopic::new(
            self.clone(),
            payloads::EditGeneralForumTopic::new(chat_id, name),
        )
    }

    type CloseGeneralForumTopic = JsonRequest<payloads::CloseGeneralForumTopic>;

    fn close_general_forum_topic<C>(&self, chat_id: C) -> Self::CloseGeneralForumTopic
    where
        C: Into<Recipient>,
    {
        Self::CloseGeneralForumTopic::new(
            self.clone(),
            payloads::CloseGeneralForumTopic::new(chat_id),
        )
    }

    type ReopenGeneralForumTopic = JsonRequest<payloads::ReopenGeneralForumTopic>;

    fn reopen_general_forum_topic<C>(&self, chat_id: C) -> Self::ReopenGeneralForumTopic
    where
        C: Into<Recipient>,
    {
        Self::ReopenGeneralForumTopic::new(
            self.clone(),
            payloads::ReopenGeneralForumTopic::new(chat_id),
        )
    }

    type HideGeneralForumTopic = JsonRequest<payloads::HideGeneralForumTopic>;

    fn hide_general_forum_topic<C>(&self, chat_id: C) -> Self::HideGeneralForumTopic
    where
        C: Into<Recipient>,
    {
        Self::HideGeneralForumTopic::new(
            self.clone(),
            payloads::HideGeneralForumTopic::new(chat_id),
        )
    }

    type UnhideGeneralForumTopic = JsonRequest<payloads::UnhideGeneralForumTopic>;

    fn unhide_general_forum_topic<C>(&self, chat_id: C) -> Self::UnhideGeneralForumTopic
    where
        C: Into<Recipient>,
    {
        Self::UnhideGeneralForumTopic::new(
            self.clone(),
            payloads::UnhideGeneralForumTopic::new(chat_id),
        )
    }

    type UnpinAllGeneralForumTopicMessages =
        JsonRequest<payloads::UnpinAllGeneralForumTopicMessages>;

    fn unpin_all_general_forum_topic_messages<C>(
        &self,
        chat_id: C,
    ) -> Self::UnpinAllGeneralForumTopicMessages
    where
        C: Into<Recipient>,
    {
        Self::UnpinAllGeneralForumTopicMessages::new(
            self.clone(),
            payloads::UnpinAllGeneralForumTopicMessages::new(chat_id),
        )
    }

    type AnswerCallbackQuery = JsonRequest<payloads::AnswerCallbackQuery>;

    fn answer_callback_query<C>(&self, callback_query_id: C) -> Self::AnswerCallbackQuery
    where
        C: Into<String>,
    {
        Self::AnswerCallbackQuery::new(
            self.clone(),
            payloads::AnswerCallbackQuery::new(callback_query_id),
        )
    }

    type GetUserChatBoosts = JsonRequest<payloads::GetUserChatBoosts>;

    fn get_user_chat_boosts<C>(&self, chat_id: C, user_id: UserId) -> Self::GetUserChatBoosts
    where
        C: Into<Recipient>,
    {
        Self::GetUserChatBoosts::new(
            self.clone(),
            payloads::GetUserChatBoosts::new(chat_id, user_id),
        )
    }

    type SetMyCommands = JsonRequest<payloads::SetMyCommands>;

    fn set_my_commands<C>(&self, commands: C) -> Self::SetMyCommands
    where
        C: IntoIterator<Item = BotCommand>,
    {
        Self::SetMyCommands::new(self.clone(), payloads::SetMyCommands::new(commands))
    }

    type GetBusinessConnection = JsonRequest<payloads::GetBusinessConnection>;

    fn get_business_connection(
        &self,
        business_connection_id: BusinessConnectionId,
    ) -> Self::GetBusinessConnection {
        Self::GetBusinessConnection::new(
            self.clone(),
            payloads::GetBusinessConnection::new(business_connection_id),
        )
    }

    type GetMyCommands = JsonRequest<payloads::GetMyCommands>;

    fn get_my_commands(&self) -> Self::GetMyCommands {
        Self::GetMyCommands::new(self.clone(), payloads::GetMyCommands::new())
    }

    type SetMyName = JsonRequest<payloads::SetMyName>;

    fn set_my_name(&self) -> Self::SetMyName {
        Self::SetMyName::new(self.clone(), payloads::SetMyName::new())
    }

    type GetMyName = JsonRequest<payloads::GetMyName>;

    fn get_my_name(&self) -> Self::GetMyName {
        Self::GetMyName::new(self.clone(), payloads::GetMyName::new())
    }

    type SetMyDescription = JsonRequest<payloads::SetMyDescription>;

    fn set_my_description(&self) -> Self::SetMyDescription {
        Self::SetMyDescription::new(self.clone(), payloads::SetMyDescription::new())
    }

    type GetMyDescription = JsonRequest<payloads::GetMyDescription>;

    fn get_my_description(&self) -> Self::GetMyDescription {
        Self::GetMyDescription::new(self.clone(), payloads::GetMyDescription::new())
    }

    type SetMyShortDescription = JsonRequest<payloads::SetMyShortDescription>;

    fn set_my_short_description(&self) -> Self::SetMyShortDescription {
        Self::SetMyShortDescription::new(self.clone(), payloads::SetMyShortDescription::new())
    }

    type GetMyShortDescription = JsonRequest<payloads::GetMyShortDescription>;
    fn get_my_short_description(&self) -> Self::GetMyShortDescription {
        Self::GetMyShortDescription::new(self.clone(), payloads::GetMyShortDescription::new())
    }

    type SetChatMenuButton = JsonRequest<payloads::SetChatMenuButton>;

    fn set_chat_menu_button(&self) -> Self::SetChatMenuButton {
        Self::SetChatMenuButton::new(self.clone(), payloads::SetChatMenuButton::new())
    }

    type GetChatMenuButton = JsonRequest<payloads::GetChatMenuButton>;

    fn get_chat_menu_button(&self) -> Self::GetChatMenuButton {
        Self::GetChatMenuButton::new(self.clone(), payloads::GetChatMenuButton::new())
    }

    type SetMyDefaultAdministratorRights = JsonRequest<payloads::SetMyDefaultAdministratorRights>;

    fn set_my_default_administrator_rights(&self) -> Self::SetMyDefaultAdministratorRights {
        Self::SetMyDefaultAdministratorRights::new(
            self.clone(),
            payloads::SetMyDefaultAdministratorRights::new(),
        )
    }

    type GetMyDefaultAdministratorRights = JsonRequest<payloads::GetMyDefaultAdministratorRights>;

    fn get_my_default_administrator_rights(&self) -> Self::GetMyDefaultAdministratorRights {
        Self::GetMyDefaultAdministratorRights::new(
            self.clone(),
            payloads::GetMyDefaultAdministratorRights::new(),
        )
    }

    type DeleteMyCommands = JsonRequest<payloads::DeleteMyCommands>;

    fn delete_my_commands(&self) -> Self::DeleteMyCommands {
        Self::DeleteMyCommands::new(self.clone(), payloads::DeleteMyCommands::new())
    }

    type AnswerInlineQuery = JsonRequest<payloads::AnswerInlineQuery>;

    fn answer_inline_query<I, R>(&self, inline_query_id: I, results: R) -> Self::AnswerInlineQuery
    where
        I: Into<String>,
        R: IntoIterator<Item = InlineQueryResult>,
    {
        Self::AnswerInlineQuery::new(
            self.clone(),
            payloads::AnswerInlineQuery::new(inline_query_id, results),
        )
    }

    type AnswerWebAppQuery = JsonRequest<payloads::AnswerWebAppQuery>;

    fn answer_web_app_query<W>(
        &self,
        web_app_query_id: W,
        result: InlineQueryResult,
    ) -> Self::AnswerWebAppQuery
    where
        W: Into<String>,
    {
        Self::AnswerWebAppQuery::new(
            self.clone(),
            payloads::AnswerWebAppQuery::new(web_app_query_id, result),
        )
    }

    type EditMessageText = JsonRequest<payloads::EditMessageText>;

    fn edit_message_text<C, T>(
        &self,
        chat_id: C,
        message_id: MessageId,
        text: T,
    ) -> Self::EditMessageText
    where
        C: Into<Recipient>,
        T: Into<String>,
    {
        Self::EditMessageText::new(
            self.clone(),
            payloads::EditMessageText::new(chat_id, message_id, text),
        )
    }

    type EditMessageTextInline = JsonRequest<payloads::EditMessageTextInline>;

    fn edit_message_text_inline<I, T>(
        &self,
        inline_message_id: I,
        text: T,
    ) -> Self::EditMessageTextInline
    where
        I: Into<String>,
        T: Into<String>,
    {
        Self::EditMessageTextInline::new(
            self.clone(),
            payloads::EditMessageTextInline::new(inline_message_id, text),
        )
    }

    type EditMessageCaption = JsonRequest<payloads::EditMessageCaption>;

    fn edit_message_caption<C>(&self, chat_id: C, message_id: MessageId) -> Self::EditMessageCaption
    where
        C: Into<Recipient>,
    {
        Self::EditMessageCaption::new(
            self.clone(),
            payloads::EditMessageCaption::new(chat_id, message_id),
        )
    }

    type EditMessageCaptionInline = JsonRequest<payloads::EditMessageCaptionInline>;

    fn edit_message_caption_inline<I>(&self, inline_message_id: I) -> Self::EditMessageCaptionInline
    where
        I: Into<String>,
    {
        Self::EditMessageCaptionInline::new(
            self.clone(),
            payloads::EditMessageCaptionInline::new(inline_message_id),
        )
    }

    type EditMessageMedia = MultipartRequest<payloads::EditMessageMedia>;

    fn edit_message_media<C>(
        &self,
        chat_id: C,
        message_id: MessageId,
        media: InputMedia,
    ) -> Self::EditMessageMedia
    where
        C: Into<Recipient>,
    {
        Self::EditMessageMedia::new(
            self.clone(),
            payloads::EditMessageMedia::new(chat_id, message_id, media),
        )
    }

    type EditMessageMediaInline = MultipartRequest<payloads::EditMessageMediaInline>;

    fn edit_message_media_inline<I>(
        &self,
        inline_message_id: I,
        media: InputMedia,
    ) -> Self::EditMessageMediaInline
    where
        I: Into<String>,
    {
        Self::EditMessageMediaInline::new(
            self.clone(),
            payloads::EditMessageMediaInline::new(inline_message_id, media),
        )
    }

    type EditMessageReplyMarkup = JsonRequest<payloads::EditMessageReplyMarkup>;

    fn edit_message_reply_markup<C>(
        &self,
        chat_id: C,
        message_id: MessageId,
    ) -> Self::EditMessageReplyMarkup
    where
        C: Into<Recipient>,
    {
        Self::EditMessageReplyMarkup::new(
            self.clone(),
            payloads::EditMessageReplyMarkup::new(chat_id, message_id),
        )
    }

    type EditMessageReplyMarkupInline = JsonRequest<payloads::EditMessageReplyMarkupInline>;

    fn edit_message_reply_markup_inline<I>(
        &self,
        inline_message_id: I,
    ) -> Self::EditMessageReplyMarkupInline
    where
        I: Into<String>,
    {
        Self::EditMessageReplyMarkupInline::new(
            self.clone(),
            payloads::EditMessageReplyMarkupInline::new(inline_message_id),
        )
    }

    type StopPoll = JsonRequest<payloads::StopPoll>;

    fn stop_poll<C>(&self, chat_id: C, message_id: MessageId) -> Self::StopPoll
    where
        C: Into<Recipient>,
    {
        Self::StopPoll::new(self.clone(), payloads::StopPoll::new(chat_id, message_id))
    }

    type DeleteMessage = JsonRequest<payloads::DeleteMessage>;

    fn delete_message<C>(&self, chat_id: C, message_id: MessageId) -> Self::DeleteMessage
    where
        C: Into<Recipient>,
    {
        Self::DeleteMessage::new(self.clone(), payloads::DeleteMessage::new(chat_id, message_id))
    }

    type DeleteMessages = JsonRequest<payloads::DeleteMessages>;
    fn delete_messages<C, M>(&self, chat_id: C, message_ids: M) -> Self::DeleteMessages
    where
        C: Into<Recipient>,
        M: IntoIterator<Item = MessageId>,
    {
        Self::DeleteMessages::new(self.clone(), payloads::DeleteMessages::new(chat_id, message_ids))
    }

    type SendSticker = MultipartRequest<payloads::SendSticker>;

    fn send_sticker<C>(&self, chat_id: C, sticker: InputFile) -> Self::SendSticker
    where
        C: Into<Recipient>,
    {
        Self::SendSticker::new(self.clone(), payloads::SendSticker::new(chat_id, sticker))
    }

    type GetStickerSet = JsonRequest<payloads::GetStickerSet>;

    fn get_sticker_set<N>(&self, name: N) -> Self::GetStickerSet
    where
        N: Into<String>,
    {
        Self::GetStickerSet::new(self.clone(), payloads::GetStickerSet::new(name))
    }

    type GetCustomEmojiStickers = JsonRequest<payloads::GetCustomEmojiStickers>;

    fn get_custom_emoji_stickers<C>(&self, custom_emoji_ids: C) -> Self::GetCustomEmojiStickers
    where
        C: IntoIterator<Item = String>,
    {
        Self::GetCustomEmojiStickers::new(
            self.clone(),
            payloads::GetCustomEmojiStickers::new(custom_emoji_ids),
        )
    }

    type UploadStickerFile = MultipartRequest<payloads::UploadStickerFile>;

    fn upload_sticker_file(
        &self,
        user_id: UserId,
        sticker: InputFile,
        sticker_format: StickerFormat,
    ) -> Self::UploadStickerFile {
        Self::UploadStickerFile::new(
            self.clone(),
            payloads::UploadStickerFile::new(user_id, sticker, sticker_format),
        )
    }

    type CreateNewStickerSet = MultipartRequest<payloads::CreateNewStickerSet>;

    fn create_new_sticker_set<N, T, S>(
        &self,
        user_id: UserId,
        name: N,
        title: T,
        stickers: S,
    ) -> Self::CreateNewStickerSet
    where
        N: Into<String>,
        T: Into<String>,
        S: IntoIterator<Item = InputSticker>,
    {
        Self::CreateNewStickerSet::new(
            self.clone(),
            payloads::CreateNewStickerSet::new(user_id, name, title, stickers),
        )
    }

    type AddStickerToSet = MultipartRequest<payloads::AddStickerToSet>;

    fn add_sticker_to_set<N>(
        &self,
        user_id: UserId,
        name: N,
        sticker: InputSticker,
    ) -> Self::AddStickerToSet
    where
        N: Into<String>,
    {
        Self::AddStickerToSet::new(
            self.clone(),
            payloads::AddStickerToSet::new(user_id, name, sticker),
        )
    }

    type SetStickerPositionInSet = JsonRequest<payloads::SetStickerPositionInSet>;

    fn set_sticker_position_in_set<S>(
        &self,
        sticker: S,
        position: u32,
    ) -> Self::SetStickerPositionInSet
    where
        S: Into<String>,
    {
        Self::SetStickerPositionInSet::new(
            self.clone(),
            payloads::SetStickerPositionInSet::new(sticker, position),
        )
    }

    type DeleteStickerFromSet = JsonRequest<payloads::DeleteStickerFromSet>;

    fn delete_sticker_from_set<S>(&self, sticker: S) -> Self::DeleteStickerFromSet
    where
        S: Into<String>,
    {
        Self::DeleteStickerFromSet::new(self.clone(), payloads::DeleteStickerFromSet::new(sticker))
    }

    type ReplaceStickerInSet = JsonRequest<payloads::ReplaceStickerInSet>;

    fn replace_sticker_in_set<N, O>(
        &self,
        user_id: UserId,
        name: N,
        old_sticker: O,
        sticker: InputSticker,
    ) -> Self::ReplaceStickerInSet
    where
        N: Into<String>,
        O: Into<String>,
    {
        Self::ReplaceStickerInSet::new(
            self.clone(),
            payloads::ReplaceStickerInSet {
                user_id,
                name: name.into(),
                old_sticker: old_sticker.into(),
                sticker,
            },
        )
    }

    type SetStickerSetThumbnail = MultipartRequest<payloads::SetStickerSetThumbnail>;

    fn set_sticker_set_thumbnail<N>(
        &self,
        name: N,
        user_id: UserId,
        format: StickerFormat,
    ) -> Self::SetStickerSetThumbnail
    where
        N: Into<String>,
    {
        Self::SetStickerSetThumbnail::new(
            self.clone(),
            payloads::SetStickerSetThumbnail::new(name, user_id, format),
        )
    }

    type SetCustomEmojiStickerSetThumbnail =
        JsonRequest<payloads::SetCustomEmojiStickerSetThumbnail>;

    fn set_custom_emoji_sticker_set_thumbnail<N>(
        &self,
        name: N,
    ) -> Self::SetCustomEmojiStickerSetThumbnail
    where
        N: Into<String>,
    {
        Self::SetCustomEmojiStickerSetThumbnail::new(
            self.clone(),
            payloads::SetCustomEmojiStickerSetThumbnail::new(name),
        )
    }

    type SetStickerSetTitle = JsonRequest<payloads::SetStickerSetTitle>;

    fn set_sticker_set_title<N, T>(&self, name: N, title: T) -> Self::SetStickerSetTitle
    where
        N: Into<String>,
        T: Into<String>,
    {
        Self::SetStickerSetTitle::new(self.clone(), payloads::SetStickerSetTitle::new(name, title))
    }

    type DeleteStickerSet = JsonRequest<payloads::DeleteStickerSet>;

    fn delete_sticker_set<N>(&self, name: N) -> Self::DeleteStickerSet
    where
        N: Into<String>,
    {
        Self::DeleteStickerSet::new(self.clone(), payloads::DeleteStickerSet::new(name))
    }

    type SetStickerEmojiList = JsonRequest<payloads::SetStickerEmojiList>;

    fn set_sticker_emoji_list<S, E>(&self, sticker: S, emoji_list: E) -> Self::SetStickerEmojiList
    where
        S: Into<String>,
        E: IntoIterator<Item = String>,
    {
        Self::SetStickerEmojiList::new(
            self.clone(),
            payloads::SetStickerEmojiList::new(sticker, emoji_list),
        )
    }

    type SetStickerKeywords = JsonRequest<payloads::SetStickerKeywords>;

    fn set_sticker_keywords<S>(&self, sticker: S) -> Self::SetStickerKeywords
    where
        S: Into<String>,
    {
        Self::SetStickerKeywords::new(self.clone(), payloads::SetStickerKeywords::new(sticker))
    }

    type SetStickerMaskPosition = JsonRequest<payloads::SetStickerMaskPosition>;

    fn set_sticker_mask_position<S>(&self, sticker: S) -> Self::SetStickerMaskPosition
    where
        S: Into<String>,
    {
        Self::SetStickerMaskPosition::new(
            self.clone(),
            payloads::SetStickerMaskPosition::new(sticker),
        )
    }

    type SendInvoice = JsonRequest<payloads::SendInvoice>;

    fn send_invoice<Ch, T, D, Pa, P, C, Pri>(
        &self,
        chat_id: Ch,
        title: T,
        description: D,
        payload: Pa,
        provider_token: P,
        currency: C,
        prices: Pri,
    ) -> Self::SendInvoice
    where
        Ch: Into<Recipient>,
        T: Into<String>,
        D: Into<String>,
        Pa: Into<String>,
        P: Into<String>,
        C: Into<String>,
        Pri: IntoIterator<Item = LabeledPrice>,
    {
        Self::SendInvoice::new(
            self.clone(),
            payloads::SendInvoice::new(
                chat_id,
                title,
                description,
                payload,
                provider_token,
                currency,
                prices,
            ),
        )
    }

    type CreateInvoiceLink = JsonRequest<payloads::CreateInvoiceLink>;

    fn create_invoice_link<T, D, Pa, P, C, Pri>(
        &self,
        title: T,
        description: D,
        payload: Pa,
        provider_token: P,
        currency: C,
        prices: Pri,
    ) -> Self::CreateInvoiceLink
    where
        T: Into<String>,
        D: Into<String>,
        Pa: Into<String>,
        P: Into<String>,
        C: Into<String>,
        Pri: IntoIterator<Item = LabeledPrice>,
    {
        Self::CreateInvoiceLink::new(
            self.clone(),
            payloads::CreateInvoiceLink::new(
                title,
                description,
                payload,
                provider_token,
                currency,
                prices,
            ),
        )
    }

    type AnswerShippingQuery = JsonRequest<payloads::AnswerShippingQuery>;

    fn answer_shipping_query<S>(&self, shipping_query_id: S, ok: bool) -> Self::AnswerShippingQuery
    where
        S: Into<String>,
    {
        Self::AnswerShippingQuery::new(
            self.clone(),
            payloads::AnswerShippingQuery::new(shipping_query_id, ok),
        )
    }

    type AnswerPreCheckoutQuery = JsonRequest<payloads::AnswerPreCheckoutQuery>;

    fn answer_pre_checkout_query<P>(
        &self,
        pre_checkout_query_id: P,
        ok: bool,
    ) -> Self::AnswerPreCheckoutQuery
    where
        P: Into<String>,
    {
        Self::AnswerPreCheckoutQuery::new(
            self.clone(),
            payloads::AnswerPreCheckoutQuery::new(pre_checkout_query_id, ok),
        )
    }

    type SetPassportDataErrors = JsonRequest<payloads::SetPassportDataErrors>;

    fn set_passport_data_errors<E>(&self, user_id: UserId, errors: E) -> Self::SetPassportDataErrors
    where
        E: IntoIterator<Item = crate::types::PassportElementError>,
    {
        Self::SetPassportDataErrors::new(
            self.clone(),
            payloads::SetPassportDataErrors::new(user_id, errors),
        )
    }

    type SendGame = JsonRequest<payloads::SendGame>;

    fn send_game<C, G>(&self, chat_id: C, game_short_name: G) -> Self::SendGame
    where
        C: Into<ChatId>,
        G: Into<String>,
    {
        Self::SendGame::new(self.clone(), payloads::SendGame::new(chat_id, game_short_name))
    }

    type SetGameScore = JsonRequest<payloads::SetGameScore>;

    fn set_game_score(
        &self,
        user_id: UserId,
        score: u64,
        chat_id: u32,
        message_id: MessageId,
    ) -> Self::SetGameScore {
        Self::SetGameScore::new(
            self.clone(),
            payloads::SetGameScore::new(user_id, score, chat_id, message_id),
        )
    }

    type SetGameScoreInline = JsonRequest<payloads::SetGameScoreInline>;

    fn set_game_score_inline<I>(
        &self,
        user_id: UserId,
        score: u64,
        inline_message_id: I,
    ) -> Self::SetGameScoreInline
    where
        I: Into<String>,
    {
        Self::SetGameScoreInline::new(
            self.clone(),
            payloads::SetGameScoreInline::new(user_id, score, inline_message_id),
        )
    }

    type GetGameHighScores = JsonRequest<payloads::GetGameHighScores>;

    fn get_game_high_scores<T>(&self, user_id: UserId, target: T) -> Self::GetGameHighScores
    where
        T: Into<crate::types::TargetMessage>,
    {
        Self::GetGameHighScores::new(
            self.clone(),
            payloads::GetGameHighScores::new(user_id, target),
        )
    }

    type LogOut = JsonRequest<payloads::LogOut>;

    fn log_out(&self) -> Self::LogOut {
        Self::LogOut::new(self.clone(), payloads::LogOut::new())
    }

    type Close = JsonRequest<payloads::Close>;

    fn close(&self) -> Self::Close {
        Self::Close::new(self.clone(), payloads::Close::new())
    }

    type CopyMessage = JsonRequest<payloads::CopyMessage>;

    fn copy_message<C, F>(
        &self,
        chat_id: C,
        from_chat_id: F,
        message_id: MessageId,
    ) -> Self::CopyMessage
    where
        C: Into<Recipient>,
        F: Into<Recipient>,
    {
        Self::CopyMessage::new(
            self.clone(),
            payloads::CopyMessage::new(chat_id, from_chat_id, message_id),
        )
    }

    type CopyMessages = JsonRequest<payloads::CopyMessages>;
    fn copy_messages<C, F, M>(
        &self,
        chat_id: C,
        from_chat_id: F,
        message_ids: M,
    ) -> Self::CopyMessages
    where
        C: Into<Recipient>,
        F: Into<Recipient>,
        M: IntoIterator<Item = MessageId>,
    {
        Self::CopyMessages::new(
            self.clone(),
            payloads::CopyMessages::new(chat_id, from_chat_id, message_ids),
        )
    }

    type UnpinAllChatMessages = JsonRequest<payloads::UnpinAllChatMessages>;

    fn unpin_all_chat_messages<C>(&self, chat_id: C) -> Self::UnpinAllChatMessages
    where
        C: Into<Recipient>,
    {
        Self::UnpinAllChatMessages::new(self.clone(), payloads::UnpinAllChatMessages::new(chat_id))
    }
}
