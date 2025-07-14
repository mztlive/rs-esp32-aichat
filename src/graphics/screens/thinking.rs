use crate::graphics::{
    colors::{BLACK, GREEN, WHITE},
    primitives::GraphicsPrimitives,
};

/// 更新思考状态
pub fn draw(graphics: &mut GraphicsPrimitives, state_timer: u32) -> anyhow::Result<()> {
    // 绘制思考界面
    graphics.draw_text("思考中...", 180, 150, WHITE, Some(BLACK))?;

    // 绘制简单的加载动画
    let dots = match (state_timer / 10) % 4 {
        0 => "   ",
        1 => ".  ",
        2 => ".. ",
        3 => "...",
        _ => "   ",
    };
    graphics.draw_text(dots, 180, 200, GREEN, Some(BLACK))?;

    Ok(())
}