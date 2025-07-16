use std::time;

use crate::{
    actors::wifi::WifiEvent,
    display::Display,
    events::{AppEvent, EventHandler, SystemEvent},
    peripherals::qmi8658::motion_detector::MotionState,
};

use anyhow::Result;

pub struct App<'a> {
    display: Display<'a>,
}

impl<'a> App<'a> {
    pub fn new(display: Display<'a>) -> Self {
        Self { display }
    }

    fn handle_motion(&mut self, motion_state: MotionState) -> Result<()> {
        let time = unsafe { esp_idf_sys::esp_timer_get_time() };
        println!("收到晃动事件: {:?}, time: {}", motion_state, time);
        self.display.on_motion(motion_state)?;
        self.display.update()?;
        Ok(())
    }

    fn handle_wifi(&mut self, wifi_event: WifiEvent) -> Result<()> {
        match wifi_event {
            WifiEvent::Connected(ip) => {
                println!("WiFi连接成功! IP: {}", ip);
            }
            WifiEvent::Disconnected => {
                println!("WiFi连接断开");
            }
            WifiEvent::ConnectionFailed(error) => {
                self.display
                    .enter_error(format!("WiFi连接失败: {}", error))?;
            }
            WifiEvent::StatusUpdate(status) => {
                println!("WiFi状态更新: {:?}", status);
            }
            WifiEvent::ScanResult(networks) => {
                println!("扫描到的网络: {:?}", networks);
            }
        }

        Ok(())
    }

    fn handle_system(&mut self, system_event: SystemEvent) -> Result<()> {
        match system_event {
            SystemEvent::LowBattery => {
                self.display.enter_error("电量不足".to_string())?;
            }
            SystemEvent::LowMemory => {
                self.display.enter_error("内存不足".to_string())?;
            }
            SystemEvent::HardwareError(error) => {
                self.display.enter_error(format!("硬件错误: {}", error))?;
            }
            SystemEvent::Shutdown => {
                println!("系统即将关闭");
            }
        }

        Ok(())
    }
}

impl<'a> EventHandler for App<'a> {
    fn handle_event(&mut self, event: AppEvent) -> Result<()> {
        match event {
            AppEvent::Motion(motion_state) => self.handle_motion(motion_state),
            AppEvent::Wifi(wifi_event) => self.handle_wifi(wifi_event),
            AppEvent::System(system_event) => self.handle_system(system_event),
        }
    }
}
