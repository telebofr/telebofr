// waffle: efficiency is not important here, and I don't want to rewrite this
#![allow(clippy::format_collect)]

use std::{borrow::Borrow, collections::HashSet, ops::Deref};

use itertools::Itertools;

use crate::codegen::{
    add_preamble,
    convert::{convert_for, Convert},
    ensure_files_contents, project_root, reformat,
    schema::{self, Doc, Method, Param, Type},
    to_uppercase,
};

#[test]
fn codegen_payloads() {
    let base_path = project_root().join("src/payloads/");
    let schema = schema::get();

    let mut files = Vec::new();

    for method in schema.methods {
        let file_name = format!("{}.rs", method.names.2);
        let path = base_path.join(&*file_name);

        let uses = uses(&method);

        let method_doc = render_doc(&method.doc, method.sibling.as_deref());
        let eq_hash_derive = eq_hash_suitable(&method).then(|| " Eq, Hash,").unwrap_or("");
        let default_derive = default_needed(&method).then(|| " Default,").unwrap_or("");

        let return_ty = method.return_ty.to_string();

        let required = params(method.params.iter().filter(|p| !matches!(&p.ty, Type::Option(_))));
        let required = match &*required {
            "" => "".to_owned(),
            _ => format!("        required {{\n{required}\n        }}"),
        };

        let optional = params(method.params.iter().filter_map(|p| match &p.ty {
            Type::Option(inner) => Some(Param {
                name: p.name.clone(),
                ty: inner.deref().clone(),
                descr: p.descr.clone(),
            }),
            _ => None,
        }));
        let optional = match &*optional {
            "" => "".to_owned(),
            _ if required.is_empty() => format!("        optional {{\n{optional}\n        }}"),
            _ => format!("\n        optional {{\n{optional}\n        }}"),
        };

        let multipart = multipart_input_file_fields(&method)
            .map(|field| format!("    @[multipart = {}]\n", field.join(", ")))
            .unwrap_or_default();

        // FIXME: CreateNewStickerSet has to be be only Debug + Clone + Serialize (maybe
        // better fix?)
        let derive = if !multipart.is_empty()
            || matches!(
                &*method.names.1,
                "SendMediaGroup"
                    | "EditMessageMedia"
                    | "EditMessageMediaInline"
                    | "CreateNewStickerSet"
            ) {
            "#[derive(Debug, Clone, Serialize)]".to_owned()
        } else {
            format!("#[derive(Debug, PartialEq,{eq_hash_derive}{default_derive} Clone, Serialize)]")
        };

        let timeout_secs = match &*method.names.2 {
            "get_updates" => "    @[timeout_secs = timeout]\n",
            _ => "",
        };

        let contents = format!(
            "\
{uses}

impl_payload! {{
{multipart}{timeout_secs}{method_doc}
    {derive}
    pub {Method} ({Method}Setters) => {return_ty} {{
{required}{optional}
    }}
}}
",
            Method = method.names.1,
        );

        files.push((path, reformat(add_preamble("codegen_payloads", contents))));
    }

    ensure_files_contents(files.iter().map(|(p, c)| (&**p, &**c)))
}

fn uses(method: &Method) -> String {
    enum Use {
        Prelude,
        Crate(String),
        External(String),
    }

    fn ty_use(ty: &Type) -> Use {
        match ty {
            Type::True => Use::Crate(String::from("use crate::types::True;")),
            Type::u8
            | Type::u16
            | Type::u32
            | Type::i32
            | Type::u64
            | Type::i64
            | Type::f64
            | Type::bool
            | Type::String => Use::Prelude,
            Type::Option(inner) | Type::ArrayOf(inner) => ty_use(inner),
            Type::RawTy(raw) => Use::Crate(["use crate::types::", raw, ";"].concat()),
            Type::Url => Use::External(String::from("use url::Url;")),
            Type::DateTime => Use::External(String::from("use chrono::{DateTime, Utc};")),
            Type::Box(raw) => Use::Crate(["use crate::types::", raw, ";"].concat()),
        }
    }

    let mut crate_uses = HashSet::new();
    let mut external_uses = HashSet::new();

    external_uses.insert(String::from("use serde::Serialize;"));

    core::iter::once(&method.return_ty)
        .chain(method.params.iter().map(|p| &p.ty))
        .map(ty_use)
        .for_each(|u| match u {
            Use::Prelude => {}
            Use::Crate(u) => {
                crate_uses.insert(u);
            }
            Use::External(u) => {
                external_uses.insert(u);
            }
        });

    let external_uses = external_uses.into_iter().join("\n");

    if crate_uses.is_empty() {
        external_uses
    } else {
        let crate_uses = crate_uses.into_iter().join("");

        format!("{external_uses}\n\n{crate_uses}",)
    }
}

