#[cfg(feature = "macros")]
use teloxide::utils::button::InlineButtons;

// We put tests here because macro expand in unit tests in module
// teloxide::utils::button was a failure

#[cfg(feature = "macros")]
#[derive(InlineButtons, Debug, PartialEq)]
enum CallbackButtons {
    Button1,
    Button2(String),
    Button3 { field1: u32 },
}

#[test]
#[cfg(feature = "macros")]
fn test_make_button() {
    use teloxide::types::InlineKeyboardButton;

    let text = "Text for button 1";
    let actual = CallbackButtons::Button1.build_button(text).unwrap();
    let expected = InlineKeyboardButton::callback(text, "Button1");
    assert_eq!(actual, expected);
}

#[test]
#[cfg(feature = "macros")]
fn test_make_button_with_unnamed_args() {
    use teloxide::types::InlineKeyboardButton;

    let text = "Text for button 2";
    let actual = CallbackButtons::Button2("data".to_owned()).build_button(text).unwrap();
    let expected = InlineKeyboardButton::callback(text, "Button2;data");
    assert_eq!(actual, expected);
}

#[test]
#[cfg(feature = "macros")]
fn test_make_button_with_named_args() {
    use teloxide::types::InlineKeyboardButton;

    let text = "Text for button 3";
    let actual = CallbackButtons::Button3 { field1: 23 }.build_button(text).unwrap();
    let expected = InlineKeyboardButton::callback(text, "Button3;23");
    assert_eq!(actual, expected);
}
