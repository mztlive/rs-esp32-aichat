use anyhow::Result;
use embedded_graphics::{
    geometry::Point,
    mono_font::{jis_x0201::FONT_10X20, MonoTextStyle},
    pixelcolor::Rgb565,
    text::{Text, TextStyleBuilder},
    Drawable,
};

use crate::lcd::{LcdController, LCD_HEIGHT, LCD_WIDTH};

pub struct GraphicsPrimitives<'a> {
    lcd: &'a mut LcdController,
}

impl<'a> GraphicsPrimitives<'a> {
    pub fn new(lcd: &'a mut LcdController) -> Self {
        Self { lcd }
    }

    /// 绘制单个像素
    pub fn draw_pixel(&self, x: i32, y: i32, color: u16) -> Result<()> {
        self.lcd.draw_pixel(x, y, color)
    }

    /// 绘制圆形（使用Bresenham算法）
    pub fn draw_circle(&self, center_x: i32, center_y: i32, radius: i32, color: u16) -> Result<()> {
        if radius <= 0 {
            return Ok(());
        }

        let mut x = 0;
        let mut y = radius;
        let mut decision = 1 - radius;

        // 绘制中心点
        self.draw_pixel(center_x, center_y, color)?;

        while x <= y {
            // 绘制八个对称点
            self.draw_pixel(center_x + x, center_y + y, color)?;
            self.draw_pixel(center_x - x, center_y + y, color)?;
            self.draw_pixel(center_x + x, center_y - y, color)?;
            self.draw_pixel(center_x - x, center_y - y, color)?;
            self.draw_pixel(center_x + y, center_y + x, color)?;
            self.draw_pixel(center_x - y, center_y + x, color)?;
            self.draw_pixel(center_x + y, center_y - x, color)?;
            self.draw_pixel(center_x - y, center_y - x, color)?;

            x += 1;
            if decision < 0 {
                decision += 2 * x + 1;
            } else {
                y -= 1;
                decision += 2 * (x - y) + 1;
            }
        }

        Ok(())
    }

    /// 绘制实心圆形（填充）
    pub fn draw_filled_circle(
        &self,
        center_x: i32,
        center_y: i32,
        radius: i32,
        color: u16,
    ) -> Result<()> {
        if radius <= 0 {
            return Ok(());
        }

        for y in -radius..=radius {
            let y_coord = center_y + y;
            if !(0..LCD_HEIGHT).contains(&y_coord) {
                continue;
            }

            // 计算当前行的半宽
            let half_width = ((radius * radius - y * y) as f32).sqrt() as i32;

            let x_start = (center_x - half_width).max(0);
            let x_end = (center_x + half_width + 1).min(LCD_WIDTH);

            if x_start < x_end {
                let line_width = (x_end - x_start) as usize;
                let line_buffer = vec![color; line_width];
                self.lcd
                    .draw_bitmap(x_start, y_coord, x_end, y_coord + 1, &line_buffer)?;
            }
        }

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

    /// 绘制位图
    pub fn draw_bitmap(&self, x: i32, y: i32, x_end: i32, y_end: i32, data: &[u16]) -> Result<()> {
        self.lcd.draw_bitmap(x, y, x_end, y_end, data)
    }
}