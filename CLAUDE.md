# CLAUDE.md

本文件为Claude Code (claude.ai/code) 在处理此仓库代码时提供指导。

## 项目概述

基于ESP32-S3的Rust项目，用于构建带有360x360 LCD显示屏(ST77916驱动)的AI聊天助手。特色包括事件驱动架构与Actor模式、WiFi连接、HTTP API集成、I2S麦克风支持，以及带有动作传感器集成的综合图形系统。

## 构建与部署命令

### 构建项目
```bash
# 使用脚本（推荐）
./scripts/build.sh [debug|release]  # 默认为release

# 直接使用cargo命令
cargo build --release   # release构建
cargo build             # debug构建
```

### 烧录到设备
```bash
# 使用脚本（推荐）
./scripts/flash.sh [debug|release]  # 默认为release

# 手动烧录
web-flash --chip esp32s3 target/xtensa-esp32s3-espidf/release/esp32-rs-std
```

### 环境设置
```bash
# 如需加载ESP-IDF环境
source ~/Development/esp32/esp-idf/export.sh
```

## 架构

### 事件驱动系统
- **事件总线**: 中央事件系统(`src/events.rs`)，包含EventBus、EventSender和EventReceiver
- **统一事件**: 所有事件整合到`AppEvent`枚举中(Motion、WiFi、System)
- **事件处理器**: 通过`EventHandler`特征实现一致的事件处理

### Actor模式实现
- **Motion Actor**: 专用线程处理动作检测，带心跳机制
- **WiFi Actor**: 管理WiFi操作和状态，包含完整生命周期
- **显示状态机**: 替换DisplayActor，实现复杂的状态管理
- **API集成**: 完整的HTTP客户端，支持聊天功能和流式响应

### 核心模块结构
- `main.rs` - 入口点，包含事件循环和actor初始化
- `app.rs` - 应用事件处理器，实现EventHandler特征
- `display.rs` - 显示状态机，包含全面的状态管理
- `events.rs` - 事件总线系统，包含EventBus、EventSender、EventReceiver
- `actors/` - Actor系统实现
  - `motion.rs` - 动作检测actor，带心跳机制
  - `wifi.rs` - WiFi管理actor，包含状态追踪
- `api/` - HTTP API客户端系统
  - `client.rs` - HTTP客户端实现，支持流式传输
  - `types.rs` - API类型定义，包含SSE事件
- `peripherals/` - 硬件驱动
  - `st77916/` - ST77916 LCD控制器，带QSPI接口
  - `qmi8658/` - 增强动作检测算法
  - `microphone/` - I2S麦克风支持
  - `wifi/` - WiFi管理和配置
- `graphics/` - 图形渲染系统
  - `primitives.rs` - 核心绘图操作
  - `layout.rs` - 360x360屏幕网格系统和坐标助手
  - `screens/` - 状态特定UI屏幕(welcome、main、dizziness等)
  - `ui/` - UI组件，包含状态栏和组件特征
  - `animation/` - 动画资源和播放系统
  - `helper.rs` - 额外图形工具

### 状态管理
应用使用事件驱动状态机：
- **显示状态**: `DisplayState`枚举，包含Welcome、Main、Settings、Thinking、Dizziness、Tilting、Error
- **事件流**: Motion/WiFi/System事件 → EventBus → 状态转换
- **自动转换**: 基于定时器的自动状态切换，带复杂的时间控制
- **API集成**: HTTP事件触发聊天响应的状态变化

### 硬件配置
- **目标**: ESP32-S3微控制器，支持WiFi
- **显示屏**: 360x360 LCD，ST77916驱动，QSPI接口@80MHz
- **动作传感器**: QMI8658 6轴IMU，通过I2C连接
- **麦克风**: I2S数字麦克风，可配置采样率
- **WiFi**: ESP32-S3内置WiFi，支持WPA2/WPA3
- **引脚映射**:
  - LCD QSPI: SCK=GPIO40, CS=GPIO21, DATA0-3=GPIO46/45/42/41
  - LCD控制: TE=GPIO18, BL=GPIO5
  - I2C: SDA=GPIO11, SCL=GPIO10
  - I2S麦克风: [可配置GPIO引脚]

### 图形系统
- **360x360屏幕**，采用3x3网格布局(每格120x120)
- **网格定位**，通过`GridPosition`枚举快速UI布局
- **绘图基元**: 圆形、矩形、文本、图像(BMP格式)
- **动画支持**: 基于帧的播放，从assets/目录加载
- **颜色系统**: RGB565格式，带预定义常量
- **布局助手**: `graphics/layout.rs`提供屏幕中心(180,180)和网格计算

