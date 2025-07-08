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

    // 先测试简单绘制
    println!("正在清屏...");
    match lcd.clear(0xF800) {
        Ok(()) => println!("清屏成功"),
        Err(e) => println!("清屏失败: {:?}", e),
    }
    FreeRtos::delay_ms(2000);
    
    println!("正在显示白色...");
    match lcd.clear(0xFFFF) {
        Ok(()) => println!("白色显示成功"),
        Err(e) => println!("白色显示失败: {:?}", e),
    }
    FreeRtos::delay_ms(2000);
    
    println!("正在显示单个像素...");
    match lcd.draw_bitmap(0, 0, 1, 1, [0xF800].as_ptr()) {
        Ok(()) => println!("单像素绘制成功"),
        Err(e) => println!("单像素绘制失败: {:?}", e),
    }
    FreeRtos::delay_ms(2000);

    // 显示测试图案
    println!("正在显示测试图案...");
    match lcd.draw_test_pattern() {
        Ok(()) => println!("测试图案显示成功"),
        Err(e) => println!("测试图案显示失败: {:?}", e),
    }

    // 保持运行
    FreeRtos::delay_ms(3_600_000);
    Ok(())
}
