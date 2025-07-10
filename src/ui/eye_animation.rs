use anyhow::Result;
use std::thread;
use std::time::Duration;

use crate::graphics::primitives::GraphicsPrimitives;
use crate::ui::eye::Eye;
use crate::lcd::{COLOR_BLACK, LCD_HEIGHT, LCD_WIDTH};

pub struct EyeAnimator<'a> {
    eye: Eye<'a>,
    primitives: &'a GraphicsPrimitives<'a>,
}

impl<'a> EyeAnimator<'a> {
    pub fn new(eye: Eye<'a>, primitives: &'a GraphicsPrimitives<'a>) -> Self {
        Self { eye, primitives }
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
        self.eye.draw_eye(left_eye_x, left_eye_y, eye_size)?;

        // 绘制右眼
        self.eye.draw_eye(right_eye_x, right_eye_y, eye_size)?;

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

        self.eye.draw_eye_with_pupil_offset(
            left_eye_x,
            left_eye_y,
            eye_size,
            pupil_offset_x,
            pupil_offset_y,
        )?;
        self.eye.draw_eye_with_pupil_offset(
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

        self.eye.draw_eye_with_pupil_offset(
            left_eye_x,
            left_eye_y,
            eye_size,
            pupil_offset_x,
            pupil_offset_y,
        )?;
        self.eye.draw_eye_with_pupil_offset(
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

        self.eye.draw_eye_with_pupil_offset(
            left_eye_x,
            left_eye_y,
            eye_size,
            pupil_offset_x,
            pupil_offset_y,
        )?;
        self.eye.draw_eye_with_pupil_offset(
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

        self.eye.draw_eye_with_pupil_offset(
            left_eye_x,
            left_eye_y,
            eye_size,
            pupil_offset_x,
            pupil_offset_y,
        )?;
        self.eye.draw_eye_with_pupil_offset(
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

        self.eye.draw_eye_blink(left_eye_x, left_eye_y, eye_size, blink_ratio)?;
        self.eye.draw_eye_blink(right_eye_x, right_eye_y, eye_size, blink_ratio)?;

        Ok(())
    }

    /// 播放眼睛动画序列
    pub fn play_eye_animation(&self) -> Result<()> {
        let frame_duration = Duration::from_millis(500);

        // 1. 正常眼睛
        println!("动画: 正常眼睛");
        self.primitives.fill_screen(COLOR_BLACK)?;
        self.draw_eyes()?;
        thread::sleep(frame_duration);

        // 2. 眨眼动画序列
        println!("动画: 眨眼");
        let blink_frames = [1.0, 0.7, 0.4, 0.1, 0.4, 0.7, 1.0];
        for &blink_ratio in &blink_frames {
            self.primitives.fill_screen(COLOR_BLACK)?;
            self.draw_eyes_blink(blink_ratio)?;
            thread::sleep(Duration::from_millis(100));
        }

        // 3. 看左边
        println!("动画: 看左边");
        self.primitives.fill_screen(COLOR_BLACK)?;
        self.draw_eyes_look_left()?;
        thread::sleep(frame_duration);

        // 4. 回到中间
        self.primitives.fill_screen(COLOR_BLACK)?;
        self.draw_eyes()?;
        thread::sleep(Duration::from_millis(300));

        // 5. 看右边
        println!("动画: 看右边");
        self.primitives.fill_screen(COLOR_BLACK)?;
        self.draw_eyes_look_right()?;
        thread::sleep(frame_duration);

        // 6. 回到中间
        self.primitives.fill_screen(COLOR_BLACK)?;
        self.draw_eyes()?;
        thread::sleep(Duration::from_millis(300));

        // 7. 看上面
        println!("动画: 看上面");
        self.primitives.fill_screen(COLOR_BLACK)?;
        self.draw_eyes_look_up()?;
        thread::sleep(frame_duration);

        // 8. 回到中间
        self.primitives.fill_screen(COLOR_BLACK)?;
        self.draw_eyes()?;
        thread::sleep(Duration::from_millis(300));

        // 9. 看下面
        println!("动画: 看下面");
        self.primitives.fill_screen(COLOR_BLACK)?;
        self.draw_eyes_look_down()?;
        thread::sleep(frame_duration);

        // 10. 回到中间
        self.primitives.fill_screen(COLOR_BLACK)?;
        self.draw_eyes()?;
        thread::sleep(Duration::from_millis(300));

        // 11. 最后再眨一次眼
        println!("动画: 最后眨眼");
        for &blink_ratio in &blink_frames {
            self.primitives.fill_screen(COLOR_BLACK)?;
            self.draw_eyes_blink(blink_ratio)?;
            thread::sleep(Duration::from_millis(80));
        }

        Ok(())
    }
}