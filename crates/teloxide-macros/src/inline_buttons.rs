use proc_macro2::TokenStream;
use quote::quote;
use syn::{spanned::Spanned, DeriveInput};

use crate::{
    button::Button, button_enum::ButtonEnum, compile_error, error::compile_error_at,
    fields_parse::impl_parse_args, fields_stringify::impl_stringify_args, unzip::Unzip3, Result,
};

pub(crate) fn inline_buttons_impl(input: DeriveInput) -> Result<TokenStream> {
    let data_enum = get_enum_data(&input)?;
    let button_enum = ButtonEnum::from_attributes(&input.attrs)?;
    let type_name = &input.ident;

    let Unzip3(var_init, var_info, var_stringify) = data_enum
        .variants
        .iter()
        .map(|variant| {
            let button = Button::new(&variant.ident.to_string(), &variant.attrs, &button_enum)?;

            if button.data_name.len() > 64 {
                return Err(compile_error_at(
                    // Limit of the TBA
                    //
                    // "64 chars" and not "64 bytes" because enum variants usually don't contain
                    // non-ascii characters, and it's more understandable.
                    "Enum variant is too long (64 chars max), please consider using \
                     `#[button(rename=\"...\")` to reduce the size",
                    variant.span(),
                ));
            }

            let variant_name = &variant.ident;
            let self_variant = quote! { Self::#variant_name };
            let button_data_name = button.data_name.clone();
            let self_string_variant = button_data_name.to_owned();
            let self_string_name = type_name.to_string();

            let parse = impl_parse_args(&variant.fields, self_variant.clone(), &button.parser);
            let stringify = impl_stringify_args(
                &variant.fields,
                self_variant,
                self_string_name,
                self_string_variant,
            );

            Ok((parse, button, stringify))
        })
        .collect::<Result<Unzip3<Vec<_>, Vec<_>, Vec<_>>>>()?;

    let fn_parse = impl_parse(&var_info, &var_init, &button_enum.fields_separator);
    let fn_stringify = impl_stringify(&var_stringify, &button_enum.fields_separator);

    let trait_impl = quote! {
        impl teloxide::utils::button::InlineButtons for #type_name {
            #fn_parse
            #fn_stringify
        }
    };

    Ok(trait_impl)
}

fn impl_parse(
    infos: &[Button],
    variants_initialization: &[proc_macro2::TokenStream],
    fields_separator: &str,
) -> proc_macro2::TokenStream {
    let matching_values = infos.iter().map(|c| c.data_name.clone());

    quote! {
         fn parse(s: &str) -> ::std::result::Result<Self, teloxide::utils::command::ParseError> {
              use std::str::FromStr;
              use teloxide::utils::command::ParseError;

              // 2 is used to only split once (=> in two parts),
              // we only need to split the enum variant and the rest of arguments.
              let mut words = s.splitn(2, #fields_separator);

              let enum_variant = words.next().unwrap();

              let args = words.next().unwrap_or("").to_owned();
              match enum_variant {
                   #(
                        #matching_values => Ok(#variants_initialization),
                   )*
                   _ => ::std::result::Result::Err(ParseError::UnknownCallbackDataVariant(enum_variant.to_owned())),
              }
         }
    }
}

fn impl_stringify(
    stringify_return: &[proc_macro2::TokenStream],
    fields_separator: &str,
) -> proc_macro2::TokenStream {
    quote! {
        fn stringify(self) -> ::std::result::Result<::std::string::String, teloxide::utils::button::StringifyError> {
            use std::string::ToString;
            use teloxide::utils::button::StringifyError;

            let fields_separator = #fields_separator;

            match self {
                #(#stringify_return)*
            }
        }
    }
}

fn get_enum_data(input: &DeriveInput) -> Result<&syn::DataEnum> {
    match &input.data {
        syn::Data::Enum(data) => Ok(data),
        _ => Err(compile_error("`InlineButtons` is only allowed for enums")),
    }
}
