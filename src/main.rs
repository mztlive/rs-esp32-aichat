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

    // 开启背光
    println!("开启背光...");
    lcd.set_backlight(true)?;

    println!("显示纯白色...");
    lcd.fill_screen(COLOR_WHITE)?;

    // 在屏幕中心绘制圆形
    println!("在屏幕中心绘制圆形...");
    let center_x = lcd::LCD_WIDTH / 2; // 180
    let center_y = lcd::LCD_HEIGHT / 2; // 180

    // 绘制红色实心圆形，半径50
    lcd.draw_filled_circle(center_x, center_y, 50, COLOR_RED)?;

    // 绘制蓝色圆形轮廓，半径70
    lcd.draw_circle(center_x, center_y, 70, COLOR_BLUE)?;

    // 绘制绿色圆形轮廓，半径30
    lcd.draw_circle(center_x, center_y, 30, COLOR_GREEN)?;

    // 保持运行
    loop {
        FreeRtos::delay_ms(10000);
    }
}
