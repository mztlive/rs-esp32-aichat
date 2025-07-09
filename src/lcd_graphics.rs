use anyhow::Result;
use embedded_graphics::mono_font::jis_x0201::FONT_10X20;
use embedded_graphics::{
    geometry::Point,
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::{Rgb565, RgbColor},
    text::{Text, TextStyleBuilder},
    Drawable,
};

use crate::lcd::{LcdController, COLOR_BLACK, COLOR_BLUE, COLOR_WHITE, LCD_HEIGHT, LCD_WIDTH};

pub struct LcdGraphics<'a> {
    lcd: &'a mut LcdController,
}

impl<'a> LcdGraphics<'a> {
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

    /// 绘制平滑文本（使用背景色进行简单的抗锯齿）
    pub fn draw_smooth_text(
        &mut self,
        text: &str,
        x: i32,
        y: i32,
        fg_color: Rgb565,
        bg_color: Rgb565,
    ) -> Result<()> {
        // 先绘制背景色的文本作为阴影（偏移1像素）
        let shadow_color = Rgb565::new(
            (fg_color.r() + bg_color.r()) / 2,
            (fg_color.g() + bg_color.g()) / 2,
            (fg_color.b() + bg_color.b()) / 2,
        );

        self.draw_text(text, x + 1, y + 1, shadow_color)?;

        // 再绘制前景色的文本
        self.draw_text(text, x, y, fg_color)?;
        Ok(())
    }

    /// 使用embedded-graphics绘制彩色文本（方便方法）
    pub fn draw_colored_text(
        &mut self,
        text: &str,
        x: i32,
        y: i32,
        r: u8,
        g: u8,
        b: u8,
    ) -> Result<()> {
        let color = Rgb565::new(r >> 3, g >> 2, b >> 3);
        self.draw_text(text, x, y, color)
    }

    /// 绘制一个眼睛
    pub fn draw_eye(&self, center_x: i32, center_y: i32, eye_size: i32) -> Result<()> {
        // 眼球半径
        let eyeball_radius = eye_size;
        // 瞳孔半径
        let pupil_radius = eye_size / 2;
        // 高光半径
        let highlight_radius = eye_size / 4;

        // 绘制眼球（蓝色）
        self.draw_filled_circle(center_x, center_y, eyeball_radius, COLOR_BLUE)?;

        // 绘制瞳孔（黑色，稍微偏右下）
        let pupil_x = center_x + eye_size / 6;
        let pupil_y = center_y + eye_size / 6;
        self.draw_filled_circle(pupil_x, pupil_y, pupil_radius, COLOR_BLACK)?;

        // 绘制高光（白色，在瞳孔左上角）
        let highlight_x = pupil_x - pupil_radius / 3;
        let highlight_y = pupil_y - pupil_radius / 3;
        self.draw_filled_circle(highlight_x, highlight_y, highlight_radius, COLOR_WHITE)?;

        Ok(())
    }

    /// 绘制两个眼睛
    pub fn draw_eyes(&self) -> Result<()> {
        // 360x360屏幕，眼睛大小为40像素半径
        let eye_size = 40;
        let eye_spacing = 120; // 眼睛之间的距离

        // 屏幕中心
        let center_x = LCD_WIDTH / 2;
        let center_y = LCD_HEIGHT / 2;

        // 左眼位置
        let left_eye_x = center_x - eye_spacing / 2;
        let left_eye_y = center_y;

        // 右眼位置
        let right_eye_x = center_x + eye_spacing / 2;
        let right_eye_y = center_y;

        // 绘制左眼
        self.draw_eye(left_eye_x, left_eye_y, eye_size)?;

        // 绘制右眼
        self.draw_eye(right_eye_x, right_eye_y, eye_size)?;

        Ok(())
    }

    /// 绘制一个眼睛，支持瞳孔位置偏移
    pub fn draw_eye_with_pupil_offset(
        &self,
        center_x: i32,
        center_y: i32,
        eye_size: i32,
        pupil_offset_x: i32,
        pupil_offset_y: i32,
    ) -> Result<()> {
        // 眼球半径
        let eyeball_radius = eye_size;
        // 瞳孔半径
        let pupil_radius = eye_size / 2;
        // 高光半径
        let highlight_radius = eye_size / 4;

        // 绘制眼球（蓝色）
        self.draw_filled_circle(center_x, center_y, eyeball_radius, COLOR_BLUE)?;

        // 绘制瞳孔（黑色，可以偏移）
        let pupil_x = center_x + pupil_offset_x;
        let pupil_y = center_y + pupil_offset_y;
        self.draw_filled_circle(pupil_x, pupil_y, pupil_radius, COLOR_BLACK)?;

        // 绘制高光（白色，在瞳孔左上角）
        let highlight_x = pupil_x - pupil_radius / 3;
        let highlight_y = pupil_y - pupil_radius / 3;
        self.draw_filled_circle(highlight_x, highlight_y, highlight_radius, COLOR_WHITE)?;

        Ok(())
    }

