use serde::{Deserialize, Serialize};

use crate::types::{BackgroundFill, Document};

/// This object describes the type of a background.
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum BackgroundType {
    Fill(BackgroundTypeFill),
    Wallpaper(BackgroundTypeWallpaper),
    Pattern(BackgroundTypePattern),
    ChatTheme(BackgroundTypeChatTheme),
}

/// The background is automatically filled based on the selected colors.
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
pub struct BackgroundTypeFill {
    /// The background fill
    pub fill: BackgroundFill,

    /// Dimming of the background in dark themes, as a percentage; 0-100
    pub dark_theme_dimming: u8,
}

/// The background is a wallpaper in the JPEG format.
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
pub struct BackgroundTypeWallpaper {
    /// Document with the wallpaper
    pub document: Document,

    /// Dimming of the background in dark themes, as a percentage; 0-100
    pub dark_theme_dimming: u8,

    /// `true`, if the wallpaper is downscaled to fit in a 450x450 square and
    /// then box-blurred with radius 12
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_blurred: bool,

    /// `true`, if the background moves slightly when the device is tilted
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_moving: bool,
}

/// The background is a PNG or TGV (gzipped subset of SVG with MIME type
/// “application/x-tgwallpattern”) pattern to be combined with the background
/// fill chosen by the user.
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
pub struct BackgroundTypePattern {
    /// Document with the wallpaper
    pub document: Document,

    /// The background fill
    pub fill: BackgroundFill,

    /// Intensity of the pattern when it is shown above the filled background;
    /// 0-100
    pub intensity: u8,

    /// `true`, if the background fill must be applied only to the pattern
    /// itself. All other pixels are black in this case. For dark themes only
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_inverted: bool,

    /// `true`, if the background moves slightly when the device is tilted
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_moving: bool,
}

/// The background is taken directly from a built-in chat theme.
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
pub struct BackgroundTypeChatTheme {
    /// Name of the chat theme, which is usually an emoji
    pub theme_name: String,
}
