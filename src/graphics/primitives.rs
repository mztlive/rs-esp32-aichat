use anyhow::Result;
use embedded_graphics::{
    geometry::Point,
    mono_font::{jis_x0201::FONT_10X20, MonoTextStyle},
    pixelcolor::Rgb565,
    primitives::{Circle, PrimitiveStyle, Styled},
    text::{Text, TextStyleBuilder},
    Drawable,
};

use crate::lcd::LcdController;

pub struct GraphicsPrimitives<'a> {
    lcd: &'a mut LcdController,
}

impl<'a> GraphicsPrimitives<'a> {
    pub fn new(lcd: &'a mut LcdController) -> Self {
        Self { lcd }
    }

    /// 绘制实心圆形（填充）- 使用embedded-graphics实现
    pub fn draw_filled_circle(
        &mut self,
        center_x: i32,
        center_y: i32,
        radius: i32,
        color: Rgb565,
    ) -> Result<()> {
        if radius <= 0 {
            return Ok(());
        }

        let circle = Circle::new(
            Point::new(center_x - radius, center_y - radius),
            (radius * 2) as u32,
        );

        let style = PrimitiveStyle::with_fill(color);
        let styled_circle = Styled::new(circle, style);
        styled_circle.draw(self.lcd)?;

        Ok(())
    }

    /// 使用embedded-graphics绘制文本
    pub fn draw_text(&mut self, text: &str, x: i32, y: i32, color: Rgb565) -> Result<()> {
        let character_style = MonoTextStyle::new(&FONT_10X20, color);
        let text_style = TextStyleBuilder::new().build();

        let text_obj = Text::with_text_style(text, Point::new(x, y), character_style, text_style);
        text_obj.draw(self.lcd)?;
        Ok(())
    }

    /// 清空屏幕
    pub fn fill_screen(&self, color: u16) -> Result<()> {
        self.lcd.fill_screen(color)
    }
}
