use anyhow::Result;

use crate::graphics::{
    colors::{BLACK, BLUE, GREEN, RED, WHITE, YELLOW},
    layout::ScreenRect,
    primitives::GraphicsPrimitives,
};

/// 应用状态枚举
#[derive(Debug, Clone, PartialEq)]
pub enum AppState {
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

/// 用户输入事件
#[derive(Debug, Clone)]
pub enum UserInput {
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

/// 主应用结构
pub struct ChatApp<'a> {
    /// 当前状态
    state: AppState,
    /// 图形绘制接口
    graphics: GraphicsPrimitives<'a>,
    /// 状态切换计时器（用于自动切换）
    state_timer: u32,
    /// 晃动状态开始时间
    dizziness_start_time: u32,
}

impl<'a> ChatApp<'a> {
    /// 创建新的应用实例
    pub fn new(graphics: GraphicsPrimitives<'a>) -> Self {
        ChatApp {
            state: AppState::Welcome,
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
            AppState::Welcome => self.update_welcome()?,
            AppState::Main => self.update_main()?,
            AppState::Settings => self.update_settings()?,
            AppState::Error(msg) => self.update_error(msg.clone())?,
            AppState::Thinking => self.update_thinking()?,
            AppState::Dizziness => self.update_dizziness()?,
            AppState::Tilting => self.update_tilting()?,
        }

        Ok(())
    }

    /// 处理用户输入
    pub fn back(&mut self) -> Result<()> {
        println!("处理返回操作");

        match &self.state {
            // 欢迎界面：任意按键进入主界面
            AppState::Welcome => {
                self.enter_main()?;
            }

            // 晃动状态：返回键回到主界面
            AppState::Dizziness => {
                if self.can_exit_dizziness() {
                    self.exit_diszziness()?;
                }
            }

            // 设备倾斜
            AppState::Tilting => {
                self.enter_main()?;
            }

            // 其他输入忽略
            _ => {}
        }

        Ok(())
    }

