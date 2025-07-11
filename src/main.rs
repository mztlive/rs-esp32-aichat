// src/main.rs
use anyhow::Result;
use esp_idf_hal::{delay::FreeRtos, i2c::I2cConfig, peripherals::Peripherals, prelude::*};

mod app;
mod graphics;
mod peripherals;

use crate::{
    app::{ChatApp, UserInput},
    graphics::primitives::GraphicsPrimitives,
    peripherals::{
        qmi8658::motion_detector::{MotionDetector, MotionState},
        st77916::lcd::LcdController,
    },
};

fn main() -> Result<()> {
    // 必须先调用，打补丁
    esp_idf_sys::link_patches();

    println!("=== ESP32 AI 聊天助手 ===");

    // 取得外设
    let p = Peripherals::take().unwrap();

    // 传感器gpio
    let sda = p.pins.gpio11;
    let scl = p.pins.gpio10;
    let i2c = p.i2c0;

    // lcd背光控制gpio
    let bl_io = p.pins.gpio5;

    // 初始化 QMI8658 传感器
    println!("正在初始化QMI8658传感器...");
    let mut qmi8658 = peripherals::qmi8658::QMI8658::new(
        i2c,
        sda,
        scl,
        peripherals::qmi8658::QMI8658_ADDRESS_HIGH,
    )?;

    // 初始化运动检测器
    let mut motion_detector = MotionDetector::new();

    // 初始化 LCD 控制器
    println!("正在初始化LCD控制器...");
    let mut lcd = LcdController::new(bl_io)?;

    // 创建图形绘制接口
    let graphics = GraphicsPrimitives::new(&mut lcd);

    // 创建应用实例
    let mut app = ChatApp::new(graphics);

    println!("应用启动成功，进入主循环...");

    // 主事件循环
    let mut loop_counter = 0;
    loop {
        let sensor_data = qmi8658.read_sensor_data()?;
        let motion_state = motion_detector.detect_motion(&sensor_data);

        println!("mation_state is: {:?}", motion_state);

        // 检测到晃动状态变化时的特殊处理
        if matches!(motion_state, MotionState::Shaking)
            && !matches!(app.get_state(), app::AppState::Dizziness)
        {
            println!("检测到晃动！进入晃动状态");
            app.enter_dizziness()?;
        }

        // 当从晃动状态恢复到静止状态时，检查是否可以返回主界面
        if matches!(motion_state, MotionState::Still | MotionState::Tilting)
            && app.can_exit_dizziness()
        {
            println!("已经可以结束晃动");
            app.handle_input(UserInput::Back)?;
        }

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
