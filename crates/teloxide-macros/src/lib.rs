extern crate proc_macro;

mod attr;
mod bot_commands;
mod button;
mod button_attr;
mod button_enum;
mod command;
mod command_attr;
mod command_enum;
mod error;
mod fields_parse;
mod fields_stringify;
mod inline_buttons;
mod rename_rules;
mod unzip;

use crate::inline_buttons::inline_buttons_impl;
pub(crate) use error::{compile_error, Result};
use syn::{parse_macro_input, DeriveInput};

use crate::bot_commands::bot_commands_impl;
use proc_macro::TokenStream;

#[proc_macro_derive(BotCommands, attributes(command))]
pub fn bot_commands_derive(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);

    bot_commands_impl(input).unwrap_or_else(<_>::into).into()
}

#[proc_macro_derive(InlineButtons, attributes(button))]
pub fn callback_data_derive(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);

    inline_buttons_impl(input).unwrap_or_else(<_>::into).into()
}
