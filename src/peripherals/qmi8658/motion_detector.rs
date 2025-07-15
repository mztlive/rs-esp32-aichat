use super::driver::SensorData;
use anyhow::{bail, Result};
use std::f32::consts::PI;

/// 运动状态枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MotionState {
    Still,   // 静止
    Shaking, // 晃动
    Tilting, // 倾斜
}

/// 运动检测配置常量
pub struct MotionConfig;

impl MotionConfig {
    /// 连续检测到晃动的最小次数（避免噪声干扰）
    pub const SHAKE_COUNT_THRESHOLD: u32 = 12;
    /// 连续稳定状态的次数阈值（重置晃动计数）
    pub const STABLE_COUNT_THRESHOLD: u32 = 10;
    /// 默认加速度变化阈值 (mg) - 大幅摇动检测
    pub const DEFAULT_ACCEL_THRESHOLD: f32 = 800.0;
    /// 默认陀螺仪阈值 (°/s) - 大幅摇动检测
    pub const DEFAULT_GYRO_THRESHOLD: f32 = 120.0;
    /// 标准重力值 (mg)
    pub const GRAVITY_NOMINAL: f32 = 1000.0;
    /// 默认倾斜角度阈值 (度)
    pub const DEFAULT_TILT_THRESHOLD: f32 = 45.0;
    /// 最小有效加速度阈值，避免除零错误
    pub const MIN_VALID_ACCEL_THRESHOLD: f32 = 10.0;
    /// 最大有效倾斜角度
    pub const MAX_TILT_ANGLE: f32 = 90.0;
}

/// 缓存的检测结果，避免重复计算
#[derive(Debug, Clone, Copy)]
struct CachedDetectionResult {
    motion_state: MotionState,
    accel_magnitude: f32,
    gyro_magnitude: f32,
    tilt_angle: f32,
    is_shaking: bool,
    is_tilting: bool,
}

/// 运动检测器主结构体
#[derive(Debug, Clone, Copy)]
pub struct MotionDetector {
    // 公开配置参数
    pub accel_threshold: f32, // 加速度变化阈值 (mg)
    pub gyro_threshold: f32,  // 陀螺仪阈值 (°/s)
    pub tilt_threshold: f32,  // 倾斜角度阈值 (度)

    // 内部状态
    prev_accel_magnitude: f32,
    shake_count: u32,
    stable_count: u32,

    // 缓存结果
    cached_result: Option<CachedDetectionResult>,
    last_sensor_data_hash: u64, // 简单的数据指纹，用于检测数据是否变化
}

impl MotionDetector {
    /// 创建新的运动检测器实例，使用默认配置
    pub fn new() -> Self {
        Self {
            accel_threshold: MotionConfig::DEFAULT_ACCEL_THRESHOLD,
            gyro_threshold: MotionConfig::DEFAULT_GYRO_THRESHOLD,
            tilt_threshold: MotionConfig::DEFAULT_TILT_THRESHOLD,
            prev_accel_magnitude: 0.0,
            shake_count: 0,
            stable_count: 0,
            cached_result: None,
            last_sensor_data_hash: 0,
        }
    }

    /// 使用自定义配置创建运动检测器
    pub fn with_config(
        accel_threshold: f32,
        gyro_threshold: f32,
        tilt_threshold: f32,
    ) -> Result<Self> {
        if accel_threshold < MotionConfig::MIN_VALID_ACCEL_THRESHOLD {
            bail!("加速度阈值过小: {}", accel_threshold);
        }
        if gyro_threshold <= 0.0 {
            bail!("陀螺仪阈值必须大于0: {}", gyro_threshold);
        }
        if tilt_threshold <= 0.0 || tilt_threshold > MotionConfig::MAX_TILT_ANGLE {
            bail!("倾斜角度阈值无效: {}", tilt_threshold);
        }

        Ok(Self {
            accel_threshold,
            gyro_threshold,
            tilt_threshold,
            prev_accel_magnitude: 0.0,
            shake_count: 0,
            stable_count: 0,
            cached_result: None,
            last_sensor_data_hash: 0,
        })
    }

