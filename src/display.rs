use anyhow::Result;

use crate::{
    graphics::{
        colors::BLACK,
        primitives::GraphicsPrimitives,
        screens::{dizziness, error, home, settings, thinking, tilting, welcome},
    },
    peripherals::qmi8658::motion_detector::MotionState,
};

/// 应用状态枚举
#[derive(Debug, Clone, PartialEq)]
pub enum DisplayState {
    /// 欢迎界面
    Welcome,
    /// 主界面
    Main,
    /// 设置界面
    Settings,

    /// 思考中状态可以用于模拟AI处理请求的过程
    Thinking,

    /// 当设备被摇晃时
    Dizziness,

    /// 设备倾斜
    Tilting,

    /// 错误界面
    Error(String),
}

/// 主应用结构
pub struct Display<'a> {
    /// 当前状态
    state: DisplayState,
    /// 图形绘制接口
    graphics: GraphicsPrimitives<'a>,
    /// 状态切换计时器（用于自动切换）
    state_timer: u32,
    /// 晃动状态开始时间
    dizziness_start_time: u32,
}

impl<'a> Display<'a> {
    /// 创建新的应用实例
    pub fn new(graphics: GraphicsPrimitives<'a>) -> Self {
        Display {
            state: DisplayState::Main,
            graphics,
            state_timer: 0,
            dizziness_start_time: 0,
        }
    }

    /// 主更新循环
    pub fn update(&mut self) -> Result<()> {
        // 增加计时器
        self.state_timer += 1;

        // 根据当前状态执行相应逻辑
        match &self.state {
            DisplayState::Welcome => welcome::draw(&mut self.graphics)?,
            DisplayState::Main => home::draw(&mut self.graphics)?,
            DisplayState::Settings => settings::draw(&mut self.graphics)?,
            DisplayState::Error(msg) => {
                error::draw(&mut self.graphics, msg)?;
                // 3秒后自动返回欢迎界面
                if self.state_timer > 150 {
                    self.enter_welcome()?;
                }
            }
            DisplayState::Thinking => thinking::draw(&mut self.graphics, self.state_timer)?,
            DisplayState::Dizziness => dizziness::draw(&mut self.graphics, self.state_timer)?,
            DisplayState::Tilting => tilting::draw(&mut self.graphics)?,
        }

        Ok(())
    }

    /// 处理用户输入
    pub fn back(&mut self) -> Result<()> {
        match &self.state {
            // 欢迎界面：任意按键进入主界面
            DisplayState::Welcome => {
                self.enter_main()?;
            }

            // 晃动状态：返回键回到主界面
            DisplayState::Dizziness => {
                self.exit_diszziness()?;
            }

            // 设备倾斜
            DisplayState::Tilting => {
                self.enter_main()?;
            }

            // 其他输入忽略
            _ => {}
        }

        Ok(())
    }

    /// 状态转换
    fn transition_to(&mut self, new_state: DisplayState) -> Result<()> {
        // 如果新状态和当前状态相同，则不进行任何操作
        if self.state == new_state {
            return Ok(());
        }

        self.state = new_state;
        self.state_timer = 0; // 重置计时器

        // 清屏准备绘制新状态
        self.graphics.fill_screen(BLACK)?;

        Ok(())
    }

    /// 获取当前状态
    pub fn get_state(&self) -> &DisplayState {
        &self.state
    }

    /// 统一的状态转换方法
    pub fn enter_welcome(&mut self) -> Result<()> {
        self.transition_to(DisplayState::Welcome)
    }

    pub fn enter_main(&mut self) -> Result<()> {
        self.transition_to(DisplayState::Main)
    }

    pub fn enter_settings(&mut self) -> Result<()> {
        self.transition_to(DisplayState::Settings)
    }

    pub fn enter_thinking(&mut self) -> Result<()> {
        self.transition_to(DisplayState::Thinking)
    }

    /// 进入摇晃状态
    ///
    /// 当检测到设备摇晃时调用，显示眩晕效果界面。
    /// 记录进入时间以控制最小持续时间。
    ///
    /// # 返回值
    /// * `Result<()>` - 状态切换结果
    ///
    /// # 特殊逻辑
    /// - 如果已经在摇晃状态，直接返回
    /// - 记录进入摇晃状态的精确时间戳（微秒级）
    /// - 用于后续的最小持续时间控制
    pub fn enter_dizziness(&mut self) -> Result<()> {
        if self.state == DisplayState::Dizziness {
            return Ok(()); // 已经在晃动状态，直接返回
        }

        // 记录进入晃动状态的绝对时间戳（微秒）
        self.dizziness_start_time = unsafe { esp_idf_sys::esp_timer_get_time() } as u32;
        self.transition_to(DisplayState::Dizziness)?;
        Ok(())
    }

