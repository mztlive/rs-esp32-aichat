use crate::graphics::{
    colors::{BLACK, BLUE, GREEN, RED, WHITE},
    primitives::GraphicsPrimitives,
};

/// 更新晃动状态
pub fn draw(graphics: &mut GraphicsPrimitives, state_timer: u32) -> anyhow::Result<()> {
    // Draw dizziness screen
    graphics.draw_text("Ah! So dizzy!", 180, 120, RED, Some(BLACK))?;

    // Draw shaking effect text
    let shake_text = match (state_timer / 5) % 3 {
        0 => "Shaking...",
        1 => "Spinning...",
        2 => "Feeling dizzy...",
        _ => "Shaking...",
    };
    graphics.draw_text(shake_text, 180, 160, WHITE, Some(BLACK))?;

    // Draw prompt message
    graphics.draw_text("Please stop shaking", 180, 200, BLUE, Some(BLACK))?;

    // Draw return hint
    graphics.draw_text("Will return when stable", 180, 240, GREEN, Some(BLACK))?;

    Ok(())
}