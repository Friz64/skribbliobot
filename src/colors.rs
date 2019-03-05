#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub struct ColorCoord {
    pub x: u32,
    pub y: u32,
}

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub const WHITE: Color = Color {
    r: 255,
    g: 255,
    b: 255,
};
pub const LIGHT_GREY: Color = Color {
    r: 139,
    g: 139,
    b: 139,
};
pub const LIGHT_RED: Color = Color {
    r: 239,
    g: 19,
    b: 11,
};
pub const LIGHT_ORANGE: Color = Color {
    r: 255,
    g: 113,
    b: 0,
};
pub const LIGHT_YELLOW: Color = Color {
    r: 255,
    g: 228,
    b: 0,
};
pub const LIGHT_GREEN: Color = Color { r: 0, g: 204, b: 0 };
pub const LIGHT_CYAN: Color = Color {
    r: 0,
    g: 178,
    b: 255,
};
pub const LIGHT_BLUE: Color = Color {
    r: 35,
    g: 31,
    b: 211,
};
pub const LIGHT_MAGENTA: Color = Color {
    r: 163,
    g: 0,
    b: 186,
};
pub const LIGHT_PINK: Color = Color {
    r: 211,
    g: 124,
    b: 170,
};
pub const LIGHT_BROWN: Color = Color {
    r: 160,
    g: 82,
    b: 45,
};
pub const BLACK: Color = Color { r: 0, g: 0, b: 0 };
pub const DARK_GREY: Color = Color {
    r: 76,
    g: 76,
    b: 76,
};
pub const DARK_RED: Color = Color {
    r: 116,
    g: 11,
    b: 7,
};
pub const DARK_ORANGE: Color = Color {
    r: 194,
    g: 56,
    b: 0,
};
pub const DARK_YELLOW: Color = Color {
    r: 232,
    g: 162,
    b: 0,
};
pub const DARK_GREEN: Color = Color { r: 0, g: 85, b: 16 };
pub const DARK_CYAN: Color = Color {
    r: 0,
    g: 86,
    b: 158,
};
pub const DARK_BLUE: Color = Color {
    r: 14,
    g: 8,
    b: 101,
};
pub const DARK_MAGENTA: Color = Color {
    r: 85,
    g: 0,
    b: 105,
};
pub const DARK_PINK: Color = Color {
    r: 167,
    g: 85,
    b: 116,
};
pub const DARK_BROWN: Color = Color {
    r: 99,
    g: 48,
    b: 13,
};
