use crate::{
    button_attr::ButtonAttrs, command_enum::variants_only_attr, error::compile_error_at,
    fields_parse::ParserType, Result,
};

const DEFAULT_CALLBACK_DATA_SEPARATOR: &str = ";";

pub(crate) struct ButtonEnum {
    pub parser_type: ParserType,
    pub fields_separator: String,
}

impl ButtonEnum {
    pub fn from_attributes(attributes: &[syn::Attribute]) -> Result<Self> {
        let attrs = ButtonAttrs::from_attributes(attributes)?;
        let ButtonAttrs { rename, fields_separator } = attrs;

        variants_only_attr![rename];

        let separator = match fields_separator {
            Some((separator, sp)) => {
                if separator.is_empty() {
                    compile_error_at("Separator can't be empty!", sp);
                }
                separator
            }
            None => String::from(DEFAULT_CALLBACK_DATA_SEPARATOR),
        };

        // We can just always use a separator parser, since the user won't ever interact
        // with that
        Ok(Self {
            parser_type: ParserType::Split { separator: Some(separator.clone()) },
            fields_separator: separator,
        })
    }
}
