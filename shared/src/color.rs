use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

// Default color constants
pub const BLUE: &str = "#408ac5";
pub const BLUE_SHADE: &str = "#27567c";
pub const CYAN: &str = "#26a6ae";
pub const CYAN_SHADE: &str = "#2e7078";
pub const DARK_BLUE: &str = "#395cab";
pub const DARK_BLUE_SHADE: &str = "#889dcd";
pub const GOLD: &str = "#95750c";
pub const GOLD_SHADE: &str = "#57452c";
pub const DARK_GREEN: &str = "#305716";
pub const DARK_GREEN_SHADE: &str = "#173718";
pub const GREEN: &str = "#6b9c49";
pub const GREEN_SHADE: &str = "#486822";
pub const LIGHT_ORANGE: &str = "#f99761";
pub const LIGHT_ORANGE_SHADE: &str = "#a86d45";
pub const OLIVE: &str = "#aea626";
pub const OLIVE_SHADE: &str = "#7e7a30";
pub const ORANGE: &str = "#cf5717";
pub const ORANGE_SHADE: &str = "#7a3a18";
pub const YELLOW: &str = "#fccb41";
pub const YELLOW_SHADE: &str = "#aa8832";
pub const VIOLET: &str = "#8f4cba";
pub const VIOLET_SHADE: &str = "#5d2d7c";
pub const PINK: &str = "#cf7aa6";
pub const PINK_SHADE: &str = "#935e7b";
pub const RED: &str = "#f24e50";
pub const RED_SHADE: &str = "#ae2f2f";
pub const WINERED: &str = "#910d06";
pub const WINERED_SHADE: &str = "#750701";
pub const WHITE: &str = "#ffffff";
pub const WHITE_SHADE: &str = "#a9b4cd";
pub const WHITE_BORDER: &str = "#274383";
pub const DEFAULT_BORDER: &str = "#383838";

// Transparent colors (RGBA hex format)
pub const TRANSPARENT_WHITE: &str = "#ffffffff";
pub const TRANSPARENT_WHITE_SHADE: &str = "#ffffffff";
pub const TRANSPARENT_WHITE_BORDER: &str = "#ffffffff";
pub const TRANSPARENT_BLACK: &str = "#000000ff";
pub const TRANSPARENT_BLACK_SHADE: &str = "#000000ff";
pub const TRANSPARENT_BLACK_BORDER: &str = "#000000ff";

// Text colors
pub const BLACK_TEXT: &str = "#000000";
pub const BLUE_TEXT: &str = "#0000ff";
pub const DEFAULT_TEXT: &str = "#ffffff";

// Color scheme definitions
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ColorScheme {
    pub name: String,
    pub color: String,
    pub shade: String,
    pub border: String,
    pub text: String,
}

impl Default for ColorScheme {
    fn default() -> Self {
        BLUE_SCHEME.clone()
    }
}

pub static BLUE_SCHEME: Lazy<ColorScheme> = Lazy::new(|| ColorScheme {
    name: "Blue".to_string(),
    color: BLUE.to_string(),
    shade: BLUE_SHADE.to_string(),
    border: DEFAULT_BORDER.to_string(),
    text: DEFAULT_TEXT.to_string(),
});

pub static CYAN_SCHEME: Lazy<ColorScheme> = Lazy::new(|| ColorScheme {
    name: "Cyan".to_string(),
    color: CYAN.to_string(),
    shade: CYAN_SHADE.to_string(),
    border: DEFAULT_BORDER.to_string(),
    text: DEFAULT_TEXT.to_string(),
});

pub static DARK_BLUE_SCHEME: Lazy<ColorScheme> = Lazy::new(|| ColorScheme {
    name: "Dark Blue".to_string(),
    color: DARK_BLUE.to_string(),
    shade: DARK_BLUE_SHADE.to_string(),
    border: DEFAULT_BORDER.to_string(),
    text: DEFAULT_TEXT.to_string(),
});

pub static GOLD_SCHEME: Lazy<ColorScheme> = Lazy::new(|| ColorScheme {
    name: "Gold".to_string(),
    color: GOLD.to_string(),
    shade: GOLD_SHADE.to_string(),
    border: DEFAULT_BORDER.to_string(),
    text: DEFAULT_TEXT.to_string(),
});

pub static DARK_GREEN_SCHEME: Lazy<ColorScheme> = Lazy::new(|| ColorScheme {
    name: "Dark Green".to_string(),
    color: DARK_GREEN.to_string(),
    shade: DARK_GREEN_SHADE.to_string(),
    border: DEFAULT_BORDER.to_string(),
    text: DEFAULT_TEXT.to_string(),
});

pub static GREEN_SCHEME: Lazy<ColorScheme> = Lazy::new(|| ColorScheme {
    name: "Green".to_string(),
    color: GREEN.to_string(),
    shade: GREEN_SHADE.to_string(),
    border: DEFAULT_BORDER.to_string(),
    text: DEFAULT_TEXT.to_string(),
});

