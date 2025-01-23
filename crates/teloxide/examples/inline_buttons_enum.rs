// An example to show how to use `InlineButtons` macro
use teloxide::{
    dispatching::UpdateFilterExt, dptree::case, prelude::*, types::InlineKeyboardMarkup,
    utils::button::InlineButtons,
};

type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(InlineButtons, Clone)]
// Just in case your callback arguments may contain the default separator ";", you can change it!
#[button(fields_separator = "|")]
enum CallbackButton {
    GithubLink,
    MessageId(i32),
    // `rename` makes the underlying callback data from "DateOfMessage" to "DoM",
    // so synax and length optimizations (telegram allows max 64 chars in callback data)
    // don't have to compromise!
    #[button(rename = "DoM")]
    DateOfMessage {
        date: String,
    },
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting inline buttons enum bot...");

    let bot = Bot::from_env();

    let handler = dptree::entry().branch(Update::filter_message().endpoint(start)).branch(
        Update::filter_callback_query()
            // Works very similar to `BotCommands` macro,
            // filters for correct data and injects the enum
            .filter_callback_data::<CallbackButton>()
            .branch(case![CallbackButton::GithubLink].endpoint(github_link))
            .branch(case![CallbackButton::MessageId(id)].endpoint(message_id))
            .branch(case![CallbackButton::DateOfMessage { date }].endpoint(date_of_message)),
    );

    Dispatcher::builder(bot, handler).enable_ctrlc_handler().build().dispatch().await;
}

async fn start(bot: Bot, msg: Message) -> HandlerResult {
    // You can easily build a keyboard with these buttons
    let keyboard = InlineKeyboardMarkup::new(vec![
        vec![CallbackButton::GithubLink.build_button("Teloxide github link").unwrap()],
        vec![CallbackButton::MessageId(msg.id.0).build_button("Message id of my message").unwrap()],
        vec![CallbackButton::DateOfMessage { date: msg.date.to_string() }
            .build_button("Date of my message")
            .unwrap()],
    ]);
    bot.send_message(msg.chat.id, "What do you want to know?").reply_markup(keyboard).await?;
    Ok(())
}

async fn github_link(bot: Bot, q: CallbackQuery) -> HandlerResult {
    let chat_id = q.regular_message().unwrap().chat.id;
    bot.answer_callback_query(q.id).await?;
    bot.send_message(chat_id, "https://github.com/teloxide/teloxide").await?;
    Ok(())
}

async fn message_id(
    bot: Bot,
    q: CallbackQuery,
    id: i32, // Available from `case![CallbackButton::MessageId(id)]`
) -> HandlerResult {
    let chat_id = q.regular_message().unwrap().chat.id;
    bot.answer_callback_query(q.id).await?;
    bot.send_message(chat_id, format!("Message id of your message: {id}")).await?;
    Ok(())
}

async fn date_of_message(bot: Bot, q: CallbackQuery, date: String) -> HandlerResult {
    let chat_id = q.regular_message().unwrap().chat.id;
    bot.answer_callback_query(q.id).await?;
    bot.send_message(chat_id, format!("Date of your message: {date}")).await?;
    Ok(())
}