    /// 绘制椭圆形眼睛（用于眨眼效果）
    pub fn draw_eye_blink(
        &self,
        center_x: i32,
        center_y: i32,
        eye_size: i32,
        blink_ratio: f32,
    ) -> Result<()> {
        let eyeball_radius = eye_size;
        let pupil_radius = eye_size / 2;
        let highlight_radius = eye_size / 4;

        // 根据眨眼比例调整眼睛高度
        let eye_height = (eyeball_radius as f32 * blink_ratio) as i32;

        if eye_height <= 2 {
            // 完全闭眼，绘制一条线
            let line_buffer = vec![COLOR_BLUE; (eyeball_radius * 2) as usize];
            self.lcd.draw_bitmap(
                center_x - eyeball_radius,
                center_y - 1,
                center_x + eyeball_radius,
                center_y + 1,
                &line_buffer,
            )?;
            return Ok(());
        }

        // 绘制压缩的眼球
        for y in -eye_height..=eye_height {
            let y_coord = center_y + y;
            if !(0..LCD_HEIGHT).contains(&y_coord) {
                continue;
            }

            let half_width = ((eyeball_radius * eyeball_radius
                - (y * eyeball_radius / eye_height) * (y * eyeball_radius / eye_height))
                as f32)
                .sqrt() as i32;
            let x_start = (center_x - half_width).max(0);
            let x_end = (center_x + half_width + 1).min(LCD_WIDTH);

            if x_start < x_end {
                let line_width = (x_end - x_start) as usize;
                let line_buffer = vec![COLOR_BLUE; line_width];
                self.lcd
                    .draw_bitmap(x_start, y_coord, x_end, y_coord + 1, &line_buffer)?;
            }
        }

        // 如果眼睛开度足够，绘制瞳孔和高光
        if eye_height > eyeball_radius / 2 {
            let pupil_x = center_x + eye_size / 6;
            let pupil_y = center_y + eye_size / 6;
            let compressed_pupil_radius = (pupil_radius as f32 * blink_ratio) as i32;

            if compressed_pupil_radius > 0 {
                self.draw_filled_circle(pupil_x, pupil_y, compressed_pupil_radius, COLOR_BLACK)?;

                let highlight_x = pupil_x - compressed_pupil_radius / 3;
                let highlight_y = pupil_y - compressed_pupil_radius / 3;
                let compressed_highlight_radius = (highlight_radius as f32 * blink_ratio) as i32;

                if compressed_highlight_radius > 0 {
                    self.draw_filled_circle(
                        highlight_x,
                        highlight_y,
                        compressed_highlight_radius,
                        COLOR_WHITE,
                    )?;
                }
            }
        }

        Ok(())
    }

    /// 绘制眼睛看向左边
    pub fn draw_eyes_look_left(&self) -> Result<()> {
        let eye_size = 40;
        let eye_spacing = 120;
        let center_x = LCD_WIDTH / 2;
        let center_y = LCD_HEIGHT / 2;

        let left_eye_x = center_x - eye_spacing / 2;
        let left_eye_y = center_y;
        let right_eye_x = center_x + eye_spacing / 2;
        let right_eye_y = center_y;

        // 瞳孔向左偏移
        let pupil_offset_x = -eye_size / 4;
        let pupil_offset_y = 0;

        self.draw_eye_with_pupil_offset(
            left_eye_x,
            left_eye_y,
            eye_size,
            pupil_offset_x,
            pupil_offset_y,
        )?;
        self.draw_eye_with_pupil_offset(
            right_eye_x,
            right_eye_y,
            eye_size,
            pupil_offset_x,
            pupil_offset_y,
        )?;

        Ok(())
    }

    /// 绘制眼睛看向右边
    pub fn draw_eyes_look_right(&self) -> Result<()> {
        let eye_size = 40;
        let eye_spacing = 120;
        let center_x = LCD_WIDTH / 2;
        let center_y = LCD_HEIGHT / 2;

        let left_eye_x = center_x - eye_spacing / 2;
        let left_eye_y = center_y;
        let right_eye_x = center_x + eye_spacing / 2;
        let right_eye_y = center_y;

        // 瞳孔向右偏移
        let pupil_offset_x = eye_size / 4;
        let pupil_offset_y = 0;

        self.draw_eye_with_pupil_offset(
            left_eye_x,
            left_eye_y,
            eye_size,
            pupil_offset_x,
            pupil_offset_y,
        )?;
        self.draw_eye_with_pupil_offset(
            right_eye_x,
            right_eye_y,
            eye_size,
            pupil_offset_x,
            pupil_offset_y,
        )?;

        Ok(())
    }

