//! Core part of the [`teloxide`] library.
//!
//! This library provides tools for making requests to the [Telegram Bot API]
//! (Currently, version `7.3` is supported) with ease. The library is fully
//! asynchronous and built using [`tokio`].
//!
//!```toml
//! teloxide-core = "0.10.1"
//! ```
//! _Compiler support: requires rustc 1.80+_.
//!
//! ```
//! # async {
//! # let chat_id = teloxide_core::types::ChatId(-1);
//! use teloxide_core::{
//!     prelude::*,
//!     types::{DiceEmoji, ParseMode},
//! };
//!
//! let bot = Bot::from_env().parse_mode(ParseMode::MarkdownV2);
//!
//! let me = bot.get_me().await?;
//!
//! bot.send_dice(chat_id).emoji(DiceEmoji::Dice).await?;
//! bot.send_message(chat_id, format!("Hi, my name is **{}** 👋", me.user.first_name)).await?;
//! # Ok::<_, Box<dyn std::error::Error>>(()) };
//! ```
//!
//! <div align="center">
//!     <img src=https://user-images.githubusercontent.com/38225716/103929465-6b91e100-512e-11eb-826d-39b096f16548.gif />
//! </div>
//!
//! [`teloxide`]: https://docs.rs/teloxide
//! [Telegram Bot API]: https://core.telegram.org/bots/api
//! [`tokio`]: https://tokio.rs
//!
//! ## Cargo features
//!
//! - `native-tls` = use [`native-tls`] tls implementation (**enabled by
//!   default**)
//! - `rustls` — use [`rustls`] tls implementation
//! - `trace_adaptor` — enables [`Trace`] bot adaptor
//! - `erased` — enables [`ErasedRequester`] bot adaptor
//! - `throttle` — enables [`Throttle`] bot adaptor
//! - `cache_me` — enables [`CacheMe`] bot adaptor
//! - `full` — enables all features except `nightly` and tls-related
//! - `nightly` — enables nightly-only features, currently:
//!   - Removes some future boxing using `#![feature(type_alias_impl_trait)]`
//!   - Used to built docs (`#![feature(doc_cfg, doc_notable_trait)]`)
//!
//! [`AutoSend`]: adaptors::AutoSend
//! [`Trace`]: adaptors::Trace
//! [`ErasedRequester`]: adaptors::ErasedRequester
//! [`Throttle`]: adaptors::Throttle
//! [`CacheMe`]: adaptors::CacheMe
//! [`native-tls`]: https://docs.rs/native-tls
//! [`rustls`]: https://docs.rs/rustls

#![doc(
    // FIXME(waffle): use github
    html_logo_url = "https://cdn.discordapp.com/attachments/224881373326999553/798598120760934410/logo.png",
    html_favicon_url = "https://cdn.discordapp.com/attachments/224881373326999553/798598120760934410/logo.png"
)]
//
// we pass "--cfg docsrs" when building docs to add `This is supported on feature="..." only.`
//
// To properly build docs of this crate run
// ```console
// $ cargo docs
// ```
// (docs alias is defined in `.cargo/config.toml`)
//
// `dep_docsrs` is used for the same purpose, but when `teloxide-core` is built as a dependency
// (see: `teloxide`). We can't use `docsrs` as it breaks tokio compilation in this case.
#![cfg_attr(
    all(any(docsrs, dep_docsrs), feature = "nightly"),
    feature(doc_cfg, doc_auto_cfg, doc_notable_trait)
)]
#![cfg_attr(feature = "nightly", feature(type_alias_impl_trait))]
#![cfg_attr(all(feature = "full", docsrs), deny(rustdoc::broken_intra_doc_links))]
//
// Lint levels
#![forbid(unsafe_code)]
//#![deny(missing_docs)]
#![warn(clippy::print_stdout, clippy::dbg_macro)]
#![allow(
    // Sometimes it's more readable to assign to a variable and return it immediately
    clippy::let_and_return,

    // When you are testing ->bool functions, it makes sense to `assert_eq!(f(..), false)`
    clippy::bool_assert_comparison,

    // Unless this becomes machine applicable, I'm not adding 334 #[must_use]s (waffle)
    clippy::return_self_not_must_use,

    // This is dumb. `T: ?Sized where T: Trait` IMO makes perfect sense
    clippy::multiple_bound_locations,

    // Workaround for CI
    // FIXME: do we still need this?
    rustdoc::bare_urls,

    // FIXME: deal with these lints
    clippy::collapsible_str_replace,
    clippy::borrow_deref_ref,
    clippy::unnecessary_lazy_evaluations,
    clippy::derive_partial_eq_without_eq
)]

// The internal helper macros.
#[macro_use]
mod local_macros;

pub use self::{
    bot::Bot,
    errors::{ApiError, DownloadError, RequestError},
};

pub mod adaptors;
pub mod errors;
pub mod net;
pub mod payloads;
pub mod prelude;
pub mod requests;
pub mod types;

// reexported
mod bot;

// implementation details
mod serde_multipart;
mod util;

#[cfg(test)]
mod codegen;
