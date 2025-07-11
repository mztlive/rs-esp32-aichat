#[allow(dead_code)]
use anyhow::Result;
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::{Gpio10, Gpio11, PinDriver};
use esp_idf_hal::i2c::{I2cConfig, I2cDriver, I2C0};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::prelude::*;
use log::{error, info};
use std::f32::consts::PI;

pub const QMI8658_ADDRESS_LOW: u8 = 0x6A;
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
    ODR8000Hz = 0x00,
    ODR4000Hz = 0x01,
    ODR2000Hz = 0x02,
    ODR1000Hz = 0x03,
    ODR500Hz = 0x04,
    ODR250Hz = 0x05,
    ODR125Hz = 0x06,
    ODR62_5Hz = 0x07,
    ODR31_25Hz = 0x08,
    ODRLowPower128Hz = 0x0C,
    ODRLowPower21Hz = 0x0D,
    ODRLowPower11Hz = 0x0E,
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
    ODR8000Hz = 0x00,
    ODR4000Hz = 0x01,
    ODR2000Hz = 0x02,
    ODR1000Hz = 0x03,
    ODR500Hz = 0x04,
    ODR250Hz = 0x05,
    ODR125Hz = 0x06,
    ODR62_5Hz = 0x07,
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

#[derive(Debug, Clone, Copy)]
pub enum MotionState {
    Still,      // 静止
    Shaking,    // 晃动
    Tilting,    // 倾斜
}

#[derive(Debug, Clone, Copy)]
pub struct MotionDetector {
    pub accel_threshold: f32,  // 加速度变化阈值 (mg)
    pub gyro_threshold: f32,   // 陀螺仪阈值 (°/s)
    pub gravity_nominal: f32,  // 标准重力值 (mg)
    pub tilt_threshold: f32,   // 倾斜角度阈值 (度)
    prev_accel_magnitude: f32,
    shake_count: u32,
    stable_count: u32,
}

pub struct QMI8658<'a> {
    i2c: I2cDriver<'a>,
    address: u8,
    accel_lsb_div: u16,
    gyro_lsb_div: u16,
    accel_unit_mps2: bool,
    gyro_unit_rads: bool,
    display_precision: i32,
    timestamp: u32,
    motion_detector: MotionDetector,
}

impl<'a> QMI8658<'a> {
    pub fn new(i2c0: I2C0, sda: Gpio11, scl: Gpio10, address: u8) -> Result<Self> {
        let config = I2cConfig::new().baudrate(400.kHz().into());
        let i2c = I2cDriver::new(i2c0, sda, scl, &config)?;

        let mut qmi8658 = QMI8658 {
            i2c,
            address,
            accel_lsb_div: 4096,
            gyro_lsb_div: 64,
            accel_unit_mps2: false,
            gyro_unit_rads: false,
            display_precision: 6,
            timestamp: 0,
            motion_detector: MotionDetector::new(),
        };

        // I2C scan to find devices
        println!("Scanning I2C bus for devices...");
        for addr in 0x08..=0x77 {
            let result = qmi8658.i2c.write(addr, &[0x00], 100);
            match result {
                Ok(_) => println!("Found device at address 0x{:02X}", addr),
                Err(_) => {} // Device not found
            }
        }

        qmi8658.init()?;
        Ok(qmi8658)
    }

