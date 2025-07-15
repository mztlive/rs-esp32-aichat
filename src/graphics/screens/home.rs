use crate::graphics::{
    colors::{BLACK, BLUE, GREEN, WHITE},
    layout::ScreenRect,
    primitives::GraphicsPrimitives,
};

/// 更新主界面
pub fn draw(graphics: &mut GraphicsPrimitives) -> anyhow::Result<()> {
    graphics.fill_screen(WHITE)?;

    Ok(())
}
