//! Support for user dialogues.
//!
//! The main type is (surprise!) [`Dialogue`]. Under the hood, it is just a
//! wrapper over [`Storage`] and a chat ID. All it does is provides convenient
//! method for manipulating the dialogue state. [`Storage`] is where all
//! dialogue states are stored; it can be either [`InMemStorage`], which is a
//! simple hash map from [`std::collections`], or an advanced database wrapper
//! such as [`SqliteStorage`]. In the latter case, your dialogues are
//! _persistent_, meaning that you can safely restart your bot and all ongoing
//! dialogues will remain in the database -- this is a preferred method for
//! production bots.
//!
//! [`examples/dialogue.rs`] clearly demonstrates the typical usage of
//! dialogues. Your dialogue state can be represented as an enumeration:
//!
//! ```no_run
//! #[derive(Clone, Default)]
//! pub enum State {
//!     #[default]
//!     Start,
//!     ReceiveFullName,
//!     ReceiveAge {
//!         full_name: String,
//!     },
//!     ReceiveLocation {
//!         full_name: String,
//!         age: u8,
//!     },
//! }
//! ```
//!
//! Each state is associated with its respective handler: e.g., when a dialogue
//! state is `ReceiveAge`, `receive_age` is invoked:
//!
//! ```no_run
//! # use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};
//! # type MyDialogue = Dialogue<State, InMemStorage<State>>;
//! # type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
//! # #[derive(Clone, Debug)] enum State { ReceiveLocation { full_name: String, age: u8 } }
//! async fn receive_age(
//!     bot: Bot,
//!     dialogue: MyDialogue,
//!     full_name: String, // Available from `State::ReceiveAge`.
//!     msg: Message,
//! ) -> HandlerResult {
//!     match msg.text().map(|text| text.parse::<u8>()) {
//!         Some(Ok(age)) => {
//!             bot.send_message(msg.chat.id, "What's your location?").await?;
//!             dialogue.update(State::ReceiveLocation { full_name, age }).await?;
//!         }
//!         _ => {
//!             bot.send_message(msg.chat.id, "Send me a number.").await?;
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! Variant's fields are passed to state handlers as single arguments like
//! `full_name: String` or tuples in case of two or more variant parameters (see
//! below). Using [`Dialogue::update`], you can update the dialogue with a new
//! state, in our case -- `State::ReceiveLocation { full_name, age }`. To exit
//! the dialogue, just call [`Dialogue::exit`] and it will be removed from the
//! underlying storage:
//!
//! ```no_run
//! # use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};
//! # type MyDialogue = Dialogue<State, InMemStorage<State>>;
//! # type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
//! # #[derive(Clone, Debug)] enum State {}
//! async fn receive_location(
//!     bot: Bot,
//!     dialogue: MyDialogue,
//!     (full_name, age): (String, u8), // Available from `State::ReceiveLocation`.
//!     msg: Message,
//! ) -> HandlerResult {
//!     match msg.text() {
//!         Some(location) => {
//!             let message =
//!                 format!("Full name: {}\nAge: {}\nLocation: {}", full_name, age, location);
//!             bot.send_message(msg.chat.id, message).await?;
//!             dialogue.exit().await?;
//!         }
//!         None => {
//!             bot.send_message(msg.chat.id, "Send me a text message.").await?;
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! [`examples/dialogue.rs`]: https://github.com/teloxide/teloxide/blob/master/crates/teloxide/examples/dialogue.rs

#[cfg(feature = "redis-storage")]
pub use self::{RedisStorage, RedisStorageError};

#[cfg(any(feature = "sqlite-storage-nativetls", feature = "sqlite-storage-rustls"))]
pub use self::{SqliteStorage, SqliteStorageError};

#[cfg(feature = "postgres-storage-nativetls")]
pub use self::{PostgresStorage, PostgresStorageError};

pub use get_chat_id::GetChatId;
pub use storage::*;

use dptree::{prelude::DependencyMap, Handler};
use teloxide_core::types::ChatId;

use std::{fmt::Debug, marker::PhantomData, sync::Arc};

use super::DpHandlerDescription;