    /// 检测运动状态 - 主要入口函数，包含结果缓存优化
    pub fn detect_motion(&mut self, data: &SensorData) -> MotionState {
        let data_hash = self.calculate_data_hash(data);

        // 如果数据没有变化，直接返回缓存结果
        if let Some(cached) = self.cached_result {
            if data_hash == self.last_sensor_data_hash {
                return cached.motion_state;
            }
        }

        // 计算新的检测结果
        let result = self.calculate_motion_state(data);

        // 缓存结果
        self.cached_result = Some(result);
        self.last_sensor_data_hash = data_hash;

        result.motion_state
    }

    /// 计算传感器数据的简单哈希值（用于检测数据变化）
    fn calculate_data_hash(&self, data: &SensorData) -> u64 {
        // 使用简单的位运算组合数据，足以检测数据变化
        let ax = (data.accel_x * 1000.0) as u32;
        let ay = (data.accel_y * 1000.0) as u32;
        let az = (data.accel_z * 1000.0) as u32;
        let gx = (data.gyro_x * 1000.0) as u32;
        let gy = (data.gyro_y * 1000.0) as u32;
        let gz = (data.gyro_z * 1000.0) as u32;

        ((ax as u64) << 40)
            | ((ay as u64) << 32)
            | ((az as u64) << 24)
            | ((gx as u64) << 16)
            | ((gy as u64) << 8)
            | (gz as u64)
    }

    /// 核心运动状态计算逻辑
    fn calculate_motion_state(&mut self, data: &SensorData) -> CachedDetectionResult {
        // 计算加速度和陀螺仪矢量大小
        let accel_magnitude = Self::calculate_magnitude(data.accel_x, data.accel_y, data.accel_z);
        let gyro_magnitude = Self::calculate_magnitude(data.gyro_x, data.gyro_y, data.gyro_z);

        // 检测晃动：加速度变化大且陀螺仪值高
        let accel_change = if self.prev_accel_magnitude > 0.0 {
            (accel_magnitude - self.prev_accel_magnitude).abs()
        } else {
            0.0
        };

        let is_shaking =
            accel_change > self.accel_threshold && gyro_magnitude > self.gyro_threshold;

        // 检测倾斜：重力矢量偏离垂直方向
        let tilt_angle = Self::calculate_tilt_angle(data.accel_x, data.accel_y, data.accel_z);
        let is_tilting = tilt_angle > self.tilt_threshold;

        // 更新历史状态
        self.prev_accel_magnitude = accel_magnitude;

        // 状态机逻辑：需要连续检测来避免噪声
        let motion_state = self.update_state_machine(is_shaking, is_tilting);

        CachedDetectionResult {
            motion_state,
            accel_magnitude,
            gyro_magnitude,
            tilt_angle,
            is_shaking,
            is_tilting,
        }
    }

    /// 状态机更新逻辑
    fn update_state_machine(&mut self, is_shaking: bool, is_tilting: bool) -> MotionState {
        if is_shaking {
            self.shake_count += 1;
            self.stable_count = 0;
            if self.shake_count >= MotionConfig::SHAKE_COUNT_THRESHOLD {
                return MotionState::Shaking;
            }
        } else {
            self.stable_count += 1;
            if self.stable_count >= MotionConfig::STABLE_COUNT_THRESHOLD {
                self.shake_count = 0;
            }
        }

        if is_tilting {
            MotionState::Tilting
        } else {
            MotionState::Still
        }
    }

    /// 计算3D矢量的大小（优化版本）
    #[inline]
    fn calculate_magnitude(x: f32, y: f32, z: f32) -> f32 {
        (x * x + y * y + z * z).sqrt()
    }

    /// 计算设备相对于垂直方向的倾斜角度
    fn calculate_tilt_angle(ax: f32, ay: f32, az: f32) -> f32 {
        let magnitude = Self::calculate_magnitude(ax, ay, az);
        if magnitude > MotionConfig::MIN_VALID_ACCEL_THRESHOLD {
            let cos_angle = (az.abs() / magnitude).clamp(0.0, 1.0); // 防止数值误差
            let angle_rad = cos_angle.acos();
            (angle_rad * 180.0 / PI).min(MotionConfig::MAX_TILT_ANGLE)
        } else {
            0.0 // 加速度太小时认为无倾斜
        }
    }

