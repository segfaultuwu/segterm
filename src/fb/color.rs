#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    // Basic
    pub const BLACK: Self = Self::rgb(0, 0, 0);
    pub const WHITE: Self = Self::rgb(255, 255, 255);
    pub const RED: Self = Self::rgb(255, 0, 0);
    pub const GREEN: Self = Self::rgb(0, 255, 0);
    pub const BLUE: Self = Self::rgb(0, 0, 255);
    pub const YELLOW: Self = Self::rgb(255, 255, 0);
    pub const CYAN: Self = Self::rgb(0, 255, 255);
    pub const MAGENTA: Self = Self::rgb(255, 0, 255);

    // Bright
    pub const BRIGHT_BLACK: Self = Self::rgb(128, 128, 128);
    pub const BRIGHT_RED: Self = Self::rgb(255, 85, 85);
    pub const BRIGHT_GREEN: Self = Self::rgb(85, 255, 85);
    pub const BRIGHT_YELLOW: Self = Self::rgb(255, 255, 85);
    pub const BRIGHT_BLUE: Self = Self::rgb(85, 85, 255);
    pub const BRIGHT_MAGENTA: Self = Self::rgb(255, 85, 255);
    pub const BRIGHT_CYAN: Self = Self::rgb(85, 255, 255);
    pub const BRIGHT_WHITE: Self = Self::rgb(255, 255, 255);

    // Grays
    pub const DARK_GRAY: Self = Self::rgb(64, 64, 64);
    pub const GRAY: Self = Self::rgb(128, 128, 128);
    pub const LIGHT_GRAY: Self = Self::rgb(192, 192, 192);

    // Other colors
    pub const ORANGE: Self = Self::rgb(255, 165, 0);
    pub const PURPLE: Self = Self::rgb(128, 0, 128);
    pub const PINK: Self = Self::rgb(255, 105, 180);
    pub const BROWN: Self = Self::rgb(139, 69, 19);

    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}
