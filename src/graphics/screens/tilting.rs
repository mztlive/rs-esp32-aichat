use crate::graphics::{
    colors::{BLACK, WHITE, YELLOW},
    primitives::GraphicsPrimitives,
};

/// 更新倾斜状态
pub fn draw(graphics: &mut GraphicsPrimitives) -> anyhow::Result<()> {
    // 绘制倾斜状态
    graphics.draw_text("Device Is Tilting", 180, 150, YELLOW, Some(BLACK))?;
    graphics.draw_text("Please Keep The Device Level", 180, 200, WHITE, Some(BLACK))?;

    Ok(())
}