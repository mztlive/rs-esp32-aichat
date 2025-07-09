// src/main.rs
use anyhow::Result;
use esp_idf_hal::{delay::FreeRtos, peripherals::Peripherals};

mod lcd;
mod lcd_cmds;
use lcd::{LcdController, COLOR_BLUE, COLOR_GREEN, COLOR_RED, COLOR_WHITE};

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
    lcd.fill_screen(COLOR_WHITE)?;

    // 使用embedded-graphics绘制文本
    println!("绘制文本示例...");

    // 绘制白色文本
    lcd.draw_colored_text("Hello ESP32!", 10, 10, 255, 255, 255)?;

    // 绘制红色文本
    lcd.draw_colored_text("Red Text", 10, 30, 255, 0, 0)?;

    // 绘制绿色文本
    lcd.draw_colored_text("Green Text", 10, 50, 0, 255, 0)?;

    // 绘制蓝色文本
    lcd.draw_colored_text("Blue Text", 10, 70, 0, 0, 255)?;

    // 在屏幕底部绘制信息
    lcd.draw_colored_text("ESP32 with Rust", 10, 330, 255, 255, 0)?;

    // 保持运行
    loop {
        FreeRtos::delay_ms(10000);
    }
}