mod get_chat_id;
mod storage;

/// A handle for controlling dialogue state.
#[derive(Debug)]
pub struct Dialogue<D, S>
where
    S: ?Sized,
{
    storage: Arc<S>,
    chat_id: ChatId,
    _phantom: PhantomData<D>,
}

// `#[derive]` requires generics to implement `Clone`, but `S` is wrapped around
// `Arc`, and `D` is wrapped around PhantomData.
impl<D, S> Clone for Dialogue<D, S>
where
    S: ?Sized,
{
    fn clone(&self) -> Self {
        Dialogue { storage: self.storage.clone(), chat_id: self.chat_id, _phantom: PhantomData }
    }
}

impl<D, S> Dialogue<D, S>
where
    D: Send + 'static,
    S: Storage<D> + ?Sized,
{
    /// Constructs a new dialogue with `storage` (where dialogues are stored)
    /// and `chat_id` of a current dialogue.
    #[must_use]
    pub fn new(storage: Arc<S>, chat_id: ChatId) -> Self {
        Self { storage, chat_id, _phantom: PhantomData }
    }

    /// Returns a chat ID associated with this dialogue.
    #[must_use]
    pub fn chat_id(&self) -> ChatId {
        self.chat_id
    }

    /// Retrieves the current state of the dialogue or `None` if there is no
    /// dialogue.
    pub async fn get(&self) -> Result<Option<D>, S::Error> {
        self.storage.clone().get_dialogue(self.chat_id).await
    }

    /// Like [`Dialogue::get`] but returns a default value if there is no
    /// dialogue.
    pub async fn get_or_default(&self) -> Result<D, S::Error>
    where
        D: Default,
    {
        match self.get().await? {
            Some(d) => Ok(d),
            None => {
                self.storage.clone().update_dialogue(self.chat_id, D::default()).await?;
                Ok(D::default())
            }
        }
    }

    /// Updates the dialogue state.
    ///
    /// The dialogue type `D` must implement `From<State>` to allow implicit
    /// conversion from `State` to `D`.
    pub async fn update<State>(&self, state: State) -> Result<(), S::Error>
    where
        D: From<State>,
    {
        let new_dialogue = state.into();
        self.storage.clone().update_dialogue(self.chat_id, new_dialogue).await?;
        Ok(())
    }

    /// Updates the dialogue with a default value.
    pub async fn reset(&self) -> Result<(), S::Error>
    where
        D: Default,
    {
        self.update(D::default()).await
    }

    /// Removes the dialogue from the storage provided to [`Dialogue::new`].
    pub async fn exit(&self) -> Result<(), S::Error> {
        self.storage.clone().remove_dialogue(self.chat_id).await
    }
}

/// Enters a dialogue context.
///
/// A call to this function is the same as `dptree::entry().enter_dialogue()`.
///
/// See [`HandlerExt::enter_dialogue`].
///
/// ## Dependency requirements
///
///  - `Arc<S>`
///  - `Upd`
///
/// [`HandlerExt::enter_dialogue`]: super::HandlerExt::enter_dialogue
#[must_use]
pub fn enter<Upd, S, D, Output>() -> Handler<'static, DependencyMap, Output, DpHandlerDescription>
where
    S: Storage<D> + ?Sized + Send + Sync + 'static,
    <S as Storage<D>>::Error: Debug + Send,
    D: Default + Send + Sync + 'static,
    Upd: GetChatId + Clone + Send + Sync + 'static,
    Output: Send + Sync + 'static,
{
    dptree::filter_map(|storage: Arc<S>, upd: Upd| {
        let chat_id = upd.chat_id()?;
        Some(Dialogue::new(storage, chat_id))
    })
    .filter_map_async(|dialogue: Dialogue<D, S>| async move {
        match dialogue.get_or_default().await {
            Ok(dialogue) => Some(dialogue),
            Err(err) => {
                log::error!("dialogue.get_or_default() failed: {:?}, falling back to default", err);
                dialogue.update(D::default()).await.ok()?;
                dialogue.get_or_default().await.ok()
            }
        }
    })
}
