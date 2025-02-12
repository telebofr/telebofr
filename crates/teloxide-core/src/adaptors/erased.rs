use std::{future::IntoFuture, sync::Arc};

use futures::{future::BoxFuture, FutureExt};
use reqwest::Url;

use crate::{
    errors::AsResponseParameters,
    payloads::*,
    requests::{HasPayload, Output, Payload, Request, Requester},
    types::*,
};

/// [`Requester`] with erased type.
pub struct ErasedRequester<'a, E> {
    inner: Arc<dyn ErasableRequester<'a, Err = E> + 'a>,
}

impl<'a, E> ErasedRequester<'a, E> {
    /// Erases type of `requester`
    ///
    /// Note: it's recommended to use [`RequesterExt::erase`] instead.
    ///
    /// [`RequesterExt::erase`]: crate::requests::RequesterExt::erase
    pub fn new<B>(requester: B) -> Self
    where
        B: Requester<Err = E> + 'a,
    {
        Self { inner: Arc::new(requester) }
    }
}

impl<E> std::fmt::Debug for ErasedRequester<'_, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ErasedRequester").finish_non_exhaustive()
    }
}

// NB. hand-written impl to avoid `E: Clone` bound
impl<E> Clone for ErasedRequester<'_, E> {
    fn clone(&self) -> Self {
        Self { inner: Arc::clone(&self.inner) }
    }
}

/// [`Request`] with erased type.
#[must_use = "Requests are lazy and do nothing unless sent"]
pub struct ErasedRequest<'a, T, E> {
    inner: Box<dyn ErasableRequest<'a, Payload = T, Err = E> + 'a>,
}

// `T: Payload` required b/c of <https://github.com/rust-lang/rust/issues/102185>
impl<'a, T: Payload, E> ErasedRequest<'a, T, E> {
    pub(crate) fn erase(request: impl Request<Payload = T, Err = E> + 'a) -> Self {
        Self { inner: Box::new(request) }
    }
}

impl<T, E> HasPayload for ErasedRequest<'_, T, E>
where
    T: Payload,
{
    type Payload = T;

    fn payload_mut(&mut self) -> &mut Self::Payload {
        self.inner.payload_mut()
    }

    fn payload_ref(&self) -> &Self::Payload {
        self.inner.payload_ref()
    }
}

impl<'a, T, E> Request for ErasedRequest<'a, T, E>
where
    T: Payload,
    E: std::error::Error + Send,
{
    type Err = E;

    type Send = BoxFuture<'a, Result<Output<Self>, Self::Err>>;

    type SendRef = BoxFuture<'a, Result<Output<Self>, Self::Err>>;

    fn send(self) -> Self::Send {
        self.inner.send_box()
    }

    fn send_ref(&self) -> Self::SendRef {
        self.inner.send_ref()
    }
}

impl<T, E> IntoFuture for ErasedRequest<'_, T, E>
where
    T: Payload,
    E: std::error::Error + Send,
{
    type Output = Result<Output<Self>, <Self as Request>::Err>;
    type IntoFuture = <Self as Request>::Send;

    fn into_future(self) -> Self::IntoFuture {
        self.send()
    }
}

/// Object safe version of [`Request`].
///
/// TODO(waffle): make [`Request`] object safe and remove this trait (this is a
/// breaking change)
trait ErasableRequest<'a>: HasPayload {
    type Err: std::error::Error + Send;

    fn send_box(self: Box<Self>) -> BoxFuture<'a, Result<Output<Self>, Self::Err>>;

    fn send_ref(&self) -> BoxFuture<'a, Result<Output<Self>, Self::Err>>;
}

impl<'a, R> ErasableRequest<'a> for R
where
    R: Request,
    <R as Request>::Send: 'a,
    <R as Request>::SendRef: 'a,
{
    type Err = R::Err;

    fn send_box(self: Box<Self>) -> BoxFuture<'a, Result<Output<Self>, Self::Err>> {
        self.send().boxed()
    }

    fn send_ref(&self) -> BoxFuture<'a, Result<Output<Self>, Self::Err>> {
        Request::send_ref(self).boxed()
    }
}

macro_rules! fty {
    ($T:ident) => {
        ErasedRequest<'a, $T, Err>
    };
}

macro_rules! fwd_erased {
    ($m:ident $this:ident ($($arg:ident : $T:ty),*)) => {
        $this.inner.$m($( fwd_erased!(@convert $m, $arg, $arg : $T) ),*)
    };

    (@convert send_media_group, $arg:ident, media : $T:ty) => {
        $arg.into_iter().collect()
    };
    (@convert $m:ident, $arg:ident, options : $T:ty) => {
        $arg.into_iter().collect()
    };
    (@convert $m:ident, $arg:ident, commands : $T:ty) => {
        $arg.into_iter().collect()
    };
    (@convert $m:ident, $arg:ident, results : $T:ty) => {
        $arg.into_iter().collect()
    };
    (@convert $m:ident, $arg:ident, prices : $T:ty) => {
        $arg.into_iter().collect()
    };
    (@convert $m:ident, $arg:ident, errors : $T:ty) => {
        $arg.into_iter().collect()
    };
    (@convert $m:ident, $arg:ident, custom_emoji_ids : $T:ty) => {
        $arg.into_iter().collect()
    };
    (@convert $m:ident, $arg:ident, stickers: $T:ty) => {
        $arg.into_iter().collect()
    };
    (@convert $m:ident, $arg:ident, emoji_list: $T:ty) => {
        $arg.into_iter().collect()
    };
    (@convert $m:ident, $arg:ident, message_ids: $T:ty) => {
        $arg.into_iter().collect()
    };
    (@convert $m:ident, $arg:ident, $arg_:ident : $T:ty) => {
        $arg.into()
    };
}

impl<'a, Err> Requester for ErasedRequester<'a, Err>
where
    Err: std::error::Error + Send + AsResponseParameters,
{
    type Err = Err;

    requester_forward! {
        get_me,
        log_out,
        close,
        get_updates,
        set_webhook,
        delete_webhook,
        get_webhook_info,
        forward_message,
        forward_messages,
        copy_message,
        copy_messages,
        send_message,
        send_photo,
        send_audio,
        send_document,
        send_video,
        send_animation,
        send_voice,
        send_video_note,
        send_media_group,
        send_location,
        edit_message_live_location,
        edit_message_live_location_inline,
        stop_message_live_location,
        stop_message_live_location_inline,
        send_venue,
        send_contact,
        send_poll,
        send_dice,
        send_chat_action,
        set_message_reaction,
        get_user_profile_photos,
        get_file,
        kick_chat_member,
        ban_chat_member,
        unban_chat_member,
        restrict_chat_member,
        promote_chat_member,
        set_chat_administrator_custom_title,
        ban_chat_sender_chat,
        unban_chat_sender_chat,
        set_chat_permissions,
        export_chat_invite_link,
        create_chat_invite_link,
        edit_chat_invite_link,
        revoke_chat_invite_link,
        set_chat_photo,
        delete_chat_photo,
        set_chat_title,
        set_chat_description,
        pin_chat_message,
        unpin_chat_message,
        unpin_all_chat_messages,
        leave_chat,
        get_chat,
        get_chat_administrators,
        get_chat_members_count,
        get_chat_member_count,
        get_chat_member,
        set_chat_sticker_set,
        delete_chat_sticker_set,
        get_forum_topic_icon_stickers,
        create_forum_topic,
        edit_forum_topic,
        close_forum_topic,
        reopen_forum_topic,
        delete_forum_topic,
        unpin_all_forum_topic_messages,
        edit_general_forum_topic,
        close_general_forum_topic,
        reopen_general_forum_topic,
        hide_general_forum_topic,
        unhide_general_forum_topic,
        unpin_all_general_forum_topic_messages,
        answer_callback_query,
        get_user_chat_boosts,
        set_my_commands,
        get_business_connection,
        get_my_commands,
        set_my_name,
        get_my_name,
        set_my_description,
        get_my_description,
        set_my_short_description,
        get_my_short_description,
        set_chat_menu_button,
        get_chat_menu_button,
        set_my_default_administrator_rights,
        get_my_default_administrator_rights,
        delete_my_commands,
        answer_inline_query,
        answer_web_app_query,
        edit_message_text,
        edit_message_text_inline,
        edit_message_caption,
        edit_message_caption_inline,
        edit_message_media,
        edit_message_media_inline,
        edit_message_reply_markup,
        edit_message_reply_markup_inline,
        stop_poll,
        delete_message,
        delete_messages,
        send_sticker,
        get_sticker_set,
        get_custom_emoji_stickers,
        upload_sticker_file,
        create_new_sticker_set,
        add_sticker_to_set,
        set_sticker_position_in_set,
        delete_sticker_from_set,
        replace_sticker_in_set,
        set_sticker_set_thumbnail,
        set_custom_emoji_sticker_set_thumbnail,
        set_sticker_set_title,
        delete_sticker_set,
        set_sticker_emoji_list,
        set_sticker_keywords,
        set_sticker_mask_position,
        send_invoice,
        create_invoice_link,
        answer_shipping_query,
        answer_pre_checkout_query,
        set_passport_data_errors,
        send_game,
        set_game_score,
        set_game_score_inline,
        get_game_high_scores,
        approve_chat_join_request,
        decline_chat_join_request
        => fwd_erased, fty
    }
}

