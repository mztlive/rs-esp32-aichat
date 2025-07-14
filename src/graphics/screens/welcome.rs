use crate::graphics::{
    colors::{BLACK, BLUE, GREEN, WHITE},
    primitives::GraphicsPrimitives,
};

/// 更新欢迎界面
pub fn draw(graphics: &mut GraphicsPrimitives) -> anyhow::Result<()> {
    // 绘制欢迎界面 - 垂直居中显示
    let center_y = 180; // 屏幕中心Y坐标

    graphics.draw_text("AI Chat", 180, center_y - 40, WHITE, Some(BLACK))?;
    graphics.draw_text("ESP32-S3", 180, center_y, GREEN, Some(BLACK))?;
    graphics.draw_text("Click Any Key", 180, center_y + 40, BLUE, Some(BLACK))?;

    Ok(())
}
