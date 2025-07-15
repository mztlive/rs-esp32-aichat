use crate::graphics::{
    colors::{BLACK, BLUE, RED, WHITE},
    primitives::GraphicsPrimitives,
};

/// 更新错误界面
pub fn draw(graphics: &mut GraphicsPrimitives, error_msg: &str) -> anyhow::Result<()> {
    // 绘制错误界面
    graphics.draw_text("错误", 180, 100, RED, Some(BLACK))?;
    graphics.draw_text(error_msg, 180, 140, WHITE, Some(BLACK))?;
    graphics.draw_text("按任意键继续", 180, 220, BLUE, Some(BLACK))?;

    Ok(())
}
