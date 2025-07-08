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

    // 显示测试图案
    lcd.draw_test_pattern()?;

    // 保持运行
    FreeRtos::delay_ms(3_600_000);
    Ok(())
}
