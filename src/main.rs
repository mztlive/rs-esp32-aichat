// src/main.rs
use anyhow::Result;
use esp_idf_hal::{delay::FreeRtos, peripherals::Peripherals};

mod lcd;
use lcd::LcdController;

fn main() -> Result<()> {
    // 必须先调用，打补丁
    esp_idf_sys::link_patches();

    // 取得外设
    let p = Peripherals::take().unwrap();

    // 初始化 LCD 控制器
    let lcd = LcdController::new(p)?;

    // 先测试纯色显示
    println!("正在清屏...");
    lcd.clear(0xF800)?; // 红色背景
    FreeRtos::delay_ms(1000);
    
    println!("正在显示绿色...");
    lcd.clear(0x07E0)?; // 绿色背景
    FreeRtos::delay_ms(1000);
    
    println!("正在显示蓝色...");
    lcd.clear(0x001F)?; // 蓝色背景
    FreeRtos::delay_ms(1000);

    // 显示测试图案
    println!("正在显示测试图案...");
    lcd.draw_test_pattern()?;

    // 保持运行
    FreeRtos::delay_ms(3_600_000);
    Ok(())
}
