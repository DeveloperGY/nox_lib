#[derive(Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self {r, g, b}
    }
}

// Constants
impl Color {
    // Core 8 colors
    pub const   WHITE: Color = Color {r: 255, g: 255, b: 255};
    pub const   BLACK: Color = Color {r:   0, g:   0, b:   0};
    pub const     RED: Color = Color {r: 255, g:   0, b:   0};
    pub const   GREEN: Color = Color {r:   0, g: 255, b:   0};
    pub const    BLUE: Color = Color {r:   0, g:   0, b: 255};
    pub const    CYAN: Color = Color {r:   0, g: 255, b: 255};
    pub const MAGENTA: Color = Color {r: 255, g:   0, b: 255};
    pub const  YELLOW: Color = Color {r: 255, g: 255, b:   0};

    // Common Extras
    pub const  ORANGE: Color = Color {r: 255, g: 127, b:   0};
    pub const  PURPLE: Color = Color {r: 127, g:   0, b: 255};

    // Special
    pub const DEFAULT: Color = Color {r:   0, g:   0, b:   0};
}