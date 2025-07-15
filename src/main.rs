// src/main.rs
use anyhow::Result;
use esp_idf_hal::{delay::FreeRtos, peripherals::Peripherals};
use esp_idf_svc::{eventloop::EspSystemEventLoop, nvs::EspDefaultNvsPartition};
use esp_idf_sys::{heap_caps_get_free_size, heap_caps_get_largest_free_block, MALLOC_CAP_INTERNAL};

mod actors;
mod api;
mod app;
mod graphics;
mod peripherals;

use crate::{
    actors::{display::DisplayActorManager, wifi::WifiActorManager},
    app::ChatApp,
    graphics::{colors::WHITE, primitives::GraphicsPrimitives},
    peripherals::{
        qmi8658::motion_detector::MotionDetector, st77916::lcd::LcdController, wifi::WifiConfig,
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

    // 然后初始化WiFi系统
    let sys_loop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;

    println!("正在初始化WiFi...");
    let wifi_actor = WifiActorManager::new(p.modem, sys_loop, Some(nvs))?;

    let wifi_config = WifiConfig::new("fushangyun", "fsy@666888");

    wifi_actor.connect(wifi_config)?;

    // mic gpio
    // let i2s = p.i2s0;
    // let ws = p.pins.gpio2;
    // let sck = p.pins.gpio15;
    // let sd = p.pins.gpio39;

    // lcd背光控制gpio - 先初始化显示系统
    let bl_io = p.pins.gpio5;
    // let app = DisplayActorManager::new(bl_io);
    let mut lcd = LcdController::new(bl_io).unwrap();
    let graphics = GraphicsPrimitives::new(&mut lcd);
    let mut app = ChatApp::new(graphics);

    println!("应用启动成功，进入主循环...");

    loop {
        let sensor_data = qmi8658.read_sensor_data()?;
        let motion_state = motion_detector.detect_motion(&sensor_data);

        app.on_motion(motion_state).unwrap();
        app.update().unwrap();

        // 处理WiFi事件
        while let Ok(wifi_event) = wifi_actor.try_recv_event() {
            match wifi_event {
                crate::actors::wifi::WifiEvent::Connected(ip) => {
                    println!("WiFi连接成功! IP: {}", ip);
                }
                crate::actors::wifi::WifiEvent::Disconnected => {
                    println!("WiFi连接断开");
                }
                crate::actors::wifi::WifiEvent::ConnectionFailed(error) => {
                    println!("WiFi连接失败: {}", error);
                }
                crate::actors::wifi::WifiEvent::StatusUpdate(status) => {
                    println!("WiFi状态更新: {:?}", status);
                }
                crate::actors::wifi::WifiEvent::ScanResult(networks) => {
                    println!("扫描到的网络: {:?}", networks);
                }
            }
        }

        FreeRtos::delay_ms(50);
    }
}

fn print_internal(tag: &str) {
    let free = unsafe { heap_caps_get_free_size(MALLOC_CAP_INTERNAL) };
    let large = unsafe { heap_caps_get_largest_free_block(MALLOC_CAP_INTERNAL) };
    println!("{tag}: free={free}  largest={large}");
}
