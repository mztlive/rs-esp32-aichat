use crate::graphics::{
    colors::{BLACK, BLUE, GREEN, WHITE},
    layout::ScreenRect,
    primitives::GraphicsPrimitives,
};

/// 更新主界面
pub fn draw(graphics: &mut GraphicsPrimitives) -> anyhow::Result<()> {
    // 绘制主界面
    graphics.draw_text("聊天界面", 180, 50, WHITE, Some(BLACK))?;

    // 绘制消息列表区域边框
    let message_area = ScreenRect::new(20, 80, 320, 200);
    graphics.draw_rect_border(&message_area, WHITE, 2)?;
    graphics.draw_text("消息区域", 180, 120, WHITE, Some(BLACK))?;

    // 绘制输入区域边框
    let input_area = ScreenRect::new(20, 290, 320, 40);
    graphics.draw_rect_border(&input_area, BLUE, 2)?;
    graphics.draw_text("输入区域", 180, 310, WHITE, Some(BLACK))?;

    // 绘制操作提示
    graphics.draw_text("按 S 键进入设置", 180, 340, GREEN, Some(BLACK))?;

    Ok(())
}
