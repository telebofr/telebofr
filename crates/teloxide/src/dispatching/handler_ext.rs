use crate::{
    dispatching::{
        dialogue::{GetChatId, Storage},
        DpHandlerDescription,
    },
    types::{Me, Message},
    utils::command::BotCommands,
};
use dptree::{di::DependencyMap, Handler};

use std::fmt::Debug;

/// Extension methods for working with `dptree` handlers.
pub trait HandlerExt<Output> {
    /// Returns a handler that accepts a parsed command `C`.
    ///
    /// ## Dependency requirements
    ///
    ///  - [`crate::types::Message`]
    ///  - [`crate::types::Me`]
    #[must_use]
    fn filter_command<C>(self) -> Self
    where
        C: BotCommands + Send + Sync + 'static;

    /// Returns a handler that accepts a parsed command `C` if the command
    /// contains a bot mention, for example `/start@my_bot`.
    ///
    /// ## Dependency requirements
    ///
    ///  - [`crate::types::Message`]
    ///  - [`crate::types::Me`]
    #[must_use]
    fn filter_mention_command<C>(self) -> Self
    where
        C: BotCommands + Send + Sync + 'static;

    /// Passes [`Dialogue<D, S>`] and `D` as handler dependencies.
    ///
    /// It does so by the following steps:
    ///
    ///  1. If an incoming update has no chat ID ([`GetChatId::chat_id`] returns
    ///     `None`), the rest of the chain will not be executed. Otherwise,
    ///     passes `Dialogue::new(storage, chat_id)` forwards.
    ///  2. If [`Dialogue::get_or_default`] on the passed dialogue returns `Ok`,
    ///     passes the dialogue state forwards. Otherwise, logs an error and the
    ///     rest of the chain is not executed.
    ///
    /// ## Dependency requirements
    ///
    ///  - `Arc<S>`
    ///  - `Upd`
    ///
    /// [`Dialogue<D, S>`]: super::dialogue::Dialogue
    /// [`Dialogue::get_or_default`]: super::dialogue::Dialogue::get_or_default
    #[must_use]
    fn enter_dialogue<Upd, S, D>(self) -> Self
    where
        S: Storage<D> + ?Sized + Send + Sync + 'static,
        <S as Storage<D>>::Error: Debug + Send,
        D: Default + Send + Sync + 'static,
        Upd: GetChatId + Clone + Send + Sync + 'static;
}

impl<Output> HandlerExt<Output> for Handler<'static, DependencyMap, Output, DpHandlerDescription>
where
    Output: Send + Sync + 'static,
{
    fn filter_command<C>(self) -> Self
    where
        C: BotCommands + Send + Sync + 'static,
    {
        self.chain(filter_command::<C, Output>())
    }

    fn filter_mention_command<C>(self) -> Self
    where
        C: BotCommands + Send + Sync + 'static,
    {
        self.chain(filter_mention_command::<C, Output>())
    }

    fn enter_dialogue<Upd, S, D>(self) -> Self
    where
        S: Storage<D> + ?Sized + Send + Sync + 'static,
        <S as Storage<D>>::Error: Debug + Send,
        D: Default + Send + Sync + 'static,
        Upd: GetChatId + Clone + Send + Sync + 'static,
    {
        self.chain(super::dialogue::enter::<Upd, S, D, Output>())
    }
}

/// Returns a handler that accepts a parsed command `C`.
///
/// A call to this function is the same as `dptree::entry().filter_command()`.
///
/// See [`HandlerExt::filter_command`].
///
/// ## Dependency requirements
///
///  - [`crate::types::Message`]
///  - [`crate::types::Me`]
#[must_use]
pub fn filter_command<C, Output>() -> Handler<'static, DependencyMap, Output, DpHandlerDescription>
where
    C: BotCommands + Send + Sync + 'static,
    Output: Send + Sync + 'static,
{
    dptree::filter_map(move |message: Message, me: Me| {
        let bot_name = me.user.username.expect("Bots must have a username");
        message.text().and_then(|text| C::parse(text, &bot_name).ok())
    })
}

/// Returns a handler that accepts a parsed command `C` if the command
/// contains a bot mention, for example `/start@my_bot`.
///
/// A call to this function is the same as
/// `dptree::entry().filter_mention_command()`.
///
/// See [`HandlerExt::filter_mention_command`].
///
/// ## Dependency requirements
///
///  - [`crate::types::Message`]
///  - [`crate::types::Me`]
#[must_use]
pub fn filter_mention_command<C, Output>(
) -> Handler<'static, DependencyMap, Output, DpHandlerDescription>
where
    C: BotCommands + Send + Sync + 'static,
    Output: Send + Sync + 'static,
{
    dptree::filter_map(move |message: Message, me: Me| {
        let bot_name = me.user.username.expect("Bots must have a username");

        let command = message.text().and_then(|text| C::parse(text, &bot_name).ok());
        // If the parsing succeeds with a bot_name,
        // but fails without - there is a mention
        let is_username_required =
            message.text().and_then(|text| C::parse(text, "").ok()).is_none();

        if !is_username_required {
            return None;
        }
        command
    })
}

