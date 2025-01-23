use quote::quote;
use syn::{spanned::Spanned, Fields, FieldsNamed, FieldsUnnamed};

pub(crate) fn impl_stringify_args(
    fields: &Fields,
    self_variant: proc_macro2::TokenStream,
    self_string_name: String,
    self_string_variant: String,
) -> proc_macro2::TokenStream {
    match fields {
        Fields::Unit => {
            quote! { #self_variant => ::std::result::Result::Ok(#self_string_variant.to_owned()), }
        }
        Fields::Unnamed(fields) => {
            impl_stringify_args_unnamed(fields, self_variant, self_string_name, self_string_variant)
        }
        Fields::Named(named) => {
            impl_stringify_args_named(named, self_variant, self_string_name, self_string_variant)
        }
    }
}

pub(crate) fn impl_stringify_args_unnamed(
    data: &FieldsUnnamed,
    variant: proc_macro2::TokenStream,
    self_string_name: String,
    string_variant: String,
) -> proc_macro2::TokenStream {
    let names =
        (0..data.unnamed.len()).map(|i| syn::Ident::new(&format!("field_{}", i), variant.span()));
    let types = data.unnamed.iter().map(|f| &f.ty);
    let mut all_fields = quote! {};
    for ((name, ty), i) in names.clone().zip(types).zip(0..data.unnamed.len()) {
        all_fields.extend(quote! { {
            let stringified = #ty::to_string(&#name);
            if stringified.contains(fields_separator) {
                return ::std::result::Result::Err(StringifyError::SeparatorInUnnamedArgument {
                    enum_variant: std::concat!(#self_string_name, "::", #string_variant).to_owned(),
                    stringified_data: stringified,
                    separator: fields_separator.to_owned(),
                    field: #i,
                });
            }
            stringified
        }, })
    }
    let all_names = quote! { #(#names),* };
    let res = quote! {
        #variant(#all_names) => ::std::result::Result::Ok(
            ::std::vec![#string_variant.to_owned(), #all_fields].join(fields_separator)
        ),
    };
    res
}

pub(crate) fn impl_stringify_args_named(
    data: &FieldsNamed,
    variant: proc_macro2::TokenStream,
    self_string_name: String,
    string_variant: String,
) -> proc_macro2::TokenStream {
    let names = data.named.iter().map(|f| f.ident.as_ref().unwrap());
    let types = data.named.iter().map(|f| &f.ty);
    let mut all_fields = quote! {};
    for (name, ty) in names.clone().zip(types) {
        all_fields.extend(quote! { {
            let stringified = #ty::to_string(&#name);
            if stringified.contains(fields_separator) {
                return ::std::result::Result::Err(StringifyError::SeparatorInNamedArgument {
                    enum_variant: ::std::concat!(#self_string_name, "::", #string_variant).to_owned(),
                    stringified_data: stringified,
                    separator: fields_separator.to_owned(),
                    argument: ::std::stringify!(#name).to_string(),
                });
            }
            stringified
        }, })
    }
    let all_names = quote! { #(#names),* };
    let res = quote! {
        #variant { #all_names } => ::std::result::Result::Ok(
            ::std::vec![#string_variant.to_owned(), #all_fields].join(fields_separator)
        ),
    };
    res
}