/// Object safe version of [`Requester`].
trait ErasableRequester<'a> {
    /// Error type returned by all requests.
    type Err: std::error::Error + Send;

    fn get_updates(&self) -> ErasedRequest<'a, GetUpdates, Self::Err>;

    fn set_webhook(&self, url: Url) -> ErasedRequest<'a, SetWebhook, Self::Err>;

    fn delete_webhook(&self) -> ErasedRequest<'a, DeleteWebhook, Self::Err>;

    fn get_webhook_info(&self) -> ErasedRequest<'a, GetWebhookInfo, Self::Err>;

    fn get_me(&self) -> ErasedRequest<'a, GetMe, Self::Err>;

    fn log_out(&self) -> ErasedRequest<'a, LogOut, Self::Err>;

    fn close(&self) -> ErasedRequest<'a, Close, Self::Err>;

    fn send_message(
        &self,
        chat_id: Recipient,
        text: String,
    ) -> ErasedRequest<'a, SendMessage, Self::Err>;

    fn forward_message(
        &self,
        chat_id: Recipient,
        from_chat_id: Recipient,
        message_id: MessageId,
    ) -> ErasedRequest<'a, ForwardMessage, Self::Err>;

    fn forward_messages(
        &self,
        chat_id: Recipient,
        from_chat_id: Recipient,
        message_ids: Vec<MessageId>,
    ) -> ErasedRequest<'a, ForwardMessages, Self::Err>;

    fn copy_message(
        &self,
        chat_id: Recipient,
        from_chat_id: Recipient,
        message_id: MessageId,
    ) -> ErasedRequest<'a, CopyMessage, Self::Err>;

    fn copy_messages(
        &self,
        chat_id: Recipient,
        from_chat_id: Recipient,
        message_ids: Vec<MessageId>,
    ) -> ErasedRequest<'a, CopyMessages, Self::Err>;

    fn send_photo(
        &self,
        chat_id: Recipient,
        photo: InputFile,
    ) -> ErasedRequest<'a, SendPhoto, Self::Err>;

    fn send_audio(
        &self,
        chat_id: Recipient,
        audio: InputFile,
    ) -> ErasedRequest<'a, SendAudio, Self::Err>;

    fn send_document(
        &self,
        chat_id: Recipient,
        document: InputFile,
    ) -> ErasedRequest<'a, SendDocument, Self::Err>;

    fn send_video(
        &self,
        chat_id: Recipient,
        video: InputFile,
    ) -> ErasedRequest<'a, SendVideo, Self::Err>;

    fn send_animation(
        &self,
        chat_id: Recipient,
        animation: InputFile,
    ) -> ErasedRequest<'a, SendAnimation, Self::Err>;

    fn send_voice(
        &self,
        chat_id: Recipient,
        voice: InputFile,
    ) -> ErasedRequest<'a, SendVoice, Self::Err>;

    fn send_video_note(
        &self,
        chat_id: Recipient,
        video_note: InputFile,
    ) -> ErasedRequest<'a, SendVideoNote, Self::Err>;

    fn send_media_group(
        &self,
        chat_id: Recipient,
        media: Vec<InputMedia>,
    ) -> ErasedRequest<'a, SendMediaGroup, Self::Err>;

    fn send_location(
        &self,
        chat_id: Recipient,
        latitude: f64,
        longitude: f64,
    ) -> ErasedRequest<'a, SendLocation, Self::Err>;

    fn edit_message_live_location(
        &self,
        chat_id: Recipient,
        message_id: MessageId,
        latitude: f64,
        longitude: f64,
    ) -> ErasedRequest<'a, EditMessageLiveLocation, Self::Err>;

    fn edit_message_live_location_inline(
        &self,
        inline_message_id: String,
        latitude: f64,
        longitude: f64,
    ) -> ErasedRequest<'a, EditMessageLiveLocationInline, Self::Err>;

    fn stop_message_live_location(
        &self,
        chat_id: Recipient,
        message_id: MessageId,
    ) -> ErasedRequest<'a, StopMessageLiveLocation, Self::Err>;

    fn stop_message_live_location_inline(
        &self,
        inline_message_id: String,
    ) -> ErasedRequest<'a, StopMessageLiveLocationInline, Self::Err>;

    fn send_venue(
        &self,
        chat_id: Recipient,
        latitude: f64,
        longitude: f64,
        title: String,
        address: String,
    ) -> ErasedRequest<'a, SendVenue, Self::Err>;

    fn send_contact(
        &self,
        chat_id: Recipient,
        phone_number: String,
        first_name: String,
    ) -> ErasedRequest<'a, SendContact, Self::Err>;

    fn send_poll(
        &self,
        chat_id: Recipient,
        question: String,
        options: Vec<InputPollOption>,
    ) -> ErasedRequest<'a, SendPoll, Self::Err>;

    fn send_dice(&self, chat_id: Recipient) -> ErasedRequest<'a, SendDice, Self::Err>;

    fn send_chat_action(
        &self,
        chat_id: Recipient,
        action: ChatAction,
    ) -> ErasedRequest<'a, SendChatAction, Self::Err>;

    fn set_message_reaction(
        &self,
        chat_id: Recipient,
        message_id: MessageId,
    ) -> ErasedRequest<'a, SetMessageReaction, Self::Err>;

    fn get_user_profile_photos(
        &self,
        user_id: UserId,
    ) -> ErasedRequest<'a, GetUserProfilePhotos, Self::Err>;

    fn get_file(&self, file_id: String) -> ErasedRequest<'a, GetFile, Self::Err>;

    fn ban_chat_member(
        &self,
        chat_id: Recipient,
        user_id: UserId,
    ) -> ErasedRequest<'a, BanChatMember, Self::Err>;

    fn kick_chat_member(
        &self,
        chat_id: Recipient,
        user_id: UserId,
    ) -> ErasedRequest<'a, KickChatMember, Self::Err>;

    fn unban_chat_member(
        &self,
        chat_id: Recipient,
        user_id: UserId,
    ) -> ErasedRequest<'a, UnbanChatMember, Self::Err>;

    fn restrict_chat_member(
        &self,
        chat_id: Recipient,
        user_id: UserId,
        permissions: ChatPermissions,
    ) -> ErasedRequest<'a, RestrictChatMember, Self::Err>;

    fn promote_chat_member(
        &self,
        chat_id: Recipient,
        user_id: UserId,
    ) -> ErasedRequest<'a, PromoteChatMember, Self::Err>;

    fn set_chat_administrator_custom_title(
        &self,
        chat_id: Recipient,
        user_id: UserId,
        custom_title: String,
    ) -> ErasedRequest<'a, SetChatAdministratorCustomTitle, Self::Err>;

    fn ban_chat_sender_chat(
        &self,
        chat_id: Recipient,
        sender_chat_id: ChatId,
    ) -> ErasedRequest<'a, BanChatSenderChat, Self::Err>;

    fn unban_chat_sender_chat(
        &self,
        chat_id: Recipient,
        sender_chat_id: ChatId,
    ) -> ErasedRequest<'a, UnbanChatSenderChat, Self::Err>;

    fn set_chat_permissions(
        &self,
        chat_id: Recipient,
        permissions: ChatPermissions,
    ) -> ErasedRequest<'a, SetChatPermissions, Self::Err>;

    fn export_chat_invite_link(
        &self,
        chat_id: Recipient,
    ) -> ErasedRequest<'a, ExportChatInviteLink, Self::Err>;

    fn create_chat_invite_link(
        &self,
        chat_id: Recipient,
    ) -> ErasedRequest<'a, CreateChatInviteLink, Self::Err>;

    fn edit_chat_invite_link(
        &self,
        chat_id: Recipient,
        invite_link: String,
    ) -> ErasedRequest<'a, EditChatInviteLink, Self::Err>;

    fn revoke_chat_invite_link(
        &self,
        chat_id: Recipient,
        invite_link: String,
    ) -> ErasedRequest<'a, RevokeChatInviteLink, Self::Err>;

    /// For Telegram documentation see [`ApproveChatJoinRequest`].
    fn approve_chat_join_request(
        &self,
        chat_id: Recipient,
        user_id: UserId,
    ) -> ErasedRequest<'a, ApproveChatJoinRequest, Self::Err>;

    /// For Telegram documentation see [`DeclineChatJoinRequest`].
    fn decline_chat_join_request(
        &self,
        chat_id: Recipient,
        user_id: UserId,
    ) -> ErasedRequest<'a, DeclineChatJoinRequest, Self::Err>;

    fn set_chat_photo(
        &self,
        chat_id: Recipient,
        photo: InputFile,
    ) -> ErasedRequest<'a, SetChatPhoto, Self::Err>;

    fn delete_chat_photo(
        &self,
        chat_id: Recipient,
    ) -> ErasedRequest<'a, DeleteChatPhoto, Self::Err>;

    fn set_chat_title(
        &self,
        chat_id: Recipient,
        title: String,
    ) -> ErasedRequest<'a, SetChatTitle, Self::Err>;

    fn set_chat_description(
        &self,
        chat_id: Recipient,
    ) -> ErasedRequest<'a, SetChatDescription, Self::Err>;

    fn pin_chat_message(
        &self,
        chat_id: Recipient,
        message_id: MessageId,
    ) -> ErasedRequest<'a, PinChatMessage, Self::Err>;

    fn unpin_chat_message(
        &self,
        chat_id: Recipient,
    ) -> ErasedRequest<'a, UnpinChatMessage, Self::Err>;

    fn unpin_all_chat_messages(
        &self,
        chat_id: Recipient,
    ) -> ErasedRequest<'a, UnpinAllChatMessages, Self::Err>;

    fn leave_chat(&self, chat_id: Recipient) -> ErasedRequest<'a, LeaveChat, Self::Err>;

    fn get_chat(&self, chat_id: Recipient) -> ErasedRequest<'a, GetChat, Self::Err>;

    fn get_chat_administrators(
        &self,
        chat_id: Recipient,
    ) -> ErasedRequest<'a, GetChatAdministrators, Self::Err>;

    fn get_chat_member_count(
        &self,
        chat_id: Recipient,
    ) -> ErasedRequest<'a, GetChatMemberCount, Self::Err>;

    fn get_chat_members_count(
        &self,
        chat_id: Recipient,
    ) -> ErasedRequest<'a, GetChatMembersCount, Self::Err>;

    fn get_chat_member(
        &self,
        chat_id: Recipient,
        user_id: UserId,
    ) -> ErasedRequest<'a, GetChatMember, Self::Err>;

    fn set_chat_sticker_set(
        &self,
        chat_id: Recipient,
        sticker_set_name: String,
    ) -> ErasedRequest<'a, SetChatStickerSet, Self::Err>;

    fn delete_chat_sticker_set(
        &self,
        chat_id: Recipient,
    ) -> ErasedRequest<'a, DeleteChatStickerSet, Self::Err>;

    fn get_forum_topic_icon_stickers(
        &self,
    ) -> ErasedRequest<'a, GetForumTopicIconStickers, Self::Err>;

    fn create_forum_topic(
        &self,
        chat_id: Recipient,
        name: String,
        icon_color: Rgb,
        icon_custom_emoji_id: String,
    ) -> ErasedRequest<'a, CreateForumTopic, Self::Err>;

    fn edit_forum_topic(
        &self,
        chat_id: Recipient,
        message_thread_id: ThreadId,
    ) -> ErasedRequest<'a, EditForumTopic, Self::Err>;

    fn close_forum_topic(
        &self,
        chat_id: Recipient,
        message_thread_id: ThreadId,
    ) -> ErasedRequest<'a, CloseForumTopic, Self::Err>;

    fn reopen_forum_topic(
        &self,
        chat_id: Recipient,
        message_thread_id: ThreadId,
    ) -> ErasedRequest<'a, ReopenForumTopic, Self::Err>;

    fn delete_forum_topic(
        &self,
        chat_id: Recipient,
        message_thread_id: ThreadId,
    ) -> ErasedRequest<'a, DeleteForumTopic, Self::Err>;

    fn unpin_all_forum_topic_messages(
        &self,
        chat_id: Recipient,
        message_thread_id: ThreadId,
    ) -> ErasedRequest<'a, UnpinAllForumTopicMessages, Self::Err>;

    fn edit_general_forum_topic(
        &self,
        chat_id: Recipient,
        name: String,
    ) -> ErasedRequest<'a, EditGeneralForumTopic, Self::Err>;

    fn close_general_forum_topic(
        &self,
        chat_id: Recipient,
    ) -> ErasedRequest<'a, CloseGeneralForumTopic, Self::Err>;

    fn reopen_general_forum_topic(
        &self,
        chat_id: Recipient,
    ) -> ErasedRequest<'a, ReopenGeneralForumTopic, Self::Err>;

    fn hide_general_forum_topic(
        &self,
        chat_id: Recipient,
    ) -> ErasedRequest<'a, HideGeneralForumTopic, Self::Err>;

    fn unhide_general_forum_topic(
        &self,
        chat_id: Recipient,
    ) -> ErasedRequest<'a, UnhideGeneralForumTopic, Self::Err>;

    fn unpin_all_general_forum_topic_messages(
        &self,
        chat_id: Recipient,
    ) -> ErasedRequest<'a, UnpinAllGeneralForumTopicMessages, Self::Err>;

    fn answer_callback_query(
        &self,
        callback_query_id: String,
    ) -> ErasedRequest<'a, AnswerCallbackQuery, Self::Err>;

    fn get_user_chat_boosts(
        &self,
        chat_id: Recipient,
        user_id: UserId,
    ) -> ErasedRequest<'a, GetUserChatBoosts, Self::Err>;

    fn set_my_commands(
        &self,
        commands: Vec<BotCommand>,
    ) -> ErasedRequest<'a, SetMyCommands, Self::Err>;

    fn get_business_connection(
        &self,
        business_connection_id: BusinessConnectionId,
    ) -> ErasedRequest<'a, GetBusinessConnection, Self::Err>;

    fn get_my_commands(&self) -> ErasedRequest<'a, GetMyCommands, Self::Err>;

    fn set_my_name(&self) -> ErasedRequest<'a, SetMyName, Self::Err>;

    fn get_my_name(&self) -> ErasedRequest<'a, GetMyName, Self::Err>;

    fn set_my_description(&self) -> ErasedRequest<'a, SetMyDescription, Self::Err>;

    fn get_my_description(&self) -> ErasedRequest<'a, GetMyDescription, Self::Err>;

    fn set_my_short_description(&self) -> ErasedRequest<'a, SetMyShortDescription, Self::Err>;

    fn get_my_short_description(&self) -> ErasedRequest<'a, GetMyShortDescription, Self::Err>;

    fn set_chat_menu_button(&self) -> ErasedRequest<'a, SetChatMenuButton, Self::Err>;

    fn get_chat_menu_button(&self) -> ErasedRequest<'a, GetChatMenuButton, Self::Err>;

    fn set_my_default_administrator_rights(
        &self,
    ) -> ErasedRequest<'a, SetMyDefaultAdministratorRights, Self::Err>;

    fn get_my_default_administrator_rights(
        &self,
    ) -> ErasedRequest<'a, GetMyDefaultAdministratorRights, Self::Err>;

    fn delete_my_commands(&self) -> ErasedRequest<'a, DeleteMyCommands, Self::Err>;

    fn answer_inline_query(
        &self,
        inline_query_id: String,
        results: Vec<InlineQueryResult>,
    ) -> ErasedRequest<'a, AnswerInlineQuery, Self::Err>;

    fn answer_web_app_query(
        &self,
        web_app_query_id: String,
        result: InlineQueryResult,
    ) -> ErasedRequest<'a, AnswerWebAppQuery, Self::Err>;

    fn edit_message_text(
        &self,
        chat_id: Recipient,
        message_id: MessageId,
        text: String,
    ) -> ErasedRequest<'a, EditMessageText, Self::Err>;

    fn edit_message_text_inline(
        &self,
        inline_message_id: String,
        text: String,
    ) -> ErasedRequest<'a, EditMessageTextInline, Self::Err>;

    fn edit_message_caption(
        &self,
        chat_id: Recipient,
        message_id: MessageId,
    ) -> ErasedRequest<'a, EditMessageCaption, Self::Err>;

    fn edit_message_caption_inline(
        &self,
        inline_message_id: String,
    ) -> ErasedRequest<'a, EditMessageCaptionInline, Self::Err>;

    fn edit_message_media(
        &self,
        chat_id: Recipient,
        message_id: MessageId,
        media: InputMedia,
    ) -> ErasedRequest<'a, EditMessageMedia, Self::Err>;

    fn edit_message_media_inline(
        &self,
        inline_message_id: String,
        media: InputMedia,
    ) -> ErasedRequest<'a, EditMessageMediaInline, Self::Err>;

    fn edit_message_reply_markup(
        &self,
        chat_id: Recipient,
        message_id: MessageId,
    ) -> ErasedRequest<'a, EditMessageReplyMarkup, Self::Err>;

    fn edit_message_reply_markup_inline(
        &self,
        inline_message_id: String,
    ) -> ErasedRequest<'a, EditMessageReplyMarkupInline, Self::Err>;

    fn stop_poll(
        &self,
        chat_id: Recipient,
        message_id: MessageId,
    ) -> ErasedRequest<'a, StopPoll, Self::Err>;

    fn delete_message(
        &self,
        chat_id: Recipient,
        message_id: MessageId,
    ) -> ErasedRequest<'a, DeleteMessage, Self::Err>;

    fn delete_messages(
        &self,
        chat_id: Recipient,
        message_ids: Vec<MessageId>,
    ) -> ErasedRequest<'a, DeleteMessages, Self::Err>;

    fn send_sticker(
        &self,
        chat_id: Recipient,
        sticker: InputFile,
    ) -> ErasedRequest<'a, SendSticker, Self::Err>;

    fn get_sticker_set(&self, name: String) -> ErasedRequest<'a, GetStickerSet, Self::Err>;

    fn get_custom_emoji_stickers(
        &self,
        custom_emoji_ids: Vec<String>,
    ) -> ErasedRequest<'a, GetCustomEmojiStickers, Self::Err>;

    fn upload_sticker_file(
        &self,
        user_id: UserId,
        sticker: InputFile,
        sticker_format: StickerFormat,
    ) -> ErasedRequest<'a, UploadStickerFile, Self::Err>;

    fn create_new_sticker_set(
        &self,
        user_id: UserId,
        name: String,
        title: String,
        stickers: Vec<InputSticker>,
    ) -> ErasedRequest<'a, CreateNewStickerSet, Self::Err>;

    fn add_sticker_to_set(
        &self,
        user_id: UserId,
        name: String,
        sticker: InputSticker,
    ) -> ErasedRequest<'a, AddStickerToSet, Self::Err>;

    fn set_sticker_position_in_set(
        &self,
        sticker: String,
        position: u32,
    ) -> ErasedRequest<'a, SetStickerPositionInSet, Self::Err>;

    fn delete_sticker_from_set(
        &self,
        sticker: String,
    ) -> ErasedRequest<'a, DeleteStickerFromSet, Self::Err>;

    fn replace_sticker_in_set(
        &self,
        user_id: UserId,
        name: String,
        old_sticker: String,
        sticker: InputSticker,
    ) -> ErasedRequest<'a, ReplaceStickerInSet, Self::Err>;

    fn set_sticker_set_thumbnail(
        &self,
        name: String,
        user_id: UserId,
        format: StickerFormat,
    ) -> ErasedRequest<'a, SetStickerSetThumbnail, Self::Err>;

    fn set_custom_emoji_sticker_set_thumbnail(
        &self,
        name: String,
    ) -> ErasedRequest<'a, SetCustomEmojiStickerSetThumbnail, Self::Err>;

    fn set_sticker_set_title(
        &self,
        name: String,
        title: String,
    ) -> ErasedRequest<'a, SetStickerSetTitle, Self::Err>;

    fn delete_sticker_set(&self, name: String) -> ErasedRequest<'a, DeleteStickerSet, Self::Err>;

    fn set_sticker_emoji_list(
        &self,
        sticker: String,
        emoji_list: Vec<String>,
    ) -> ErasedRequest<'a, SetStickerEmojiList, Self::Err>;

    fn set_sticker_keywords(
        &self,
        sticker: String,
    ) -> ErasedRequest<'a, SetStickerKeywords, Self::Err>;

    fn set_sticker_mask_position(
        &self,
        sticker: String,
    ) -> ErasedRequest<'a, SetStickerMaskPosition, Self::Err>;

    // we can't change telegram API
    #[allow(clippy::too_many_arguments)]
    fn send_invoice(
        &self,
        chat_id: Recipient,
        title: String,
        description: String,
        payload: String,
        provider_token: String,
        currency: String,
        prices: Vec<LabeledPrice>,
    ) -> ErasedRequest<'a, SendInvoice, Self::Err>;

    #[allow(clippy::too_many_arguments)]
    fn create_invoice_link(
        &self,
        title: String,
        description: String,
        payload: String,
        provider_token: String,
        currency: String,
        prices: Vec<LabeledPrice>,
    ) -> ErasedRequest<'a, CreateInvoiceLink, Self::Err>;

    fn answer_shipping_query(
        &self,
        shipping_query_id: String,
        ok: bool,
    ) -> ErasedRequest<'a, AnswerShippingQuery, Self::Err>;

    fn answer_pre_checkout_query(
        &self,
        pre_checkout_query_id: String,
        ok: bool,
    ) -> ErasedRequest<'a, AnswerPreCheckoutQuery, Self::Err>;

    fn set_passport_data_errors(
        &self,
        user_id: UserId,
        errors: Vec<PassportElementError>,
    ) -> ErasedRequest<'a, SetPassportDataErrors, Self::Err>;

    fn send_game(
        &self,
        chat_id: ChatId,
        game_short_name: String,
    ) -> ErasedRequest<'a, SendGame, Self::Err>;

    fn set_game_score(
        &self,
        user_id: UserId,
        score: u64,
        chat_id: u32,
        message_id: MessageId,
    ) -> ErasedRequest<'a, SetGameScore, Self::Err>;

    fn set_game_score_inline(
        &self,
        user_id: UserId,
        score: u64,
        inline_message_id: String,
    ) -> ErasedRequest<'a, SetGameScoreInline, Self::Err>;

    fn get_game_high_scores(
        &self,
        user_id: UserId,
        target: TargetMessage,
    ) -> ErasedRequest<'a, GetGameHighScores, Self::Err>;
}