## 关键开发要点

### 内存与性能
- **分块传输**减少大图形的内存使用
- **资源嵌入**通过`include_bytes!`在编译时完成
- **QSPI @ 80MHz**带DMA加速的显示更新
- **静态分配**在actor线程中使用`Box::leak`处理LCD控制器

### 错误处理
- **anyhow crate**用于整个应用的错误传播
- **边界检查**防止360x360显示屏坐标溢出
- **硬件验证**在初始化时提供详细错误报告

## 常见开发任务

### 添加图形元素
1. 在`graphics/primitives.rs`中添加绘图方法
2. 如需要，在`graphics/layout.rs`中定义新的布局位置
3. 在`graphics/colors.rs`中添加颜色常量

### 修改UI屏幕
1. 编辑`graphics/screens/`中的状态特定屏幕
2. 在`app.rs`中更新状态机转换
3. 将动画资源放在`assets/`目录
4. 使用`GraphicsPrimitives`方法进行渲染

### 动作检测变更
1. 修改`peripherals/qmi8658/motion_detector.rs`中的算法
2. 如需要，添加新的`MotionState`变体
3. 更新`actors/display.rs`中的事件处理

### 调试图形问题
- 使用`draw_debug_grid!`宏进行网格可视化
- 检查`peripherals/st77916/lcd.rs`中的引脚映射
- 验证RGB565颜色格式和字节序

## 依赖项
- **esp-idf-svc/hal**: ESP-IDF硬件抽象层
- **embedded-graphics**: 核心图形渲染
- **tinybmp**: 资源的BMP图像解析
- **anyhow**: 整个应用的错误处理
- **embuild**: ESP32构建系统集成

## 重要说明
- **ESP-IDF环境**必须在构建前正确配置
- **QSPI显示**需要`lcd_cmds.rs`中的特定初始化序列
- **RGB565格式**需要正确的字节序处理图像数据
- **动画时序**通过延迟控制以保持一致帧率
- **背光控制**通过GPIO5，默认启用
- **动作检测**使用I2C接口，带可配置阈值
- **状态机**在主循环中以~20fps运行(50ms延迟)
- **Actor线程**将显示操作从传感器轮询中隔离
- **永远不要主动构建** - 始终使用提供的脚本或让用户选择构建命令

## 事件系统架构

### 事件流
1. **硬件事件** → Actors(Motion、WiFi)
2. **Actors** → 事件总线(EventSender)
3. **事件总线** → 事件处理器(App、Display)
4. **事件处理器** → 状态变更 → UI更新

### 事件类型
- **MotionEvent**: Still、Shaking、Tilting状态
- **WifiEvent**: Connected、Disconnected、Error状态
- **SystemEvent**: Timer、Error、Shutdown事件
- **ApiEvent**: HTTP请求/响应事件

### 添加新事件
1. 在`src/events.rs`中定义新的事件变体
2. 在相关actor中实现事件发送
3. 在`app.rs`事件处理器中处理事件
4. 如需要，更新显示状态机

## API集成详情

### HTTP客户端配置
- **基础URL**: 可配置的API端点
- **超时**: 可配置的请求超时
- **流式传输**: 服务器发送事件(SSE)支持
- **错误处理**: 综合错误处理和重试逻辑

### API类型
- **请求类型**: 聊天消息、配置
- **响应类型**: 聊天响应、流式事件
- **SSE事件**: 数据块、完成事件

### 使用示例
```rust
// 发送聊天消息
let response = api_client.send_chat_message("Hello!").await?;

// 处理流式响应
for event in response.events() {
    match event {
        SseEvent::Data(data) => {
            // 处理输入数据
        }
        SseEvent::Done => {
            // 响应完成
        }
    }
}
```

## 开发指南

### 事件驱动开发
- **始终使用事件总线**进行组件间通信
- **实现EventHandler特征**用于新的事件处理器
- **从actors和peripherals通过EventSender发送事件**
- **在主事件循环或专用处理器中处理事件**

### API开发
- **使用ApiClient**进行所有HTTP操作
- **适当处理流式响应**
- **为网络操作实现适当的错误处理**
- **为所有网络请求配置超时**

### 状态管理
- **使用DisplayState枚举**进行UI状态管理
- **通过事件处理实现状态转换**
- **在适当时使用基于定时器的自动转换**
- **在事件处理过程中保持状态一致性**
