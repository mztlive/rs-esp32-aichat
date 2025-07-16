use std::thread;
use std::time::Duration;

use anyhow::Result;
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::{Gpio10, Gpio11};
use esp_idf_hal::i2c::I2C0;
use esp_idf_svc::timer::EspTimer;
use esp_idf_sys::esp_timer_get_time;

use crate::peripherals::qmi8658::{
    driver::{QMI8658Driver, SensorData},
    motion_detector::{MotionDetector, MotionState},
    QMI8658_ADDRESS_HIGH,
};

pub struct MotionActor<'a> {
    qmi8658: QMI8658Driver<'a>,
    motion_detector: MotionDetector,
    app_event_sender: crate::events::EventSender,
    last_state: Option<MotionState>,
}

impl<'a> MotionActor<'a> {
    pub fn new(
        i2c: I2C0,
        sda: Gpio11,
        scl: Gpio10,
        app_event_sender: crate::events::EventSender,
    ) -> Result<Self> {
        let qmi8658 = QMI8658Driver::new(i2c, sda, scl, QMI8658_ADDRESS_HIGH)?;
        let motion_detector = MotionDetector::new();

        Ok(Self {
            qmi8658,
            motion_detector,
            app_event_sender,
            last_state: None,
        })
    }

    pub fn run(&mut self) {
        println!("Motion actor started");

        loop {
            // 读取传感器数据并检测运动
            match self.qmi8658.read_sensor_data() {
                Ok(sensor_data) => {
                    let motion_state = self.motion_detector.detect_motion(&sensor_data);

                    let time = unsafe { esp_timer_get_time() };
                    println!("读取到运动状态: {:?}, time: {}", motion_state, time);

                    // 如果最后一次的状态和当前状态不一致，发送事件
                    if self.last_state != Some(motion_state) {
                        self.last_state = Some(motion_state);

                        // 发送运动事件到主事件总线
                        if let Err(e) =
                            crate::events::send_motion_event(&self.app_event_sender, motion_state)
                        {
                            eprintln!("Failed to send motion event: {}", e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Sensor read error: {}", e);
                }
            }

            FreeRtos::delay_ms(500);
        }
    }
}

pub struct MotionActorManager {
    // 简化版本不需要命令通道，只是启动后台线程
}

impl MotionActorManager {
    pub fn new(
        i2c: I2C0,
        sda: Gpio11,
        scl: Gpio10,
        app_event_sender: crate::events::EventSender,
    ) -> Result<Self> {
        // 先在当前线程创建actor，这样生命周期明确
        let mut actor = MotionActor::new(i2c, sda, scl, app_event_sender)?;

        thread::spawn(move || {
            actor.run();
        });

        Ok(Self {})
    }
}
