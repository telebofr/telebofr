use teloxide::{macros::DialogueState, prelude2::*, types::Me, utils::command::BotCommand};

// include!("redis_config.in");
include!("sqlite_config.in");

type MyDialogue = Dialogue<State, MyStorage>;

#[derive(DialogueState, Clone, serde::Serialize, serde::Deserialize)]
#[handler_out(anyhow::Result<()>)]
pub enum State {
    #[handler(handle_start)]
    Start,

    #[handler(handle_got_number)]
    GotNumber(i32),
}

impl Default for State {
    fn default() -> Self {
        Self::Start
    }
}

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These commands are supported:")]
pub enum Command {
    #[command(description = "get your number.")]
    Get,
    #[command(description = "reset your number.")]
    Reset,
}

#[tokio::main]
async fn main() {
    let bot = Bot::from_env().auto_send();

    let storage = open_storage().await;
    let handler = Update::filter_message()
        .enter_dialogue::<Message, MyStorage, State>()
        .dispatch_by::<State>();

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![storage])
        .build()
        .setup_ctrlc_handler()
        .dispatch()
        .await;
}

async fn handle_start(
    bot: AutoSend<Bot>,
    msg: Message,
    dialogue: MyDialogue,
) -> anyhow::Result<()> {
    match msg.text().unwrap().parse() {
        Ok(number) => {
            dialogue.update(State::GotNumber(number)).await?;
            bot.send_message(
                msg.chat.id,
                format!("Remembered number {}. Now use /get or /reset", number),
            )
            .await?;
        }
        _ => {
            bot.send_message(msg.chat.id, "Please, send me a number").await?;
        }
    }

    Ok(())
}

async fn handle_got_number(
    bot: AutoSend<Bot>,
    msg: Message,
    dialogue: MyDialogue,
    num: i32,
    me: Me,
) -> anyhow::Result<()> {
    let ans = msg.text().unwrap();
    let bot_name = me.user.username.unwrap();

    match Command::parse(ans, bot_name) {
        Ok(cmd) => match cmd {
            Command::Get => {
                bot.send_message(msg.chat.id, format!("Here is your number: {}", num)).await?;
            }
            Command::Reset => {
                dialogue.reset().await?;
                bot.send_message(msg.chat.id, "Number resetted").await?;
            }
        },
        Err(_) => {
            bot.send_message(msg.chat.id, "Please, send /get or /reset").await?;
        }
    }

    Ok(())
}