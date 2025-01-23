#[cfg(feature = "macros")]
use teloxide::utils::button::InlineButtons;

// We put tests here because macro expand in unit tests in module
// teloxide::utils::button was a failure

#[test]
#[cfg(feature = "macros")]
fn parse_and_stringify_button_with_args() {
    #[derive(InlineButtons, Debug, PartialEq)]
    enum DefaultData {
        Fruit(String),
        Other,
    }

    let data = "Fruit;apple";
    let expected = DefaultData::Fruit("apple".to_string());
    let actual = DefaultData::parse(data).unwrap();
    assert_eq!(actual, expected);
    assert_eq!(actual.stringify().unwrap(), data.to_owned())
}

#[test]
#[cfg(feature = "macros")]
fn parse_and_stringify_button_with_non_string_arg() {
    #[derive(InlineButtons, Debug, PartialEq)]
    enum DefaultData {
        Fruit(i32),
        Other,
    }

    let data = "Fruit;-50";
    let expected = DefaultData::Fruit("-50".parse().unwrap());
    let actual = DefaultData::parse(data).unwrap();
    assert_eq!(actual, expected);
    assert_eq!(actual.stringify().unwrap(), data.to_owned())
}

#[test]
#[cfg(feature = "macros")]
fn stringify_button_error() {
    use teloxide::utils::button::StringifyError;

    #[derive(InlineButtons, Debug, PartialEq)]
    #[button(fields_separator = ";")]
    enum DefaultData {
        Fruit(String),
        Other,
    }

    let button = DefaultData::Fruit("test;test2".to_string());

    match button.stringify() {
        Err(StringifyError::SeparatorInUnnamedArgument {
            enum_variant,
            stringified_data,
            separator,
            field,
        }) => {
            assert_eq!(field, 0);
            assert_eq!(enum_variant, "DefaultData::Fruit");
            assert_eq!(stringified_data, "test;test2");
            assert_eq!(separator, ";");
        }
        _ => panic!("Expected an error!"),
    }
}

#[test]
#[cfg(feature = "macros")]
fn parse_and_stringify_with_fields_separator1() {
    #[derive(InlineButtons, Debug, PartialEq)]
    #[button(fields_separator = ":")]
    enum DefaultData {
        Other,
    }

    let data = "Other";
    let expected = DefaultData::Other;
    let actual = DefaultData::parse(data).unwrap();
    assert_eq!(actual, expected);
    assert_eq!(actual.stringify().unwrap(), data.to_owned())
}

#[test]
#[cfg(feature = "macros")]
fn parse_and_stringify_with_fields_separator2() {
    #[derive(InlineButtons, Debug, PartialEq)]
    #[button(fields_separator = ":")]
    enum DefaultData {
        Fruit(u8),
        Other,
    }

    let data = "Fruit:10";
    let expected = DefaultData::Fruit(10);
    let actual = DefaultData::parse("Fruit:10").unwrap();
    assert_eq!(actual, expected);
    assert_eq!(actual.stringify().unwrap(), data.to_owned())
}

#[test]
#[cfg(feature = "macros")]
fn parse_and_stringify_named_fields() {
    #[derive(InlineButtons, Debug, PartialEq)]
    enum DefaultData {
        Fruit { num: u8, data: String },
        Other,
    }

    let data = "Fruit;10;hello";
    let expected = DefaultData::Fruit { num: 10, data: "hello".to_string() };
    let actual = DefaultData::parse(data).unwrap();
    assert_eq!(actual, expected);
    assert_eq!(actual.stringify().unwrap(), data.to_owned())
}

#[test]
#[cfg(feature = "macros")]
fn stringify_button_named_fields_error() {
    use teloxide::utils::button::StringifyError;

    #[derive(InlineButtons, Debug, PartialEq)]
    #[button(fields_separator = ";")]
    enum DefaultData {
        Fruit { num: u8, data: String },
        Other,
    }

    let button = DefaultData::Fruit { num: 9, data: "test;test2".to_owned() };
    match button.stringify() {
        Err(StringifyError::SeparatorInNamedArgument {
            enum_variant,
            stringified_data,
            separator,
            argument,
        }) => {
            assert_eq!(argument, "data".to_owned());
            assert_eq!(enum_variant, "DefaultData::Fruit");
            assert_eq!(stringified_data, "test;test2");
            assert_eq!(separator, ";");
        }
        _ => panic!("Expected an error!"),
    }
}
