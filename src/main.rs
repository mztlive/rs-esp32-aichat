// src/main.rs
use anyhow::Result;
use esp_idf_hal::{delay::FreeRtos, peripherals::Peripherals};

mod lcd;
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

    // 开启背光
    println!("开启背光...");
    lcd.set_backlight(true)?;

    // 简单纯色测试
    println!("显示纯红色...");
    lcd.fill_screen(COLOR_RED)?;
    FreeRtos::delay_ms(2000);

    println!("显示纯绿色...");
    lcd.fill_screen(COLOR_GREEN)?;
    FreeRtos::delay_ms(2000);

    println!("显示纯蓝色...");
    lcd.fill_screen(COLOR_BLUE)?;
    FreeRtos::delay_ms(2000);

    println!("显示纯白色...");
    lcd.fill_screen(COLOR_WHITE)?;

    // 保持运行
    loop {
        FreeRtos::delay_ms(10000);
    }
}