    /// 进入倾斜状态
    ///
    /// 当检测到设备倾斜时调用，显示倾斜状态界面。
    ///
    /// # 返回值
    /// * `Result<()>` - 状态切换结果
    ///
    /// # 特殊逻辑
    /// 如果当前已经在摇晃状态，优先保持摇晃状态（摇晃优先级更高）
    pub fn enter_tilting(&mut self) -> Result<()> {
        if self.state == DisplayState::Dizziness {
            return Ok(()); // 已经在晃动状态，直接返回
        }

        self.transition_to(DisplayState::Tilting)
    }

    /// 进入错误状态
    ///
    /// 当发生错误时调用，显示错误信息界面。
    ///
    /// # 参数
    /// * `error_msg` - 错误消息字符串，将显示在错误界面上
    ///
    /// # 返回值
    /// * `Result<()>` - 状态切换结果
    ///
    /// # 注意
    /// 错误状态会在3秒后自动返回欢迎界面（在update()方法中处理）
    pub fn enter_error(&mut self, error_msg: String) -> Result<()> {
        self.transition_to(DisplayState::Error(error_msg))
    }

    /// 检查是否可以退出摇晃状态
    ///
    /// 确保摇晃状态至少持续3秒，避免过于频繁的状态切换。
    ///
    /// # 返回值
    /// * `bool` - true表示可以退出，false表示需要继续保持摇晃状态
    ///
    /// # 检查逻辑
    /// 1. 确认当前确实在摇晃状态
    /// 2. 计算已经持续的时间
    /// 3. 判断是否达到最小持续时间（3秒）
    ///
    /// # 时间处理
    /// 使用wrapping_sub()处理时间戳溢出情况
    pub fn can_exit_dizziness(&self) -> bool {
        if self.state != DisplayState::Dizziness {
            return false;
        }

        // 晃动状态至少持续3秒（3,000,000微秒）
        const MIN_DIZZINESS_DURATION_US: u32 = 3_000_000;
        let current_time = unsafe { esp_idf_sys::esp_timer_get_time() } as u32;
        let elapsed_time = current_time.wrapping_sub(self.dizziness_start_time);
        let can_exit = elapsed_time >= MIN_DIZZINESS_DURATION_US;

        can_exit
    }

    /// 尝试退出摇晃状态
    ///
    /// 检查是否满足退出条件（最小持续时间），如果满足则返回主界面。
    ///
    /// # 返回值
    /// * `Result<()>` - 操作结果，状态切换失败时返回Err
    ///
    /// # 退出逻辑
    /// - 调用can_exit_dizziness()检查是否可以退出
    /// - 如果可以退出，切换到主界面
    /// - 如果不能退出，保持当前状态并记录日志
    ///
    /// # 注意
    /// 方法名中的拼写错误(diszziness)保持原样以避免破坏现有调用
    pub fn exit_diszziness(&mut self) -> Result<()> {
        if self.can_exit_dizziness() {
            log::info!("退出晃动状态");
            self.enter_main()?;
        } else {
            log::info!("无法退出晃动状态，持续时间不足");
        }

        Ok(())
    }

    /// 处理运动传感器事件
    ///
    /// 根据检测到的运动状态触发相应的显示状态切换。
    ///
    /// # 参数
    /// * `state` - 运动传感器检测到的运动状态
    ///
    /// # 返回值
    /// * `Result<()>` - 操作结果，状态切换失败时返回Err
    ///
    /// # 运动状态处理
    /// - Shaking: 进入摇晃状态，显示眩晕效果
    /// - Still: 设备静止，触发返回操作
    /// - Tilting: 进入倾斜状态，显示倾斜界面
    ///
    /// # 注意
    /// 这是传感器事件与UI状态之间的桥梁方法
    pub fn on_motion(&mut self, state: MotionState) -> Result<()> {
        match state {
            MotionState::Shaking => {
                self.enter_dizziness()?;
            }
            MotionState::Still => {
                self.back()?;
            }
            MotionState::Tilting => {
                self.enter_tilting()?;
            }
        }

        Ok(())
    }
}
