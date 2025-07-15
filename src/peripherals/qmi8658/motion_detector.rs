use super::driver::SensorData;
use std::f32::consts::PI;

#[derive(Debug, Clone, Copy)]
pub enum MotionState {
    Still,   // 静止
    Shaking, // 晃动
    Tilting, // 倾斜
}

#[derive(Debug, Clone, Copy)]
pub struct MotionDetector {
    pub accel_threshold: f32, // 加速度变化阈值 (mg)
    pub gyro_threshold: f32,  // 陀螺仪阈值 (°/s)
    pub gravity_nominal: f32, // 标准重力值 (mg)
    pub tilt_threshold: f32,  // 倾斜角度阈值 (度)
    prev_accel_magnitude: f32,
    shake_count: u32,
    stable_count: u32,
}

impl MotionDetector {
    pub fn new() -> Self {
        Self {
            accel_threshold: 800.0,  // 加速度变化阈值 800mg (大幅摇动)
            gyro_threshold: 120.0,   // 陀螺仪阈值 120°/s (大幅摇动)
            gravity_nominal: 1000.0, // 标准重力 1000mg
            tilt_threshold: 45.0,    // 倾斜角度阈值 45°
            prev_accel_magnitude: 0.0,
            shake_count: 0,
            stable_count: 0,
        }
    }

    pub fn detect_motion(&mut self, data: &SensorData) -> MotionState {
        // 计算加速度矢量大小
        let accel_magnitude =
            (data.accel_x.powi(2) + data.accel_y.powi(2) + data.accel_z.powi(2)).sqrt();

        // 计算陀螺仪矢量大小
        let gyro_magnitude =
            (data.gyro_x.powi(2) + data.gyro_y.powi(2) + data.gyro_z.powi(2)).sqrt();

        // 检测晃动：加速度变化大或陀螺仪值高
        let accel_change = if self.prev_accel_magnitude > 0.0 {
            (accel_magnitude - self.prev_accel_magnitude).abs()
        } else {
            0.0
        };

        // 只有加速度变化和陀螺仪值都高时才认为是晃动，避免单纯翻转被误判
        let is_shaking =
            accel_change > self.accel_threshold && gyro_magnitude > self.gyro_threshold;

        // 检测倾斜：重力矢量偏离垂直方向
        let tilt_angle = Self::calculate_tilt_angle(data.accel_x, data.accel_y, data.accel_z);
        let is_tilting = tilt_angle > self.tilt_threshold;

        self.prev_accel_magnitude = accel_magnitude;

        // 状态机逻辑：需要连续检测来避免噪声
        if is_shaking {
            self.shake_count += 1;
            self.stable_count = 0;
            if self.shake_count >= 12 {
                // 连续12次检测到晃动 (大幅重复摇动)
                return MotionState::Shaking;
            }
        } else {
            self.stable_count += 1;
            if self.stable_count >= 10 {
                // 连续10次稳定后重置 (提高要求)
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

    pub fn set_tilt_threshold(&mut self, tilt_threshold: f32) {
        self.tilt_threshold = tilt_threshold;
    }

    pub fn reset(&mut self) {
        self.prev_accel_magnitude = 0.0;
        self.shake_count = 0;
        self.stable_count = 0;
    }

    pub fn is_shaking(&mut self, data: &SensorData) -> bool {
        matches!(self.detect_motion(data), MotionState::Shaking)
    }

    pub fn is_tilting(&mut self, data: &SensorData) -> bool {
        matches!(self.detect_motion(data), MotionState::Tilting)
    }

    pub fn is_still(&mut self, data: &SensorData) -> bool {
        matches!(self.detect_motion(data), MotionState::Still)
    }

    pub fn get_shake_count(&self) -> u32 {
        self.shake_count
    }

    pub fn get_stable_count(&self) -> u32 {
        self.stable_count
    }

    pub fn get_prev_accel_magnitude(&self) -> f32 {
        self.prev_accel_magnitude
    }
}

impl Default for MotionDetector {
    fn default() -> Self {
        Self::new()
    }
}
