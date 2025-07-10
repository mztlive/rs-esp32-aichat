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

    let bmps = DONGHUA_IMAGES
        .iter()
        .map(|&img| {
            // 解析 BMP 文件
            Bmp::<Rgb565>::from_slice(img).unwrap()
        })
        .collect::<Vec<_>>();

    loop {
        for bmp in &bmps {
            primitives
                .draw_image_at_grid(graphics::layout::GridPosition::MiddleCenter, bmp)
                .unwrap();

            // 添加延迟让动画更流畅，每帧间隔80ms
            FreeRtos::delay_ms(80);
        }
    }
}
