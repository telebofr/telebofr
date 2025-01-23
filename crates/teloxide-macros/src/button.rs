use crate::{button_attr::ButtonAttrs, button_enum::ButtonEnum, fields_parse::ParserType, Result};

pub(crate) struct Button {
    /// Callback data name, with all renames already applied.
    pub data_name: String,
    pub parser: ParserType,
}

impl Button {
    pub fn new(
        data_name: &str,
        attributes: &[syn::Attribute],
        global_options: &ButtonEnum,
    ) -> Result<Self> {
        let attrs = ButtonAttrs::from_attributes(attributes)?;
        let ButtonAttrs {
            rename,
            // FIXME: error on/do not ignore separator
            fields_separator: _,
        } = attrs;

        let data_name = match rename {
            Some((rename, _)) => rename,
            None => String::from(data_name),
        };

        Ok(Self { data_name, parser: global_options.parser_type.clone() })
    }
}
