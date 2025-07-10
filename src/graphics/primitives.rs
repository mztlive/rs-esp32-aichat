use anyhow::Result;
use embedded_graphics::{
    geometry::{Dimensions, Point, Size},
    image::Image,
    mono_font::{jis_x0201::FONT_10X20, MonoTextStyle},
    pixelcolor::Rgb565,
    primitives::{Circle, PrimitiveStyle, Rectangle, Styled},
    text::{renderer::CharacterStyle, Text, TextStyleBuilder},
    Drawable,
};
use tinybmp::Bmp;

use crate::{
    graphics::{
        layout::{GridPosition, ScreenRect},
        ui::traits::UIComponent,
    },
    peripherals::st77916::lcd::{LcdController, LCD_HEIGHT, LCD_WIDTH},
};

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
    pub fn draw_text(
        &mut self,
        text: &str,
        x: i32,
        y: i32,
        color: Rgb565,
        background_color: Option<Rgb565>,
    ) -> Result<()> {
        let mut character_style = MonoTextStyle::new(&FONT_10X20, color);
        character_style.set_background_color(background_color);

        let text_style = TextStyleBuilder::new().build();

        let text_obj = Text::with_text_style(text, Point::new(x, y), character_style, text_style);
        text_obj.draw(self.lcd)?;
        Ok(())
    }

    /// 用指定颜色填充整个屏幕
    ///
    /// 将LCD屏幕的所有像素设置为指定的颜色，相当于清空屏幕操作。
    /// 使用embedded-graphics库绘制一个覆盖整个屏幕的矩形。
    ///
    /// # 参数
    ///
    /// * `color` - 填充屏幕的颜色值，Rgb565格式
    ///
    /// # 返回值
    ///
    /// * `Ok(())` - 填充成功
    /// * `Err(anyhow::Error)` - 填充失败，通常是由于LCD通信错误
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use embedded_graphics::pixelcolor::Rgb565;
    /// use crate::graphics::colors::*;
    ///
    /// // 用黑色填充屏幕
    /// graphics.fill_screen(BLACK)?;
    ///
    /// // 用白色填充屏幕  
    /// graphics.fill_screen(WHITE)?;
    ///
    /// // 用红色填充屏幕
    /// graphics.fill_screen(RED)?;
    /// ```
    pub fn fill_screen(&mut self, color: Rgb565) -> Result<()> {
        let screen_size = Size::new(LCD_WIDTH as u32, LCD_HEIGHT as u32);
        let rectangle = Rectangle::new(Point::zero(), screen_size);
        let style = PrimitiveStyle::with_fill(color);
        let styled_rectangle = Styled::new(rectangle, style);
        styled_rectangle.draw(self.lcd)?;
        Ok(())
    }

    /// 在九宫格指定位置绘制圆形
    ///
    /// 在九宫格的指定位置绘制一个圆形，圆形会自动居中在该格子中。
    ///
    /// # 参数
    ///
    /// * `position` - 九宫格位置枚举
    /// * `radius` - 圆形半径
    /// * `color` - 圆形颜色
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use crate::graphics::layout::GridPosition;
    /// use crate::graphics::colors::RED;
    ///
    /// // 在九宫格中心绘制红色圆形
    /// graphics.draw_circle_at_grid(GridPosition::MiddleCenter, 30, RED)?;
    /// ```
    pub fn draw_circle_at_grid(
        &mut self,
        position: GridPosition,
        radius: i32,
        color: Rgb565,
    ) -> Result<()> {
        let (center_x, center_y) = position.get_center();
        self.draw_filled_circle(center_x, center_y, radius, color)
    }

    /// 在九宫格指定位置绘制文本
    ///
    /// 在九宫格的指定位置绘制文本，文本会自动定位在该格子的中心。
    ///
    /// # 参数
    ///
    /// * `position` - 九宫格位置枚举
    /// * `text` - 要显示的文本
    /// * `color` - 文本颜色
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use crate::graphics::layout::GridPosition;
    /// use crate::graphics::colors::BLACK;
    ///
    /// // 在九宫格左上角绘制文本
    /// graphics.draw_text_at_grid(GridPosition::TopLeft, "Hello", BLACK)?;
    /// ```
    pub fn draw_text_at_grid(
        &mut self,
        position: GridPosition,
        text: &str,
        color: Rgb565,
        background_color: Option<Rgb565>,
    ) -> Result<()> {
        let (center_x, center_y) = position.get_center();

        // 计算文本尺寸并调整位置使其居中
        let text_width = text.len() as i32 * 10; // 每个字符10像素宽
        let text_height = 20; // 字体高度20像素

        let text_x = center_x - text_width / 2;
        let text_y = center_y - text_height / 2;

        self.draw_text(text, text_x, text_y, color, background_color)
    }

    /// 在九宫格指定位置绘制图像
    ///
    /// 在九宫格的指定位置绘制图像，图像会自动居中在该格子中。
    ///
    /// # 参数
    ///
    /// * `position` - 九宫格位置枚举
    /// * `image` - BMP图像引用
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use crate::graphics::layout::GridPosition;
    /// use tinybmp::Bmp;
    ///
    /// let bmp = Bmp::from_slice(image_data).unwrap();
    /// graphics.draw_image_at_grid(GridPosition::MiddleCenter, &bmp)?;
    /// ```
    pub fn draw_image_at_grid(
        &mut self,
        position: GridPosition,
        image: &Bmp<Rgb565>,
    ) -> Result<()> {
        let (center_x, center_y) = position.get_center();

        // 获取图像尺寸
        let image_size = image.bounding_box().size;
        let image_width = image_size.width as i32;
        let image_height = image_size.height as i32;

        // 计算图像左上角位置使其居中
        let image_x = center_x - image_width / 2;
        let image_y = center_y - image_height / 2;

        self.draw_image(image, image_x, image_y)
    }

    /// 在屏幕矩形区域内绘制填充矩形
    ///
    /// 在指定的屏幕矩形区域内绘制一个填充的矩形。
    ///
    /// # 参数
    ///
    /// * `rect` - 屏幕矩形区域
    /// * `color` - 填充颜色
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use crate::graphics::layout::{ScreenRect, STATUS_BAR};
    /// use crate::graphics::colors::BLUE;
    ///
    /// // 填充状态栏区域
    /// graphics.fill_rect(&STATUS_BAR, BLUE)?;
    /// ```
    pub fn fill_rect(&mut self, rect: &ScreenRect, color: Rgb565) -> Result<()> {
        let rectangle = Rectangle::new(
            Point::new(rect.x, rect.y),
            Size::new(rect.width as u32, rect.height as u32),
        );
        let style = PrimitiveStyle::with_fill(color);
        let styled_rectangle = Styled::new(rectangle, style);
        styled_rectangle.draw(self.lcd)?;
        Ok(())
    }

    /// 在屏幕矩形区域内绘制边框矩形
    ///
    /// 在指定的屏幕矩形区域内绘制一个边框矩形。
    ///
    /// # 参数
    ///
    /// * `rect` - 屏幕矩形区域
    /// * `color` - 边框颜色
    /// * `thickness` - 边框厚度
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use crate::graphics::layout::{ScreenRect, CONTENT_AREA};
    /// use crate::graphics::colors::BLACK;
    ///
    /// // 绘制内容区域边框
    /// graphics.draw_rect_border(&CONTENT_AREA, BLACK, 2)?;
    /// ```
    pub fn draw_rect_border(
        &mut self,
        rect: &ScreenRect,
        color: Rgb565,
        thickness: u32,
    ) -> Result<()> {
        let rectangle = Rectangle::new(
            Point::new(rect.x, rect.y),
            Size::new(rect.width as u32, rect.height as u32),
        );
        let style = PrimitiveStyle::with_stroke(color, thickness);
        let styled_rectangle = Styled::new(rectangle, style);
        styled_rectangle.draw(self.lcd)?;
        Ok(())
    }

    /// 在屏幕中心绘制圆形
    ///
    /// 在屏幕的正中心绘制一个圆形。
    ///
    /// # 参数
    ///
    /// * `radius` - 圆形半径
    /// * `color` - 圆形颜色
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use crate::graphics::colors::GREEN;
    ///
    /// // 在屏幕中心绘制绿色圆形
    /// graphics.draw_circle_at_center(50, GREEN)?;
    /// ```
    pub fn draw_circle_at_center(&mut self, radius: i32, color: Rgb565) -> Result<()> {
        use crate::graphics::layout::{SCREEN_CENTER_X, SCREEN_CENTER_Y};
        self.draw_filled_circle(SCREEN_CENTER_X, SCREEN_CENTER_Y, radius, color)
    }

    /// 在屏幕中心绘制文本
    ///
    /// 在屏幕的正中心绘制文本。
    ///
    /// # 参数
    ///
    /// * `text` - 要显示的文本
    /// * `color` - 文本颜色
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use crate::graphics::colors::BLACK;
    ///
    /// // 在屏幕中心绘制文本
    /// graphics.draw_text_at_center("ESP32", BLACK)?;
    /// ```
    pub fn draw_text_at_center(
        &mut self,
        text: &str,
        color: Rgb565,
        background_color: Option<Rgb565>,
    ) -> Result<()> {
        use crate::graphics::layout::{SCREEN_CENTER_X, SCREEN_CENTER_Y};

        // 计算文本尺寸并调整位置使其居中
        let text_width = text.len() as i32 * 10; // 每个字符10像素宽
        let text_height = 20; // 字体高度20像素

        let text_x = SCREEN_CENTER_X - text_width / 2;
        let text_y = SCREEN_CENTER_Y - text_height / 2;

        self.draw_text(text, text_x, text_y, color, background_color)
    }

    /// 在指定位置绘制多行文本
    ///
    /// 在指定位置绘制多行文本，每行自动换行。
    ///
    /// # 参数
    ///
    /// * `lines` - 文本行数组
    /// * `start_x` - 起始X坐标
    /// * `start_y` - 起始Y坐标
    /// * `color` - 文本颜色
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use crate::graphics::colors::BLACK;
    ///
    /// let lines = vec!["Line 1", "Line 2", "Line 3"];
    /// graphics.draw_multiline_text(&lines, 20, 50, BLACK)?;
    /// ```
    pub fn draw_multiline_text(
        &mut self,
        lines: &[&str],
        start_x: i32,
        start_y: i32,
        color: Rgb565,
        background_color: Option<Rgb565>,
    ) -> Result<()> {
        use crate::graphics::layout::TEXT_LINE_HEIGHT;

        for (i, line) in lines.iter().enumerate() {
            let y = start_y + i as i32 * TEXT_LINE_HEIGHT;
            self.draw_text(line, start_x, y, color, background_color)?;
        }
        Ok(())
    }

    /// 绘制圆形边框
    ///
    /// 在指定位置绘制一个圆形边框（空心圆）。
    ///
    /// # 参数
    ///
    /// * `center_x` - 圆心X坐标
    /// * `center_y` - 圆心Y坐标
    /// * `radius` - 圆形半径
    /// * `color` - 边框颜色
    /// * `thickness` - 边框厚度
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use crate::graphics::colors::BLUE;
    ///
    /// // 绘制蓝色圆形边框
    /// graphics.draw_circle_border(180, 180, 50, BLUE, 3)?;
    /// ```
    pub fn draw_circle_border(
        &mut self,
        center_x: i32,
        center_y: i32,
        radius: i32,
        color: Rgb565,
        thickness: u32,
    ) -> Result<()> {
        if radius <= 0 {
            anyhow::bail!("半径必须为正数，当前为 {}", radius);
        }

        let circle = Circle::new(
            Point::new(center_x - radius, center_y - radius),
            (radius * 2) as u32,
        );

        let style = PrimitiveStyle::with_stroke(color, thickness);
        let styled_circle = Styled::new(circle, style);
        styled_circle.draw(self.lcd)?;

        Ok(())
    }

    /// 清除九宫格指定区域
    ///
    /// 用指定颜色清除九宫格的指定区域。
    ///
    /// # 参数
    ///
    /// * `position` - 九宫格位置枚举
    /// * `color` - 清除颜色
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use crate::graphics::layout::GridPosition;
    /// use crate::graphics::colors::WHITE;
    ///
    /// // 清除九宫格中心区域
    /// graphics.clear_grid_area(GridPosition::MiddleCenter, WHITE)?;
    /// ```
    pub fn clear_grid_area(&mut self, position: GridPosition, color: Rgb565) -> Result<()> {
        use crate::graphics::layout::GRID_SIZE;

        let (top_left_x, top_left_y) = position.get_top_left();
        let rect = ScreenRect::new(top_left_x, top_left_y, GRID_SIZE, GRID_SIZE);
        self.fill_rect(&rect, color)
    }

    /// 绘制UI组件
    ///
    /// 使用UI组件的render方法来绘制组件。
    ///
    /// # 参数
    ///
    /// * `component` - 实现了UIComponent trait的组件
    ///
    /// # 返回值
    ///
    /// * `Ok(())` - 绘制成功
    /// * `Err(anyhow::Error)` - 绘制失败
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use crate::graphics::ui::StatusBar;
    /// use crate::graphics::colors::WHITE;
    ///
    /// let mut statusbar = StatusBar::new(WHITE);
    /// statusbar.add_text("Hello", StatusBarPosition::Left, BLACK);
    /// graphics.draw_component(&statusbar)?;
    /// ```
    pub fn draw_component<T: UIComponent>(&mut self, component: &T) -> Result<()> {
        component.render(self)
    }
}
