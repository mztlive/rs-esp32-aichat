use std::thread;

use anyhow::Result;
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::{Gpio10, Gpio11};
use esp_idf_hal::i2c::I2C0;
use esp_idf_sys::esp_timer_get_time;

/// 心跳间隔时间（微秒）
///
/// 用于确保即使运动状态未改变，也会定期发送心跳事件。
/// 设置为5秒（5,000,000微秒）以保持与应用程序的连接活跃。
const HEARTBEAT_INTERVAL_US: i64 = 5_000_000;

use crate::peripherals::qmi8658::{
    driver::QMI8658Driver,
    motion_detector::{MotionDetector, MotionState},
    QMI8658_ADDRESS_HIGH,
};

/// 运动传感器Actor
///
/// 负责在独立线程中运行运动检测逻辑，包括：
/// - 读取QMI8658传感器数据
/// - 检测运动状态变化
/// - 发送运动事件到应用程序事件总线
/// - 管理心跳机制确保连接活跃
pub struct MotionActor<'a> {
    /// QMI8658传感器驱动器实例
    qmi8658: QMI8658Driver<'a>,
    /// 运动检测器，用于分析传感器数据并识别运动模式
    motion_detector: MotionDetector,
    /// 应用程序事件发送器，用于发送运动事件到主事件总线
    app_event_sender: crate::events::EventSender,
    /// 上次检测到的运动状态，用于状态变化检测
    last_state: Option<MotionState>,
    /// 上次发送事件的时间戳（微秒），用于心跳机制
    last_sent_time: i64,
}

impl<'a> MotionActor<'a> {
    /// 创建新的运动传感器Actor实例
    ///
    /// # 参数
    /// * `i2c` - I2C0外设实例，用于与QMI8658传感器通信
    /// * `sda` - I2C数据线GPIO引脚（GPIO11）
    /// * `scl` - I2C时钟线GPIO引脚（GPIO10）
    /// * `app_event_sender` - 应用程序事件发送器，用于发送运动事件
    ///
    /// # 返回值
    /// * `Result<Self>` - 成功时返回MotionActor实例，失败时返回错误
    ///
    /// # 错误
    /// 如果QMI8658传感器初始化失败，将返回相应的错误信息
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
            last_sent_time: 0,
        })
    }

    /// 运行运动检测主循环
    ///
    /// 这是运动传感器Actor的核心方法，在独立线程中运行。
    /// 负责：
    /// - 定期读取QMI8658传感器数据
    /// - 检测运动状态变化
    /// - 发送运动事件到应用程序事件总线
    /// - 管理心跳机制
    ///
    /// # 循环逻辑
    /// 1. 读取传感器数据
    /// 2. 检测运动状态
    /// 3. 判断是否需要发送事件（状态变化或心跳超时）
    /// 4. 发送事件到应用程序
    /// 5. 延迟500ms后重复
    ///
    /// # 注意
    /// 此方法包含无限循环，应在独立线程中调用
    pub fn run(&mut self) {
        loop {
            // 读取传感器数据并检测运动
            match self.qmi8658.read_sensor_data() {
                Ok(sensor_data) => {
                    let motion_state = self.motion_detector.detect_motion(&sensor_data);

                    let time = unsafe { esp_timer_get_time() };

                    let should_send = self.last_state != Some(motion_state)
                        || (time - self.last_sent_time) >= HEARTBEAT_INTERVAL_US;

                    if should_send {
                        self.last_state = Some(motion_state);
                        self.last_sent_time = time;

                        // 发送运动事件到主事件总线
                        if let Err(e) =
                            crate::events::send_motion_event(&self.app_event_sender, motion_state)
                        {
                            log::info!("Failed to send motion event: {}", e);
                        }
                    }
                }
                Err(e) => {
                    log::info!("Sensor read error: {}", e);
                }
            }

            FreeRtos::delay_ms(500);
        }
    }
}

/// 运动传感器Actor管理器
///
/// 负责创建和管理运动传感器Actor的生命周期。
/// 这是一个简化版本，只负责启动后台线程，不提供命令通道。
///
/// # 特点
/// - 创建时自动启动独立线程运行MotionActor
/// - 不提供停止或控制机制（适合嵌入式系统的简单需求）
/// - 线程一旦启动将持续运行直到程序结束
pub struct MotionActorManager {
    // 简化版本不需要命令通道，只是启动后台线程
}

impl MotionActorManager {
    /// 创建新的运动传感器Actor管理器
    ///
    /// 此方法会立即创建MotionActor实例并在新线程中启动运行。
    ///
    /// # 参数
    /// * `i2c` - I2C0外设实例，用于与QMI8658传感器通信
    /// * `sda` - I2C数据线GPIO引脚（GPIO11）
    /// * `scl` - I2C时钟线GPIO引脚（GPIO10）
    /// * `app_event_sender` - 应用程序事件发送器，用于发送运动事件
    ///
    /// # 返回值
    /// * `Result<Self>` - 成功时返回MotionActorManager实例，失败时返回错误
    ///
    /// # 错误
    /// 如果MotionActor创建失败（通常是传感器初始化失败），将返回相应的错误信息
    ///
    /// # 注意
    /// - 此方法会立即启动后台线程
    /// - 线程将持续运行直到程序结束
    /// - 调用者无需手动管理线程生命周期
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
