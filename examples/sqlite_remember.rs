use teloxide::{
    dispatching2::dialogue::{serializer::Json, SqliteStorage, Storage},
    prelude2::*,
    RequestError,
};
use thiserror::Error;

type MyDialogue = Dialogue<DialogueState, SqliteStorage<Json>>;
type StorageError = <SqliteStorage<Json> as Storage<DialogueState>>::Error;

#[derive(Debug, Error)]
enum Error {
    #[error("error from Telegram: {0}")]
    TelegramError(#[from] RequestError),
    #[error("error from storage: {0}")]
    StorageError(#[from] StorageError),
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum DialogueState {
    Start,
    GotNumber(i32),
}

impl Default for DialogueState {
    fn default() -> Self {
        Self::Start
    }
}

async fn handle_message(
    bot: AutoSend<Bot>,
    msg: Message,
    dialogue: MyDialogue,
) -> Result<(), Error> {
    match msg.text() {
        None => {
            bot.send_message(msg.chat.id, "Send me a text message.").await?;
        }
        Some(ans) => {
            let state = dialogue.get_or_default().await?;
            match state {
                DialogueState::Start => {
                    if let Ok(number) = ans.parse() {
                        dialogue.update(DialogueState::GotNumber(number)).await?;
                        bot.send_message(
                            msg.chat.id,
                            format!("Remembered number {}. Now use /get or /reset", number),
                        )
                        .await?;
                    } else {
                        bot.send_message(msg.chat.id, "Please, send me a number").await?;
                    }
                }
                DialogueState::GotNumber(num) => {
                    if ans.starts_with("/get") {
                        bot.send_message(msg.chat.id, format!("Here is your number: {}", num))
                            .await?;
                    } else if ans.starts_with("/reset") {
                        dialogue.reset().await?;
                        bot.send_message(msg.chat.id, "Resetted number").await?;
                    } else {
                        bot.send_message(msg.chat.id, "Please, send /get or /reset").await?;
                    }
                }
            }
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    let bot = Bot::from_env().auto_send();
    let storage = SqliteStorage::open("db.sqlite", Json).await.unwrap();

    let handler = Update::filter_message()
        .add_dialogue::<Message, SqliteStorage<Json>, DialogueState>()
        .branch(dptree::endpoint(handle_message));

    DispatcherBuilder::new(bot, handler)
        .dependencies(dptree::deps![storage])
        .build()
        .dispatch()
        .await;
}
