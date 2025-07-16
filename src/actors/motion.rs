use std::thread;
use std::time::Duration;

use anyhow::Result;
use esp_idf_hal::gpio::{Gpio10, Gpio11};
use esp_idf_hal::i2c::I2C0;

use crate::peripherals::qmi8658::{
    driver::{QMI8658Driver, SensorData},
    motion_detector::{MotionDetector, MotionState},
    QMI8658_ADDRESS_HIGH,
};

pub struct MotionActor {
    qmi8658: Box<QMI8658Driver<'static>>,
    motion_detector: MotionDetector,
    app_event_sender: crate::events::EventSender,
}

impl MotionActor {
    pub fn new(
        i2c: I2C0,
        sda: Gpio11,
        scl: Gpio10,
        app_event_sender: crate::events::EventSender,
    ) -> Result<Self> {
        let qmi8658 = QMI8658Driver::new(i2c, sda, scl, QMI8658_ADDRESS_HIGH)?;
        let qmi8658_boxed: Box<QMI8658Driver<'static>> =
            unsafe { std::mem::transmute(Box::new(qmi8658)) };
        let motion_detector = MotionDetector::new();

        Ok(Self {
            qmi8658: qmi8658_boxed,
            motion_detector,
            app_event_sender,
        })
    }

    pub fn run(&mut self) {
        println!("Motion actor started");

        loop {
            // 读取传感器数据并检测运动
            match self.qmi8658.read_sensor_data() {
                Ok(sensor_data) => {
                    let motion_state = self.motion_detector.detect_motion(&sensor_data);

                    // 发送运动事件到主事件总线
                    if let Err(e) =
                        crate::events::send_motion_event(&self.app_event_sender, motion_state)
                    {
                        eprintln!("Failed to send motion event: {}", e);
                    }
                }
                Err(e) => {
                    eprintln!("Sensor read error: {}", e);
                }
            }

            // 50ms间隔，约20Hz采样率
            std::thread::sleep(Duration::from_millis(50));
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
        thread::Builder::new()
            .stack_size(32 * 1024)
            .name("motion_actor".to_string())
            .spawn(
                move || match MotionActor::new(i2c, sda, scl, app_event_sender) {
                    Ok(mut actor) => {
                        actor.run();
                    }
                    Err(e) => {
                        eprintln!("Failed to create motion actor: {}", e);
                    }
                },
            )
            .expect("Failed to spawn motion actor thread");

        Ok(Self {})
    }
}
