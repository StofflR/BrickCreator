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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorScheme {
    pub name: &'static str,
    pub color: &'static str,
    pub shade: &'static str,
    pub border: &'static str,
    pub text: &'static str,
}

impl Default for ColorScheme {
    fn default() -> Self {
        BLUE_SCHEME
    }
}

pub const BLUE_SCHEME: ColorScheme = ColorScheme {
    name: "Blue",
    color: BLUE,
    shade: BLUE_SHADE,
    border: DEFAULT_BORDER,
    text: DEFAULT_TEXT,
};

pub const CYAN_SCHEME: ColorScheme = ColorScheme {
    name: "Cyan",
    color: CYAN,
    shade: CYAN_SHADE,
    border: DEFAULT_BORDER,
    text: DEFAULT_TEXT,
};

pub const DARK_BLUE_SCHEME: ColorScheme = ColorScheme {
    name: "Dark Blue",
    color: DARK_BLUE,
    shade: DARK_BLUE_SHADE,
    border: DEFAULT_BORDER,
    text: DEFAULT_TEXT,
};

pub const GOLD_SCHEME: ColorScheme = ColorScheme {
    name: "Gold",
    color: GOLD,
    shade: GOLD_SHADE,
    border: DEFAULT_BORDER,
    text: DEFAULT_TEXT,
};

pub const DARK_GREEN_SCHEME: ColorScheme = ColorScheme {
    name: "Dark Green",
    color: DARK_GREEN,
    shade: DARK_GREEN_SHADE,
    border: DEFAULT_BORDER,
    text: DEFAULT_TEXT,
};

pub const GREEN_SCHEME: ColorScheme = ColorScheme {
    name: "Green",
    color: GREEN,
    shade: GREEN_SHADE,
    border: DEFAULT_BORDER,
    text: DEFAULT_TEXT,
};

pub const LIGHT_ORANGE_SCHEME: ColorScheme = ColorScheme {
    name: "Light Orange",
    color: LIGHT_ORANGE,
    shade: LIGHT_ORANGE_SHADE,
    border: DEFAULT_BORDER,
    text: DEFAULT_TEXT,
};

pub const OLIVE_SCHEME: ColorScheme = ColorScheme {
    name: "Olive",
    color: OLIVE,
    shade: OLIVE_SHADE,
    border: DEFAULT_BORDER,
    text: DEFAULT_TEXT,
};

pub const ORANGE_SCHEME: ColorScheme = ColorScheme {
    name: "Orange",
    color: ORANGE,
    shade: ORANGE_SHADE,
    border: DEFAULT_BORDER,
    text: DEFAULT_TEXT,
};

pub const YELLOW_SCHEME: ColorScheme = ColorScheme {
    name: "Yellow",
    color: YELLOW,
    shade: YELLOW_SHADE,
    border: DEFAULT_BORDER,
    text: DEFAULT_TEXT,
};

pub const VIOLET_SCHEME: ColorScheme = ColorScheme {
    name: "Violet",
    color: VIOLET,
    shade: VIOLET_SHADE,
    border: DEFAULT_BORDER,
    text: DEFAULT_TEXT,
};

pub const PINK_SCHEME: ColorScheme = ColorScheme {
    name: "Pink",
    color: PINK,
    shade: PINK_SHADE,
    border: DEFAULT_BORDER,
    text: DEFAULT_TEXT,
};

pub const RED_SCHEME: ColorScheme = ColorScheme {
    name: "Red",
    color: RED,
    shade: RED_SHADE,
    border: DEFAULT_BORDER,
    text: DEFAULT_TEXT,
};

pub const WINERED_SCHEME: ColorScheme = ColorScheme {
    name: "Wine Red",
    color: WINERED,
    shade: WINERED_SHADE,
    border: DEFAULT_BORDER,
    text: DEFAULT_TEXT,
};

pub const WHITE_SCHEME: ColorScheme = ColorScheme {
    name: "White",
    color: WHITE,
    shade: WHITE_SHADE,
    border: WHITE_BORDER,
    text: BLUE_TEXT,
};

pub const TRANSPARENT_WHITE_SCHEME: ColorScheme = ColorScheme {
    name: "Transparent White",
    color: TRANSPARENT_WHITE,
    shade: TRANSPARENT_WHITE_SHADE,
    border: TRANSPARENT_WHITE_BORDER,
    text: BLACK_TEXT,
};

pub const TRANSPARENT_BLACK_SCHEME: ColorScheme = ColorScheme {
    name: "Transparent Black",
    color: TRANSPARENT_BLACK,
    shade: TRANSPARENT_BLACK_SHADE,
    border: TRANSPARENT_BLACK_BORDER,
    text: DEFAULT_TEXT,
};

pub const ALL_COLOR_SCHEMES: &[ColorScheme] = &[
    BLUE_SCHEME,
    CYAN_SCHEME,
    DARK_BLUE_SCHEME,
    GOLD_SCHEME,
    DARK_GREEN_SCHEME,
    GREEN_SCHEME,
    LIGHT_ORANGE_SCHEME,
    OLIVE_SCHEME,
    ORANGE_SCHEME,
    YELLOW_SCHEME,
    VIOLET_SCHEME,
    PINK_SCHEME,
    RED_SCHEME,
    WINERED_SCHEME,
    WHITE_SCHEME,
    TRANSPARENT_WHITE_SCHEME,
    TRANSPARENT_BLACK_SCHEME,
];
