use crate::graphics::{
    colors::{BLACK, GREEN, WHITE},
    primitives::GraphicsPrimitives,
};

/// 更新设置界面
pub fn draw(graphics: &mut GraphicsPrimitives) -> anyhow::Result<()> {
    // 绘制设置界面
    graphics.draw_text("设置", 180, 50, WHITE, Some(BLACK))?;

    // 设置选项
    graphics.draw_text("● 主题设置", 80, 120, WHITE, Some(BLACK))?;
    graphics.draw_text("● 网络设置", 80, 160, WHITE, Some(BLACK))?;
    graphics.draw_text("● 语言设置", 80, 200, WHITE, Some(BLACK))?;
    graphics.draw_text("● 关于", 80, 240, WHITE, Some(BLACK))?;

    // 操作提示
    graphics.draw_text("按 B 键返回", 180, 320, GREEN, Some(BLACK))?;

    Ok(())
}
