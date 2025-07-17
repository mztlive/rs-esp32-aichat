# ESP32 AI聊天系统设计方案

## 项目概述

基于ESP32-S3的AI聊天助手，支持声音唤醒（"小福，小福"）、连续录音、PCM数据传输到服务端，以及流畅的对话体验。

## 1. 系统架构

### 1.1 整体流程图

```
[待机状态] 
    ↓ 检测到"小福，小福"
[唤醒确认] 
    ↓ 显示"我在听"
[录音状态] 
    ↓ 检测到静音或手动停止
[处理状态] 
    ↓ 发送PCM到服务端
[回复状态] 
    ↓ 显示AI回复
[待机状态] ← 可在任何时候被新唤醒打断
```

### 1.2 核心组件

- **WakeWordDetector** - 唤醒词检测器
- **ChatManager** - 聊天状态管理器
- **AudioRecorder** - 录音管理器
- **VoiceActivityDetector** - 语音活动检测
- **ChatApiClient** - 聊天API客户端（扩展现有client）

## 2. 状态管理

### 2.1 聊天状态枚举

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum ChatState {
    /// 待机状态 - 持续监听唤醒词
    Idle,
    
    /// 唤醒确认状态 - 检测到唤醒词，准备录音
    WakeWordDetected,
    
    /// 录音状态 - 正在录制用户语音
    Recording {
        start_time: u64,
        duration: u32,
    },
    
    /// 处理状态 - 发送数据到服务端并等待回复
    Processing {
        audio_data: Vec<u8>,
    },
    
    /// 回复状态 - 显示AI回复内容
    Speaking {
        response: String,
        start_time: u64,
    },
    
    /// 错误状态
    Error {
        message: String,
        auto_recover_time: u64,
    },
}
```

### 2.2 聊天事件系统

```rust
#[derive(Debug, Clone)]
pub enum ChatEvent {
    /// 检测到唤醒词
    WakeWordDetected,
    
    /// 开始录音
    RecordingStarted,
    
    /// 录音数据块
    AudioData(Vec<u8>),
    
    /// 录音结束
    RecordingFinished(Vec<u8>),
    
    /// 检测到语音活动
    VoiceActivityDetected,
    
    /// 检测到静音
    SilenceDetected,
    
    /// API请求开始
    ApiRequestStarted,
    
    /// API响应接收
    ApiResponseReceived(String),
    
    /// API请求失败
    ApiRequestFailed(String),
    
    /// 用户请求停止/取消
    UserCancelled,
    
    /// 系统超时
    Timeout,
}
```

## 3. 声音检测系统

### 3.1 唤醒词检测

**实现策略：**
1. **简单音频特征匹配**（推荐）
   - 基于音调、频率、时长的简单模式匹配
   - 低CPU占用，适合ESP32
   - 可调节灵敏度

2. **关键词检测**
   - 使用轻量级音频处理
   - 检测特定音频模式

**技术实现：**
```rust
pub struct WakeWordDetector {
    /// 音频缓冲区
    audio_buffer: CircularBuffer<i16>,
    
    /// 检测参数
    threshold: f32,
    min_duration: u32,
    max_duration: u32,
    
    /// 状态追踪
    detection_state: WakeWordState,
}

#[derive(Debug)]
enum WakeWordState {
    Idle,
    FirstWord,  // 检测到第一个"小福"
    SecondWord, // 检测到第二个"小福"
    Confirmed,  // 确认唤醒
}
```

### 3.2 语音活动检测 (VAD)

**功能：**
- 检测用户是否在说话
- 区分语音和环境噪音
- 自动停止录音（静音超过2秒）

**实现：**
```rust
pub struct VoiceActivityDetector {
    /// 能量阈值
    energy_threshold: f32,
    
    /// 静音计数器
    silence_counter: u32,
    
    /// 静音阈值（毫秒）
    silence_threshold_ms: u32,
    
    /// 状态
    is_speech_active: bool,
}
```

## 4. 录音管理

### 4.1 录音器组件

```rust
pub struct AudioRecorder {
    /// I2S麦克风实例
    microphone: I2sMicrophone,
    
    /// 录音缓冲区
    recording_buffer: Vec<u8>,
    
    /// 录音状态
    is_recording: bool,
    
    /// 录音参数
    sample_rate: u32,
    max_duration: u32,
    
    /// 事件发送器
    event_sender: EventSender,
}
```

### 4.2 录音流程

1. **持续监听模式**
   - 始终保持I2S麦克风开启
   - 循环读取音频数据到循环缓冲区
   - 并行进行唤醒词检测

2. **录音模式**
   - 检测到唤醒词后切换到录音模式
   - 将音频数据保存到录音缓冲区
   - 同时进行VAD检测

3. **结束录音**
   - 静音超过2秒自动停止
   - 用户通过动作（摇晃）手动停止
   - 达到最大录音时长自动停止

## 5. API集成

### 5.1 扩展现有API客户端

```rust
impl ApiClient {
    /// 发送PCM音频数据进行语音识别和聊天
    pub fn send_audio_message(
        &self,
        session_id: &str,
        pcm_data: &[u8],
        sample_rate: u32,
        channels: u8,
    ) -> Result<String> {
        // 发送二进制PCM数据到服务端
        // 服务端处理: PCM -> 文字 -> AI回复
        // 返回AI回复文本
    }
    
