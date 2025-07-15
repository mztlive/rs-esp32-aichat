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