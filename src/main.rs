// src/main.rs
use anyhow::Result;
use embedded_graphics::{image::Image, prelude::Point, Drawable};
use esp_idf_hal::{delay::FreeRtos, peripherals::Peripherals};

mod graphics;
mod lcd;
mod lcd_cmds;
use lcd::{LcdController, COLOR_BLACK, COLOR_RED, COLOR_WHITE};
use tinybmp::Bmp;

use crate::graphics::{primitives::GraphicsPrimitives, rgb565_from_u16};

fn main() -> Result<()> {
    // 必须先调用，打补丁
    esp_idf_sys::link_patches();

    println!("=== ESP32 LCD 诊断测试程序 ===");

    // 取得外设
    let p = Peripherals::take().unwrap();

    // 初始化 LCD 控制器
    println!("正在初始化LCD控制器...");
    let mut lcd = LcdController::new(p)?;

    lcd.fill_screen(COLOR_WHITE)?;

    let bmp_data = include_bytes!("../assets/xk.bmp");
    // Parse the BMP file.
    let bmp = Bmp::from_slice(bmp_data).unwrap();

    let mut primitives = GraphicsPrimitives::new(&mut lcd);
    // let mut eye = Eye::new(&mut primitives);
    // let mut graphics = EyeAnimator::new(&mut eye);

    loop {
        // 播放一轮完整的眼睛动画
        // graphics.play_eye_animation()?;

        // 显示文本
        // graphics.draw_text("Phoenix.H!", 140, 280, Rgb565::new(31, 63, 31))?;

        primitives.draw_image(&bmp, 200, 200)?;

        primitives.draw_filled_circle(100, 100, 50, rgb565_from_u16(COLOR_RED))?;

        // 等待3秒后重新开始动画
        FreeRtos::delay_ms(3000);
    }
}
