use crate::{
    attr::{fold_attrs, Attr},
    error::compile_error_at,
    Result,
};

use proc_macro2::Span;
use syn::Attribute;

/// All attributes that can be used for `derive(InlineButtons)`
pub(crate) struct ButtonAttrs {
    pub rename: Option<(String, Span)>,
    pub fields_separator: Option<(String, Span)>,
}

/// A single k/v attribute for `InlineButtons` derive macro.
///
/// Similar to [`crate::command_attr::CommandAttr`]
struct ButtonAttr {
    kind: ButtonAttrKind,
    sp: Span,
}

/// Kind of [`ButtonAttr`].
enum ButtonAttrKind {
    Rename(String),
    FieldsSeparator(String),
}

impl ButtonAttrs {
    pub fn from_attributes(attributes: &[Attribute]) -> Result<Self> {
        use ButtonAttrKind::*;

        fold_attrs(
            attributes,
            is_button_attribute,
            ButtonAttr::parse,
            Self { rename: None, fields_separator: None },
            |mut this, attr| {
                fn insert<T>(opt: &mut Option<(T, Span)>, x: T, sp: Span) -> Result<()> {
                    match opt {
                        slot @ None => {
                            *slot = Some((x, sp));
                            Ok(())
                        }
                        Some(_) => Err(compile_error_at("duplicate attribute", sp)),
                    }
                }

                match attr.kind {
                    Rename(r) => insert(&mut this.rename, r, attr.sp),
                    FieldsSeparator(s) => insert(&mut this.fields_separator, s, attr.sp),
                }?;

                Ok(this)
            },
        )
    }
}

impl ButtonAttr {
    fn parse(attr: Attr) -> Result<Self> {
        use ButtonAttrKind::*;

        let sp = attr.span();
        let Attr { mut key, value } = attr;

        let outermost_key = key.pop().unwrap(); // `Attr`'s invariants ensure `key.len() > 0`

        let kind = match &*outermost_key.to_string() {
            "button" => {
                let Some(attr) = key.pop() else {
                    return Err(compile_error_at(
                        "expected an attribute name",
                        outermost_key.span(),
                    ));
                };

                if let Some(unexpected_key) = key.last() {
                    return Err(compile_error_at(
                        &format!("{attr} can't have nested attributes"),
                        unexpected_key.span(),
                    ));
                }

                match &*attr.to_string() {
                    "rename" => Rename(value.expect_string()?),
                    "fields_separator" => FieldsSeparator(value.expect_string()?),
                    _ => {
                        return Err(compile_error_at(
                            "unexpected attribute name (expected one of `rename` or \
                             `fields_separator`",
                            attr.span(),
                        ))
                    }
                }
            }

            _ => {
                return Err(compile_error_at(
                    "unexpected attribute (expected `button`)",
                    outermost_key.span(),
                ))
            }
        };

        Ok(Self { kind, sp })
    }
}

fn is_button_attribute(a: &Attribute) -> bool {
    matches!(a.path.get_ident(), Some(ident) if ident == "button")
}
