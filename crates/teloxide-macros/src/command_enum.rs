use crate::{
    command_attr::CommandAttrs, error::compile_error_at, fields_parse::ParserType,
    rename_rules::RenameRule, Result,
};

/// Create a if block that checks if the given attribute is applied to a enum
/// itself, if so, return an error
macro_rules! variants_only_attr {
    ($($attr: ident),+) => {
        $(
            if let Some((_, sp)) = $attr {
                return Err(compile_error_at(
                    concat!("`", stringify!($attr), "` attribute can only be applied to enums *variants*"),
                    sp,
                ));
            }
        )+
    };
}

pub(crate) use variants_only_attr;

pub(crate) struct CommandEnum {
    pub prefix: String,
    /// The bool is true if the description contains a doc comment
    pub description: Option<(String, bool)>,
    pub command_separator: String,
    pub rename_rule: RenameRule,
    pub parser_type: ParserType,
}

impl CommandEnum {
    pub fn from_attributes(attributes: &[syn::Attribute]) -> Result<Self> {
        let attrs = CommandAttrs::from_attributes(attributes)?;
        let CommandAttrs {
            prefix,
            description,
            rename_rule,
            rename,
            parser,
            aliases,
            command_separator,
            separator,
            hide,
            hide_aliases,
        } = attrs;

        variants_only_attr![rename, hide, hide_aliases, aliases];

        let mut parser = parser.map(|(p, _)| p).unwrap_or(ParserType::Default);

        // FIXME: Error on unused separator
        if let (ParserType::Split { separator }, Some((s, _))) = (&mut parser, &separator) {
            *separator = Some(s.clone())
        }

        Ok(Self {
            prefix: prefix.map(|(p, _)| p).unwrap_or_else(|| "/".to_owned()),
            description: description.map(|(d, is_doc, _)| (d, is_doc)),
            command_separator: command_separator
                .map(|(s, _)| s)
                .unwrap_or_else(|| String::from(" ")),
            rename_rule: rename_rule.map(|(rr, _)| rr).unwrap_or(RenameRule::Identity),
            parser_type: parser,
        })
    }
}
