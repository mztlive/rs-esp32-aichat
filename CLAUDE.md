# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 项目简介

这是一个基于ESP32-S3的Rust项目，用于控制360x360像素的LCD显示屏（ST77916驱动）。项目实现了完整的图形绘制系统，支持图像显示、动画播放、文本渲染等功能。

## 常用命令

### 构建项目
```bash
# 使用脚本构建（推荐）
./scripts/build.sh [debug|release]  # 默认为release模式

# 直接使用cargo
cargo build --release   # 发布版本
cargo build             # 调试版本
```

### 刷机部署
```bash
# 使用脚本刷机（推荐）
./scripts/flash.sh [debug|release]  # 默认为release模式

# 使用web-flash工具
web-flash --chip esp32s3 target/xtensa-esp32s3-espidf/release/esp32-rs-std
```

### 环境准备
```bash
# 加载ESP-IDF环境（如果需要）
source ~/Development/esp32/esp-idf/export.sh
```

## 代码架构

### 模块结构
- `main.rs` - 主程序入口，实现动画循环显示
- `lcd.rs` - LCD控制器，负责硬件初始化和底层绘制
- `lcd_cmds.rs` - LCD命令定义和初始化序列
- `graphics/` - 图形绘制模块
  - `mod.rs` - 模块导出
  - `layout.rs` - 屏幕布局和坐标定义（九宫格、中心点等）
  - `primitives.rs` - 基本图形绘制原语
  - `colors.rs` - 颜色常量定义
  - `helper.rs` - 辅助函数（颜色转换、几何计算等）
  - `ui/` - UI组件
    - `statusbar.rs` - 状态栏组件
    - `traits.rs` - UI组件接口定义

### 硬件配置
- **目标芯片**: ESP32-S3
- **显示屏**: 360x360像素，ST77916驱动，QSPI接口
- **引脚映射**: 
  - LCD_SCK: GPIO40
  - LCD_CS: GPIO21  
  - LCD_SDA0-3: GPIO46/45/42/41
  - LCD_TE: GPIO18
  - LCD_BL: GPIO5

### 图形系统设计

#### 坐标系统
- **屏幕尺寸**: 360x360像素
- **九宫格布局**: 每个格子120x120像素
- **中心点**: (180, 180)
- 支持九宫格位置快速定位和屏幕中心绘制

#### 绘制功能
- 基本图形：圆形、矩形、线条
- 文本渲染：支持多行文本、居中对齐
- 图像显示：BMP格式支持
- 动画播放：连续图像帧播放
- 颜色管理：RGB565格式，预定义颜色常量

## 开发要点

### 内存管理
- 使用分块传输减少内存占用
- 大的图像数据通过`include_bytes!`宏在编译时嵌入
- 显示缓冲区按需分配

### 性能优化
- QSPI接口高速传输（80MHz）
- 硬件DMA加速
- 批量像素传输减少系统调用

### 错误处理
- 使用`anyhow`库进行错误处理
- 坐标边界检查防止越界
- 硬件初始化错误的详细报告

## 常见开发任务

### 添加新的图形元素
1. 在`graphics/primitives.rs`中添加绘制方法
2. 如果需要新的布局位置，在`graphics/layout.rs`中定义
3. 在`graphics/colors.rs`中添加新颜色常量（如需要）

### 修改显示内容
1. 主要逻辑在`main.rs`中修改
2. 动画图像放在`assets/`目录下
3. 使用`GraphicsPrimitives`提供的方法进行绘制

### 调试显示问题
1. 检查硬件连接和引脚配置
2. 使用`draw_debug_grid!`宏显示九宫格辅助线
3. 验证颜色格式和字节序

## 依赖库

### 核心依赖
- `esp-idf-svc` - ESP-IDF服务层
- `esp-idf-hal` - 硬件抽象层
- `embedded-graphics` - 图形绘制库
- `tinybmp` - BMP图像解析

### 构建依赖
- `embuild` - ESP构建工具
- Rust工具链: `esp` channel (见`rust-toolchain.toml`)

## 注意事项

- 项目使用ESP-IDF框架，需要正确配置ESP-IDF环境
- 显示屏使用QSPI接口，需要专门的初始化序列
- 图像数据使用RGB565格式，注意大端序处理
- 动画播放时需要适当的延迟以控制帧率
- 背光控制通过GPIO5管理，默认开启