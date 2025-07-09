// src/main.rs
use anyhow::Result;
use esp_idf_hal::{delay::FreeRtos, peripherals::Peripherals};

mod lcd;
mod lcd_cmds;
use embedded_graphics::pixelcolor::Rgb565;
use lcd::{LcdController, COLOR_WHITE};

use crate::lcd::{COLOR_BLACK, COLOR_RED};

fn main() -> Result<()> {
    // 必须先调用，打补丁
    esp_idf_sys::link_patches();

    println!("=== ESP32 LCD 诊断测试程序 ===");

    // 取得外设
    let p = Peripherals::take().unwrap();

    // 初始化 LCD 控制器
    println!("正在初始化LCD控制器...");
    let mut lcd = LcdController::new(p)?;

    println!("显示纯白色...");
    lcd.fill_screen(COLOR_WHITE)?; // 保持运行

    // 暂停3秒
    FreeRtos::delay_ms(3000);

    lcd.fill_screen(COLOR_RED)?;

    // 暂停3秒
    FreeRtos::delay_ms(3000);

    // 暂停3秒
    lcd.fill_screen(COLOR_BLACK)?;

    // lcd.fill_screen(ColorRe)

    lcd.draw_text("Hello ESP32!", 180, 180, Rgb565::new(31, 63, 31))?;

    loop {
        FreeRtos::delay_ms(10000);
    }
}