    /// 创建或获取聊天会话
    pub fn ensure_chat_session(&self) -> Result<String> {
        // 确保有有效的聊天会话
        // 如果没有则创建新会话
    }
}
```

### 5.2 API数据格式

**请求格式：**
```json
{
    "session_id": "session_123",
    "audio_data": "base64_encoded_pcm_data",
    "sample_rate": 16000,
    "channels": 1,
    "format": "pcm_s16le"
}
```

**响应格式：**
```json
{
    "success": true,
    "data": {
        "transcription": "用户说的话",
        "response": "AI的回复",
        "session_id": "session_123"
    }
}
```

## 6. 用户界面设计

### 6.1 状态显示

**待机状态：**
- 显示"小福待命中..."
- 显示WiFi连接状态
- 显示当前时间

**唤醒确认：**
- 显示"我在听..."
- 显示音量条动画

**录音状态：**
- 显示"正在录音..."
- 显示录音时长
- 显示音量波形

**处理状态：**
- 显示"思考中..."
- 显示转圈动画

**回复状态：**
- 显示AI回复文本
- 支持文本滚动
- 显示"说完了"提示

### 6.2 视觉反馈

- **音量指示器**：实时显示音频输入强度
- **状态指示灯**：不同颜色表示不同状态
- **动画效果**：平滑的状态转换动画

## 7. 错误处理

### 7.1 错误类型

```rust
#[derive(Debug)]
pub enum ChatError {
    /// 麦克风错误
    MicrophoneError(String),
    
    /// 网络错误
    NetworkError(String),
    
    /// API错误
    ApiError(String),
    
    /// 录音超时
    RecordingTimeout,
    
    /// 唤醒词检测失败
    WakeWordDetectionFailed,
    
    /// 音频处理错误
    AudioProcessingError(String),
}
```

### 7.2 错误恢复策略

- **网络错误**：自动重试3次，失败后显示错误信息
- **麦克风错误**：尝试重新初始化麦克风
- **API错误**：显示错误信息，10秒后自动恢复到待机状态
- **超时错误**：自动回到待机状态

## 8. 可打断性设计

### 8.1 打断机制

**任何状态都可以被新的唤醒词打断：**
- 检测到新的"小福，小福"立即切换到录音状态
- 清除当前状态的所有缓冲数据
- 重置所有计时器

### 8.2 打断优先级

1. **唤醒词检测**（最高优先级）
2. **用户手动取消**（动作传感器）
3. **系统错误**
4. **超时机制**

## 9. 性能优化

### 9.1 内存管理

- **循环缓冲区**：避免频繁内存分配
- **数据流式处理**：及时释放处理完的音频数据
- **缓冲区大小调优**：平衡内存使用和性能

### 9.2 CPU优化

- **音频处理**：使用整数运算代替浮点运算
- **检测算法**：优化唤醒词检测算法
- **并行处理**：音频采集和处理在不同线程

## 10. 配置参数

### 10.1 音频配置

```rust
pub struct AudioConfig {
    /// 采样率
    pub sample_rate: u32,        // 16000 Hz
    
    /// 声道数
    pub channels: u8,            // 1 (单声道)
    
    /// 位深度
    pub bit_depth: u8,           // 16 bits
    
    /// 缓冲区大小
    pub buffer_size: usize,      // 1024 samples
    
    /// 最大录音时长（秒）
    pub max_recording_duration: u32,  // 30 seconds
}
```

### 10.2 检测配置

```rust
pub struct DetectionConfig {
    /// 唤醒词能量阈值
    pub wake_word_threshold: f32,     // 0.5
    
    /// VAD能量阈值
    pub vad_threshold: f32,           // 0.3
    
    /// 静音超时（毫秒）
    pub silence_timeout_ms: u32,      // 2000
    
    /// 唤醒词超时（毫秒）
    pub wake_word_timeout_ms: u32,    // 3000
}
```

## 11. 实现阶段

### 阶段1：基础录音功能
- 实现AudioRecorder基本功能
- 集成到事件系统
- 基本的开始/停止录音

### 阶段2：唤醒词检测
- 实现WakeWordDetector
- 简单的音频特征匹配
- 集成到主循环

### 阶段3：语音活动检测
- 实现VAD功能
- 自动停止录音
- 优化检测算法

### 阶段4：API集成
- 扩展现有API客户端
- 实现PCM数据传输
- 错误处理和重试机制

### 阶段5：用户界面
- 实现状态显示
- 添加动画效果
- 优化用户体验

### 阶段6：优化和测试
- 性能优化
- 内存优化
- 全面测试

## 12. 技术难点和解决方案

### 12.1 唤醒词检测准确性

**挑战：**
- 在嘈杂环境中准确检测
- 避免误触发
- 低功耗持续监听

**解决方案：**
- 多阈值检测机制
- 自适应噪声抑制
- 时间窗口验证

### 12.2 实时性能

**挑战：**
- 低延迟音频处理
- 流畅的状态切换
- 避免阻塞主线程

**解决方案：**
- 专用音频处理线程
- 无锁数据结构
- 优化的音频算法

### 12.3 网络稳定性

**挑战：**
- 网络连接不稳定
- 大数据量传输
- 超时处理

**解决方案：**
- 自动重连机制
- 数据压缩传输
- 智能超时策略

## 13. 测试策略

### 13.1 单元测试
- 音频处理函数
- 状态转换逻辑
- API客户端功能

### 13.2 集成测试
- 完整对话流程
- 错误恢复机制
- 性能基准测试

### 13.3 用户测试
- 不同环境下的准确性
- 用户体验评估
- 长时间运行稳定性

---

**文档版本：** v1.0
**最后更新：** 2025-01-17
**状态：** 设计阶段