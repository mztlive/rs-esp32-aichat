// src/main.rs
use anyhow::Result;
use esp_idf_hal::{delay::FreeRtos, peripherals::Peripherals};

mod actors;
mod app;
mod graphics;
mod peripherals;

use crate::{
    actors::display::DisplayActorManager,
    peripherals::{
        microphone::i2s_microphone::{AudioBuffer, I2sMicrophone},
        qmi8658::motion_detector::MotionDetector,
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

    // lcd背光控制gpio
    let bl_io = p.pins.gpio5;
    let app = DisplayActorManager::new(bl_io);

    // mic gpio
    let i2s = p.i2s0;
    let ws = p.pins.gpio2;
    let sck = p.pins.gpio15;
    let sd = p.pins.gpio39;

    println!("正在初始化I2S麦克风...");
    let mut mic = match I2sMicrophone::new(i2s, ws, sck, sd, 16000) {
        Ok(m) => {
            println!("I2S麦克风初始化成功");
            m
        }
        Err(e) => {
            println!("I2S麦克风初始化失败: {}", e);
            return Err(e);
        }
    };

    mic.start_recording()?;
    println!("mic initialized");

    // 录制3秒音频，方法内部自动管理缓冲区大小
    match mic.record_duration(3, None) {
        Ok(mut audio_buffer) => {
            println!(
                "缓冲区状态: 已用={}, 可用={}",
                audio_buffer.available_read(),
                audio_buffer.available_write()
            );

            // 可以进一步处理音频数据
            let mut sample_data = vec![0i16; 1000];
            let read_count = audio_buffer.read(&mut sample_data);
            println!("从缓冲区读取了 {} 个样本进行分析", read_count);
        }
        Err(e) => println!("录音失败: {}", e),
    }

    println!("应用启动成功，进入主循环...");

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
