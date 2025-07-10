use embedded_graphics::pixelcolor::Rgb565;

pub const BLACK: Rgb565 = Rgb565::new(0, 0, 0);
pub const WHITE: Rgb565 = Rgb565::new(31, 63, 31);
pub const RED: Rgb565 = Rgb565::new(31, 0, 0);
pub const GREEN: Rgb565 = Rgb565::new(0, 63, 0);
pub const BLUE: Rgb565 = Rgb565::new(0, 0, 31);
pub const YELLOW: Rgb565 = Rgb565::new(31, 63, 0);
pub const CYAN: Rgb565 = Rgb565::new(0, 63, 31);
pub const MAGENTA: Rgb565 = Rgb565::new(31, 0, 31);
pub const ORANGE: Rgb565 = Rgb565::new(31, 32, 0);
pub const PURPLE: Rgb565 = Rgb565::new(16, 0, 16);
pub const PINK: Rgb565 = Rgb565::new(31, 20, 31);
pub const BROWN: Rgb565 = Rgb565::new(20, 16, 8);
pub const GRAY: Rgb565 = Rgb565::new(16, 32, 16);
pub const DARK_GRAY: Rgb565 = Rgb565::new(8, 16, 8);
pub const LIGHT_GRAY: Rgb565 = Rgb565::new(24, 48, 24);
pub const NAVY: Rgb565 = Rgb565::new(0, 0, 16);
pub const LIME: Rgb565 = Rgb565::new(16, 63, 0);
pub const SILVER: Rgb565 = Rgb565::new(24, 48, 24);
pub const MAROON: Rgb565 = Rgb565::new(16, 0, 0);
pub const OLIVE: Rgb565 = Rgb565::new(16, 32, 0);

pub fn get_all_colors() -> Vec<Rgb565> {
    vec![
        BLACK, WHITE, RED, GREEN, BLUE, YELLOW, CYAN, MAGENTA, ORANGE, PURPLE, PINK, BROWN, GRAY,
        DARK_GRAY, LIGHT_GRAY, NAVY, LIME, SILVER, MAROON, OLIVE,
    ]
}