    fn init(&mut self) -> Result<()> {
        let who_am_i = self.get_who_am_i()?;
        println!("QMI8658 WHO_AM_I: 0x{:02X}", who_am_i);
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

    fn write_register(&mut self, reg: QMI8658Register, value: u8) -> Result<()> {
        let data = [reg as u8, value];
        self.i2c.write(self.address, &data, 1000)?;
        Ok(())
    }

    fn read_register(&mut self, reg: QMI8658Register, buffer: &mut [u8]) -> Result<()> {
        let reg_addr = [reg as u8];
        self.i2c.write_read(self.address, &reg_addr, buffer, 1000)?;
        Ok(())
    }

    pub fn get_who_am_i(&mut self) -> Result<u8> {
        let mut buffer = [0u8; 1];
        println!("Reading WHO_AM_I from address 0x{:02X}...", self.address);
        match self.read_register(QMI8658Register::WhoAmI, &mut buffer) {
            Ok(_) => {
                println!("WHO_AM_I read successfully: 0x{:02X}", buffer[0]);
                Ok(buffer[0])
            }
            Err(e) => {
                println!("Failed to read WHO_AM_I: {:?}", e);
                Err(e)
            }
        }
    }

    pub fn set_accel_range(&mut self, range: AccelRange) -> Result<()> {
        self.accel_lsb_div = match range {
            AccelRange::Range2G => 16384,
            AccelRange::Range4G => 8192,
            AccelRange::Range8G => 4096,
            AccelRange::Range16G => 2048,
        };

        self.write_register(QMI8658Register::Ctrl2, ((range as u8) << 4) | 0x03)
    }

    pub fn set_accel_odr(&mut self, odr: AccelODR) -> Result<()> {
        let mut current_ctrl2 = [0u8; 1];
        self.read_register(QMI8658Register::Ctrl2, &mut current_ctrl2)?;

        let new_ctrl2 = (current_ctrl2[0] & 0xF0) | (odr as u8);
        self.write_register(QMI8658Register::Ctrl2, new_ctrl2)
    }

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

    pub fn set_gyro_odr(&mut self, odr: GyroODR) -> Result<()> {
        let mut current_ctrl3 = [0u8; 1];
        self.read_register(QMI8658Register::Ctrl3, &mut current_ctrl3)?;

        let new_ctrl3 = (current_ctrl3[0] & 0xF0) | (odr as u8);
        self.write_register(QMI8658Register::Ctrl3, new_ctrl3)
    }

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

    pub fn enable_sensors(&mut self, enable_flags: u8) -> Result<()> {
        self.write_register(QMI8658Register::Ctrl7, enable_flags & 0x0F)
    }

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

    pub fn read_temperature(&mut self) -> Result<f32> {
        let mut buffer = [0u8; 2];
        self.read_register(QMI8658Register::TempL, &mut buffer)?;

        let raw_temp = i16::from_le_bytes([buffer[0], buffer[1]]);
        Ok(raw_temp as f32 / 256.0)
    }

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

    pub fn is_data_ready(&mut self) -> Result<bool> {
        let mut status = [0u8; 1];
        self.read_register(QMI8658Register::Status0, &mut status)?;
        Ok((status[0] & 0x03) != 0)
    }

    pub fn reset(&mut self) -> Result<()> {
        self.write_register(QMI8658Register::Ctrl1, 0x80)
    }

    pub fn set_accel_unit_mps2(&mut self, use_mps2: bool) {
        self.accel_unit_mps2 = use_mps2;
    }

    pub fn set_accel_unit_mg(&mut self, use_mg: bool) {
        self.accel_unit_mps2 = !use_mg;
    }

    pub fn set_gyro_unit_rads(&mut self, use_rads: bool) {
        self.gyro_unit_rads = use_rads;
    }

    pub fn set_gyro_unit_dps(&mut self, use_dps: bool) {
        self.gyro_unit_rads = !use_dps;
    }

    pub fn set_display_precision(&mut self, decimals: i32) {
        if decimals >= 0 && decimals <= 10 {
            self.display_precision = decimals;
        }
    }

    pub fn set_display_precision_enum(&mut self, precision: Precision) {
        self.display_precision = precision as i32;
    }

    pub fn get_display_precision(&self) -> i32 {
        self.display_precision
    }

    pub fn is_accel_unit_mps2(&self) -> bool {
        self.accel_unit_mps2
    }

    pub fn is_accel_unit_mg(&self) -> bool {
        !self.accel_unit_mps2
    }

    pub fn is_gyro_unit_rads(&self) -> bool {
        self.gyro_unit_rads
    }

    pub fn is_gyro_unit_dps(&self) -> bool {
        !self.gyro_unit_rads
    }

    pub fn enable_wake_on_motion(&mut self, threshold: u8) -> Result<()> {
        self.enable_sensors(QMI8658_DISABLE_ALL)?;
        self.set_accel_range(AccelRange::Range2G)?;
        self.set_accel_odr(AccelODR::ODRLowPower21Hz)?;
        self.write_register(QMI8658Register::Ctrl1, threshold)?;
        self.write_register(QMI8658Register::Ctrl2, 0x00)?;
        self.enable_sensors(QMI8658_ENABLE_ACCEL)
    }

    pub fn disable_wake_on_motion(&mut self) -> Result<()> {
        self.enable_sensors(QMI8658_DISABLE_ALL)?;
        self.write_register(QMI8658Register::Ctrl1, 0x00)
    }

    pub fn read_accel_mg(&mut self) -> Result<(f32, f32, f32)> {
        let old_unit = self.accel_unit_mps2;
        self.accel_unit_mps2 = false;
        let result = self.read_accel();
        self.accel_unit_mps2 = old_unit;
        result
    }

    pub fn read_accel_mps2(&mut self) -> Result<(f32, f32, f32)> {
        let old_unit = self.accel_unit_mps2;
        self.accel_unit_mps2 = true;
        let result = self.read_accel();
        self.accel_unit_mps2 = old_unit;
        result
    }

    pub fn read_gyro_dps(&mut self) -> Result<(f32, f32, f32)> {
        let old_unit = self.gyro_unit_rads;
        self.gyro_unit_rads = false;
        let result = self.read_gyro();
        self.gyro_unit_rads = old_unit;
        result
    }

    pub fn read_gyro_rads(&mut self) -> Result<(f32, f32, f32)> {
        let old_unit = self.gyro_unit_rads;
        self.gyro_unit_rads = true;
        let result = self.read_gyro();
        self.gyro_unit_rads = old_unit;
        result
    }

    pub fn detect_motion(&mut self, sensor_data: &SensorData) -> MotionState {
        self.motion_detector.detect_motion(sensor_data)
    }

    pub fn is_shaking(&mut self, sensor_data: &SensorData) -> bool {
        matches!(self.detect_motion(sensor_data), MotionState::Shaking)
    }
}

impl MotionDetector {
    pub fn new() -> Self {
        Self {
            accel_threshold: 300.0,    // 加速度变化阈值 300mg (提高阈值)
            gyro_threshold: 50.0,      // 陀螺仪阈值 50°/s (提高阈值)
            gravity_nominal: 1000.0,   // 标准重力 1000mg
            tilt_threshold: 45.0,      // 倾斜角度阈值 45°
            prev_accel_magnitude: 0.0,
            shake_count: 0,
            stable_count: 0,
        }
    }

