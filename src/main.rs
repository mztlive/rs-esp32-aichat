// src/main.rs
use anyhow::Result;
use esp_idf_hal::{delay::FreeRtos, peripherals::Peripherals};

mod actors;
mod api;
mod app;
mod graphics;
mod peripherals;

use crate::{
    actors::display::DisplayActorManager, peripherals::qmi8658::motion_detector::MotionDetector,
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

    // 然后初始化WiFi系统
    // let sys_loop = EspSystemEventLoop::take()?;
    // let nvs = EspDefaultNvsPartition::take()?;
    // println!("正在初始化WiFi...");
    // let mut wifi_manager = WifiManager::new(p.modem, sys_loop, Some(nvs))?;

    // let wifi_config = WifiConfig::new("fushangyun", "fsy@666888");

    // println!("尝试连接WiFi: {}", wifi_config.ssid);
    // match wifi_manager.connect_with_config(&wifi_config) {
    //     Ok(_) => {
    //         println!("WiFi连接成功!");
    //         if let Ok(ip) = wifi_manager.get_ip_info() {
    //             println!("IP地址: {:?}", ip);
    //         }
    //     }
    //     Err(e) => {
    //         println!("WiFi连接失败: {:?}", e);
    //     }
    // }

    // mic gpio
    // let i2s = p.i2s0;
    // let ws = p.pins.gpio2;
    // let sck = p.pins.gpio15;
    // let sd = p.pins.gpio39;

    // lcd背光控制gpio - 先初始化显示系统
    let bl_io = p.pins.gpio5;
    let app = DisplayActorManager::new(bl_io);

    println!("应用启动成功，进入主循环...");

    loop {
        let sensor_data = qmi8658.read_sensor_data()?;
        let motion_state = motion_detector.detect_motion(&sensor_data);

        // app.on_motion(motion_state)?;

        FreeRtos::delay_ms(50);
    }
}
