use std::sync;

// src/main.rs
use anyhow::Result;
use esp_idf_hal::{delay::FreeRtos, i2c::I2cConfig, peripherals::Peripherals, prelude::*};

mod actors;
mod app;
mod graphics;
mod peripherals;

use crate::{
    actors::display::{DisplayActor, DisplayActorManager},
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

    println!("应用启动成功，进入主循环...");
    let app = DisplayActorManager::new(bl_io);

    loop {
        let sensor_data = qmi8658.read_sensor_data()?;
        let motion_state = motion_detector.detect_motion(&sensor_data);

        println!("mation_state is: {:?}", motion_state);
        app.on_motion(motion_state)?;

        // // 更新应用状态
        // app.update()?;

        // // 模拟用户输入处理（实际项目中这里会读取按键/触摸输入）
        // if let Some(input) = simulate_user_input(loop_counter) {
        //     app.handle_input(input)?;
        // }

        // // 控制更新频率 (约20fps)
        // FreeRtos::delay_ms(50);
        // loop_counter += 1;

        FreeRtos::delay_ms(50);
    }
}
