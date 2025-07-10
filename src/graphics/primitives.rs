use anyhow::Result;
use embedded_graphics::{
    geometry::Point,
    image::Image,
    mono_font::{jis_x0201::FONT_10X20, MonoTextStyle},
    pixelcolor::Rgb565,
    primitives::{Circle, PrimitiveStyle, Styled},
    text::{Text, TextStyleBuilder},
    Drawable,
};
use tinybmp::Bmp;

use crate::lcd::LcdController;

/// 图形基元绘制器
///
/// 提供基于embedded-graphics库的图形绘制功能，包括图像、圆形、文本等基本图形的绘制。
/// 所有绘制操作都通过内部的LCD控制器来执行。
pub struct GraphicsPrimitives<'a> {
    lcd: &'a mut LcdController,
}

impl<'a> GraphicsPrimitives<'a> {
    /// 创建新的图形基元绘制器实例
    ///
    /// # 参数
    ///
    /// * `lcd` - LCD控制器的可变引用，用于执行实际的绘制操作
    ///
    /// # 返回值
    ///
    /// 返回绑定到指定LCD控制器的GraphicsPrimitives实例
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use crate::graphics::primitives::GraphicsPrimitives;
    /// use crate::lcd::LcdController;
    ///
    /// let mut lcd = LcdController::new(/* 参数 */);
    /// let mut graphics = GraphicsPrimitives::new(&mut lcd);
    /// ```
    pub fn new(lcd: &'a mut LcdController) -> Self {
        Self { lcd }
    }

    /// 绘制RGB565格式的BMP图片
    ///
    /// 在LCD屏幕的指定位置绘制一个BMP格式的图片。图片必须是RGB565颜色格式。
    ///
    /// # 参数
    ///
    /// * `image` - 要绘制的BMP图片的引用，必须是RGB565格式
    /// * `x` - 图片左上角在屏幕上的X坐标
    /// * `y` - 图片左上角在屏幕上的Y坐标
    ///
    /// # 返回值
    ///
    /// * `Ok(())` - 绘制成功
    /// * `Err(anyhow::Error)` - 绘制失败，可能原因包括坐标超出屏幕范围或LCD通信错误
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use tinybmp::Bmp;
    /// use embedded_graphics::pixelcolor::Rgb565;
    ///
    /// let image_data = include_bytes!("image.bmp");
    /// let bmp: Bmp<Rgb565> = Bmp::from_slice(image_data).unwrap();
    /// graphics.draw_image(&bmp, 10, 20)?;
    /// ```
    pub fn draw_image(&mut self, image: &Bmp<Rgb565>, x: i32, y: i32) -> Result<()> {
        Image::new(image, Point::new(x, y)).draw(self.lcd)?;
        Ok(())
    }

    /// 绘制填充的圆形
    ///
    /// 在LCD屏幕上绘制一个指定颜色的实心圆形。使用embedded-graphics库实现。
    ///
    /// # 参数
    ///
    /// * `center_x` - 圆心的X坐标
    /// * `center_y` - 圆心的Y坐标
    /// * `radius` - 圆的半径，必须为正数
    /// * `color` - 圆形的填充颜色，RGB565格式
    ///
    /// # 返回值
    ///
    /// * `Ok(())` - 绘制成功
    /// * `Err(anyhow::Error)` - 绘制失败，可能原因包括：
    ///   - 半径为负数或零
    ///   - 坐标超出屏幕范围
    ///   - LCD通信错误
    ///
    /// # 错误
    ///
    /// 如果半径小于等于0，将返回错误。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use embedded_graphics::pixelcolor::Rgb565;
    ///
    /// // 在屏幕中心绘制一个红色的圆形，半径为50像素
    /// let red = Rgb565::new(31, 0, 0);
    /// graphics.draw_filled_circle(160, 120, 50, red)?;
    /// ```
    pub fn draw_filled_circle(
        &mut self,
        center_x: i32,
        center_y: i32,
        radius: i32,
        color: Rgb565,
    ) -> Result<()> {
        if radius <= 0 {
            anyhow::bail!("半径必须为正数，当前为 {}", radius);
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

    /// 绘制文本
    ///
    /// 在LCD屏幕的指定位置绘制文本字符串。使用embedded-graphics库的单色字体实现。
    ///
    /// # 参数
    ///
    /// * `text` - 要绘制的文本字符串
    /// * `x` - 文本起始位置的X坐标（左上角）
    /// * `y` - 文本起始位置的Y坐标（左上角）
    /// * `color` - 文本的颜色，RGB565格式
    ///
    /// # 返回值
    ///
    /// * `Ok(())` - 绘制成功
    /// * `Err(anyhow::Error)` - 绘制失败，可能原因包括坐标超出屏幕范围或LCD通信错误
    ///
    /// # 字体信息
    ///
    /// 使用的字体是FONT_10X20，每个字符的尺寸为10x20像素。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use embedded_graphics::pixelcolor::Rgb565;
    ///
    /// // 在坐标(10, 30)处绘制白色文本
    /// let white = Rgb565::new(31, 63, 31);
    /// graphics.draw_text("Hello, ESP32!", 10, 30, white)?;
    /// ```
    pub fn draw_text(&mut self, text: &str, x: i32, y: i32, color: Rgb565) -> Result<()> {
        let character_style = MonoTextStyle::new(&FONT_10X20, color);
        let text_style = TextStyleBuilder::new().build();

        let text_obj = Text::with_text_style(text, Point::new(x, y), character_style, text_style);
        text_obj.draw(self.lcd)?;
        Ok(())
    }

    /// 用指定颜色填充整个屏幕
    ///
    /// 将LCD屏幕的所有像素设置为指定的颜色，相当于清空屏幕操作。
    ///
    /// # 参数
    ///
    /// * `color` - 填充屏幕的颜色值，16位RGB格式（5-6-5位分别对应R-G-B分量）
    ///
    /// # 返回值
    ///
    /// * `Ok(())` - 填充成功
    /// * `Err(anyhow::Error)` - 填充失败，通常是由于LCD通信错误
    ///
    /// # 颜色格式
    ///
    /// 颜色值使用16位RGB格式：
    /// - 位[15:11]: 红色分量（5位）
    /// - 位[10:5]:  绿色分量（6位）
    /// - 位[4:0]:   蓝色分量（5位）
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// // 用黑色填充屏幕
    /// graphics.fill_screen(0x0000)?;
    ///
    /// // 用白色填充屏幕  
    /// graphics.fill_screen(0xFFFF)?;
    ///
    /// // 用红色填充屏幕
    /// graphics.fill_screen(0xF800)?;
    /// ```
    pub fn fill_screen(&self, color: u16) -> Result<()> {
        self.lcd.fill_screen(color)
    }
}
