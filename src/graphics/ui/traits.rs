use crate::graphics::primitives::GraphicsPrimitives;
use anyhow::Result;

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
