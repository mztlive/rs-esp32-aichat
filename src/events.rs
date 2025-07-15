// src/events.rs
use std::sync::mpsc;
use crate::{
    actors::wifi::WifiEvent,
    peripherals::qmi8658::motion_detector::MotionState,
};

/// 应用事件枚举，用于统一处理来自各个子线程的消息
#[derive(Debug, Clone)]
pub enum AppEvent {
    /// 运动传感器事件
    Motion(MotionState),
    
    /// WiFi事件
    Wifi(WifiEvent),
    
    /// 用户输入事件
    UserInput(UserInputEvent),
    
    /// 定时器事件
    Timer(TimerEvent),
    
    /// 系统事件
    System(SystemEvent),
}

/// 用户输入事件
#[derive(Debug, Clone)]
pub enum UserInputEvent {
    /// 按键按下
    ButtonPress,
    /// 确认操作
    Confirm,
    /// 取消操作
    Cancel,
    /// 进入设置
    Settings,
    /// 返回主界面
    Back,
}

/// 定时器事件
#[derive(Debug, Clone)]
pub enum TimerEvent {
    /// 主循环定时器
    MainLoop,
    /// 状态超时
    StateTimeout,
    /// 动画帧更新
    AnimationFrame,
}

/// 系统事件
#[derive(Debug, Clone)]
pub enum SystemEvent {
    /// 低电量警告
    LowBattery,
    /// 内存不足
    LowMemory,
    /// 硬件错误
    HardwareError(String),
    /// 应用退出
    Shutdown,
}

/// 事件发送器类型
pub type EventSender = mpsc::Sender<AppEvent>;

/// 事件接收器类型
pub type EventReceiver = mpsc::Receiver<AppEvent>;

/// 事件总线管理器
pub struct EventBus {
    /// 事件发送器
    sender: EventSender,
    /// 事件接收器
    receiver: EventReceiver,
}

impl EventBus {
    /// 创建新的事件总线
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        Self { sender, receiver }
    }

    /// 获取事件发送器的克隆
    pub fn get_sender(&self) -> EventSender {
        self.sender.clone()
    }

    /// 获取事件接收器的可变引用
    pub fn get_receiver(&mut self) -> &mut EventReceiver {
        &mut self.receiver
    }

    /// 尝试接收事件（非阻塞）
    pub fn try_recv(&self) -> Result<AppEvent, mpsc::TryRecvError> {
        self.receiver.try_recv()
    }

    /// 接收事件（阻塞）
    pub fn recv(&self) -> Result<AppEvent, mpsc::RecvError> {
        self.receiver.recv()
    }
}

/// 事件处理器接口
pub trait EventHandler {
    /// 处理应用事件
    fn handle_event(&mut self, event: AppEvent) -> anyhow::Result<()>;
}

/// 事件发送器辅助函数
pub fn send_motion_event(sender: &EventSender, motion_state: MotionState) -> Result<(), mpsc::SendError<AppEvent>> {
    sender.send(AppEvent::Motion(motion_state))
}

pub fn send_wifi_event(sender: &EventSender, wifi_event: WifiEvent) -> Result<(), mpsc::SendError<AppEvent>> {
    sender.send(AppEvent::Wifi(wifi_event))
}

pub fn send_user_input_event(sender: &EventSender, user_input: UserInputEvent) -> Result<(), mpsc::SendError<AppEvent>> {
    sender.send(AppEvent::UserInput(user_input))
}

pub fn send_timer_event(sender: &EventSender, timer_event: TimerEvent) -> Result<(), mpsc::SendError<AppEvent>> {
    sender.send(AppEvent::Timer(timer_event))
}

pub fn send_system_event(sender: &EventSender, system_event: SystemEvent) -> Result<(), mpsc::SendError<AppEvent>> {
    sender.send(AppEvent::System(system_event))
}