pub static LIGHT_ORANGE_SCHEME: Lazy<ColorScheme> = Lazy::new(|| ColorScheme {
    name: "Light Orange".to_string(),
    color: LIGHT_ORANGE.to_string(),
    shade: LIGHT_ORANGE_SHADE.to_string(),
    border: DEFAULT_BORDER.to_string(),
    text: DEFAULT_TEXT.to_string(),
});

pub static OLIVE_SCHEME: Lazy<ColorScheme> = Lazy::new(|| ColorScheme {
    name: "Olive".to_string(),
    color: OLIVE.to_string(),
    shade: OLIVE_SHADE.to_string(),
    border: DEFAULT_BORDER.to_string(),
    text: DEFAULT_TEXT.to_string(),
});

pub static ORANGE_SCHEME: Lazy<ColorScheme> = Lazy::new(|| ColorScheme {
    name: "Orange".to_string(),
    color: ORANGE.to_string(),
    shade: ORANGE_SHADE.to_string(),
    border: DEFAULT_BORDER.to_string(),
    text: DEFAULT_TEXT.to_string(),
});

pub static YELLOW_SCHEME: Lazy<ColorScheme> = Lazy::new(|| ColorScheme {
    name: "Yellow".to_string(),
    color: YELLOW.to_string(),
    shade: YELLOW_SHADE.to_string(),
    border: DEFAULT_BORDER.to_string(),
    text: DEFAULT_TEXT.to_string(),
});

pub static VIOLET_SCHEME: Lazy<ColorScheme> = Lazy::new(|| ColorScheme {
    name: "Violet".to_string(),
    color: VIOLET.to_string(),
    shade: VIOLET_SHADE.to_string(),
    border: DEFAULT_BORDER.to_string(),
    text: DEFAULT_TEXT.to_string(),
});

pub static PINK_SCHEME: Lazy<ColorScheme> = Lazy::new(|| ColorScheme {
    name: "Pink".to_string(),
    color: PINK.to_string(),
    shade: PINK_SHADE.to_string(),
    border: DEFAULT_BORDER.to_string(),
    text: DEFAULT_TEXT.to_string(),
});

pub static RED_SCHEME: Lazy<ColorScheme> = Lazy::new(|| ColorScheme {
    name: "Red".to_string(),
    color: RED.to_string(),
    shade: RED_SHADE.to_string(),
    border: DEFAULT_BORDER.to_string(),
    text: DEFAULT_TEXT.to_string(),
});

pub static WINERED_SCHEME: Lazy<ColorScheme> = Lazy::new(|| ColorScheme {
    name: "Wine Red".to_string(),
    color: WINERED.to_string(),
    shade: WINERED_SHADE.to_string(),
    border: DEFAULT_BORDER.to_string(),
    text: DEFAULT_TEXT.to_string(),
});

pub static WHITE_SCHEME: Lazy<ColorScheme> = Lazy::new(|| ColorScheme {
    name: "White".to_string(),
    color: WHITE.to_string(),
    shade: WHITE_SHADE.to_string(),
    border: WHITE_BORDER.to_string(),
    text: BLUE_TEXT.to_string(),
});

pub static TRANSPARENT_WHITE_SCHEME: Lazy<ColorScheme> = Lazy::new(|| ColorScheme {
    name: "Transparent White".to_string(),
    color: TRANSPARENT_WHITE.to_string(),
    shade: TRANSPARENT_WHITE_SHADE.to_string(),
    border: TRANSPARENT_WHITE_BORDER.to_string(),
    text: BLACK_TEXT.to_string(),
});

pub static TRANSPARENT_BLACK_SCHEME: Lazy<ColorScheme> = Lazy::new(|| ColorScheme {
    name: "Transparent Black".to_string(),
    color: TRANSPARENT_BLACK.to_string(),
    shade: TRANSPARENT_BLACK_SHADE.to_string(),
    border: TRANSPARENT_BLACK_BORDER.to_string(),
    text: DEFAULT_TEXT.to_string(),
});

pub static ALL_COLOR_SCHEMES: Lazy<Vec<ColorScheme>> = Lazy::new(|| {
    vec![
        BLUE_SCHEME.clone(),
        CYAN_SCHEME.clone(),
        DARK_BLUE_SCHEME.clone(),
        GOLD_SCHEME.clone(),
        DARK_GREEN_SCHEME.clone(),
        GREEN_SCHEME.clone(),
        LIGHT_ORANGE_SCHEME.clone(),
        OLIVE_SCHEME.clone(),
        ORANGE_SCHEME.clone(),
        YELLOW_SCHEME.clone(),
        VIOLET_SCHEME.clone(),
        PINK_SCHEME.clone(),
        RED_SCHEME.clone(),
        WINERED_SCHEME.clone(),
        WHITE_SCHEME.clone(),
        TRANSPARENT_WHITE_SCHEME.clone(),
        TRANSPARENT_BLACK_SCHEME.clone(),
    ]
});
