/// Define as a non_exhaustive struct to behave as an "enum" with constant values
/// Doing it this way avoids explicit type casting with Rust enums
#[non_exhaustive]
pub struct Color;

impl Color {
    pub const BLACK: u32 = 0x00000000;
    pub const BLUE: u32 = 0x000000ff;
    pub const CYAN: u32 = 0x0000ffff;
    pub const GREEN: u32 = 0x0000ff00;
    pub const MAGENTA: u32 = 0x00ff00ff;
    pub const RED: u32 = 0x00ff0000;
    pub const WHITE: u32 = 0xffffffff;
    pub const YELLOW: u32 = 0x00ffff00;
}

#[allow(unused)]
pub fn blend_colors(foreground: u32, background: u32, blend_factor: u8) -> u32 {
    let fg_ratio = blend_factor as u32;
    let bg_ratio = 255 - fg_ratio as u32;

    let r = (((foreground >> 16) & 0xff) * fg_ratio + ((background >> 16) & 0xff) * bg_ratio) / 255;
    let g = (((foreground >> 8) & 0xff) * fg_ratio + ((background >> 8) & 0xff) * bg_ratio) / 255;
    let b = ((foreground & 0xff) * fg_ratio + (background & 0xff) * bg_ratio) / 255;

    (r << 16) | (g << 8) | b
}