fn render_doc(doc: &Doc, sibling: Option<&str>) -> String {
    let links = match &doc.md_links {
        links if links.is_empty() => String::new(),
        links => {
            let l: String =
                links.iter().map(|(name, link)| format!("\n    /// [{name}]: {link}")).collect();

            format!("\n    ///{l}")
        }
    };

    let sibling_note = sibling
        .map(|s| {
            format!(
                "\n    /// \n    /// See also: [`{s}`](crate::payloads::{s})",
                s = to_uppercase(s)
            )
        })
        .unwrap_or_default();

    ["    /// ", &doc.md.replace('\n', "\n    /// "), &sibling_note, &links].concat()
}

fn eq_hash_suitable(method: &Method) -> bool {
    fn ty_eq_hash_suitable(ty: &Type) -> bool {
        match ty {
            Type::f64 => false,
            Type::Option(inner) | Type::ArrayOf(inner) => ty_eq_hash_suitable(&*inner),

            Type::True
            | Type::u8
            | Type::u16
            | Type::u32
            | Type::i32
            | Type::u64
            | Type::i64
            | Type::bool
            | Type::String => true,

            Type::Url | Type::DateTime => true,

            Type::RawTy(raw) => {
                raw != "InputSticker" && raw != "MaskPosition" && raw != "InlineQueryResult"
            }

            Type::Box(_) => true,
        }
    }

    method.params.iter().all(|p| ty_eq_hash_suitable(&p.ty))
}

fn default_needed(method: &Method) -> bool {
    method.params.iter().all(|p| matches!(p.ty, Type::Option(_)))
}

fn params(params: impl Iterator<Item = impl Borrow<Param>>) -> String {
    params
        .map(|param| {
            let param = param.borrow();
            let doc = render_doc(&param.descr, None).replace('\n', "\n        ");
            let field = &param.name;
            let ty = &param.ty;
            let flatten = match ty {
                Type::RawTy(s) if s == "MessageId" && field == "reply_to_message_id" => {
                    "\n            #[serde(serialize_with = \
                     \"crate::types::serialize_reply_to_message_id\")]"
                }
                Type::RawTy(s)
                    if s == "MessageId" || s == "TargetMessage" || s == "StickerType" =>
                {
                    "\n            #[serde(flatten)]"
                }
                _ => "",
            };
            let with = match ty {
                Type::DateTime => {
                    "\n            #[serde(with = \
                     \"crate::types::serde_opt_date_from_unix_timestamp\")]"
                }
                _ => "",
            };
            let rename = match field.strip_suffix('_') {
                Some(field) => format!("\n            #[serde(rename = \"{field}\")]"),
                None => "".to_owned(),
            };
            let convert = match convert_for(ty) {
                Convert::Id(_) => "",
                Convert::Into(_) => " [into]",
                Convert::Collect(_) => " [collect]",
            };
            format!("        {doc}{flatten}{with}{rename}\n            pub {field}: {ty}{convert},")
        })
        .join("\n")
}

fn multipart_input_file_fields(m: &Method) -> Option<Vec<&str>> {
    let fields: Vec<_> =
        m.params.iter().filter(|&p| ty_is_multiparty(&p.ty)).map(|p| &*p.name).collect();

    if fields.is_empty() {
        None
    } else {
        Some(fields)
    }
}

fn ty_is_multiparty(ty: &Type) -> bool {
    matches!(ty, Type::RawTy(x) if x == "InputFile" || x == "InputSticker")
        || matches!(ty, Type::Option(inner) if ty_is_multiparty(inner))
}