    /// 绘制眼睛看向上方
    pub fn draw_eyes_look_up(&self) -> Result<()> {
        let eye_size = 40;
        let eye_spacing = 120;
        let center_x = LCD_WIDTH / 2;
        let center_y = LCD_HEIGHT / 2;

        let left_eye_x = center_x - eye_spacing / 2;
        let left_eye_y = center_y;
        let right_eye_x = center_x + eye_spacing / 2;
        let right_eye_y = center_y;

        // 瞳孔向上偏移
        let pupil_offset_x = 0;
        let pupil_offset_y = -eye_size / 4;

        self.draw_eye_with_pupil_offset(
            left_eye_x,
            left_eye_y,
            eye_size,
            pupil_offset_x,
            pupil_offset_y,
        )?;
        self.draw_eye_with_pupil_offset(
            right_eye_x,
            right_eye_y,
            eye_size,
            pupil_offset_x,
            pupil_offset_y,
        )?;

        Ok(())
    }

    /// 绘制眼睛看向下方
    pub fn draw_eyes_look_down(&self) -> Result<()> {
        let eye_size = 40;
        let eye_spacing = 120;
        let center_x = LCD_WIDTH / 2;
        let center_y = LCD_HEIGHT / 2;

        let left_eye_x = center_x - eye_spacing / 2;
        let left_eye_y = center_y;
        let right_eye_x = center_x + eye_spacing / 2;
        let right_eye_y = center_y;

        // 瞳孔向下偏移
        let pupil_offset_x = 0;
        let pupil_offset_y = eye_size / 4;

        self.draw_eye_with_pupil_offset(
            left_eye_x,
            left_eye_y,
            eye_size,
            pupil_offset_x,
            pupil_offset_y,
        )?;
        self.draw_eye_with_pupil_offset(
            right_eye_x,
            right_eye_y,
            eye_size,
            pupil_offset_x,
            pupil_offset_y,
        )?;

        Ok(())
    }

    /// 绘制眨眼动画
    pub fn draw_eyes_blink(&self, blink_ratio: f32) -> Result<()> {
        let eye_size = 40;
        let eye_spacing = 120;
        let center_x = LCD_WIDTH / 2;
        let center_y = LCD_HEIGHT / 2;

        let left_eye_x = center_x - eye_spacing / 2;
        let left_eye_y = center_y;
        let right_eye_x = center_x + eye_spacing / 2;
        let right_eye_y = center_y;

        self.draw_eye_blink(left_eye_x, left_eye_y, eye_size, blink_ratio)?;
        self.draw_eye_blink(right_eye_x, right_eye_y, eye_size, blink_ratio)?;

        Ok(())
    }

    /// 播放眼睛动画序列
    pub fn play_eye_animation(&self) -> Result<()> {
        use std::thread;
        use std::time::Duration;

        let frame_duration = Duration::from_millis(500);

        // 1. 正常眼睛
        println!("动画: 正常眼睛");
        self.lcd.fill_screen(COLOR_BLACK)?;
        self.draw_eyes()?;
        thread::sleep(frame_duration);

        // 2. 眨眼动画序列
        println!("动画: 眨眼");
        let blink_frames = [1.0, 0.7, 0.4, 0.1, 0.4, 0.7, 1.0];
        for &blink_ratio in &blink_frames {
            self.lcd.fill_screen(COLOR_BLACK)?;
            self.draw_eyes_blink(blink_ratio)?;
            thread::sleep(Duration::from_millis(100));
        }

        // 3. 看左边
        println!("动画: 看左边");
        self.lcd.fill_screen(COLOR_BLACK)?;
        self.draw_eyes_look_left()?;
        thread::sleep(frame_duration);

        // 4. 回到中间
        self.lcd.fill_screen(COLOR_BLACK)?;
        self.draw_eyes()?;
        thread::sleep(Duration::from_millis(300));

        // 5. 看右边
        println!("动画: 看右边");
        self.lcd.fill_screen(COLOR_BLACK)?;
        self.draw_eyes_look_right()?;
        thread::sleep(frame_duration);

        // 6. 回到中间
        self.lcd.fill_screen(COLOR_BLACK)?;
        self.draw_eyes()?;
        thread::sleep(Duration::from_millis(300));

        // 7. 看上面
        println!("动画: 看上面");
        self.lcd.fill_screen(COLOR_BLACK)?;
        self.draw_eyes_look_up()?;
        thread::sleep(frame_duration);

        // 8. 回到中间
        self.lcd.fill_screen(COLOR_BLACK)?;
        self.draw_eyes()?;
        thread::sleep(Duration::from_millis(300));

        // 9. 看下面
        println!("动画: 看下面");
        self.lcd.fill_screen(COLOR_BLACK)?;
        self.draw_eyes_look_down()?;
        thread::sleep(frame_duration);

        // 10. 回到中间
        self.lcd.fill_screen(COLOR_BLACK)?;
        self.draw_eyes()?;
        thread::sleep(Duration::from_millis(300));

        // 11. 最后再眨一次眼
        println!("动画: 最后眨眼");
        for &blink_ratio in &blink_frames {
            self.lcd.fill_screen(COLOR_BLACK)?;
            self.draw_eyes_blink(blink_ratio)?;
            thread::sleep(Duration::from_millis(80));
        }

        Ok(())
    }
}
