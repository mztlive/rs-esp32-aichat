# ESP32-S3-LCD-1.85 / ESP32-S3-Touch-LCD-1.85

硬件规格 & GPIO 引脚映射手册

> 资料整理自 Waveshare 官方 Wiki（最后访问：2025-07-07）。 [oai_citation:0‡waveshare.com](https://www.waveshare.com/wiki/ESP32-S3-LCD-1.85)

---

## 1. 核心硬件

| 项目            | 规格                                                                            |
| --------------- | ------------------------------------------------------------------------------- |
| 主控            | **ESP32-S3R8** Xtensa® LX7 双核 240 MHz                                         |
| 片上 SRAM / ROM | 512 KB / 384 KB                                                                 |
| 板载存储        | 16 MB QSPI Flash、8 MB PSRAM                                                    |
| 无线            | 2.4 GHz Wi-Fi 802.11 b/g/n、Bluetooth 5 (BLE)，板载陶瓷天线 + IPEX 外接天线选项 |
| 电源            | USB-C 5 V 供电或 3.7 V 锂电；板载充电 & 3.3 V 800 mA LDO                        |

---

## 2. 显示与人机交互

| 项目   | 规格                                        |
| ------ | ------------------------------------------- |
| LCD    | 1.85″ TFT 圆形屏，**360 × 360 px** 262 K 色 |
| 控制器 | **ST77916**（4-线 QSPI 总线）               |
| 触摸   | I²C 电容触控（仅触摸版），带中断            |
| 按键   | BOOT、RESET、PWR 及音量 ±                   |
| 背光   | PWM 调光，默认连接 **GPIO 5**               |

---

## 3. 多媒体 & 传感

| 类别         | 芯片                                        | 备注 |
| ------------ | ------------------------------------------- | ---- |
| DAC & 扬声器 | **PCM5101** + 2 W 功放，2030 8 Ω 扬声器接口 |
| 麦克风       | 板载 MEMS，I²S 数字麦                       |
| IMU          | **QMI8658** 6-轴陀螺 / 加速度               |
| RTC          | **PCF85063**  低功耗实时时钟                |
| TF 卡槽      | Micro-SD，4-bit SDMMC                       |

---

## 4. GPIO / 总线引脚映射

### 4.1 LCD (ST77916)

| 信号        | ESP32-S3 引脚           |
| ----------- | ----------------------- |
| LCD_SDA0    | GPIO 46                 |
| LCD_SDA1    | GPIO 45                 |
| LCD_SDA2    | GPIO 42                 |
| LCD_SDA3    | GPIO 41                 |
| LCD_SCK     | GPIO 40                 |
| LCD_CS      | GPIO 21                 |
| LCD_TE      | GPIO 18                 |
| **LCD_RST** | EXIO2 (TCA9554 扩展 IO) |
| **LCD_BL**  | GPIO 5                  |

[oai_citation:1‡waveshare.com](https://www.waveshare.com/wiki/ESP32-S3-LCD-1.85)

### 4.2 TF 卡 (SD/MMC)

| 信号           | ESP32-S3 引脚 |
| -------------- | ------------- |
| SD_D0 / MISO   | GPIO 16       |
| SD_CMD / MOSI  | GPIO 17       |
| SD_CLK         | GPIO 14       |
| **SD_D3 / CS** | EXIO3         |
| SD_D1 / D2     | 未接          |

[oai_citation:2‡waveshare.com](https://www.waveshare.com/wiki/ESP32-S3-LCD-1.85)

### 4.3 IMU (QMI8658)

| 信号     | ESP32-S3 引脚 |
| -------- | ------------- |
| IMU_SCL  | GPIO 10 (I²C) |
| IMU_SDA  | GPIO 11 (I²C) |
| IMU_INT1 | EXIO5         |
| IMU_INT2 | EXIO4         |

[oai_citation:3‡waveshare.com](https://www.waveshare.com/wiki/ESP32-S3-LCD-1.85)

### 4.4 RTC (PCF85063)

| 信号    | ESP32-S3 引脚 |
| ------- | ------------- |
| RTC_SCL | GPIO 10       |
| RTC_SDA | GPIO 11       |
| RTC_INT | GPIO 9        |

[oai_citation:4‡waveshare.com](https://www.waveshare.com/wiki/ESP32-S3-LCD-1.85)

### 4.5 数字麦克风 (I²S)

| 信号    | ESP32-S3 引脚 |
| ------- | ------------- |
| MIC_WS  | GPIO 2        |
| MIC_SCK | GPIO 15       |
| MIC_SD  | GPIO 39       |

[oai_citation:5‡waveshare.com](https://www.waveshare.com/wiki/ESP32-S3-LCD-1.85)

### 4.6 PCM5101 DAC / 扬声器 (I²S)

| 信号 | ESP32-S3 引脚 |
| ---- | ------------- |
| DIN  | GPIO 47       |
| LRCK | GPIO 38       |
| BCK  | GPIO 48       |

[oai_citation:6‡waveshare.com](https://www.waveshare.com/wiki/ESP32-S3-LCD-1.85)

### 4.7 I²C / UART 外部排针

| 接口     | 功能    | ESP32-S3 引脚 |
| -------- | ------- | ------------- |
| I²C SCL  | 时钟    | GPIO 10       |
| I²C SDA  | 数据    | GPIO 11       |
| UART TXD | 串口 TX | GPIO 43       |
| UART RXD | 串口 RX | GPIO 44       |

[oai_citation:7‡waveshare.com](https://www.waveshare.com/wiki/ESP32-S3-LCD-1.85)

### 4.8 TCA9554 扩展 IO 对应关系

| EXIO 号 | 典型用途      |
| ------- | ------------- |
| EXIO2   | LCD_RST       |
| EXIO3   | TF_CS (SD_D3) |
| EXIO4   | IMU_INT2      |
| EXIO5   | IMU_INT1      |

> 扩展器 I²C 地址默认 **0x20**，与主板 I²C 总线 (GPIO 10/11) 共线。 [oai_citation:8‡waveshare.com](https://www.waveshare.com/wiki/ESP32-S3-LCD-1.85)

---

## 5. 电源 & 充电

- 3 × LED 指示：Power、Charge、User-configurable
- 锂电池插座：MX1.25-2P，支持充放电
- **ME6217C33M5G** 3.3 V LDO，最大 800 mA 输出 [oai_citation:9‡waveshare.com](https://www.waveshare.com/wiki/ESP32-S3-LCD-1.85)

---

## 6. 机械尺寸

| 长 (H)       | 宽 (V)       |
| ------------ | ------------ |
| **49.95 mm** | **48.08 mm** |

[oai_citation:10‡waveshare.com](https://www.waveshare.com/wiki/ESP32-S3-LCD-1.85)

---

## 7. 型号区分

| 型号                    | 触摸 | 备注     |
| ----------------------- | ---- | -------- |
| ESP32-S3-LCD-1.85       | 无   | 基础版   |
| ESP32-S3-Touch-LCD-1.85 | 有   | 电容触控 |

[oai_citation:11‡waveshare.com](https://www.waveshare.com/wiki/ESP32-S3-LCD-1.85)

> **提示**：更多示例源码、原理图与 3D 尺寸图可在官方 Wiki “Resources” 部分获取。 [oai_citation:12‡waveshare.com](https://www.waveshare.com/wiki/ESP32-S3-LCD-1.85)
