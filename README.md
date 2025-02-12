<div align="center">
  <img src="./media/teloxide-logo.png" width="250"/>
  <h1><code>teloxide</code></h1>
  <a href="https://docs.rs/teloxide/">
    <img src="https://docs.rs/teloxide/badge.svg">
  </a>
  <a href="https://github.com/teloxide/teloxide/actions">
    <img src="https://github.com/teloxide/teloxide/workflows/Continuous%20integration/badge.svg">
  </a>
  <a href="https://crates.io/crates/teloxide">
    <img src="https://img.shields.io/crates/v/teloxide.svg">
  </a>
  <a href="https://core.telegram.org/bots/api">
    <img src="https://img.shields.io/badge/API%20coverage-Up%20to%207.3%20(inclusively)-green.svg">
  </a>
  <a href="https://t.me/teloxide">
    <img src="https://img.shields.io/badge/support-t.me%2Fteloxide-blueviolet">
  </a>
  <a href="https://devpod.sh/open#https://github.com/teloxide/teloxide">
    <img src="https://img.shields.io/badge/Open_in-DevPod-blueviolet">
  </a>

  A full-featured framework that empowers you to easily build [Telegram bots](https://telegram.org/blog/bot-revolution) using [Rust](https://www.rust-lang.org/). It handles all the difficult stuff so you can focus only on your business logic.
</div>

## Highlights

 - **Declarative design.** `teloxide` is based upon [`dptree`], a functional [chain of responsibility] pattern that allows you to express pipelines of message processing in a highly declarative and extensible style.

[`dptree`]: https://github.com/teloxide/dptree
[chain of responsibility]: https://en.wikipedia.org/wiki/Chain-of-responsibility_pattern

 - **Feature-rich.** You can use both long polling and webhooks, configure an underlying HTTPS client, set a custom URL of a Telegram API server, do graceful shutdown, and much more.

 - **Simple dialogues.** Our dialogues subsystem is simple and easy-to-use, and, furthermore, is agnostic of how/where dialogues are stored. For example, you can just replace a one line to achieve [persistence]. Out-of-the-box storages include [Redis] and [Sqlite].

[persistence]: https://en.wikipedia.org/wiki/Persistence_(computer_science)
[Redis]: https://redis.io/
[Sqlite]: https://www.sqlite.org

 - **Strongly typed commands.** Define bot commands as an `enum` and teloxide will parse them automatically — just like JSON structures in [`serde-json`] and command-line arguments in [`structopt`].

[`structopt`]: https://github.com/TeXitoi/structopt
[`serde-json`]: https://github.com/serde-rs/json

## Setting up your environment

 1. [Download Rust](http://rustup.rs/).
 2. Create a new bot using [@Botfather](https://t.me/botfather) to get a token in the format `123456789:blablabla`.
 3. Initialise the `TELOXIDE_TOKEN` environmental variable to your token:
```bash
# Unix-like
$ export TELOXIDE_TOKEN=<Your token here>

# Windows command line
$ set TELOXIDE_TOKEN=<Your token here>

# Windows PowerShell
$ $env:TELOXIDE_TOKEN=<Your token here>
```

 4. Make sure that your Rust compiler is up to date (`teloxide` currently requires rustc at least version 1.80):
```bash
# If you're using stable
$ rustup update stable
$ rustup override set stable

# If you're using nightly
$ rustup update nightly
$ rustup override set nightly
```

 5. Run `cargo new my_bot`, enter the directory and put these lines into your `Cargo.toml`:
```toml
[dependencies]
teloxide = { version = "0.13", features = ["macros"] }
log = "0.4"
pretty_env_logger = "0.5"
tokio = { version =  "1.8", features = ["rt-multi-thread", "macros"] }
```

## API overview

### The dices bot

This bot replies with a dice to each received message:

[[`examples/throw_dice.rs`](crates/teloxide/examples/throw_dice.rs)]

```rust,no_run
use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");

    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        bot.send_dice(msg.chat.id).await?;
        Ok(())
    })
    .await;
}
```

<div align="center">
    <img src="./media/throw-dice.gif" width="420" />
</div>

### Commands

Commands are strongly typed and defined declaratively, similar to how we define CLI using [structopt] and JSON structures in [serde-json]. The following bot accepts these commands:

 - `/username <your username>`
 - `/usernameandage <your username> <your age>`
 - `/help`

[structopt]: https://docs.rs/structopt/0.3.9/structopt/
[serde-json]: https://github.com/serde-rs/json

[[`examples/command.rs`](crates/teloxide/examples/command.rs)]

```rust,no_run
use teloxide::{prelude::*, utils::command::BotCommands};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let bot = Bot::from_env();

    Command::repl(bot, answer).await;
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "handle a username.")]
    Username(String),
    #[command(description = "handle a username and an age.", parse_with = "split")]
    UsernameAndAge { username: String, age: u8 },
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
        Command::Username(username) => {
            bot.send_message(msg.chat.id, format!("Your username is @{username}.")).await?
        }
        Command::UsernameAndAge { username, age } => {
            bot.send_message(msg.chat.id, format!("Your username is @{username} and age is {age}."))
                .await?
        }
    };

    Ok(())
}
```

<div align="center">
    <img src="./media/command.gif" width="420" />
</div>

### Dialogues management

A dialogue is typically described by an enumeration where each variant is one possible state of the dialogue. There are also _state handler functions_, which may turn a dialogue from one state to another, thereby forming an [FSM].

[FSM]: https://en.wikipedia.org/wiki/Finite-state_machine

Below is a bot that asks you three questions and then sends the answers back to you:

[[`examples/dialogue.rs`](crates/teloxide/examples/dialogue.rs)]

```rust,ignore
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};

type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    ReceiveFullName,
    ReceiveAge {
        full_name: String,
    },
    ReceiveLocation {
        full_name: String,
        age: u8,
    },
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting dialogue bot...");

    let bot = Bot::from_env();

    Dispatcher::builder(
        bot,
        Update::filter_message()
            .enter_dialogue::<Message, InMemStorage<State>, State>()
            .branch(dptree::case![State::Start].endpoint(start))
            .branch(dptree::case![State::ReceiveFullName].endpoint(receive_full_name))
            .branch(dptree::case![State::ReceiveAge { full_name }].endpoint(receive_age))
            .branch(
                dptree::case![State::ReceiveLocation { full_name, age }].endpoint(receive_location),
            ),
    )
    .dependencies(dptree::deps![InMemStorage::<State>::new()])
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
}

async fn start(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Let's start! What's your full name?").await?;
    dialogue.update(State::ReceiveFullName).await?;
    Ok(())
}

async fn receive_full_name(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    match msg.text() {
        Some(text) => {
            bot.send_message(msg.chat.id, "How old are you?").await?;
            dialogue.update(State::ReceiveAge { full_name: text.into() }).await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Send me plain text.").await?;
        }
    }

    Ok(())
}

async fn receive_age(
    bot: Bot,
    dialogue: MyDialogue,
    full_name: String, // Available from `State::ReceiveAge`.
    msg: Message,
) -> HandlerResult {
    match msg.text().map(|text| text.parse::<u8>()) {
        Some(Ok(age)) => {
            bot.send_message(msg.chat.id, "What's your location?").await?;
            dialogue.update(State::ReceiveLocation { full_name, age }).await?;
        }
        _ => {
            bot.send_message(msg.chat.id, "Send me a number.").await?;
        }
    }

    Ok(())
}

async fn receive_location(
    bot: Bot,
    dialogue: MyDialogue,
    (full_name, age): (String, u8), // Available from `State::ReceiveLocation`.
    msg: Message,
) -> HandlerResult {
    match msg.text() {
        Some(location) => {
            let report = format!("Full name: {full_name}\nAge: {age}\nLocation: {location}");
            bot.send_message(msg.chat.id, report).await?;
            dialogue.exit().await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Send me plain text.").await?;
        }
    }

    Ok(())
}
```

<div align="center">
    <img src="./media/dialogue.gif" width="420" />
</div>

[More examples >>](crates/teloxide/examples/)

## Testing

A community made crate [`teloxide_tests`](https://github.com/LasterAlex/teloxide_tests) can be used to test your bots.

[Some testing examples >>](https://github.com/LasterAlex/teloxide_tests/tree/master/examples)

## Tutorials

 - [_"Migrating my family finance bot from Python to Rust (teloxide) because I am tired of exceptions (part 1)"_](https://web.archive.org/web/20230130112018/https://trkohler.com/posts/i-migrated-my-family-finance-bot-from-python-to-rust-because-i-am-tired-of-exceptions/) by Troy Köhler.
 - [_"Migrating my family finance bot from Python to Rust (teloxide) [part 2]"_](https://web.archive.org/web/20240529200929/https://trkohler.com/posts/migrating-my-family-finance-bot-from-python-to-rust-teloxide-part-2/) by Troy Köhler.

## FAQ

**Q: Where I can ask questions?**

A:

 - [Issues] is a good place for well-formed questions about the library design, enhancements, and bug reports.
 - [GitHub Discussions] is a place where you can ask us for help in a less formal manner.
 - If you need quick help in real-time, you should ask a question in [our official Telegram group].

[Issues]: https://github.com/teloxide/teloxide/issues
[our official Telegram group]: https://t.me/teloxide
[GitHub Discussions]: https://github.com/teloxide/teloxide/discussions

**Q: Do you support the Telegram API for clients?**

A: No, only the bots API.

**Q: Can I use webhooks?**

A: You can! `teloxide` has a built-in support for webhooks in `dispatching::update_listeners::webhooks` module. See how it's used in [`examples/ngrok_ping_pong_bot`](crates/teloxide/examples/ngrok_ping_pong.rs) and [`examples/heroku_ping_pong_bot`](crates/teloxide/examples/heroku_ping_pong.rs).

**Q: Can I handle both callback queries and messages within a single dialogue?**

A: Yes, see [`examples/purchase.rs`](crates/teloxide/examples/purchase.rs).

## Community bots

Feel free to propose your own bot to our collection!

 - [`raine/tgreddit`](https://github.com/raine/tgreddit) — A bot that sends the top posts of your favorite subreddits to Telegram.
 - [`magnickolas/remindee-bot`](https://github.com/magnickolas/remindee-bot) — Telegram bot for managing reminders.
 - [`WaffleLapkin/crate_upd_bot`](https://github.com/WaffleLapkin/crate_upd_bot) — A bot that notifies about crate updates.
 - [`mattrighetti/GroupActivityBot`](https://github.com/mattrighetti/group-activity-bot-rs) — Telegram bot that keeps track of user activity in groups.
 - [`alenpaul2001/AurSearchBot`](https://gitlab.com/alenpaul2001/aursearchbot) — Telegram bot for searching in Arch User Repository (AUR).
 - [`ArtHome12/vzmuinebot`](https://github.com/ArtHome12/vzmuinebot) — Telegram bot for food menu navigate.
 - [`studiedlist/EddieBot`](https://gitlab.com/studiedlist/eddie-bot) — Chatting bot with several entertainment features.
 - [`modos189/tg_blackbox_bot`](https://gitlab.com/modos189/tg_blackbox_bot) — Anonymous feedback for your Telegram project.
 - [`0xNima/spacecraft`](https://github.com/0xNima/spacecraft) — Yet another telegram bot to downloading Twitter spaces.
 - [`0xNima/Twideo`](https://github.com/0xNima/Twideo) — Simple Telegram Bot for downloading videos from Twitter via their links.
 - [`mattrighetti/libgen-bot-rs`](https://github.com/mattrighetti/libgen-bot-rs) — Telegram bot to interface with libgen.
 - [`zamazan4ik/npaperbot-telegram`](https://github.com/zamazan4ik/npaperbot-telegram) — Telegram bot for searching via C++ proposals.
 - [`studentenherz/dlebot`](https://github.com/studentenherz/dlebot) — A bot to query definitions of words from the Spanish Language Dictionary.
 - [`fr0staman/fr0staman_bot`](https://github.com/fr0staman/fr0staman_bot) — Feature rich Telegram game-like bot with pigs 🐽.
 - [`franciscofigueira/transferBot`](https://github.com/franciscofigueira/transferBot) — Telegram bot that notifies of crypto token transfers.

<details>
<summary>Show bots using `teloxide` older than v0.6.0</summary>

 - [`mxseev/logram`](https://github.com/mxseev/logram) — Utility that takes logs from anywhere and sends them to Telegram.
 - [`alexkonovalov/PedigreeBot`](https://github.com/alexkonovalov/PedigreeBot) — A Telegram bot for building family trees.
 - [`Hermitter/tepe`](https://github.com/Hermitter/tepe) — A CLI to command a bot to send messages and files over Telegram.
 - [`myblackbeard/basketball-betting-bot`](https://github.com/myblackbeard/basketball-betting-bot) — The bot lets you bet on NBA games against your buddies.
 - [`dracarys18/grpmr-rs`](https://github.com/dracarys18/grpmr-rs) — Modular Telegram Group Manager Bot written in Rust.
 - [`ArtHome12/cognito_bot`](https://github.com/ArtHome12/cognito_bot) — The bot is designed to anonymize messages to a group.
 - [`crapstone/hsctt`](https://codeberg.org/crapstones-bots/hsctt) — A bot that converts HTTP status codes into text.

</details>

See [1900+ other public repositories using `teloxide` >>](https://github.com/teloxide/teloxide/network/dependents)

## Contributing

See [`CONRIBUTING.md`](CONTRIBUTING.md).
