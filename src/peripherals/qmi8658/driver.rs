//! QMI8658 6轴IMU传感器驱动
//!
//! 该模块提供了对QMI8658 6轴惯性测量单元(IMU)传感器的完整驱动支持。
//! QMI8658集成了3轴加速度计和3轴陀螺仪，支持多种测量范围和输出数据率。
//!
//! # 功能特性
//!
//! - 支持I2C通信协议
//! - 可配置的加速度计测量范围(±2g到±16g)
//! - 可配置的陀螺仪测量范围(±32dps到±4096dps)
//! - 多种输出数据率(ODR)选择
//! - 温度测量
//! - 运动唤醒功能
//! - 支持多种单位输出(m/s²、mg、rad/s、dps)
//!
//! # 使用示例
//!
//! ```rust
//! use esp_idf_hal::peripherals::Peripherals;
//! use crate::peripherals::qmi8658::driver::QMI8658Driver;
//!
//! let peripherals = Peripherals::take().unwrap();
//! let mut sensor = QMI8658Driver::new(
//!     peripherals.i2c0,
//!     peripherals.pins.gpio11,
//!     peripherals.pins.gpio10,
//!     0x6A
//! )?;
//!
//! let sensor_data = sensor.read_sensor_data()?;
//! println!("Accel: {:.2}, {:.2}, {:.2}",
//!          sensor_data.accel_x, sensor_data.accel_y, sensor_data.accel_z);
//! ```

use anyhow::Result;
use esp_idf_hal::gpio::{Gpio10, Gpio11};
use esp_idf_hal::i2c::{I2cConfig, I2cDriver, I2C0};
use esp_idf_hal::prelude::*;
use log::{error, info};
use std::f32::consts::PI;

/// QMI8658 I2C地址(当SA0引脚接地时)
pub const QMI8658_ADDRESS_LOW: u8 = 0x6A;
/// QMI8658 I2C地址(当SA0引脚接高电平时)
pub const QMI8658_ADDRESS_HIGH: u8 = 0x6B;

const M_PI: f32 = PI;
const ONE_G: f32 = 9.807;

