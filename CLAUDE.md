# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

ESP32-S3 based Rust project for an AI chat assistant with 360x360 LCD display (ST77916 driver). Features complete graphics system, animation playback, text rendering, and motion sensor integration using actor pattern architecture.

## Build & Deploy Commands

### Build Project
```bash
# Using scripts (recommended)
./scripts/build.sh [debug|release]  # defaults to release

# Direct cargo commands
cargo build --release   # release build
cargo build             # debug build
```

### Flash to Device
```bash
# Using scripts (recommended) 
./scripts/flash.sh [debug|release]  # defaults to release

# Manual flash
web-flash --chip esp32s3 target/xtensa-esp32s3-espidf/release/esp32-rs-std
```

### Environment Setup
```bash
# Load ESP-IDF environment if needed
source ~/Development/esp32/esp-idf/export.sh
```

## Architecture

### Actor Pattern Implementation
- **DisplayActor**: Handles UI rendering and state management in separate thread
- **DisplayActorManager**: Message passing interface for sensor events
- **Event System**: Motion events processed through mpsc channels

### Core Module Structure
- `main.rs` - Entry point with sensor loop and actor initialization
- `app.rs` - State machine managing chat assistant UI states
- `actors/display.rs` - Actor pattern implementation for UI thread
- `peripherals/` - Hardware drivers
  - `st77916/lcd.rs` - ST77916 LCD controller with QSPI interface
  - `qmi8658/motion_detector.rs` - Motion detection algorithms
- `graphics/` - Graphics rendering system
  - `primitives.rs` - Core drawing operations
  - `layout.rs` - 360x360 screen grid system and coordinate helpers
  - `screens/` - State-specific UI screens (welcome, main, dizziness, etc.)

### State Management
Application uses enum-based state machine:
- `AppState::Welcome` → `AppState::Main` → `AppState::Settings`
- Motion triggers: `MotionState::Shaking` → `AppState::Dizziness`
- Auto-transitions with timer-based state management

### Hardware Configuration
- **Target**: ESP32-S3 microcontroller
- **Display**: 360x360 LCD with ST77916 driver, QSPI interface @ 80MHz
- **Motion Sensor**: QMI8658 6-axis IMU via I2C
- **Pin Mapping**:
  - LCD QSPI: SCK=GPIO40, CS=GPIO21, DATA0-3=GPIO46/45/42/41
  - LCD Control: TE=GPIO18, BL=GPIO5
  - I2C: SDA=GPIO11, SCL=GPIO10

### Graphics System
- **360x360 screen** with 3x3 grid layout (120x120 per cell)  
- **Grid positioning** via `GridPosition` enum for quick UI placement
- **Drawing primitives**: circles, rectangles, text, images (BMP format)
- **Animation support**: frame-based playback from assets/ directory
- **Color system**: RGB565 format with predefined constants
- **Layout helpers**: `graphics/layout.rs` provides screen center (180,180) and grid calculations

## Key Development Notes

### Memory & Performance
- **Chunked transfers** reduce memory usage for large graphics
- **Asset embedding** via `include_bytes!` at compile time
- **QSPI @ 80MHz** with DMA acceleration for display updates
- **Static allocation** using `Box::leak` for LCD controller in actor thread

### Error Handling
- **anyhow crate** for error propagation throughout application
- **Boundary checking** prevents coordinate overflow on 360x360 display
- **Hardware validation** with detailed error reporting during init

## 常见开发任务

### 添加新的图形元素
1. 在`graphics/primitives.rs`中添加绘制方法
2. 如果需要新的布局位置，在`graphics/layout.rs`中定义
3. 在`graphics/colors.rs`中添加新颜色常量（如需要）

### 修改显示内容
1. 应用状态管理在`app.rs`中修改
2. 主循环和传感器逻辑在`main.rs`中修改
3. 动画图像放在`assets/`目录下
4. 使用`GraphicsPrimitives`提供的方法进行绘制

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

### 添加新的传感器功能
1. 在`peripherals/qmi8658/motion_detector.rs`中添加新的运动检测算法
2. 在`app.rs`中添加新的应用状态来响应传感器事件
3. 在主循环中集成新的传感器数据处理逻辑

### 应用状态管理
应用使用状态机模式管理不同界面：
- `AppState::Welcome` - 欢迎界面
- `AppState::Main` - 主聊天界面
- `AppState::Settings` - 设置界面
- `AppState::Thinking` - AI思考状态
- `AppState::Dizziness` - 设备被摇晃状态
- `AppState::Error` - 错误状态

状态转换通过`UserInput`枚举触发，支持按键、确认、取消等操作。

## 注意事项

- 项目使用ESP-IDF框架，需要正确配置ESP-IDF环境
- 显示屏使用QSPI接口，需要专门的初始化序列
- 图像数据使用RGB565格式，注意大端序处理
- 动画播放时需要适当的延迟以控制帧率
- 背光控制通过GPIO5管理，默认开启
- QMI8658传感器使用I2C接口，支持运动检测和摇晃检测
- 应用状态机运行在主循环中，约20fps更新频率
- 永远不要主动cargo check 或者 build、 run，把编译的选择权交给用户