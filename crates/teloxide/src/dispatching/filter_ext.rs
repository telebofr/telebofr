#![allow(clippy::redundant_closure_call)]
// Required for the `filter_from` currently
#![allow(deprecated)]

use dptree::{di::DependencyMap, Handler};

use crate::{
    dispatching::DpHandlerDescription,
    types::{AllowedUpdate, Message, Update, UpdateKind},
};

macro_rules! define_ext {
    ($ext_name:ident, $for_ty:ty => $( ($func:ident, $proj_fn:expr, $fn_doc:expr $(, $Allowed:ident)? ) ,)*) => {
        #[doc = concat!("Filter methods for [`", stringify!($for_ty), "`].")]
        pub trait $ext_name<Out>: private::Sealed {
            $( define_ext!(@sig $func, $fn_doc); )*
        }

        impl<Out> $ext_name<Out> for $for_ty
        where
            Out: Send + Sync + 'static,
        {
            $( define_ext!(@impl $for_ty, $func, $proj_fn $(, $Allowed )? ); )*
        }
    };

    (@sig $func:ident, $fn_doc:expr) => {
        #[doc = $fn_doc]
        fn $func() -> Handler<'static, DependencyMap, Out, DpHandlerDescription>;
    };

    (@impl $for_ty:ty, $func:ident, $proj_fn:expr, $Allowed:ident) => {
        fn $func() -> Handler<'static, DependencyMap, Out, DpHandlerDescription> {
            dptree::filter_map_with_description(DpHandlerDescription::of(AllowedUpdate::$Allowed), move |input: $for_ty| {
                $proj_fn(input)
            })
        }
    };

    (@impl $for_ty:ty, $func:ident, $proj_fn:expr) => {
        fn $func() -> Handler<'static, DependencyMap, Out, DpHandlerDescription> {
            dptree::filter_map(move |input: $for_ty| {
                $proj_fn(input)
            })
        }
    };
}

mod private {
    use teloxide_core::types::{Message, Update};

    pub trait Sealed {}

    impl Sealed for Update {}
    impl Sealed for Message {}
}

// FIXME: rewrite this macro to allow the usage of functions returning small
// values without borrowing
macro_rules! define_message_ext {
    ($( ($func:ident, $fn_name:path) ,)*) => {
        define_ext! {
            MessageFilterExt, Message =>
            $((
                $func,
                (|x| $fn_name(&x).map(ToOwned::to_owned)),
                concat!("Applies the [`", stringify!($fn_name), "`] filter.")
            ),)*
        }
    }
}

