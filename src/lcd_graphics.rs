use anyhow::Result;
use embedded_graphics::pixelcolor::Rgb565;

use crate::graphics::primitives::GraphicsPrimitives;
use crate::lcd::LcdController;
use crate::ui::eye::Eye;
use crate::ui::eye_animation::EyeAnimator;

pub struct LcdGraphics<'a> {
    primitives: GraphicsPrimitives<'a>,
}

impl<'a> LcdGraphics<'a> {
    pub fn new(lcd: &'a mut LcdController) -> Self {
        let primitives = GraphicsPrimitives::new(lcd);

        Self { primitives }
    }

    // 基础绘制API - 直接委托给primitives
    pub fn draw_pixel(&self, x: i32, y: i32, color: u16) -> Result<()> {
        self.primitives.draw_pixel(x, y, color)
    }

    pub fn draw_circle(&self, center_x: i32, center_y: i32, radius: i32, color: u16) -> Result<()> {
        self.primitives
            .draw_circle(center_x, center_y, radius, color)
    }

    pub fn draw_filled_circle(
        &self,
        center_x: i32,
        center_y: i32,
        radius: i32,
        color: u16,
    ) -> Result<()> {
        self.primitives
            .draw_filled_circle(center_x, center_y, radius, color)
    }

    pub fn draw_text(&mut self, text: &str, x: i32, y: i32, color: Rgb565) -> Result<()> {
        self.primitives.draw_text(text, x, y, color)
    }

    pub fn fill_screen(&self, color: u16) -> Result<()> {
        self.primitives.fill_screen(color)
    }

    // 眼睛相关API - 临时创建组件来执行
    pub fn draw_eyes(&self) -> Result<()> {
        let eye = Eye::new(&self.primitives);
        let animator = EyeAnimator::new(eye, &self.primitives);
        animator.draw_eyes()
    }

    pub fn draw_eyes_look_left(&self) -> Result<()> {
        let eye = Eye::new(&self.primitives);
        let animator = EyeAnimator::new(eye, &self.primitives);
        animator.draw_eyes_look_left()
    }

    pub fn draw_eyes_look_right(&self) -> Result<()> {
        let eye = Eye::new(&self.primitives);
        let animator = EyeAnimator::new(eye, &self.primitives);
        animator.draw_eyes_look_right()
    }

    pub fn draw_eyes_look_up(&self) -> Result<()> {
        let eye = Eye::new(&self.primitives);
        let animator = EyeAnimator::new(eye, &self.primitives);
        animator.draw_eyes_look_up()
    }

    pub fn draw_eyes_look_down(&self) -> Result<()> {
        let eye = Eye::new(&self.primitives);
        let animator = EyeAnimator::new(eye, &self.primitives);
        animator.draw_eyes_look_down()
    }

    pub fn draw_eyes_blink(&self, blink_ratio: f32) -> Result<()> {
        let eye = Eye::new(&self.primitives);
        let animator = EyeAnimator::new(eye, &self.primitives);
        animator.draw_eyes_blink(blink_ratio)
    }

    pub fn play_eye_animation(&self) -> Result<()> {
        let eye = Eye::new(&self.primitives);
        let animator = EyeAnimator::new(eye, &self.primitives);
        animator.play_eye_animation()
    }
}