    /// 设置加速度和陀螺仪阈值（带验证）
    pub fn set_thresholds(&mut self, accel_threshold: f32, gyro_threshold: f32) -> Result<()> {
        if accel_threshold < MotionConfig::MIN_VALID_ACCEL_THRESHOLD {
            bail!("加速度阈值过小: {}", accel_threshold);
        }
        if gyro_threshold <= 0.0 {
            bail!("陀螺仪阈值必须大于0: {}", gyro_threshold);
        }

        self.accel_threshold = accel_threshold;
        self.gyro_threshold = gyro_threshold;
        self.invalidate_cache();
        Ok(())
    }

    /// 设置倾斜角度阈值（带验证）
    pub fn set_tilt_threshold(&mut self, tilt_threshold: f32) -> Result<()> {
        if tilt_threshold <= 0.0 || tilt_threshold > MotionConfig::MAX_TILT_ANGLE {
            bail!("倾斜角度阈值无效: {}", tilt_threshold);
        }

        self.tilt_threshold = tilt_threshold;
        self.invalidate_cache();
        Ok(())
    }

    /// 清除缓存（当配置改变时）
    fn invalidate_cache(&mut self) {
        self.cached_result = None;
        self.last_sensor_data_hash = 0;
    }

    /// 重置检测器状态
    pub fn reset(&mut self) {
        self.prev_accel_magnitude = 0.0;
        self.shake_count = 0;
        self.stable_count = 0;
        self.invalidate_cache();
    }

    /// 检查是否为晃动状态（优化版，使用缓存）
    pub fn is_shaking(&mut self, data: &SensorData) -> bool {
        self.detect_motion(data) == MotionState::Shaking
    }

    /// 检查是否为倾斜状态（优化版，使用缓存）
    pub fn is_tilting(&mut self, data: &SensorData) -> bool {
        self.detect_motion(data) == MotionState::Tilting
    }

    /// 检查是否为静止状态（优化版，使用缓存）
    pub fn is_still(&mut self, data: &SensorData) -> bool {
        self.detect_motion(data) == MotionState::Still
    }

    /// 获取详细的检测结果（避免重复计算）
    pub fn get_detailed_result(&mut self, data: &SensorData) -> (MotionState, f32, f32, f32) {
        let motion_state = self.detect_motion(data);
        if let Some(cached) = self.cached_result {
            (
                motion_state,
                cached.accel_magnitude,
                cached.gyro_magnitude,
                cached.tilt_angle,
            )
        } else {
            // 这种情况理论上不应该发生，但提供后备方案
            let accel_mag = Self::calculate_magnitude(data.accel_x, data.accel_y, data.accel_z);
            let gyro_mag = Self::calculate_magnitude(data.gyro_x, data.gyro_y, data.gyro_z);
            let tilt = Self::calculate_tilt_angle(data.accel_x, data.accel_y, data.accel_z);
            (motion_state, accel_mag, gyro_mag, tilt)
        }
    }

    /// 获取当前晃动计数
    pub fn get_shake_count(&self) -> u32 {
        self.shake_count
    }

    /// 获取当前稳定计数
    pub fn get_stable_count(&self) -> u32 {
        self.stable_count
    }

    /// 获取上一次的加速度大小
    pub fn get_prev_accel_magnitude(&self) -> f32 {
        self.prev_accel_magnitude
    }

    /// 获取当前配置的阈值
    pub fn get_thresholds(&self) -> (f32, f32, f32) {
        (
            self.accel_threshold,
            self.gyro_threshold,
            self.tilt_threshold,
        )
    }

    /// 检查缓存是否有效
    pub fn has_cached_result(&self) -> bool {
        self.cached_result.is_some()
    }
}

impl Default for MotionDetector {
    fn default() -> Self {
        Self::new()
    }
}
