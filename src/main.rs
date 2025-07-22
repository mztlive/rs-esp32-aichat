use std::time::Duration;

// src/main.rs
use anyhow::Result;
use esp_idf_hal::{delay::FreeRtos, peripherals::Peripherals};
use esp_idf_svc::{eventloop::EspSystemEventLoop, nvs::EspDefaultNvsPartition};
use esp_idf_sys::{
    esp_timer_get_time, heap_caps_get_free_size, heap_caps_get_largest_free_block,
    sr::{
        afe_config_init, afe_mode_t_AFE_MODE_HIGH_PERF, afe_type_t_AFE_TYPE_SR,
        esp_afe_handle_from_config, esp_srmodel_init,
    },
    MALLOC_CAP_INTERNAL,
};

mod actors;
mod api;
mod app;
mod display;
mod events;
mod graphics;
mod peripherals;

use crate::{
    actors::{motion::MotionActorManager, wifi::WifiActorManager},
    api::{
        client::ApiClient,
        pcm_client::{PcmClient, PcmClientConfig},
    },
    app::App,
    display::Display,
    events::{EventBus, EventHandler},
    graphics::primitives::GraphicsPrimitives,
    peripherals::{microphone, st77916::lcd::LcdController, wifi::WifiConfig},
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

    // 创建事件总线
    let event_bus = EventBus::new();
    let event_sender = event_bus.get_sender();

    // 初始化运动检测actor（自动启动后台线程）
    println!("正在初始化运动检测器...");
    let _motion_actor = MotionActorManager::new(i2c, sda, scl, event_sender.clone())?;

    // 然后初始化WiFi系统
    let sys_loop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;

    println!("正在初始化WiFi...");
    let wifi_actor = WifiActorManager::new(p.modem, sys_loop, Some(nvs), event_sender.clone())?;

    let wifi_config = WifiConfig::new("fushangyun", "fsy@666888");

    wifi_actor.connect(wifi_config)?;

    // mic gpio
    let i2s = p.i2s0;
    let ws = p.pins.gpio2;
    let sck = p.pins.gpio15;
    let sd = p.pins.gpio39;
    let mic = microphone::i2s_microphone::I2sMicrophone::new(i2s, ws, sck, sd, 16000)?;

    // lcd背光控制gpio - 先初始化显示系统
    let bl_io = p.pins.gpio5;
    // let app = DisplayActorManager::new(bl_io);
    let mut lcd = LcdController::new(bl_io).unwrap();
    let graphics = GraphicsPrimitives::new(&mut lcd);
    let display = Display::new(graphics);

    let mut app = App::new(display, mic);

    println!("应用启动成功，进入主循环...");

    loop {
        // 处理事件
        while let Ok(event) = event_bus.try_recv() {
            if let Err(e) = app.handle_event(event) {
                eprintln!("处理事件失败: {}", e);
            }
        }

        // 定期更新显示（用于动画和UI刷新，但时间计算不再依赖此频率）
        if let Err(e) = app.update() {
            eprintln!("显示更新失败: {}", e);
        }

        FreeRtos::delay_ms(50);
    }
}