const QMI8658_DISABLE_ALL: u8 = 0x00;
const QMI8658_ENABLE_ACCEL: u8 = 0x01;
const QMI8658_ENABLE_GYRO: u8 = 0x02;
const QMI8658_ENABLE_MAG: u8 = 0x04;
const QMI8658_ENABLE_AE: u8 = 0x08;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum QMI8658Register {
    WhoAmI = 0x00,
    Revision = 0x01,
    Ctrl1 = 0x02,
    Ctrl2 = 0x03,
    Ctrl3 = 0x04,
    Ctrl4 = 0x05,
    Ctrl5 = 0x06,
    Ctrl6 = 0x07,
    Ctrl7 = 0x08,
    Ctrl8 = 0x09,
    Ctrl9 = 0x0A,
    Status0 = 0x2E,
    Status1 = 0x2F,
    TimestampL = 0x30,
    TimestampM = 0x31,
    TimestampH = 0x32,
    TempL = 0x33,
    TempH = 0x34,
    AxL = 0x35,
    AxH = 0x36,
    AyL = 0x37,
    AyH = 0x38,
    AzL = 0x39,
    AzH = 0x3A,
    GxL = 0x3B,
    GxH = 0x3C,
    GyL = 0x3D,
    GyH = 0x3E,
    GzL = 0x3F,
    GzH = 0x40,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum AccelRange {
    Range2G = 0x00,
    Range4G = 0x01,
    Range8G = 0x02,
    Range16G = 0x03,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum AccelODR {
    /// 8000Hz输出数据率
    ODR8000Hz = 0x00,
    /// 4000Hz输出数据率
    ODR4000Hz = 0x01,
    /// 2000Hz输出数据率
    ODR2000Hz = 0x02,
    /// 1000Hz输出数据率
    ODR1000Hz = 0x03,
    /// 500Hz输出数据率
    ODR500Hz = 0x04,
    /// 250Hz输出数据率
    ODR250Hz = 0x05,
    /// 125Hz输出数据率
    ODR125Hz = 0x06,
    /// 62.5Hz输出数据率
    ODR62_5Hz = 0x07,
    /// 31.25Hz输出数据率
    ODR31_25Hz = 0x08,
    /// 低功耗128Hz输出数据率
    ODRLowPower128Hz = 0x0C,
    /// 低功耗21Hz输出数据率
    ODRLowPower21Hz = 0x0D,
    /// 低功耗11Hz输出数据率
    ODRLowPower11Hz = 0x0E,
    /// 低功耗3Hz输出数据率
    ODRLowPower3Hz = 0x0F,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum GyroRange {
    Range32DPS = 0x00,
    Range64DPS = 0x01,
    Range128DPS = 0x02,
    Range256DPS = 0x03,
    Range512DPS = 0x04,
    Range1024DPS = 0x05,
    Range2048DPS = 0x06,
    Range4096DPS = 0x07,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum GyroODR {
    /// 8000Hz输出数据率
    ODR8000Hz = 0x00,
    /// 4000Hz输出数据率
    ODR4000Hz = 0x01,
    /// 2000Hz输出数据率
    ODR2000Hz = 0x02,
    /// 1000Hz输出数据率
    ODR1000Hz = 0x03,
    /// 500Hz输出数据率
    ODR500Hz = 0x04,
    /// 250Hz输出数据率
    ODR250Hz = 0x05,
    /// 125Hz输出数据率
    ODR125Hz = 0x06,
    /// 62.5Hz输出数据率
    ODR62_5Hz = 0x07,
    /// 31.25Hz输出数据率
    ODR31_25Hz = 0x08,
}

#[derive(Debug, Clone, Copy)]
pub enum Precision {
    Two = 2,
    Four = 4,
    Six = 6,
}

#[derive(Debug, Clone, Copy)]
pub struct SensorData {
    pub accel_x: f32,
    pub accel_y: f32,
    pub accel_z: f32,
    pub gyro_x: f32,
    pub gyro_y: f32,
    pub gyro_z: f32,
    pub temperature: f32,
    pub timestamp: u32,
}

pub struct QMI8658Driver<'a> {
    i2c: I2cDriver<'a>,
    address: u8,
    accel_lsb_div: u16,
    gyro_lsb_div: u16,
    accel_unit_mps2: bool,
    gyro_unit_rads: bool,
    display_precision: i32,
    timestamp: u32,
}

impl<'a> std::fmt::Debug for QMI8658Driver<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("QMI8658Driver")
            .field("address", &self.address)
            .field("accel_lsb_div", &self.accel_lsb_div)
            .field("gyro_lsb_div", &self.gyro_lsb_div)
            .field("accel_unit_mps2", &self.accel_unit_mps2)
            .field("gyro_unit_rads", &self.gyro_unit_rads)
            .field("display_precision", &self.display_precision)
            .field("timestamp", &self.timestamp)
            .finish()
    }
}

impl<'a> QMI8658Driver<'a> {
    /// 创建新的QMI8658驱动器实例
    /// 
    /// # 参数
    /// 
    /// * `i2c0` - I2C外设实例
    /// * `sda` - SDA引脚(GPIO11)
    /// * `scl` - SCL引脚(GPIO10)
    /// * `address` - 设备I2C地址
    /// 
    /// # 返回
    /// 
    /// 返回配置好的驱动器实例或错误
    pub fn new(i2c0: I2C0, sda: Gpio11, scl: Gpio10, address: u8) -> Result<Self> {
        let config = I2cConfig::new().baudrate(400.kHz().into());
        let i2c = I2cDriver::new(i2c0, sda, scl, &config)?;

        let mut driver = QMI8658Driver {
            i2c,
            address,
            accel_lsb_div: 4096,
            gyro_lsb_div: 64,
            accel_unit_mps2: false,
            gyro_unit_rads: false,
            display_precision: 6,
            timestamp: 0,
        };

        for addr in 0x08..=0x77 {
            // todo: 这里可能会有问题
            let _ = driver.i2c.write(addr, &[0x00], 100);
        }

        driver.init()?;
        Ok(driver)
    }

    /// 初始化传感器
    /// 
    /// 设置默认配置：8G加速度计范围，512DPS陀螺仪范围，1000Hz ODR
    fn init(&mut self) -> Result<()> {
        let who_am_i = self.get_who_am_i()?;
        if who_am_i != 0x05 {
            error!("Invalid WHO_AM_I value: 0x{:02X}, expected 0x05", who_am_i);
            return Err(anyhow::anyhow!("Invalid WHO_AM_I"));
        }

        self.write_register(QMI8658Register::Ctrl1, 0x60)?;

        self.set_accel_range(AccelRange::Range8G)?;
        self.set_accel_odr(AccelODR::ODR1000Hz)?;
        self.set_gyro_range(GyroRange::Range512DPS)?;
        self.set_gyro_odr(GyroODR::ODR1000Hz)?;
        self.enable_sensors(QMI8658_ENABLE_ACCEL | QMI8658_ENABLE_GYRO)?;

        info!("QMI8658 initialized successfully");
        Ok(())
    }

    /// 写入寄存器
    /// 
    /// # 参数
    /// 
    /// * `reg` - 寄存器地址
    /// * `value` - 要写入的值
    fn write_register(&mut self, reg: QMI8658Register, value: u8) -> Result<()> {
        let data = [reg as u8, value];
        self.i2c.write(self.address, &data, 1000)?;
        Ok(())
    }

    /// 读取寄存器
    /// 
    /// # 参数
    /// 
    /// * `reg` - 寄存器地址
    /// * `buffer` - 存储读取数据的缓冲区
    fn read_register(&mut self, reg: QMI8658Register, buffer: &mut [u8]) -> Result<()> {
        let reg_addr = [reg as u8];
        self.i2c.write_read(self.address, &reg_addr, buffer, 1000)?;
        Ok(())
    }

    /// 获取设备ID
    /// 
    /// # 返回
    /// 
    /// 返回设备ID，QMI8658应该返回0x05
    pub fn get_who_am_i(&mut self) -> Result<u8> {
        let mut buffer = [0u8; 1];
        match self.read_register(QMI8658Register::WhoAmI, &mut buffer) {
            Ok(_) => Ok(buffer[0]),
            Err(e) => Err(e),
        }
    }

    /// 设置加速度计测量范围
    /// 
    /// # 参数
    /// 
    /// * `range` - 测量范围配置
    pub fn set_accel_range(&mut self, range: AccelRange) -> Result<()> {
        self.accel_lsb_div = match range {
            AccelRange::Range2G => 16384,
            AccelRange::Range4G => 8192,
            AccelRange::Range8G => 4096,
            AccelRange::Range16G => 2048,
        };

        self.write_register(QMI8658Register::Ctrl2, ((range as u8) << 4) | 0x03)
    }

    /// 设置加速度计输出数据率
    /// 
    /// # 参数
    /// 
    /// * `odr` - 输出数据率配置
    pub fn set_accel_odr(&mut self, odr: AccelODR) -> Result<()> {
        let mut current_ctrl2 = [0u8; 1];
        self.read_register(QMI8658Register::Ctrl2, &mut current_ctrl2)?;

        let new_ctrl2 = (current_ctrl2[0] & 0xF0) | (odr as u8);
        self.write_register(QMI8658Register::Ctrl2, new_ctrl2)
    }

    /// 设置陀螺仪测量范围
    /// 
    /// # 参数
    /// 
    /// * `range` - 测量范围配置
    pub fn set_gyro_range(&mut self, range: GyroRange) -> Result<()> {
        self.gyro_lsb_div = match range {
            GyroRange::Range32DPS => 1024,
            GyroRange::Range64DPS => 512,
            GyroRange::Range128DPS => 256,
            GyroRange::Range256DPS => 128,
            GyroRange::Range512DPS => 64,
            GyroRange::Range1024DPS => 32,
            GyroRange::Range2048DPS => 16,
            GyroRange::Range4096DPS => 8,
        };

        self.write_register(QMI8658Register::Ctrl3, ((range as u8) << 4) | 0x03)
    }

    /// 设置陀螺仪输出数据率
    /// 
    /// # 参数
    /// 
    /// * `odr` - 输出数据率配置
    pub fn set_gyro_odr(&mut self, odr: GyroODR) -> Result<()> {
        let mut current_ctrl3 = [0u8; 1];
        self.read_register(QMI8658Register::Ctrl3, &mut current_ctrl3)?;

        let new_ctrl3 = (current_ctrl3[0] & 0xF0) | (odr as u8);
        self.write_register(QMI8658Register::Ctrl3, new_ctrl3)
    }

    /// 启用或禁用加速度计
    /// 
    /// # 参数
    /// 
    /// * `enable` - true启用，false禁用
    pub fn enable_accel(&mut self, enable: bool) -> Result<()> {
        let mut current_ctrl7 = [0u8; 1];
        self.read_register(QMI8658Register::Ctrl7, &mut current_ctrl7)?;

        let new_ctrl7 = if enable {
            current_ctrl7[0] | QMI8658_ENABLE_ACCEL
        } else {
            current_ctrl7[0] & !QMI8658_ENABLE_ACCEL
        };

        self.write_register(QMI8658Register::Ctrl7, new_ctrl7)
    }

    /// 启用或禁用陀螺仪
    /// 
    /// # 参数
    /// 
    /// * `enable` - true启用，false禁用
    pub fn enable_gyro(&mut self, enable: bool) -> Result<()> {
        let mut current_ctrl7 = [0u8; 1];
        self.read_register(QMI8658Register::Ctrl7, &mut current_ctrl7)?;

        let new_ctrl7 = if enable {
            current_ctrl7[0] | QMI8658_ENABLE_GYRO
        } else {
            current_ctrl7[0] & !QMI8658_ENABLE_GYRO
        };

        self.write_register(QMI8658Register::Ctrl7, new_ctrl7)
    }

    /// 启用传感器
    /// 
    /// # 参数
    /// 
    /// * `enable_flags` - 启用标志位组合
    pub fn enable_sensors(&mut self, enable_flags: u8) -> Result<()> {
        self.write_register(QMI8658Register::Ctrl7, enable_flags & 0x0F)
    }

    /// 读取加速度计数据
    /// 
    /// # 返回
    /// 
    /// 返回(x, y, z)轴的加速度值
    pub fn read_accel(&mut self) -> Result<(f32, f32, f32)> {
        let mut buffer = [0u8; 6];
        self.read_register(QMI8658Register::AxL, &mut buffer)?;

        let raw_x = i16::from_le_bytes([buffer[0], buffer[1]]);
        let raw_y = i16::from_le_bytes([buffer[2], buffer[3]]);
        let raw_z = i16::from_le_bytes([buffer[4], buffer[5]]);

        let (x, y, z) = if self.accel_unit_mps2 {
            (
                (raw_x as f32 * ONE_G) / self.accel_lsb_div as f32,
                (raw_y as f32 * ONE_G) / self.accel_lsb_div as f32,
                (raw_z as f32 * ONE_G) / self.accel_lsb_div as f32,
            )
        } else {
            (
                (raw_x as f32 * 1000.0) / self.accel_lsb_div as f32,
                (raw_y as f32 * 1000.0) / self.accel_lsb_div as f32,
                (raw_z as f32 * 1000.0) / self.accel_lsb_div as f32,
            )
        };

        Ok((x, y, z))
    }

    /// 读取陀螺仪数据
    /// 
    /// # 返回
    /// 
    /// 返回(x, y, z)轴的角速度值
    pub fn read_gyro(&mut self) -> Result<(f32, f32, f32)> {
        let mut buffer = [0u8; 6];
        self.read_register(QMI8658Register::GxL, &mut buffer)?;

        let raw_x = i16::from_le_bytes([buffer[0], buffer[1]]);
        let raw_y = i16::from_le_bytes([buffer[2], buffer[3]]);
        let raw_z = i16::from_le_bytes([buffer[4], buffer[5]]);

        let (x, y, z) = if self.gyro_unit_rads {
            (
                (raw_x as f32 * M_PI / 180.0) / self.gyro_lsb_div as f32,
                (raw_y as f32 * M_PI / 180.0) / self.gyro_lsb_div as f32,
                (raw_z as f32 * M_PI / 180.0) / self.gyro_lsb_div as f32,
            )
        } else {
            (
                raw_x as f32 / self.gyro_lsb_div as f32,
                raw_y as f32 / self.gyro_lsb_div as f32,
                raw_z as f32 / self.gyro_lsb_div as f32,
            )
        };

        Ok((x, y, z))
    }

    /// 读取温度数据
    /// 
    /// # 返回
    /// 
    /// 返回芯片温度值(°C)
    pub fn read_temperature(&mut self) -> Result<f32> {
        let mut buffer = [0u8; 2];
        self.read_register(QMI8658Register::TempL, &mut buffer)?;

        let raw_temp = i16::from_le_bytes([buffer[0], buffer[1]]);
        Ok(raw_temp as f32 / 256.0)
    }

    /// 读取完整传感器数据
    /// 
    /// # 返回
    /// 
    /// 返回包含加速度计、陀螺仪、温度和时间戳的完整数据
    pub fn read_sensor_data(&mut self) -> Result<SensorData> {
        let mut timestamp_buffer = [0u8; 3];
        if self
            .read_register(QMI8658Register::TimestampL, &mut timestamp_buffer)
            .is_ok()
        {
            let timestamp = ((timestamp_buffer[2] as u32) << 16)
                | ((timestamp_buffer[1] as u32) << 8)
                | (timestamp_buffer[0] as u32);

            if timestamp > self.timestamp {
                self.timestamp = timestamp;
            } else {
                self.timestamp = timestamp + 0x1000000 - self.timestamp;
            }
        }

        let mut sensor_buffer = [0u8; 12];
        self.read_register(QMI8658Register::AxL, &mut sensor_buffer)?;

        let raw_ax = i16::from_le_bytes([sensor_buffer[0], sensor_buffer[1]]);
        let raw_ay = i16::from_le_bytes([sensor_buffer[2], sensor_buffer[3]]);
        let raw_az = i16::from_le_bytes([sensor_buffer[4], sensor_buffer[5]]);

        let raw_gx = i16::from_le_bytes([sensor_buffer[6], sensor_buffer[7]]);
        let raw_gy = i16::from_le_bytes([sensor_buffer[8], sensor_buffer[9]]);
        let raw_gz = i16::from_le_bytes([sensor_buffer[10], sensor_buffer[11]]);

        let (accel_x, accel_y, accel_z) = if self.accel_unit_mps2 {
            (
                (raw_ax as f32 * ONE_G) / self.accel_lsb_div as f32,
                (raw_ay as f32 * ONE_G) / self.accel_lsb_div as f32,
                (raw_az as f32 * ONE_G) / self.accel_lsb_div as f32,
            )
        } else {
            (
                (raw_ax as f32 * 1000.0) / self.accel_lsb_div as f32,
                (raw_ay as f32 * 1000.0) / self.accel_lsb_div as f32,
                (raw_az as f32 * 1000.0) / self.accel_lsb_div as f32,
            )
        };

        let (gyro_x, gyro_y, gyro_z) = if self.gyro_unit_rads {
            (
                (raw_gx as f32 * M_PI / 180.0) / self.gyro_lsb_div as f32,
                (raw_gy as f32 * M_PI / 180.0) / self.gyro_lsb_div as f32,
                (raw_gz as f32 * M_PI / 180.0) / self.gyro_lsb_div as f32,
            )
        } else {
            (
                raw_gx as f32 / self.gyro_lsb_div as f32,
                raw_gy as f32 / self.gyro_lsb_div as f32,
                raw_gz as f32 / self.gyro_lsb_div as f32,
            )
        };

        let temperature = self.read_temperature()?;

        Ok(SensorData {
            accel_x,
            accel_y,
            accel_z,
            gyro_x,
            gyro_y,
            gyro_z,
            temperature,
            timestamp: self.timestamp,
        })
    }

    /// 检查数据是否准备就绪
    /// 
    /// # 返回
    /// 
    /// 返回true表示有新数据可读取
    pub fn is_data_ready(&mut self) -> Result<bool> {
        let mut status = [0u8; 1];
        self.read_register(QMI8658Register::Status0, &mut status)?;
        Ok((status[0] & 0x03) != 0)
    }

    /// 重置传感器
    /// 
    /// 执行软件重置操作
    pub fn reset(&mut self) -> Result<()> {
        self.write_register(QMI8658Register::Ctrl1, 0x80)
    }

    /// 设置加速度计单位为m/s²
    /// 
    /// # 参数
    /// 
    /// * `use_mps2` - true使用m/s²，false使用mg
    pub fn set_accel_unit_mps2(&mut self, use_mps2: bool) {
        self.accel_unit_mps2 = use_mps2;
    }

    /// 设置加速度计单位为mg
    /// 
    /// # 参数
    /// 
    /// * `use_mg` - true使用mg，false使用m/s²
    pub fn set_accel_unit_mg(&mut self, use_mg: bool) {
        self.accel_unit_mps2 = !use_mg;
    }

    /// 设置陀螺仪单位为弧度/秒
    /// 
    /// # 参数
    /// 
    /// * `use_rads` - true使用rad/s，false使用dps
    pub fn set_gyro_unit_rads(&mut self, use_rads: bool) {
        self.gyro_unit_rads = use_rads;
    }

    /// 设置陀螺仪单位为度/秒
    /// 
    /// # 参数
    /// 
    /// * `use_dps` - true使用dps，false使用rad/s
    pub fn set_gyro_unit_dps(&mut self, use_dps: bool) {
        self.gyro_unit_rads = !use_dps;
    }

    /// 设置显示精度
    /// 
    /// # 参数
    /// 
    /// * `decimals` - 小数位数(0-10)
    pub fn set_display_precision(&mut self, decimals: i32) {
        if decimals >= 0 && decimals <= 10 {
            self.display_precision = decimals;
        }
    }

    /// 使用枚举设置显示精度
    /// 
    /// # 参数
    /// 
    /// * `precision` - 精度枚举值
    pub fn set_display_precision_enum(&mut self, precision: Precision) {
        self.display_precision = precision as i32;
    }

    /// 获取当前显示精度
    /// 
    /// # 返回
    /// 
    /// 返回当前设置的小数位数
    pub fn get_display_precision(&self) -> i32 {
        self.display_precision
    }

    /// 检查加速度计单位是否为m/s²
    /// 
    /// # 返回
    /// 
    /// 返回true表示使用m/s²单位
    pub fn is_accel_unit_mps2(&self) -> bool {
        self.accel_unit_mps2
    }

    /// 检查加速度计单位是否为mg
    /// 
    /// # 返回
    /// 
    /// 返回true表示使用mg单位
    pub fn is_accel_unit_mg(&self) -> bool {
        !self.accel_unit_mps2
    }

    /// 检查陀螺仪单位是否为弧度/秒
    /// 
    /// # 返回
    /// 
    /// 返回true表示使用rad/s单位
    pub fn is_gyro_unit_rads(&self) -> bool {
        self.gyro_unit_rads
    }

    /// 检查陀螺仪单位是否为度/秒
    /// 
    /// # 返回
    /// 
    /// 返回true表示使用dps单位
    pub fn is_gyro_unit_dps(&self) -> bool {
        !self.gyro_unit_rads
    }

    /// 启用运动唤醒功能
    /// 
    /// # 参数
    /// 
    /// * `threshold` - 运动检测阈值
    pub fn enable_wake_on_motion(&mut self, threshold: u8) -> Result<()> {
        self.enable_sensors(QMI8658_DISABLE_ALL)?;
        self.set_accel_range(AccelRange::Range2G)?;
        self.set_accel_odr(AccelODR::ODRLowPower21Hz)?;
        self.write_register(QMI8658Register::Ctrl1, threshold)?;
        self.write_register(QMI8658Register::Ctrl2, 0x00)?;
        self.enable_sensors(QMI8658_ENABLE_ACCEL)
    }

    /// 禁用运动唤醒功能
    pub fn disable_wake_on_motion(&mut self) -> Result<()> {
        self.enable_sensors(QMI8658_DISABLE_ALL)?;
        self.write_register(QMI8658Register::Ctrl1, 0x00)
    }

    /// 以mg单位读取加速度计数据
    /// 
    /// # 返回
    /// 
    /// 返回(x, y, z)轴的加速度值(mg)
    pub fn read_accel_mg(&mut self) -> Result<(f32, f32, f32)> {
        let old_unit = self.accel_unit_mps2;
        self.accel_unit_mps2 = false;
        let result = self.read_accel();
        self.accel_unit_mps2 = old_unit;
        result
    }

    /// 以m/s²单位读取加速度计数据
    /// 
    /// # 返回
    /// 
    /// 返回(x, y, z)轴的加速度值(m/s²)
    pub fn read_accel_mps2(&mut self) -> Result<(f32, f32, f32)> {
        let old_unit = self.accel_unit_mps2;
        self.accel_unit_mps2 = true;
        let result = self.read_accel();
        self.accel_unit_mps2 = old_unit;
        result
    }

    /// 以度/秒单位读取陀螺仪数据
    /// 
    /// # 返回
    /// 
    /// 返回(x, y, z)轴的角速度值(dps)
    pub fn read_gyro_dps(&mut self) -> Result<(f32, f32, f32)> {
        let old_unit = self.gyro_unit_rads;
        self.gyro_unit_rads = false;
        let result = self.read_gyro();
        self.gyro_unit_rads = old_unit;
        result
    }

    /// 以弧度/秒单位读取陀螺仪数据
    /// 
    /// # 返回
    /// 
    /// 返回(x, y, z)轴的角速度值(rad/s)
    pub fn read_gyro_rads(&mut self) -> Result<(f32, f32, f32)> {
        let old_unit = self.gyro_unit_rads;
        self.gyro_unit_rads = true;
        let result = self.read_gyro();
        self.gyro_unit_rads = old_unit;
        result
    }
}
