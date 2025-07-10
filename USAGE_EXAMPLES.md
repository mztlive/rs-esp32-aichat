# ESP32 360x360 屏幕快速绘制使用示例

本文档介绍了为 1.85 寸 360x360 屏幕定制的快速绘制常量和方法的使用方法。

## 布局常量

### 屏幕尺寸和中心点

```rust
use crate::graphics::layout::*;

// 屏幕尺寸
SCREEN_WIDTH   // 360
SCREEN_HEIGHT  // 360

// 屏幕中心点
SCREEN_CENTER_X // 180
SCREEN_CENTER_Y // 180
```

### 九宫格布局

屏幕被划分为 3x3 的九宫格，每个格子 120x120 像素：

```rust
// 九宫格位置枚举
GridPosition::TopLeft
GridPosition::TopCenter
GridPosition::TopRight
GridPosition::MiddleLeft
GridPosition::MiddleCenter
GridPosition::MiddleRight
GridPosition::BottomLeft
GridPosition::BottomCenter
GridPosition::BottomRight
```

### 预定义区域

```rust
// 内容区域（带边距）
CONTENT_AREA_START_X  // 20
CONTENT_AREA_START_Y  // 20
CONTENT_AREA_END_X    // 340
CONTENT_AREA_END_Y    // 340

// 状态栏和操作栏
STATUS_BAR    // 顶部30像素高度
ACTION_BAR    // 底部30像素高度
```

## 快速绘制方法

### 在九宫格位置绘制

#### 绘制圆形

```rust
// 在九宫格中心绘制红色圆形
primitives.draw_circle_at_grid(GridPosition::MiddleCenter, 30, RED)?;

// 在九宫格各个位置绘制不同颜色的圆形
primitives.draw_circle_at_grid(GridPosition::TopLeft, 25, BLUE)?;
primitives.draw_circle_at_grid(GridPosition::TopRight, 20, GREEN)?;
```

#### 绘制文本

```rust
// 在九宫格位置绘制居中文本
primitives.draw_text_at_grid(GridPosition::MiddleLeft, "ESP32", RED)?;
primitives.draw_text_at_grid(GridPosition::MiddleRight, "Rust", BLUE)?;
```

#### 绘制图像

```rust
// 在九宫格位置绘制居中图像
let bmp = Bmp::from_slice(image_data).unwrap();
primitives.draw_image_at_grid(GridPosition::BottomCenter, &bmp)?;
```

### 在屏幕中心绘制

```rust
// 在屏幕中心绘制圆形
primitives.draw_circle_at_center(50, GREEN)?;

// 在屏幕中心绘制文本
primitives.draw_text_at_center("360x360", YELLOW)?;
```

### 矩形区域操作

```rust
// 填充矩形区域
let rect = ScreenRect::new(100, 100, 160, 160);
primitives.fill_rect(&rect, BLUE)?;

// 绘制矩形边框
primitives.draw_rect_border(&rect, RED, 2)?;

// 使用预定义区域
primitives.fill_rect(&STATUS_BAR, BLUE)?;
```

### 圆形边框

```rust
// 绘制空心圆
primitives.draw_circle_border(180, 180, 80, RED, 3)?;
```

### 多行文本

```rust
// 绘制多行文本
let lines = vec!["Line 1", "Line 2", "Line 3"];
primitives.draw_multiline_text(&lines, 20, 50, BLACK)?;
```

### 清除区域

```rust
// 清除九宫格指定区域
primitives.clear_grid_area(GridPosition::TopLeft, WHITE)?;
```

## 辅助函数

### 颜色转换

```rust
use crate::graphics::helper::*;

// 从RGB值创建颜色
let custom_color = rgb_to_rgb565(255, 128, 0); // 橙色

// 从十六进制创建颜色
let hex_color = hex_to_rgb565(0xFF8000); // 橙色
```

### 坐标和距离计算

```rust
// 检查点是否在矩形区域内
let in_area = is_point_in_area(100, 100, 50, 50, 100, 100);

// 检查点是否在圆形区域内
let in_circle = is_point_in_circle(100, 100, 50, 50, 30);

// 计算两点距离
let dist = distance(0, 0, 100, 100);
```

### 调试宏

```rust
// 绘制九宫格网格线（调试用）
draw_debug_grid!(primitives, GRAY);

// 绘制九宫格序号（调试用）
draw_grid_numbers!(primitives, BLACK);
```

## 完整示例

```rust
use anyhow::Result;
use esp_idf_hal::{delay::FreeRtos, peripherals::Peripherals};
use crate::graphics::{
    colors::{RED, WHITE, BLUE, GREEN, YELLOW},
    layout::GridPosition,
    primitives::GraphicsPrimitives,
};
use crate::lcd::LcdController;

fn demo_screen_layout() -> Result<()> {
    let p = Peripherals::take().unwrap();
    let mut lcd = LcdController::new(p)?;
    let mut primitives = GraphicsPrimitives::new(&mut lcd);

    // 清空屏幕
    primitives.fill_screen(WHITE)?;

    // 在九宫格各位置绘制圆形
    primitives.draw_circle_at_grid(GridPosition::TopLeft, 30, RED)?;
    primitives.draw_circle_at_grid(GridPosition::TopCenter, 25, BLUE)?;
    primitives.draw_circle_at_grid(GridPosition::TopRight, 20, GREEN)?;

    // 在中间位置绘制文本
    primitives.draw_text_at_grid(GridPosition::MiddleLeft, "ESP32", RED)?;
    primitives.draw_text_at_center("360x360", YELLOW)?;
    primitives.draw_text_at_grid(GridPosition::MiddleRight, "Rust", BLUE)?;

    // 绘制装饰边框
    primitives.draw_circle_border(180, 180, 100, RED, 2)?;

    Ok(())
}
```

## 常用尺寸参考

- **字符尺寸**: 10x20 像素（使用 FONT_10X20 字体）
- **九宫格尺寸**: 120x120 像素
- **常用圆形半径**:
  - 小: 20 像素
  - 中: 40 像素
  - 大: 60 像素
  - 特大: 80 像素
- **常用边距**:
  - 小: 10 像素
  - 中: 20 像素
  - 大: 30 像素

这些常量和方法让您可以快速在 360x360 屏幕上进行精确定位和绘制，无需手动计算坐标。