impl<'a, B> ErasableRequester<'a> for B
where
    B: Requester + 'a,
{
    type Err = B::Err;

    fn get_updates(&self) -> ErasedRequest<'a, GetUpdates, Self::Err> {
        Requester::get_updates(self).erase()
    }

    fn set_webhook(&self, url: Url) -> ErasedRequest<'a, SetWebhook, Self::Err> {
        Requester::set_webhook(self, url).erase()
    }

    fn delete_webhook(&self) -> ErasedRequest<'a, DeleteWebhook, Self::Err> {
        Requester::delete_webhook(self).erase()
    }

    fn get_webhook_info(&self) -> ErasedRequest<'a, GetWebhookInfo, Self::Err> {
        Requester::get_webhook_info(self).erase()
    }

    fn get_me(&self) -> ErasedRequest<'a, GetMe, Self::Err> {
        Requester::get_me(self).erase()
    }

    fn log_out(&self) -> ErasedRequest<'a, LogOut, Self::Err> {
        Requester::log_out(self).erase()
    }

    fn close(&self) -> ErasedRequest<'a, Close, Self::Err> {
        Requester::close(self).erase()
    }

    fn send_message(
        &self,
        chat_id: Recipient,
        text: String,
    ) -> ErasedRequest<'a, SendMessage, Self::Err> {
        Requester::send_message(self, chat_id, text).erase()
    }

    fn forward_message(
        &self,
        chat_id: Recipient,
        from_chat_id: Recipient,
        message_id: MessageId,
    ) -> ErasedRequest<'a, ForwardMessage, Self::Err> {
        Requester::forward_message(self, chat_id, from_chat_id, message_id).erase()
    }

    fn forward_messages(
        &self,
        chat_id: Recipient,
        from_chat_id: Recipient,
        message_ids: Vec<MessageId>,
    ) -> ErasedRequest<'a, ForwardMessages, Self::Err> {
        Requester::forward_messages(self, chat_id, from_chat_id, message_ids).erase()
    }

    fn copy_message(
        &self,
        chat_id: Recipient,
        from_chat_id: Recipient,
        message_id: MessageId,
    ) -> ErasedRequest<'a, CopyMessage, Self::Err> {
        Requester::copy_message(self, chat_id, from_chat_id, message_id).erase()
    }

    fn copy_messages(
        &self,
        chat_id: Recipient,
        from_chat_id: Recipient,
        message_ids: Vec<MessageId>,
    ) -> ErasedRequest<'a, CopyMessages, Self::Err> {
        Requester::copy_messages(self, chat_id, from_chat_id, message_ids).erase()
    }

    fn send_photo(
        &self,
        chat_id: Recipient,
        photo: InputFile,
    ) -> ErasedRequest<'a, SendPhoto, Self::Err> {
        Requester::send_photo(self, chat_id, photo).erase()
    }

    fn send_audio(
        &self,
        chat_id: Recipient,
        audio: InputFile,
    ) -> ErasedRequest<'a, SendAudio, Self::Err> {
        Requester::send_audio(self, chat_id, audio).erase()
    }

    fn send_document(
        &self,
        chat_id: Recipient,
        document: InputFile,
    ) -> ErasedRequest<'a, SendDocument, Self::Err> {
        Requester::send_document(self, chat_id, document).erase()
    }

    fn send_video(
        &self,
        chat_id: Recipient,
        video: InputFile,
    ) -> ErasedRequest<'a, SendVideo, Self::Err> {
        Requester::send_video(self, chat_id, video).erase()
    }

    fn send_animation(
        &self,
        chat_id: Recipient,
        animation: InputFile,
    ) -> ErasedRequest<'a, SendAnimation, Self::Err> {
        Requester::send_animation(self, chat_id, animation).erase()
    }

    fn send_voice(
        &self,
        chat_id: Recipient,
        voice: InputFile,
    ) -> ErasedRequest<'a, SendVoice, Self::Err> {
        Requester::send_voice(self, chat_id, voice).erase()
    }

    fn send_video_note(
        &self,
        chat_id: Recipient,
        video_note: InputFile,
    ) -> ErasedRequest<'a, SendVideoNote, Self::Err> {
        Requester::send_video_note(self, chat_id, video_note).erase()
    }

    fn send_media_group(
        &self,
        chat_id: Recipient,
        media: Vec<InputMedia>,
    ) -> ErasedRequest<'a, SendMediaGroup, Self::Err> {
        Requester::send_media_group(self, chat_id, media).erase()
    }

    fn send_location(
        &self,
        chat_id: Recipient,
        latitude: f64,
        longitude: f64,
    ) -> ErasedRequest<'a, SendLocation, Self::Err> {
        Requester::send_location(self, chat_id, latitude, longitude).erase()
    }

    fn edit_message_live_location(
        &self,
        chat_id: Recipient,
        message_id: MessageId,
        latitude: f64,
        longitude: f64,
    ) -> ErasedRequest<'a, EditMessageLiveLocation, Self::Err> {
        Requester::edit_message_live_location(self, chat_id, message_id, latitude, longitude)
            .erase()
    }

    fn edit_message_live_location_inline(
        &self,
        inline_message_id: String,
        latitude: f64,
        longitude: f64,
    ) -> ErasedRequest<'a, EditMessageLiveLocationInline, Self::Err> {
        Requester::edit_message_live_location_inline(self, inline_message_id, latitude, longitude)
            .erase()
    }

    fn stop_message_live_location(
        &self,
        chat_id: Recipient,
        message_id: MessageId,
    ) -> ErasedRequest<'a, StopMessageLiveLocation, Self::Err> {
        Requester::stop_message_live_location(self, chat_id, message_id).erase()
    }

    fn stop_message_live_location_inline(
        &self,
        inline_message_id: String,
    ) -> ErasedRequest<'a, StopMessageLiveLocationInline, Self::Err> {
        Requester::stop_message_live_location_inline(self, inline_message_id).erase()
    }

    fn send_venue(
        &self,
        chat_id: Recipient,
        latitude: f64,
        longitude: f64,
        title: String,
        address: String,
    ) -> ErasedRequest<'a, SendVenue, Self::Err> {
        Requester::send_venue(self, chat_id, latitude, longitude, title, address).erase()
    }

    fn send_contact(
        &self,
        chat_id: Recipient,
        phone_number: String,
        first_name: String,
    ) -> ErasedRequest<'a, SendContact, Self::Err> {
        Requester::send_contact(self, chat_id, phone_number, first_name).erase()
    }

    fn send_poll(
        &self,
        chat_id: Recipient,
        question: String,
        options: Vec<InputPollOption>,
    ) -> ErasedRequest<'a, SendPoll, Self::Err> {
        Requester::send_poll(self, chat_id, question, options).erase()
    }

    fn send_dice(&self, chat_id: Recipient) -> ErasedRequest<'a, SendDice, Self::Err> {
        Requester::send_dice(self, chat_id).erase()
    }

    fn send_chat_action(
        &self,
        chat_id: Recipient,
        action: ChatAction,
    ) -> ErasedRequest<'a, SendChatAction, Self::Err> {
        Requester::send_chat_action(self, chat_id, action).erase()
    }

    fn set_message_reaction(
        &self,
        chat_id: Recipient,
        message_id: MessageId,
    ) -> ErasedRequest<'a, SetMessageReaction, Self::Err> {
        Requester::set_message_reaction(self, chat_id, message_id).erase()
    }

    fn get_user_profile_photos(
        &self,
        user_id: UserId,
    ) -> ErasedRequest<'a, GetUserProfilePhotos, Self::Err> {
        Requester::get_user_profile_photos(self, user_id).erase()
    }

    fn get_file(&self, file_id: String) -> ErasedRequest<'a, GetFile, Self::Err> {
        Requester::get_file(self, file_id).erase()
    }

    fn ban_chat_member(
        &self,
        chat_id: Recipient,
        user_id: UserId,
    ) -> ErasedRequest<'a, BanChatMember, Self::Err> {
        Requester::ban_chat_member(self, chat_id, user_id).erase()
    }

    fn kick_chat_member(
        &self,
        chat_id: Recipient,
        user_id: UserId,
    ) -> ErasedRequest<'a, KickChatMember, Self::Err> {
        Requester::kick_chat_member(self, chat_id, user_id).erase()
    }

    fn unban_chat_member(
        &self,
        chat_id: Recipient,
        user_id: UserId,
    ) -> ErasedRequest<'a, UnbanChatMember, Self::Err> {
        Requester::unban_chat_member(self, chat_id, user_id).erase()
    }

    fn restrict_chat_member(
        &self,
        chat_id: Recipient,
        user_id: UserId,
        permissions: ChatPermissions,
    ) -> ErasedRequest<'a, RestrictChatMember, Self::Err> {
        Requester::restrict_chat_member(self, chat_id, user_id, permissions).erase()
    }

    fn promote_chat_member(
        &self,
        chat_id: Recipient,
        user_id: UserId,
    ) -> ErasedRequest<'a, PromoteChatMember, Self::Err> {
        Requester::promote_chat_member(self, chat_id, user_id).erase()
    }

    fn set_chat_administrator_custom_title(
        &self,
        chat_id: Recipient,
        user_id: UserId,
        custom_title: String,
    ) -> ErasedRequest<'a, SetChatAdministratorCustomTitle, Self::Err> {
        Requester::set_chat_administrator_custom_title(self, chat_id, user_id, custom_title).erase()
    }

    fn ban_chat_sender_chat(
        &self,
        chat_id: Recipient,
        sender_chat_id: ChatId,
    ) -> ErasedRequest<'a, BanChatSenderChat, Self::Err> {
        Requester::ban_chat_sender_chat(self, chat_id, sender_chat_id).erase()
    }

    fn unban_chat_sender_chat(
        &self,
        chat_id: Recipient,
        sender_chat_id: ChatId,
    ) -> ErasedRequest<'a, UnbanChatSenderChat, Self::Err> {
        Requester::unban_chat_sender_chat(self, chat_id, sender_chat_id).erase()
    }

    fn set_chat_permissions(
        &self,
        chat_id: Recipient,
        permissions: ChatPermissions,
    ) -> ErasedRequest<'a, SetChatPermissions, Self::Err> {
        Requester::set_chat_permissions(self, chat_id, permissions).erase()
    }

    fn export_chat_invite_link(
        &self,
        chat_id: Recipient,
    ) -> ErasedRequest<'a, ExportChatInviteLink, Self::Err> {
        Requester::export_chat_invite_link(self, chat_id).erase()
    }

    fn create_chat_invite_link(
        &self,
        chat_id: Recipient,
    ) -> ErasedRequest<'a, CreateChatInviteLink, Self::Err> {
        Requester::create_chat_invite_link(self, chat_id).erase()
    }

    fn edit_chat_invite_link(
        &self,
        chat_id: Recipient,
        invite_link: String,
    ) -> ErasedRequest<'a, EditChatInviteLink, Self::Err> {
        Requester::edit_chat_invite_link(self, chat_id, invite_link).erase()
    }

    fn revoke_chat_invite_link(
        &self,
        chat_id: Recipient,
        invite_link: String,
    ) -> ErasedRequest<'a, RevokeChatInviteLink, Self::Err> {
        Requester::revoke_chat_invite_link(self, chat_id, invite_link).erase()
    }

    /// For Telegram documentation see [`ApproveChatJoinRequest`].
    fn approve_chat_join_request(
        &self,
        chat_id: Recipient,
        user_id: UserId,
    ) -> ErasedRequest<'a, ApproveChatJoinRequest, Self::Err> {
        Requester::approve_chat_join_request(self, chat_id, user_id).erase()
    }

    /// For Telegram documentation see [`DeclineChatJoinRequest`].
    fn decline_chat_join_request(
        &self,
        chat_id: Recipient,
        user_id: UserId,
    ) -> ErasedRequest<'a, DeclineChatJoinRequest, Self::Err> {
        Requester::decline_chat_join_request(self, chat_id, user_id).erase()
    }

    fn set_chat_photo(
        &self,
        chat_id: Recipient,
        photo: InputFile,
    ) -> ErasedRequest<'a, SetChatPhoto, Self::Err> {
        Requester::set_chat_photo(self, chat_id, photo).erase()
    }

    fn delete_chat_photo(
        &self,
        chat_id: Recipient,
    ) -> ErasedRequest<'a, DeleteChatPhoto, Self::Err> {
        Requester::delete_chat_photo(self, chat_id).erase()
    }

    fn set_chat_title(
        &self,
        chat_id: Recipient,
        title: String,
    ) -> ErasedRequest<'a, SetChatTitle, Self::Err> {
        Requester::set_chat_title(self, chat_id, title).erase()
    }

    fn set_chat_description(
        &self,
        chat_id: Recipient,
    ) -> ErasedRequest<'a, SetChatDescription, Self::Err> {
        Requester::set_chat_description(self, chat_id).erase()
    }

    fn pin_chat_message(
        &self,
        chat_id: Recipient,
        message_id: MessageId,
    ) -> ErasedRequest<'a, PinChatMessage, Self::Err> {
        Requester::pin_chat_message(self, chat_id, message_id).erase()
    }

    fn unpin_chat_message(
        &self,
        chat_id: Recipient,
    ) -> ErasedRequest<'a, UnpinChatMessage, Self::Err> {
        Requester::unpin_chat_message(self, chat_id).erase()
    }

    fn unpin_all_chat_messages(
        &self,
        chat_id: Recipient,
    ) -> ErasedRequest<'a, UnpinAllChatMessages, Self::Err> {
        Requester::unpin_all_chat_messages(self, chat_id).erase()
    }

    fn leave_chat(&self, chat_id: Recipient) -> ErasedRequest<'a, LeaveChat, Self::Err> {
        Requester::leave_chat(self, chat_id).erase()
    }

    fn get_chat(&self, chat_id: Recipient) -> ErasedRequest<'a, GetChat, Self::Err> {
        Requester::get_chat(self, chat_id).erase()
    }

    fn get_chat_administrators(
        &self,
        chat_id: Recipient,
    ) -> ErasedRequest<'a, GetChatAdministrators, Self::Err> {
        Requester::get_chat_administrators(self, chat_id).erase()
    }

    fn get_chat_member_count(
        &self,
        chat_id: Recipient,
    ) -> ErasedRequest<'a, GetChatMemberCount, Self::Err> {
        Requester::get_chat_member_count(self, chat_id).erase()
    }

    fn get_chat_members_count(
        &self,
        chat_id: Recipient,
    ) -> ErasedRequest<'a, GetChatMembersCount, Self::Err> {
        Requester::get_chat_members_count(self, chat_id).erase()
    }

    fn get_chat_member(
        &self,
        chat_id: Recipient,
        user_id: UserId,
    ) -> ErasedRequest<'a, GetChatMember, Self::Err> {
        Requester::get_chat_member(self, chat_id, user_id).erase()
    }

    fn set_chat_sticker_set(
        &self,
        chat_id: Recipient,
        sticker_set_name: String,
    ) -> ErasedRequest<'a, SetChatStickerSet, Self::Err> {
        Requester::set_chat_sticker_set(self, chat_id, sticker_set_name).erase()
    }

    fn delete_chat_sticker_set(
        &self,
        chat_id: Recipient,
    ) -> ErasedRequest<'a, DeleteChatStickerSet, Self::Err> {
        Requester::delete_chat_sticker_set(self, chat_id).erase()
    }

    fn get_forum_topic_icon_stickers(
        &self,
    ) -> ErasedRequest<'a, GetForumTopicIconStickers, Self::Err> {
        Requester::get_forum_topic_icon_stickers(self).erase()
    }

    fn create_forum_topic(
        &self,
        chat_id: Recipient,
        name: String,
        icon_color: Rgb,
        icon_custom_emoji_id: String,
    ) -> ErasedRequest<'a, CreateForumTopic, Self::Err> {
        Requester::create_forum_topic(self, chat_id, name, icon_color, icon_custom_emoji_id).erase()
    }

    fn edit_forum_topic(
        &self,
        chat_id: Recipient,
        message_thread_id: ThreadId,
    ) -> ErasedRequest<'a, EditForumTopic, Self::Err> {
        Requester::edit_forum_topic(self, chat_id, message_thread_id).erase()
    }

    fn close_forum_topic(
        &self,
        chat_id: Recipient,
        message_thread_id: ThreadId,
    ) -> ErasedRequest<'a, CloseForumTopic, Self::Err> {
        Requester::close_forum_topic(self, chat_id, message_thread_id).erase()
    }

    fn reopen_forum_topic(
        &self,
        chat_id: Recipient,
        message_thread_id: ThreadId,
    ) -> ErasedRequest<'a, ReopenForumTopic, Self::Err> {
        Requester::reopen_forum_topic(self, chat_id, message_thread_id).erase()
    }

    fn delete_forum_topic(
        &self,
        chat_id: Recipient,
        message_thread_id: ThreadId,
    ) -> ErasedRequest<'a, DeleteForumTopic, Self::Err> {
        Requester::delete_forum_topic(self, chat_id, message_thread_id).erase()
    }

    fn unpin_all_forum_topic_messages(
        &self,
        chat_id: Recipient,
        message_thread_id: ThreadId,
    ) -> ErasedRequest<'a, UnpinAllForumTopicMessages, Self::Err> {
        Requester::unpin_all_forum_topic_messages(self, chat_id, message_thread_id).erase()
    }

    fn edit_general_forum_topic(
        &self,
        chat_id: Recipient,
        name: String,
    ) -> ErasedRequest<'a, EditGeneralForumTopic, Self::Err> {
        Requester::edit_general_forum_topic(self, chat_id, name).erase()
    }

    fn close_general_forum_topic(
        &self,
        chat_id: Recipient,
    ) -> ErasedRequest<'a, CloseGeneralForumTopic, Self::Err> {
        Requester::close_general_forum_topic(self, chat_id).erase()
    }

    fn reopen_general_forum_topic(
        &self,
        chat_id: Recipient,
    ) -> ErasedRequest<'a, ReopenGeneralForumTopic, Self::Err> {
        Requester::reopen_general_forum_topic(self, chat_id).erase()
    }

    fn hide_general_forum_topic(
        &self,
        chat_id: Recipient,
    ) -> ErasedRequest<'a, HideGeneralForumTopic, Self::Err> {
        Requester::hide_general_forum_topic(self, chat_id).erase()
    }

    fn unhide_general_forum_topic(
        &self,
        chat_id: Recipient,
    ) -> ErasedRequest<'a, UnhideGeneralForumTopic, Self::Err> {
        Requester::unhide_general_forum_topic(self, chat_id).erase()
    }

    fn unpin_all_general_forum_topic_messages(
        &self,
        chat_id: Recipient,
    ) -> ErasedRequest<'a, UnpinAllGeneralForumTopicMessages, Self::Err> {
        Requester::unpin_all_general_forum_topic_messages(self, chat_id).erase()
    }

    fn answer_callback_query(
        &self,
        callback_query_id: String,
    ) -> ErasedRequest<'a, AnswerCallbackQuery, Self::Err> {
        Requester::answer_callback_query(self, callback_query_id).erase()
    }

    fn get_user_chat_boosts(
        &self,
        chat_id: Recipient,
        user_id: UserId,
    ) -> ErasedRequest<'a, GetUserChatBoosts, Self::Err> {
        Requester::get_user_chat_boosts(self, chat_id, user_id).erase()
    }

    fn set_my_commands(
        &self,
        commands: Vec<BotCommand>,
    ) -> ErasedRequest<'a, SetMyCommands, Self::Err> {
        Requester::set_my_commands(self, commands).erase()
    }

    fn get_business_connection(
        &self,
        business_connection_id: BusinessConnectionId,
    ) -> ErasedRequest<'a, GetBusinessConnection, Self::Err> {
        Requester::get_business_connection(self, business_connection_id).erase()
    }

    fn get_my_commands(&self) -> ErasedRequest<'a, GetMyCommands, Self::Err> {
        Requester::get_my_commands(self).erase()
    }

    fn set_my_name(&self) -> ErasedRequest<'a, SetMyName, Self::Err> {
        Requester::set_my_name(self).erase()
    }

    fn get_my_name(&self) -> ErasedRequest<'a, GetMyName, Self::Err> {
        Requester::get_my_name(self).erase()
    }

    fn set_my_description(&self) -> ErasedRequest<'a, SetMyDescription, Self::Err> {
        Requester::set_my_description(self).erase()
    }

    fn get_my_description(&self) -> ErasedRequest<'a, GetMyDescription, Self::Err> {
        Requester::get_my_description(self).erase()
    }

    fn set_my_short_description(&self) -> ErasedRequest<'a, SetMyShortDescription, Self::Err> {
        Requester::set_my_short_description(self).erase()
    }

    fn get_my_short_description(&self) -> ErasedRequest<'a, GetMyShortDescription, Self::Err> {
        Requester::get_my_short_description(self).erase()
    }

    fn set_chat_menu_button(&self) -> ErasedRequest<'a, SetChatMenuButton, Self::Err> {
        Requester::set_chat_menu_button(self).erase()
    }

    fn get_chat_menu_button(&self) -> ErasedRequest<'a, GetChatMenuButton, Self::Err> {
        Requester::get_chat_menu_button(self).erase()
    }

    fn set_my_default_administrator_rights(
        &self,
    ) -> ErasedRequest<'a, SetMyDefaultAdministratorRights, Self::Err> {
        Requester::set_my_default_administrator_rights(self).erase()
    }

    fn get_my_default_administrator_rights(
        &self,
    ) -> ErasedRequest<'a, GetMyDefaultAdministratorRights, Self::Err> {
        Requester::get_my_default_administrator_rights(self).erase()
    }

    fn delete_my_commands(&self) -> ErasedRequest<'a, DeleteMyCommands, Self::Err> {
        Requester::delete_my_commands(self).erase()
    }

    fn answer_inline_query(
        &self,
        inline_query_id: String,
        results: Vec<InlineQueryResult>,
    ) -> ErasedRequest<'a, AnswerInlineQuery, Self::Err> {
        Requester::answer_inline_query(self, inline_query_id, results).erase()
    }

    fn answer_web_app_query(
        &self,
        web_app_query_id: String,
        result: InlineQueryResult,
    ) -> ErasedRequest<'a, AnswerWebAppQuery, Self::Err> {
        Requester::answer_web_app_query(self, web_app_query_id, result).erase()
    }

    fn edit_message_text(
        &self,
        chat_id: Recipient,
        message_id: MessageId,
        text: String,
    ) -> ErasedRequest<'a, EditMessageText, Self::Err> {
        Requester::edit_message_text(self, chat_id, message_id, text).erase()
    }

    fn edit_message_text_inline(
        &self,
        inline_message_id: String,
        text: String,
    ) -> ErasedRequest<'a, EditMessageTextInline, Self::Err> {
        Requester::edit_message_text_inline(self, inline_message_id, text).erase()
    }

    fn edit_message_caption(
        &self,
        chat_id: Recipient,
        message_id: MessageId,
    ) -> ErasedRequest<'a, EditMessageCaption, Self::Err> {
        Requester::edit_message_caption(self, chat_id, message_id).erase()
    }

    fn edit_message_caption_inline(
        &self,
        inline_message_id: String,
    ) -> ErasedRequest<'a, EditMessageCaptionInline, Self::Err> {
        Requester::edit_message_caption_inline(self, inline_message_id).erase()
    }

    fn edit_message_media(
        &self,
        chat_id: Recipient,
        message_id: MessageId,
        media: InputMedia,
    ) -> ErasedRequest<'a, EditMessageMedia, Self::Err> {
        Requester::edit_message_media(self, chat_id, message_id, media).erase()
    }

    fn edit_message_media_inline(
        &self,
        inline_message_id: String,
        media: InputMedia,
    ) -> ErasedRequest<'a, EditMessageMediaInline, Self::Err> {
        Requester::edit_message_media_inline(self, inline_message_id, media).erase()
    }

    fn edit_message_reply_markup(
        &self,
        chat_id: Recipient,
        message_id: MessageId,
    ) -> ErasedRequest<'a, EditMessageReplyMarkup, Self::Err> {
        Requester::edit_message_reply_markup(self, chat_id, message_id).erase()
    }

    fn edit_message_reply_markup_inline(
        &self,
        inline_message_id: String,
    ) -> ErasedRequest<'a, EditMessageReplyMarkupInline, Self::Err> {
        Requester::edit_message_reply_markup_inline(self, inline_message_id).erase()
    }

    fn stop_poll(
        &self,
        chat_id: Recipient,
        message_id: MessageId,
    ) -> ErasedRequest<'a, StopPoll, Self::Err> {
        Requester::stop_poll(self, chat_id, message_id).erase()
    }

    fn delete_message(
        &self,
        chat_id: Recipient,
        message_id: MessageId,
    ) -> ErasedRequest<'a, DeleteMessage, Self::Err> {
        Requester::delete_message(self, chat_id, message_id).erase()
    }

    fn delete_messages(
        &self,
        chat_id: Recipient,
        message_ids: Vec<MessageId>,
    ) -> ErasedRequest<'a, DeleteMessages, Self::Err> {
        Requester::delete_messages(self, chat_id, message_ids).erase()
    }

    fn send_sticker(
        &self,
        chat_id: Recipient,
        sticker: InputFile,
    ) -> ErasedRequest<'a, SendSticker, Self::Err> {
        Requester::send_sticker(self, chat_id, sticker).erase()
    }

    fn get_sticker_set(&self, name: String) -> ErasedRequest<'a, GetStickerSet, Self::Err> {
        Requester::get_sticker_set(self, name).erase()
    }

    fn get_custom_emoji_stickers(
        &self,
        custom_emoji_ids: Vec<String>,
    ) -> ErasedRequest<'a, GetCustomEmojiStickers, Self::Err> {
        Requester::get_custom_emoji_stickers(self, custom_emoji_ids).erase()
    }

    fn upload_sticker_file(
        &self,
        user_id: UserId,
        sticker: InputFile,
        sticker_format: StickerFormat,
    ) -> ErasedRequest<'a, UploadStickerFile, Self::Err> {
        Requester::upload_sticker_file(self, user_id, sticker, sticker_format).erase()
    }

    fn create_new_sticker_set(
        &self,
        user_id: UserId,
        name: String,
        title: String,
        stickers: Vec<InputSticker>,
    ) -> ErasedRequest<'a, CreateNewStickerSet, Self::Err> {
        Requester::create_new_sticker_set(self, user_id, name, title, stickers).erase()
    }

    fn add_sticker_to_set(
        &self,
        user_id: UserId,
        name: String,
        sticker: InputSticker,
    ) -> ErasedRequest<'a, AddStickerToSet, Self::Err> {
        Requester::add_sticker_to_set(self, user_id, name, sticker).erase()
    }

    fn set_sticker_position_in_set(
        &self,
        sticker: String,
        position: u32,
    ) -> ErasedRequest<'a, SetStickerPositionInSet, Self::Err> {
        Requester::set_sticker_position_in_set(self, sticker, position).erase()
    }

    fn delete_sticker_from_set(
        &self,
        sticker: String,
    ) -> ErasedRequest<'a, DeleteStickerFromSet, Self::Err> {
        Requester::delete_sticker_from_set(self, sticker).erase()
    }

    fn replace_sticker_in_set(
        &self,
        user_id: UserId,
        name: String,
        old_sticker: String,
        sticker: InputSticker,
    ) -> ErasedRequest<'a, ReplaceStickerInSet, Self::Err> {
        Requester::replace_sticker_in_set(self, user_id, name, old_sticker, sticker).erase()
    }

    fn set_sticker_set_thumbnail(
        &self,
        name: String,
        user_id: UserId,
        format: StickerFormat,
    ) -> ErasedRequest<'a, SetStickerSetThumbnail, Self::Err> {
        Requester::set_sticker_set_thumbnail(self, name, user_id, format).erase()
    }

    fn set_custom_emoji_sticker_set_thumbnail(
        &self,
        name: String,
    ) -> ErasedRequest<'a, SetCustomEmojiStickerSetThumbnail, Self::Err> {
        Requester::set_custom_emoji_sticker_set_thumbnail(self, name).erase()
    }

    fn set_sticker_set_title(
        &self,
        name: String,
        title: String,
    ) -> ErasedRequest<'a, SetStickerSetTitle, Self::Err> {
        Requester::set_sticker_set_title(self, name, title).erase()
    }

    fn delete_sticker_set(&self, name: String) -> ErasedRequest<'a, DeleteStickerSet, Self::Err> {
        Requester::delete_sticker_set(self, name).erase()
    }

    fn set_sticker_emoji_list(
        &self,
        sticker: String,
        emoji_list: Vec<String>,
    ) -> ErasedRequest<'a, SetStickerEmojiList, Self::Err> {
        Requester::set_sticker_emoji_list(self, sticker, emoji_list).erase()
    }

    fn set_sticker_keywords(
        &self,
        sticker: String,
    ) -> ErasedRequest<'a, SetStickerKeywords, Self::Err> {
        Requester::set_sticker_keywords(self, sticker).erase()
    }

    fn set_sticker_mask_position(
        &self,
        sticker: String,
    ) -> ErasedRequest<'a, SetStickerMaskPosition, Self::Err> {
        Requester::set_sticker_mask_position(self, sticker).erase()
    }

    fn send_invoice(
        &self,
        chat_id: Recipient,
        title: String,
        description: String,
        payload: String,
        provider_token: String,
        currency: String,
        prices: Vec<LabeledPrice>,
    ) -> ErasedRequest<'a, SendInvoice, Self::Err> {
        Requester::send_invoice(
            self,
            chat_id,
            title,
            description,
            payload,
            provider_token,
            currency,
            prices,
        )
        .erase()
    }

    #[allow(clippy::too_many_arguments)]
    fn create_invoice_link(
        &self,
        title: String,
        description: String,
        payload: String,
        provider_token: String,
        currency: String,
        prices: Vec<LabeledPrice>,
    ) -> ErasedRequest<'a, CreateInvoiceLink, Self::Err> {
        Requester::create_invoice_link(
            self,
            title,
            description,
            payload,
            provider_token,
            currency,
            prices,
        )
        .erase()
    }

    fn answer_shipping_query(
        &self,
        shipping_query_id: String,
        ok: bool,
    ) -> ErasedRequest<'a, AnswerShippingQuery, Self::Err> {
        Requester::answer_shipping_query(self, shipping_query_id, ok).erase()
    }

    fn answer_pre_checkout_query(
        &self,
        pre_checkout_query_id: String,
        ok: bool,
    ) -> ErasedRequest<'a, AnswerPreCheckoutQuery, Self::Err> {
        Requester::answer_pre_checkout_query(self, pre_checkout_query_id, ok).erase()
    }

    fn set_passport_data_errors(
        &self,
        user_id: UserId,
        errors: Vec<PassportElementError>,
    ) -> ErasedRequest<'a, SetPassportDataErrors, Self::Err> {
        Requester::set_passport_data_errors(self, user_id, errors).erase()
    }

    fn send_game(
        &self,
        chat_id: ChatId,
        game_short_name: String,
    ) -> ErasedRequest<'a, SendGame, Self::Err> {
        Requester::send_game(self, chat_id, game_short_name).erase()
    }

    fn set_game_score(
        &self,
        user_id: UserId,
        score: u64,
        chat_id: u32,
        message_id: MessageId,
    ) -> ErasedRequest<'a, SetGameScore, Self::Err> {
        Requester::set_game_score(self, user_id, score, chat_id, message_id).erase()
    }

    fn set_game_score_inline(
        &self,
        user_id: UserId,
        score: u64,
        inline_message_id: String,
    ) -> ErasedRequest<'a, SetGameScoreInline, Self::Err> {
        Requester::set_game_score_inline(self, user_id, score, inline_message_id).erase()
    }

    fn get_game_high_scores(
        &self,
        user_id: UserId,
        target: TargetMessage,
    ) -> ErasedRequest<'a, GetGameHighScores, Self::Err> {
        Requester::get_game_high_scores(self, user_id, target).erase()
    }
}
