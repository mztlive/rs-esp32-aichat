use anyhow::Result;
use embedded_graphics::pixelcolor::Rgb565;
use crate::graphics::primitives::GraphicsPrimitives;

/// UI组件通用trait
///
/// 所有UI组件都应该实现这个trait，以提供统一的绘制接口
pub trait UIComponent {
    /// 渲染组件到graphics primitives
    ///
    /// # 参数
    ///
    /// * `graphics` - 图形绘制器引用
    ///
    /// # 返回值
    ///
    /// * `Ok(())` - 绘制成功
    /// * `Err(anyhow::Error)` - 绘制失败
    fn render(&self, graphics: &mut GraphicsPrimitives) -> Result<()>;
    
    /// 获取组件的边界框
    ///
    /// # 返回值
    ///
    /// 返回(x, y, width, height)
    fn get_bounds(&self) -> (i32, i32, i32, i32);
    
    /// 检查组件是否需要重绘
    ///
    /// # 返回值
    ///
    /// * `true` - 需要重绘
    /// * `false` - 不需要重绘
    fn needs_redraw(&self) -> bool {
        true // 默认总是需要重绘
    }
}

/// 带缓存的UI组件trait
///
/// 为了优化性能，组件可以实现这个trait来支持缓存绘制结果
pub trait CachedUIComponent: UIComponent {
    /// 清除缓存
    fn clear_cache(&mut self);
    
    /// 标记为需要重绘
    fn mark_dirty(&mut self);
    
    /// 检查是否为脏状态（需要重绘）
    fn is_dirty(&self) -> bool;
}

/// 绘制指令枚举
///
/// 用于描述具体的绘制操作，避免多次调用primitives方法
#[derive(Debug, Clone)]
pub enum DrawCommand {
    /// 填充矩形
    FillRect {
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        color: Rgb565,
    },
    /// 绘制文本
    DrawText {
        text: String,
        x: i32,
        y: i32,
        color: Rgb565,
    },
    /// 绘制填充圆形
    FillCircle {
        center_x: i32,
        center_y: i32,
        radius: i32,
        color: Rgb565,
    },
    /// 绘制圆形边框
    DrawCircleBorder {
        center_x: i32,
        center_y: i32,
        radius: i32,
        color: Rgb565,
        thickness: u32,
    },
    /// 绘制矩形边框
    DrawRectBorder {
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        color: Rgb565,
        thickness: u32,
    },
}

/// 批量绘制的UI组件trait
///
/// 为了性能优化，组件可以实现这个trait来生成批量绘制指令
pub trait BatchDrawableUIComponent: UIComponent {
    /// 生成绘制指令列表
    ///
    /// # 返回值
    ///
    /// 返回绘制指令的向量
    fn generate_draw_commands(&self) -> Vec<DrawCommand>;
}