    pub fn detect_motion(&mut self, data: &SensorData) -> MotionState {
        // 计算加速度矢量大小
        let accel_magnitude = (data.accel_x.powi(2) + data.accel_y.powi(2) + data.accel_z.powi(2)).sqrt();
        
        // 计算陀螺仪矢量大小
        let gyro_magnitude = (data.gyro_x.powi(2) + data.gyro_y.powi(2) + data.gyro_z.powi(2)).sqrt();
        
        // 检测晃动：加速度变化大或陀螺仪值高
        let accel_change = if self.prev_accel_magnitude > 0.0 {
            (accel_magnitude - self.prev_accel_magnitude).abs()
        } else {
            0.0
        };
        
        let is_shaking = accel_change > self.accel_threshold || gyro_magnitude > self.gyro_threshold;
        
        // 检测倾斜：重力矢量偏离垂直方向
        let tilt_angle = Self::calculate_tilt_angle(data.accel_x, data.accel_y, data.accel_z);
        let is_tilting = tilt_angle > self.tilt_threshold;
        
        self.prev_accel_magnitude = accel_magnitude;
        
        // 状态机逻辑：需要连续检测来避免噪声
        if is_shaking {
            self.shake_count += 1;
            self.stable_count = 0;
            if self.shake_count >= 5 {  // 连续5次检测到晃动 (提高要求)
                return MotionState::Shaking;
            }
        } else {
            self.stable_count += 1;
            if self.stable_count >= 10 {  // 连续10次稳定后重置 (提高要求)
                self.shake_count = 0;
            }
        }
        
        if is_tilting {
            MotionState::Tilting
        } else {
            MotionState::Still
        }
    }
    
    fn calculate_tilt_angle(ax: f32, ay: f32, az: f32) -> f32 {
        // 计算与垂直方向的夹角
        let magnitude = (ax.powi(2) + ay.powi(2) + az.powi(2)).sqrt();
        if magnitude > 0.0 {
            let cos_angle = az.abs() / magnitude;
            let angle_rad = cos_angle.acos();
            angle_rad * 180.0 / PI
        } else {
            0.0
        }
    }
    
    pub fn set_thresholds(&mut self, accel_threshold: f32, gyro_threshold: f32) {
        self.accel_threshold = accel_threshold;
        self.gyro_threshold = gyro_threshold;
    }
}