    /// 状态转换
    fn transition_to(&mut self, new_state: AppState) -> Result<()> {
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

    /// 更新欢迎界面
    fn update_welcome(&mut self) -> Result<()> {
        // 绘制欢迎界面 - 垂直居中显示
        let center_y = 180; // 屏幕中心Y坐标

        self.graphics
            .draw_text("AI Chat", 180, center_y - 40, WHITE, Some(BLACK))?;
        self.graphics
            .draw_text("ESP32-S3", 180, center_y, GREEN, Some(BLACK))?;
        self.graphics
            .draw_text("Click Any Key", 180, center_y + 40, BLUE, Some(BLACK))?;

        Ok(())
    }

    /// 更新主界面
    fn update_main(&mut self) -> Result<()> {
        // 绘制主界面
        self.graphics
            .draw_text("聊天界面", 180, 50, WHITE, Some(BLACK))?;

        // 绘制消息列表区域边框
        let message_area = ScreenRect::new(20, 80, 320, 200);
        self.graphics.draw_rect_border(&message_area, WHITE, 2)?;
        self.graphics
            .draw_text("消息区域", 180, 120, WHITE, Some(BLACK))?;

        // 绘制输入区域边框
        let input_area = ScreenRect::new(20, 290, 320, 40);
        self.graphics.draw_rect_border(&input_area, BLUE, 2)?;
        self.graphics
            .draw_text("输入区域", 180, 310, WHITE, Some(BLACK))?;

        // 绘制操作提示
        self.graphics
            .draw_text("按 S 键进入设置", 180, 340, GREEN, Some(BLACK))?;

        Ok(())
    }

    /// 更新设置界面
    fn update_settings(&mut self) -> Result<()> {
        // 绘制设置界面
        self.graphics
            .draw_text("设置", 180, 50, WHITE, Some(BLACK))?;

        // 设置选项
        self.graphics
            .draw_text("● 主题设置", 80, 120, WHITE, Some(BLACK))?;
        self.graphics
            .draw_text("● 网络设置", 80, 160, WHITE, Some(BLACK))?;
        self.graphics
            .draw_text("● 语言设置", 80, 200, WHITE, Some(BLACK))?;
        self.graphics
            .draw_text("● 关于", 80, 240, WHITE, Some(BLACK))?;

        // 操作提示
        self.graphics
            .draw_text("按 B 键返回", 180, 320, GREEN, Some(BLACK))?;

        Ok(())
    }

    /// 更新错误界面
    fn update_error(&mut self, error_msg: String) -> Result<()> {
        // 绘制错误界面
        self.graphics
            .draw_text("错误", 180, 100, RED, Some(BLACK))?;
        self.graphics
            .draw_text(&error_msg, 180, 140, WHITE, Some(BLACK))?;
        self.graphics
            .draw_text("按任意键继续", 180, 220, BLUE, Some(BLACK))?;

        // 3秒后自动返回欢迎界面
        if self.state_timer > 150 {
            self.enter_welcome()?;
        }

        Ok(())
    }

    /// 获取当前状态
    pub fn get_state(&self) -> &AppState {
        &self.state
    }

    /// 统一的状态转换方法
    pub fn enter_welcome(&mut self) -> Result<()> {
        self.transition_to(AppState::Welcome)
    }

    pub fn enter_main(&mut self) -> Result<()> {
        self.transition_to(AppState::Main)
    }

    pub fn enter_settings(&mut self) -> Result<()> {
        self.transition_to(AppState::Settings)
    }

    pub fn enter_thinking(&mut self) -> Result<()> {
        self.transition_to(AppState::Thinking)
    }

    /// 进入晃动状态
    pub fn enter_dizziness(&mut self) -> Result<()> {
        if self.state == AppState::Dizziness {
            return Ok(()); // 已经在晃动状态，直接返回
        }

        // 记录进入晃动状态的全局时间，而不是相对于状态转换的时间
        self.dizziness_start_time = self.state_timer;
        println!("进入晃动状态，记录开始时间: {}", self.dizziness_start_time);
        self.transition_to(AppState::Dizziness)?;
        // 重新设置开始时间，因为transition_to会重置state_timer
        self.dizziness_start_time = 0;
        Ok(())
    }

    pub fn enter_tilting(&mut self) -> Result<()> {
        if self.state == AppState::Dizziness {
            return Ok(()); // 已经在晃动状态，直接返回
        }

        self.transition_to(AppState::Tilting)
    }

    pub fn enter_error(&mut self, error_msg: String) -> Result<()> {
        self.transition_to(AppState::Error(error_msg))
    }

    /// 检查是否可以退出晃动状态
    pub fn can_exit_dizziness(&self) -> bool {
        if self.state != AppState::Dizziness {
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

    /// 更新思考状态
    fn update_thinking(&mut self) -> Result<()> {
        // 绘制思考界面
        self.graphics
            .draw_text("思考中...", 180, 150, WHITE, Some(BLACK))?;

        // 绘制简单的加载动画
        let dots = match (self.state_timer / 10) % 4 {
            0 => "   ",
            1 => ".  ",
            2 => ".. ",
            3 => "...",
            _ => "   ",
        };
        self.graphics
            .draw_text(dots, 180, 200, GREEN, Some(BLACK))?;

        Ok(())
    }

    /// 更新晃动状态
    fn update_dizziness(&mut self) -> Result<()> {
        // Draw dizziness screen
        self.graphics
            .draw_text("Ah! So dizzy!", 180, 120, RED, Some(BLACK))?;

        // Draw shaking effect text
        let shake_text = match (self.state_timer / 5) % 3 {
            0 => "Shaking...",
            1 => "Spinning...",
            2 => "Feeling dizzy...",
            _ => "Shaking...",
        };
        self.graphics
            .draw_text(shake_text, 180, 160, WHITE, Some(BLACK))?;

        // Draw prompt message
        self.graphics
            .draw_text("Please stop shaking", 180, 200, BLUE, Some(BLACK))?;

        // Draw return hint
        self.graphics
            .draw_text("Will return when stable", 180, 240, GREEN, Some(BLACK))?;

        Ok(())
    }

    /// 更新倾斜状态
    fn update_tilting(&mut self) -> Result<()> {
        // 绘制倾斜状态
        self.graphics
            .draw_text("Device Is Tilting", 180, 150, YELLOW, Some(BLACK))?;
        self.graphics
            .draw_text("Please Keep The Device Level", 180, 200, WHITE, Some(BLACK))?;

        Ok(())
    }
}
