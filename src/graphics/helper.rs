// 绘制辅助函数和宏

use embedded_graphics::pixelcolor::Rgb565;

/// 计算文本在指定区域内的居中位置
///
/// # 参数
///
/// * `text` - 要显示的文本
/// * `area_x` - 区域左上角X坐标
/// * `area_y` - 区域左上角Y坐标
/// * `area_width` - 区域宽度
/// * `area_height` - 区域高度
///
/// # 返回值
///
/// 返回文本左上角的坐标 (x, y)
pub fn center_text_in_area(
    text: &str,
    area_x: i32,
    area_y: i32,
    area_width: i32,
    area_height: i32,
) -> (i32, i32) {
    let text_width = text.len() as i32 * 10; // 每个字符10像素宽
    let text_height = 20; // 字体高度20像素

    let text_x = area_x + (area_width - text_width) / 2;
    let text_y = area_y + (area_height - text_height) / 2;

    (text_x, text_y)
}

/// 计算图像在指定区域内的居中位置
///
/// # 参数
///
/// * `image_width` - 图像宽度
/// * `image_height` - 图像高度
/// * `area_x` - 区域左上角X坐标
/// * `area_y` - 区域左上角Y坐标
/// * `area_width` - 区域宽度
/// * `area_height` - 区域高度
///
/// # 返回值
///
/// 返回图像左上角的坐标 (x, y)
pub fn center_image_in_area(
    image_width: i32,
    image_height: i32,
    area_x: i32,
    area_y: i32,
    area_width: i32,
    area_height: i32,
) -> (i32, i32) {
    let image_x = area_x + (area_width - image_width) / 2;
    let image_y = area_y + (area_height - image_height) / 2;

    (image_x, image_y)
}

/// 从RGB值创建Rgb565颜色
///
/// # 参数
///
/// * `r` - 红色分量 (0-255)
/// * `g` - 绿色分量 (0-255)
/// * `b` - 蓝色分量 (0-255)
///
/// # 返回值
///
/// 返回Rgb565颜色值
pub fn rgb_to_rgb565(r: u8, g: u8, b: u8) -> Rgb565 {
    let r5 = (r as u16 * 31 / 255) as u8;
    let g6 = (g as u16 * 63 / 255) as u8;
    let b5 = (b as u16 * 31 / 255) as u8;

    Rgb565::new(r5, g6, b5)
}

/// 从十六进制颜色值创建Rgb565颜色
///
/// # 参数
///
/// * `hex` - 十六进制颜色值 (例如: 0xFF0000 表示红色)
///
/// # 返回值
///
/// 返回Rgb565颜色值
pub fn hex_to_rgb565(hex: u32) -> Rgb565 {
    let r = ((hex >> 16) & 0xFF) as u8;
    let g = ((hex >> 8) & 0xFF) as u8;
    let b = (hex & 0xFF) as u8;

    rgb_to_rgb565(r, g, b)
}

/// 判断坐标是否在指定区域内
///
/// # 参数
///
/// * `x` - 要检查的X坐标
/// * `y` - 要检查的Y坐标
/// * `area_x` - 区域左上角X坐标
/// * `area_y` - 区域左上角Y坐标
/// * `area_width` - 区域宽度
/// * `area_height` - 区域高度
///
/// # 返回值
///
/// 如果坐标在区域内返回true，否则返回false
pub fn is_point_in_area(
    x: i32,
    y: i32,
    area_x: i32,
    area_y: i32,
    area_width: i32,
    area_height: i32,
) -> bool {
    x >= area_x && x < area_x + area_width && y >= area_y && y < area_y + area_height
}

/// 计算两点之间的距离
///
/// # 参数
///
/// * `x1` - 第一个点的X坐标
/// * `y1` - 第一个点的Y坐标
/// * `x2` - 第二个点的X坐标
/// * `y2` - 第二个点的Y坐标
///
/// # 返回值
///
/// 返回两点之间的距离
pub fn distance(x1: i32, y1: i32, x2: i32, y2: i32) -> f32 {
    let dx = (x2 - x1) as f32;
    let dy = (y2 - y1) as f32;
    (dx * dx + dy * dy).sqrt()
}

/// 判断坐标是否在圆形区域内
///
/// # 参数
///
/// * `x` - 要检查的X坐标
/// * `y` - 要检查的Y坐标
/// * `center_x` - 圆心X坐标
/// * `center_y` - 圆心Y坐标
/// * `radius` - 圆形半径
///
/// # 返回值
///
/// 如果坐标在圆形区域内返回true，否则返回false
pub fn is_point_in_circle(x: i32, y: i32, center_x: i32, center_y: i32, radius: i32) -> bool {
    distance(x, y, center_x, center_y) <= radius as f32
}

/// 限制值在指定范围内
///
/// # 参数
///
/// * `value` - 要限制的值
/// * `min` - 最小值
/// * `max` - 最大值
///
/// # 返回值
///
/// 返回限制后的值
pub fn clamp(value: i32, min: i32, max: i32) -> i32 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

/// 线性插值
///
/// # 参数
///
/// * `a` - 起始值
/// * `b` - 结束值
/// * `t` - 插值因子 (0.0 到 1.0)
///
/// # 返回值
///
/// 返回插值结果
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// 将值从一个范围映射到另一个范围
///
/// # 参数
///
/// * `value` - 要映射的值
/// * `from_min` - 原始范围的最小值
/// * `from_max` - 原始范围的最大值
/// * `to_min` - 目标范围的最小值
/// * `to_max` - 目标范围的最大值
///
/// # 返回值
///
/// 返回映射后的值
pub fn map_range(value: f32, from_min: f32, from_max: f32, to_min: f32, to_max: f32) -> f32 {
    let t = (value - from_min) / (from_max - from_min);
    lerp(to_min, to_max, t)
}

/// 绘制坐标系宏
///
/// 用于快速绘制坐标系用于调试
#[macro_export]
macro_rules! draw_debug_grid {
    ($graphics:expr, $color:expr) => {
        use crate::graphics::layout::GRID_SIZE;

        // 绘制垂直线
        for i in 0..4 {
            let x = i * GRID_SIZE;
            let rect = crate::graphics::layout::ScreenRect::new(x, 0, 1, 360);
            $graphics.fill_rect(&rect, $color)?;
        }

        // 绘制水平线
        for i in 0..4 {
            let y = i * GRID_SIZE;
            let rect = crate::graphics::layout::ScreenRect::new(0, y, 360, 1);
            $graphics.fill_rect(&rect, $color)?;
        }
    };
}

/// 快速绘制九宫格序号宏
///
/// 用于调试九宫格布局
#[macro_export]
macro_rules! draw_grid_numbers {
    ($graphics:expr, $color:expr) => {
        use crate::graphics::layout::GridPosition;

        $graphics.draw_text_at_grid(GridPosition::TopLeft, "1", $color)?;
        $graphics.draw_text_at_grid(GridPosition::TopCenter, "2", $color)?;
        $graphics.draw_text_at_grid(GridPosition::TopRight, "3", $color)?;
        $graphics.draw_text_at_grid(GridPosition::MiddleLeft, "4", $color)?;
        $graphics.draw_text_at_grid(GridPosition::MiddleCenter, "5", $color)?;
        $graphics.draw_text_at_grid(GridPosition::MiddleRight, "6", $color)?;
        $graphics.draw_text_at_grid(GridPosition::BottomLeft, "7", $color)?;
        $graphics.draw_text_at_grid(GridPosition::BottomCenter, "8", $color)?;
        $graphics.draw_text_at_grid(GridPosition::BottomRight, "9", $color)?;
    };
}
