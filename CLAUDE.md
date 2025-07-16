# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

ESP32-S3 based Rust project for an AI chat assistant with 360x360 LCD display (ST77916 driver). Features event-driven architecture with actor pattern, WiFi connectivity, HTTP API integration, I2S microphone support, and comprehensive graphics system with motion sensor integration.

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

### Event-Driven System
- **Event Bus**: Central event system (`src/events.rs`) with EventBus, EventSender, and EventReceiver
- **Unified Events**: All events consolidated into `AppEvent` enum (Motion, WiFi, System)
- **Event Handlers**: Consistent event processing via `EventHandler` trait

### Actor Pattern Implementation
- **Motion Actor**: Dedicated thread for motion detection with heartbeat mechanism
- **WiFi Actor**: Manages WiFi operations and state with comprehensive lifecycle
- **Display State Machine**: Replaces DisplayActor with sophisticated state management
- **API Integration**: Complete HTTP client for chat functionality and streaming responses

### Core Module Structure
- `main.rs` - Entry point with event loop and actor initialization
- `app.rs` - Application event handler implementing EventHandler trait
- `display.rs` - Display state machine with comprehensive state management
- `events.rs` - Event bus system with EventBus, EventSender, EventReceiver
- `actors/` - Actor system implementation
  - `motion.rs` - Motion detection actor with heartbeat mechanism
  - `wifi.rs` - WiFi management actor with state tracking
- `api/` - HTTP API client system
  - `client.rs` - HTTP client implementation with streaming support
  - `types.rs` - API type definitions including SSE events
- `peripherals/` - Hardware drivers
  - `st77916/` - ST77916 LCD controller with QSPI interface
  - `qmi8658/` - Enhanced motion detection algorithms
  - `microphone/` - I2S microphone support
  - `wifi/` - WiFi management and configuration
- `graphics/` - Graphics rendering system
  - `primitives.rs` - Core drawing operations
  - `layout.rs` - 360x360 screen grid system and coordinate helpers
  - `screens/` - State-specific UI screens (welcome, main, dizziness, etc.)
  - `ui/` - UI components with status bar and component traits
  - `animation/` - Animation assets and playback system
  - `helper.rs` - Additional graphics utilities

### State Management
Application uses event-driven state machine:
- **Display States**: `DisplayState` enum with Welcome, Main, Settings, Thinking, Dizziness, Tilting, Error
- **Event Flow**: Motion/WiFi/System events → EventBus → State transitions
- **Auto-transitions**: Timer-based automatic state switching with sophisticated timing controls
- **API Integration**: HTTP events trigger state changes for chat responses

### Hardware Configuration
- **Target**: ESP32-S3 microcontroller with WiFi capability
- **Display**: 360x360 LCD with ST77916 driver, QSPI interface @ 80MHz
- **Motion Sensor**: QMI8658 6-axis IMU via I2C
- **Microphone**: I2S digital microphone with configurable sampling rate
- **WiFi**: Built-in ESP32-S3 WiFi with WPA2/WPA3 support
- **Pin Mapping**:
  - LCD QSPI: SCK=GPIO40, CS=GPIO21, DATA0-3=GPIO46/45/42/41
  - LCD Control: TE=GPIO18, BL=GPIO5
  - I2C: SDA=GPIO11, SCL=GPIO10
  - I2S Microphone: [Configurable GPIO pins]

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

## Common Development Tasks

### Adding Graphics Elements
1. Add drawing methods to `graphics/primitives.rs`
2. Define new layout positions in `graphics/layout.rs` if needed
3. Add color constants to `graphics/colors.rs`

### Modifying UI Screens
1. Edit state-specific screens in `graphics/screens/`
2. Update state machine transitions in `app.rs`
3. Place animation assets in `assets/` directory
4. Use `GraphicsPrimitives` methods for rendering

### Motion Detection Changes
1. Modify algorithms in `peripherals/qmi8658/motion_detector.rs`
2. Add new `MotionState` variants if needed
3. Update event handling in `actors/display.rs`

### Debug Graphics Issues
- Use `draw_debug_grid!` macro for grid visualization
- Check pin mappings in `peripherals/st77916/lcd.rs`
- Verify RGB565 color format and endianness

## Dependencies
- **esp-idf-svc/hal**: ESP-IDF hardware abstraction
- **embedded-graphics**: Core graphics rendering
- **tinybmp**: BMP image parsing for assets
- **anyhow**: Error handling throughout application
- **embuild**: ESP32 build system integration

## Important Notes
- **ESP-IDF environment** must be properly configured before building
- **QSPI display** requires specific initialization sequence in `lcd_cmds.rs`
- **RGB565 format** with proper endianness handling for image data
- **Animation timing** controlled via delays for consistent frame rates
- **Backlight control** via GPIO5, enabled by default
- **Motion detection** uses I2C interface with configurable thresholds
- **State machine** runs at ~20fps (50ms delays) in main loop
- **Actor threading** isolates display operations from sensor polling
- **Never initiate builds** - always use provided scripts or let user choose build commands

## Event System Architecture

### Event Flow
1. **Hardware Events** → Actors (Motion, WiFi)
2. **Actors** → Event Bus (EventSender)
3. **Event Bus** → Event Handlers (App, Display)
4. **Event Handlers** → State Changes → UI Updates

### Event Types
- **MotionEvent**: Still, Shaking, Tilting states
- **WifiEvent**: Connected, Disconnected, Error states
- **SystemEvent**: Timer, Error, Shutdown events
- **ApiEvent**: HTTP request/response events

### Adding New Events
1. Define new event variant in `src/events.rs`
2. Implement event sending in relevant actor
3. Handle event in `app.rs` event handler
4. Update display state machine if needed

## API Integration Details

### HTTP Client Configuration
- **Base URL**: Configurable API endpoint
- **Timeout**: Configurable request timeout
- **Streaming**: Server-Sent Events (SSE) support
- **Error Handling**: Comprehensive error handling and retry logic

### API Types
- **Request Types**: Chat messages, configuration
- **Response Types**: Chat responses, streaming events
- **SSE Events**: Data chunks, completion events

### Usage Example
```rust
// Send chat message
let response = api_client.send_chat_message("Hello!").await?;

// Process streaming response
for event in response.events() {
    match event {
        SseEvent::Data(data) => {
            // Handle incoming data
        }
        SseEvent::Done => {
            // Response complete
        }
    }
}
```

## Development Guidelines

### Event-Driven Development
- **Always use event bus** for communication between components
- **Implement EventHandler trait** for new event processors
- **Send events through EventSender** from actors and peripherals
- **Handle events in main event loop** or dedicated handlers

### API Development
- **Use ApiClient** for all HTTP operations
- **Handle streaming responses** appropriately
- **Implement proper error handling** for network operations
- **Configure timeouts** for all network requests

### State Management
- **Use DisplayState enum** for UI state management
- **Implement state transitions** through event handling
- **Use timer-based auto-transitions** where appropriate
- **Maintain state consistency** across event processing