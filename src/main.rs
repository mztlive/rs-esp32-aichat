// src/main.rs
use anyhow::Result;
use esp_idf_hal::{delay::FreeRtos, peripherals::Peripherals};

mod app;
mod graphics;
mod peripherals;

use crate::{
    app::{ChatApp, UserInput},
    graphics::primitives::GraphicsPrimitives,
    peripherals::st77916::lcd::LcdController,
};

fn main() -> Result<()> {
    // 必须先调用，打补丁
    esp_idf_sys::link_patches();

    println!("=== ESP32 AI 聊天助手 ===");

    // 取得外设
    let p = Peripherals::take().unwrap();

    // 初始化 LCD 控制器
    println!("正在初始化LCD控制器...");
    let mut lcd = LcdController::new(p)?;

    // 创建图形绘制接口
    let graphics = GraphicsPrimitives::new(&mut lcd);

    // 创建应用实例
    let mut app = ChatApp::new(graphics);

    println!("应用启动成功，进入主循环...");

    // 主事件循环
    let mut loop_counter = 0;
    loop {
        // 更新应用状态
        app.update()?;

        // 模拟用户输入处理（实际项目中这里会读取按键/触摸输入）
        if let Some(input) = simulate_user_input(loop_counter) {
            app.handle_input(input)?;
        }

        // 控制更新频率 (约20fps)
        FreeRtos::delay_ms(50);
        loop_counter += 1;
    }
}

/// 模拟用户输入（实际项目中替换为真实的输入处理）
fn simulate_user_input(loop_counter: u32) -> Option<UserInput> {
    match loop_counter {
        // 模拟一些用户操作来演示状态转换
        300 => Some(UserInput::ButtonPress), // 10秒后模拟按键
        600 => Some(UserInput::Settings),    // 20秒后进入设置
        900 => Some(UserInput::Back),        // 30秒后返回主界面
        _ => None,
    }
}
