use crate::graphics::primitives::GraphicsPrimitives;
use crate::graphics::rgb565_from_u16;
use crate::lcd::{COLOR_BLACK, COLOR_BLUE, COLOR_WHITE, LCD_HEIGHT, LCD_WIDTH};
use anyhow::Result;

pub struct Eye<'a> {
    primitives: &'a mut GraphicsPrimitives<'a>,
}

impl<'a> Eye<'a> {
    pub fn new(primitives: &'a mut GraphicsPrimitives<'a>) -> Self {
        Self { primitives }
    }

    /// 绘制一个眼睛
    pub fn draw_eye(&mut self, center_x: i32, center_y: i32, eye_size: i32) -> Result<()> {
        // 眼球半径
        let eyeball_radius = eye_size;
        // 瞳孔半径
        let pupil_radius = eye_size / 2;
        // 高光半径
        let highlight_radius = eye_size / 4;

        // 计算瞳孔和高光位置
        let pupil_x = center_x + eye_size / 6;
        let pupil_y = center_y + eye_size / 6;
        let highlight_x = pupil_x - pupil_radius / 3;
        let highlight_y = pupil_y - pupil_radius / 3;

        // 创建缓冲区
        let diameter = (eyeball_radius * 2) as usize;
        let buffer_size = diameter * diameter;
        let mut buffer = vec![COLOR_BLUE; buffer_size];

        // 在缓冲区中绘制眼睛
        for y in 0..diameter {
            for x in 0..diameter {
                let pixel_x = x as i32 - eyeball_radius;
                let pixel_y = y as i32 - eyeball_radius;
                let world_x = center_x + pixel_x;
                let world_y = center_y + pixel_y;

                // 检查是否在眼球范围内
                if pixel_x * pixel_x + pixel_y * pixel_y <= eyeball_radius * eyeball_radius {
                    let mut color = COLOR_BLUE;

                    // 检查是否在瞳孔范围内
                    let pupil_dx = world_x - pupil_x;
                    let pupil_dy = world_y - pupil_y;
                    if pupil_dx * pupil_dx + pupil_dy * pupil_dy <= pupil_radius * pupil_radius {
                        color = COLOR_BLACK;

                        // 检查是否在高光范围内
                        let highlight_dx = world_x - highlight_x;
                        let highlight_dy = world_y - highlight_y;
                        if highlight_dx * highlight_dx + highlight_dy * highlight_dy
                            <= highlight_radius * highlight_radius
                        {
                            color = COLOR_WHITE;
                        }
                    }

                    buffer[y * diameter + x] = color;
                } else {
                    // 眼球外部设为透明（这里使用黑色作为背景）
                    buffer[y * diameter + x] = COLOR_BLACK;
                }
            }
        }

        // 一次性绘制整个眼睛
        self.primitives.draw_bitmap(
            center_x - eyeball_radius,
            center_y - eyeball_radius,
            center_x + eyeball_radius,
            center_y + eyeball_radius,
            &buffer,
        )?;

        Ok(())
    }

    /// 绘制一个眼睛，支持瞳孔位置偏移
    pub fn draw_eye_with_pupil_offset(
        &mut self,
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

        // 计算瞳孔和高光位置
        let pupil_x = center_x + pupil_offset_x;
        let pupil_y = center_y + pupil_offset_y;
        let highlight_x = pupil_x - pupil_radius / 3;
        let highlight_y = pupil_y - pupil_radius / 3;

        // 创建缓冲区
        let diameter = (eyeball_radius * 2) as usize;
        let buffer_size = diameter * diameter;
        let mut buffer = vec![COLOR_BLUE; buffer_size];

        // 在缓冲区中绘制眼睛
        for y in 0..diameter {
            for x in 0..diameter {
                let pixel_x = x as i32 - eyeball_radius;
                let pixel_y = y as i32 - eyeball_radius;
                let world_x = center_x + pixel_x;
                let world_y = center_y + pixel_y;

                // 检查是否在眼球范围内
                if pixel_x * pixel_x + pixel_y * pixel_y <= eyeball_radius * eyeball_radius {
                    let mut color = COLOR_BLUE;

                    // 检查是否在瞳孔范围内
                    let pupil_dx = world_x - pupil_x;
                    let pupil_dy = world_y - pupil_y;
                    if pupil_dx * pupil_dx + pupil_dy * pupil_dy <= pupil_radius * pupil_radius {
                        color = COLOR_BLACK;

                        // 检查是否在高光范围内
                        let highlight_dx = world_x - highlight_x;
                        let highlight_dy = world_y - highlight_y;
                        if highlight_dx * highlight_dx + highlight_dy * highlight_dy
                            <= highlight_radius * highlight_radius
                        {
                            color = COLOR_WHITE;
                        }
                    }

                    buffer[y * diameter + x] = color;
                } else {
                    // 眼球外部设为透明（这里使用黑色作为背景）
                    buffer[y * diameter + x] = COLOR_BLACK;
                }
            }
        }

        // 一次性绘制整个眼睛
        self.primitives.draw_bitmap(
            center_x - eyeball_radius,
            center_y - eyeball_radius,
            center_x + eyeball_radius,
            center_y + eyeball_radius,
            &buffer,
        )?;

        Ok(())
    }

    /// 绘制椭圆形眼睛（用于眨眼效果）
    pub fn draw_eye_blink(
        &mut self,
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
                self.primitives
                    .draw_bitmap(x_start, y_coord, x_end, y_coord + 1, &line_buffer)?;
            }
        }

        // 如果眼睛开度足够，绘制瞳孔和高光
        if eye_height > eyeball_radius / 2 {
            let pupil_x = center_x + eye_size / 6;
            let pupil_y = center_y + eye_size / 6;
            let compressed_pupil_radius = (pupil_radius as f32 * blink_ratio) as i32;

            if compressed_pupil_radius > 0 {
                self.primitives.draw_filled_circle(
                    pupil_x,
                    pupil_y,
                    compressed_pupil_radius,
                    rgb565_from_u16(COLOR_BLACK),
                )?;

                let highlight_x = pupil_x - compressed_pupil_radius / 3;
                let highlight_y = pupil_y - compressed_pupil_radius / 3;
                let compressed_highlight_radius = (highlight_radius as f32 * blink_ratio) as i32;

                if compressed_highlight_radius > 0 {
                    self.primitives.draw_filled_circle(
                        highlight_x,
                        highlight_y,
                        compressed_highlight_radius,
                        rgb565_from_u16(COLOR_WHITE),
                    )?;
                }
            }
        }

        Ok(())
    }
}