#[cfg(test)]
#[cfg(feature = "macros")]
mod tests {
    use crate::{self as teloxide, dispatching::UpdateFilterExt, utils::command::BotCommands};
    use chrono::DateTime;
    use dptree::deps;
    use teloxide_core::types::{
        Chat, ChatId, ChatKind, ChatPrivate, LinkPreviewOptions, Me, MediaKind, MediaText, Message,
        MessageCommon, MessageId, MessageKind, Update, UpdateId, UpdateKind, User, UserId,
    };

    use super::HandlerExt;

    #[derive(BotCommands, Clone)]
    #[command(rename_rule = "lowercase")]
    enum Cmd {
        Test,
    }

    fn make_update(text: String) -> Update {
        let timestamp = 1_569_518_829;
        let date = DateTime::from_timestamp(timestamp, 0).unwrap();
        Update {
            id: UpdateId(326_170_274),
            kind: UpdateKind::Message(Message {
                via_bot: None,
                id: MessageId(5042),
                thread_id: None,
                from: Some(User {
                    id: UserId(109_998_024),
                    is_bot: false,
                    first_name: String::from("Laster"),
                    last_name: None,
                    username: Some(String::from("laster_alex")),
                    language_code: Some(String::from("en")),
                    is_premium: false,
                    added_to_attachment_menu: false,
                }),
                sender_chat: None,
                is_topic_message: false,
                sender_business_bot: None,
                date,
                chat: Chat {
                    id: ChatId(109_998_024),
                    kind: ChatKind::Private(ChatPrivate {
                        username: Some(String::from("Laster")),
                        first_name: Some(String::from("laster_alex")),
                        last_name: None,
                    }),
                },
                kind: MessageKind::Common(MessageCommon {
                    reply_to_message: None,
                    forward_origin: None,
                    external_reply: None,
                    quote: None,
                    edit_date: None,
                    media_kind: MediaKind::Text(MediaText {
                        text,
                        entities: vec![],
                        link_preview_options: Some(LinkPreviewOptions {
                            is_disabled: true,
                            url: None,
                            prefer_small_media: false,
                            prefer_large_media: false,
                            show_above_text: false,
                        }),
                    }),
                    reply_markup: None,
                    author_signature: None,
                    is_automatic_forward: false,
                    has_protected_content: false,
                    reply_to_story: None,
                    sender_boost_count: None,
                    is_from_offline: false,
                    business_connection_id: None,
                }),
            }),
        }
    }

    fn make_me() -> Me {
        Me {
            user: User {
                id: UserId(42),
                is_bot: true,
                first_name: "First".to_owned(),
                last_name: None,
                username: Some("SomethingSomethingBot".to_owned()),
                language_code: None,
                is_premium: false,
                added_to_attachment_menu: false,
            },
            can_join_groups: false,
            can_read_all_group_messages: false,
            supports_inline_queries: false,
            can_connect_to_business: false,
        }
    }

    #[tokio::test]
    async fn test_filter_command() {
        let h = dptree::entry()
            .branch(Update::filter_message().filter_command::<Cmd>().endpoint(|| async {}));
        let me = make_me();

        let update = make_update("/test@".to_owned() + me.username());
        let result = h.dispatch(deps![update, me.clone()]).await;
        assert!(result.is_break());

        let update = make_update("/test@".to_owned() + "SomeOtherBot");
        let result = h.dispatch(deps![update, me.clone()]).await;
        assert!(result.is_continue());

        let update = make_update("/test".to_owned());
        let result = h.dispatch(deps![update, me.clone()]).await;
        assert!(result.is_break());
    }

    #[tokio::test]
    async fn test_filter_mention_command() {
        let h = dptree::entry()
            .branch(Update::filter_message().filter_mention_command::<Cmd>().endpoint(|| async {}));
        let me = make_me();

        let update = make_update("/test@".to_owned() + me.username());
        let result = h.dispatch(deps![update, me.clone()]).await;
        assert!(result.is_break());

        let update = make_update("/test@".to_owned() + "SomeOtherBot");
        let result = h.dispatch(deps![update, me.clone()]).await;
        assert!(result.is_continue());

        let update = make_update("/test".to_owned());
        let result = h.dispatch(deps![update, me.clone()]).await;
        assert!(result.is_continue());
    }
}
