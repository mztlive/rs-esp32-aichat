use embedded_graphics::pixelcolor::Rgb565;

pub mod primitives;

pub fn rgb565_from_u16(color: u16) -> Rgb565 {
    let r = ((color >> 11) & 0x1F) as u8;
    let g = ((color >> 5) & 0x3F) as u8;
    let b = (color & 0x1F) as u8;
    Rgb565::new(r, g, b)
}
