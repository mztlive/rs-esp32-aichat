// src/main.rs
use anyhow::Result;
use embedded_graphics::pixelcolor::Rgb565;
use esp_idf_hal::{delay::FreeRtos, peripherals::Peripherals};

mod graphics;
mod lcd;
mod lcd_cmds;
use lcd::LcdController;
use tinybmp::Bmp;

use crate::graphics::{
    animation::FrameAnimation,
    colors::{BLUE, WHITE},
    primitives::GraphicsPrimitives,
};

// 定义donghua动画图片数据数组
const DONGHUA_IMAGES: [&[u8]; 12] = [
    include_bytes!("../assets/donghua/1.bmp"),
    include_bytes!("../assets/donghua/2.bmp"),
    include_bytes!("../assets/donghua/3.bmp"),
    include_bytes!("../assets/donghua/4.bmp"),
    include_bytes!("../assets/donghua/5.bmp"),
    include_bytes!("../assets/donghua/6.bmp"),
    include_bytes!("../assets/donghua/7.bmp"),
    include_bytes!("../assets/donghua/8.bmp"),
    include_bytes!("../assets/donghua/9.bmp"),
    include_bytes!("../assets/donghua/10.bmp"),
    include_bytes!("../assets/donghua/11.bmp"),
    include_bytes!("../assets/donghua/12.bmp"),
];

fn main() -> Result<()> {
    // 必须先调用，打补丁
    esp_idf_sys::link_patches();

    println!("=== ESP32 LCD 诊断测试程序 ===");

    // 取得外设
    let p = Peripherals::take().unwrap();

    // 初始化 LCD 控制器
    println!("正在初始化LCD控制器...");
    let mut lcd = LcdController::new(p)?;

    // lcd.fill_screen(COLOR_BLACK)?;

    // let bmp_data = include_bytes!("../assets/xk.bmp");
    // // Parse the BMP file.
    // let bmp = Bmp::from_slice(bmp_data).unwrap();

    let mut primitives = GraphicsPrimitives::new(&mut lcd);

    primitives.fill_screen(WHITE)?;
    draw_debug_grid!(primitives, BLUE);

    // println!("StatusBar已绘制完成！");

    // 创建动画系统，设置帧率为12.5fps (80ms每帧)
    let mut animation = FrameAnimation::with_fps(12);

    // 添加所有动画帧
    for &frame_data in &DONGHUA_IMAGES {
        animation.add_frame(frame_data);
    }

    // 主循环
    loop {
        // 更新动画状态，使用硬件定时器自动计算时间差
        if animation.update() {
            // 帧发生变化，获取当前帧数据并绘制
            if let Some(frame_data) = animation.get_current_frame() {
                // 解析 BMP 文件
                let bmp = Bmp::<Rgb565>::from_slice(frame_data).unwrap();

                // 绘制当前帧
                primitives
                    .draw_image_at_grid(graphics::layout::GridPosition::MiddleCenter, &bmp)
                    .unwrap();
            }
        }

        // 使用较短的延迟，让动画更平滑
        FreeRtos::delay_ms(1);
    }
}
