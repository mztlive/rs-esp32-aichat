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
                if self.can_exit_dizziness() {
                    self.exit_diszziness()?;
                }
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

        println!("状态转换: {:?} -> {:?}", self.state, new_state);
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

    /// 进入晃动状态
    pub fn enter_dizziness(&mut self) -> Result<()> {
        if self.state == DisplayState::Dizziness {
            return Ok(()); // 已经在晃动状态，直接返回
        }

        // 记录进入晃动状态的全局时间，而不是相对于状态转换的时间
        self.dizziness_start_time = self.state_timer;
        self.transition_to(DisplayState::Dizziness)?;
        // 重新设置开始时间，因为transition_to会重置state_timer
        self.dizziness_start_time = 0;
        Ok(())
    }

    pub fn enter_tilting(&mut self) -> Result<()> {
        if self.state == DisplayState::Dizziness {
            return Ok(()); // 已经在晃动状态，直接返回
        }

        self.transition_to(DisplayState::Tilting)
    }

    pub fn enter_error(&mut self, error_msg: String) -> Result<()> {
        self.transition_to(DisplayState::Error(error_msg))
    }

    /// 检查是否可以退出晃动状态
    pub fn can_exit_dizziness(&self) -> bool {
        if self.state != DisplayState::Dizziness {
            return false;
        }

        // 晃动状态至少持续3秒（20fps * 60 = 1200帧 = 1分钟，所以60帧 = 3秒）
        const MIN_DIZZINESS_DURATION: u32 = 60;
        // 由于transition_to会重置state_timer，我们直接使用state_timer作为持续时间
        let can_exit = self.state_timer >= MIN_DIZZINESS_DURATION;

        can_exit
    }

    pub fn exit_diszziness(&mut self) -> Result<()> {
        if self.can_exit_dizziness() {
            log::info!("退出晃动状态");
            self.enter_main()?;
        } else {
            log::info!("无法退出晃动状态，持续时间不足");
        }

        Ok(())
    }

    pub fn on_motion(&mut self, state: MotionState) -> Result<()> {
        match state {
            MotionState::Shaking => {
                self.enter_dizziness()?;
            }
            MotionState::Still => {
                self.back()?;
            }
            MotionState::Tilting => self.enter_tilting()?,
        }

        Ok(())
    }
}
