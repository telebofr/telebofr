use serde::{Deserialize, Serialize};

use crate::types::Rgb;

/// This object describes the way a background is filled based on the selected
/// colors.
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum BackgroundFill {
    Solid(BackgroundFillSolid),
    Gradient(BackgroundFillGradient),
    FreeformGradient(BackgroundFillFreeformGradient),
}

/// The background is filled using the selected color.
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub struct BackgroundFillSolid {
    /// The color of the background fill in the RGB24 format
    pub color: Rgb,
}

/// The background is a gradient fill.
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub struct BackgroundFillGradient {
    /// Top color of the gradient in the RGB24 format
    pub top_color: Rgb,

    /// Bottom color of the gradient in the RGB24 format
    pub bottom_color: Rgb,

    /// Clockwise rotation angle of the background fill in degrees; 0-359
    // FIXME: use/add a specialized rotation type?
    pub rotation_angle: u16,
}

/// The background is a freeform gradient that rotates after every message in
/// the chat.
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub struct BackgroundFillFreeformGradient {
    /// A list of the 3 or 4 base colors that are used to generate the freeform
    /// gradient in the RGB24 format
    pub colors: Vec<Rgb>,
}
