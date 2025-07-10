// src/main.rs
use anyhow::Result;
use esp_idf_hal::{delay::FreeRtos, peripherals::Peripherals};

mod graphics;
mod lcd;
mod lcd_cmds;
use lcd::LcdController;
use tinybmp::Bmp;

use crate::graphics::{
    colors::{BLUE, GREEN, RED, WHITE, YELLOW},
    layout::{GridPosition, STATUS_BAR},
    primitives::GraphicsPrimitives,
};

fn main() -> Result<()> {
    // 必须先调用，打补丁
    esp_idf_sys::link_patches();

    println!("=== ESP32 LCD 诊断测试程序 ===");

    // 取得外设
    let p = Peripherals::take().unwrap();

    // 初始化 LCD 控制器
    println!("正在初始化LCD控制器...");
    let mut lcd = LcdController::new(p)?;

    // lcd.fill_screen(COLOR_BLACK)?;

    // let bmp_data = include_bytes!("../assets/xk.bmp");
    // // Parse the BMP file.
    // let bmp = Bmp::from_slice(bmp_data).unwrap();

    let mut primitives = GraphicsPrimitives::new(&mut lcd);

    // 演示新的快速绘制功能
    primitives.fill_screen(WHITE)?;
    primitives.fill_rect(&STATUS_BAR, RED)?;

    loop {
        FreeRtos::delay_ms(1000);
    }
}
