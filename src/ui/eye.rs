use anyhow::Result;
use crate::graphics::primitives::GraphicsPrimitives;
use crate::lcd::{COLOR_BLACK, COLOR_BLUE, COLOR_WHITE, LCD_HEIGHT, LCD_WIDTH};

pub struct Eye<'a> {
    primitives: &'a GraphicsPrimitives<'a>,
}

impl<'a> Eye<'a> {
    pub fn new(primitives: &'a GraphicsPrimitives<'a>) -> Self {
        Self { primitives }
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
        self.primitives.draw_filled_circle(center_x, center_y, eyeball_radius, COLOR_BLUE)?;

        // 绘制瞳孔（黑色，稍微偏右下）
        let pupil_x = center_x + eye_size / 6;
        let pupil_y = center_y + eye_size / 6;
        self.primitives.draw_filled_circle(pupil_x, pupil_y, pupil_radius, COLOR_BLACK)?;

        // 绘制高光（白色，在瞳孔左上角）
        let highlight_x = pupil_x - pupil_radius / 3;
        let highlight_y = pupil_y - pupil_radius / 3;
        self.primitives.draw_filled_circle(highlight_x, highlight_y, highlight_radius, COLOR_WHITE)?;

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
        self.primitives.draw_filled_circle(center_x, center_y, eyeball_radius, COLOR_BLUE)?;

        // 绘制瞳孔（黑色，可以偏移）
        let pupil_x = center_x + pupil_offset_x;
        let pupil_y = center_y + pupil_offset_y;
        self.primitives.draw_filled_circle(pupil_x, pupil_y, pupil_radius, COLOR_BLACK)?;

        // 绘制高光（白色，在瞳孔左上角）
        let highlight_x = pupil_x - pupil_radius / 3;
        let highlight_y = pupil_y - pupil_radius / 3;
        self.primitives.draw_filled_circle(highlight_x, highlight_y, highlight_radius, COLOR_WHITE)?;

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
            self.primitives.draw_bitmap(
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
                self.primitives.draw_bitmap(x_start, y_coord, x_end, y_coord + 1, &line_buffer)?;
            }
        }

        // 如果眼睛开度足够，绘制瞳孔和高光
        if eye_height > eyeball_radius / 2 {
            let pupil_x = center_x + eye_size / 6;
            let pupil_y = center_y + eye_size / 6;
            let compressed_pupil_radius = (pupil_radius as f32 * blink_ratio) as i32;

            if compressed_pupil_radius > 0 {
                self.primitives.draw_filled_circle(pupil_x, pupil_y, compressed_pupil_radius, COLOR_BLACK)?;

                let highlight_x = pupil_x - compressed_pupil_radius / 3;
                let highlight_y = pupil_y - compressed_pupil_radius / 3;
                let compressed_highlight_radius = (highlight_radius as f32 * blink_ratio) as i32;

                if compressed_highlight_radius > 0 {
                    self.primitives.draw_filled_circle(
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
}