use super::traits::UIComponent;
use crate::graphics::layout::{SCREEN_WIDTH, STATUS_BAR, TEXT_CHAR_WIDTH, TEXT_LINE_HEIGHT};
use crate::graphics::primitives::GraphicsPrimitives;
use anyhow::Result;
use embedded_graphics::pixelcolor::Rgb565;

/// 状态栏位置枚举
#[derive(Debug, Clone, Copy)]
pub enum StatusBarPosition {
    Left,
    Center,
    Right,
}

/// 状态栏文本项
#[derive(Debug, Clone)]
pub struct StatusBarText {
    pub text: String,
    pub position: StatusBarPosition,
    pub color: Rgb565,
}

/// 状态栏组件
///
/// 提供屏幕顶部状态栏的功能，支持左、中、右三个位置的文本显示。
/// 自动计算文本位置，不直接绘制，需要传递给primitives进行绘制。
#[derive(Debug, Clone)]
pub struct StatusBar {
    /// 背景色
    pub background_color: Rgb565,
    /// 文本项列表
    pub text_items: Vec<StatusBarText>,
    /// 状态栏高度
    pub height: i32,
}

impl StatusBar {
    /// 创建新的状态栏实例
    ///
    /// # 参数
    ///
    /// * `background_color` - 状态栏背景色
    ///
    /// # 返回值
    ///
    /// 返回StatusBar实例
    pub fn new(background_color: Rgb565) -> Self {
        Self {
            background_color,
            text_items: Vec::new(),
            height: STATUS_BAR.height,
        }
    }

    /// 添加文本项
    ///
    /// # 参数
    ///
    /// * `text` - 文本内容
    /// * `position` - 文本位置（左、中、右）
    /// * `color` - 文本颜色
    pub fn add_text(
        &mut self,
        text: impl Into<String>,
        position: StatusBarPosition,
        color: Rgb565,
    ) {
        self.text_items.push(StatusBarText {
            text: text.into(),
            position,
            color,
        });
    }

    /// 清除所有文本项
    pub fn clear_text(&mut self) {
        self.text_items.clear();
    }

    /// 设置背景色
    pub fn set_background_color(&mut self, color: Rgb565) {
        self.background_color = color;
    }

    /// 计算文本的绘制位置
    ///
    /// # 参数
    ///
    /// * `text` - 文本内容
    /// * `position` - 文本位置
    ///
    /// # 返回值
    ///
    /// 返回文本绘制的(x, y)坐标
    pub fn calculate_text_position(&self, text: &str, position: StatusBarPosition) -> (i32, i32) {
        let text_width = text.len() as i32 * TEXT_CHAR_WIDTH;

        // 垂直居中：状态栏顶部 + 文本基线偏移
        // embedded-graphics的文本绘制是基于基线的，FONT_10X20的字体高度是20，基线大约在距离顶部16的位置
        let y = STATUS_BAR.y + (self.height + 16) / 2; // 基线位置，让文字在状态栏中垂直居中

        // 水平位置计算
        let x = match position {
            StatusBarPosition::Left => STATUS_BAR.x + 10, // 左边距10px
            StatusBarPosition::Center => STATUS_BAR.x + (SCREEN_WIDTH - text_width) / 2,
            StatusBarPosition::Right => STATUS_BAR.x + SCREEN_WIDTH - text_width - 10, // 右边距10px
        };

        (x, y)
    }

    /// 获取状态栏区域信息
    pub fn get_rect(&self) -> (i32, i32, i32, i32) {
        (STATUS_BAR.x, STATUS_BAR.y, STATUS_BAR.width, self.height)
    }

    /// 获取背景色
    pub fn get_background_color(&self) -> Rgb565 {
        self.background_color
    }

    /// 获取文本项列表
    pub fn get_text_items(&self) -> &Vec<StatusBarText> {
        &self.text_items
    }
}

impl Default for StatusBar {
    fn default() -> Self {
        use crate::graphics::colors::WHITE;
        Self::new(WHITE)
    }
}

impl UIComponent for StatusBar {
    fn render(&self, graphics: &mut GraphicsPrimitives) -> Result<()> {
        // 绘制背景
        let rect = crate::graphics::layout::ScreenRect::new(
            STATUS_BAR.x,
            STATUS_BAR.y,
            STATUS_BAR.width,
            self.height,
        );
        graphics.fill_rect(&rect, self.background_color)?;

        // 绘制所有文本项
        for item in &self.text_items {
            let (x, y) = self.calculate_text_position(&item.text, item.position);
            graphics.draw_text(&item.text, x, y, item.color)?;
        }

        Ok(())
    }

    fn get_bounds(&self) -> (i32, i32, i32, i32) {
        (STATUS_BAR.x, STATUS_BAR.y, STATUS_BAR.width, self.height)
    }
}