// FIXME: change macro so that we can filter things without getters
// May be expanded in the future.
define_message_ext! {
    // MessageCommon
    (filter_from, Message::from),
    // MediaKind variants of the MessageKind::Common
    (filter_animation, Message::animation),
    (filter_audio, Message::audio),
    (filter_contact, Message::contact),
    (filter_document, Message::document),
    (filter_game, Message::game),
    (filter_venue, Message::venue),
    (filter_location, Message::location),
    (filter_photo, Message::photo),
    (filter_poll, Message::poll),
    (filter_sticker, Message::sticker),
    (filter_story, Message::story),
    (filter_text, Message::text),
    (filter_video, Message::video),
    (filter_video_note, Message::video_note),
    (filter_voice, Message::voice),
    (filter_migration, Message::chat_migration),
    (filter_migration_from, Message::migrate_from_chat_id),
    (filter_migration_to, Message::migrate_to_chat_id),
    (filter_reply_to_message, Message::reply_to_message),
    (filter_forward_origin, Message::forward_origin),
    (filter_reply_to_story, Message::reply_to_story),
    // Rest variants of a MessageKind
    (filter_new_chat_members, Message::new_chat_members),
    (filter_left_chat_member, Message::left_chat_member),
    (filter_new_chat_title, Message::new_chat_title),
    (filter_new_chat_photo, Message::new_chat_photo),
    (filter_delete_chat_photo, Message::delete_chat_photo),
    (filter_group_chat_created, Message::group_chat_created),
    (filter_supergroup_chat_created, Message::super_group_chat_created),
    (filter_channel_chat_created, Message::channel_chat_created),
    (filter_message_auto_delete_timer_changed, Message::message_auto_delete_timer_changed),
    (filter_pinned, Message::pinned_message),
    (filter_invoice, Message::invoice),
    (filter_successful_payment, Message::successful_payment),
    (filter_connected_website, Message::connected_website),
    (filter_write_access_allowed, Message::write_access_allowed),
    (filter_passport_data, Message::passport_data),
    (filter_dice, Message::dice),
    (filter_proximity_alert_triggered, Message::proximity_alert_triggered),
    (filter_boost_added, Message::boost_added),
    (filter_chat_background_set, Message::chat_background_set),
    (filter_forum_topic_created, Message::forum_topic_created),
    (filter_forum_topic_edited, Message::forum_topic_edited),
    (filter_forum_topic_closed, Message::forum_topic_closed),
    (filter_forum_topic_reopened, Message::forum_topic_reopened),
    (filter_general_forum_topic_hidden, Message::general_forum_topic_hidden),
    (filter_general_forum_topic_unhidden, Message::general_forum_topic_unhidden),
    (filter_giveaway, Message::giveaway),
    (filter_giveaway_completed, Message::giveaway_completed),
    (filter_giveaway_created, Message::giveaway_created),
    (filter_giveaway_winners, Message::giveaway_winners),
    (filter_video_chat_scheduled, Message::video_chat_scheduled),
    (filter_video_chat_started, Message::video_chat_started),
    (filter_video_chat_ended, Message::video_chat_ended),
    (filter_video_chat_participants_invited, Message::video_chat_participants_invited),
    (filter_web_app_data, Message::web_app_data),
}

macro_rules! define_update_ext {
    ($( ($func:ident, $kind:path, $Allowed:ident) ,)*) => {
        define_ext! {
            UpdateFilterExt, Update =>
            $((
                $func,
                |update: Update| match update.kind {
                    $kind(x) => Some(x),
                    _ => None,
                },
                concat!("Filters out [`", stringify!($kind), "`] objects."),
                $Allowed
            ),)*
        }
    }
}

define_update_ext! {
    (filter_message, UpdateKind::Message, Message),
    (filter_edited_message, UpdateKind::EditedMessage, EditedMessage),
    (filter_channel_post, UpdateKind::ChannelPost, ChannelPost),
    (filter_edited_channel_post, UpdateKind::EditedChannelPost, EditedChannelPost),
    (filter_business_connection, UpdateKind::BusinessConnection, BusinessConnection),
    (filter_business_message, UpdateKind::BusinessMessage, BusinessMessage),
    (filter_edited_business_message, UpdateKind::EditedBusinessMessage, EditedBusinessMessage),
    (filter_deleted_business_messages, UpdateKind::DeletedBusinessMessages, DeletedBusinessMessages),
    (filter_message_reaction_updated, UpdateKind::MessageReaction, MessageReaction),
    (filter_message_reaction_count_updated, UpdateKind::MessageReactionCount, MessageReactionCount),
    (filter_inline_query, UpdateKind::InlineQuery, InlineQuery),
    (filter_chosen_inline_result, UpdateKind::ChosenInlineResult, ChosenInlineResult),
    (filter_callback_query, UpdateKind::CallbackQuery, CallbackQuery),
    (filter_shipping_query, UpdateKind::ShippingQuery, ShippingQuery),
    (filter_pre_checkout_query, UpdateKind::PreCheckoutQuery, PreCheckoutQuery),
    (filter_poll, UpdateKind::Poll, Poll),
    (filter_poll_answer, UpdateKind::PollAnswer, PollAnswer),
    (filter_my_chat_member, UpdateKind::MyChatMember, MyChatMember),
    (filter_chat_member, UpdateKind::ChatMember, ChatMember),
    (filter_chat_join_request, UpdateKind::ChatJoinRequest, ChatJoinRequest),
    (filter_chat_boost, UpdateKind::ChatBoost, ChatBoost),
    (filter_removed_chat_boost, UpdateKind::RemovedChatBoost, RemovedChatBoost),
}
