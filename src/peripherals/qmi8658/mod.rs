pub mod driver;
pub mod motion_detector;

pub use driver::*;
pub use motion_detector::*;

use anyhow::Result;
use driver::{QMI8658Driver, SensorData};
use motion_detector::{MotionDetector, MotionState};

#[derive(Debug)]
pub struct QMI8658<'a> {
    driver: QMI8658Driver<'a>,
}

impl<'a> QMI8658<'a> {
    pub fn new(
        i2c0: esp_idf_hal::i2c::I2C0,
        sda: esp_idf_hal::gpio::Gpio11,
        scl: esp_idf_hal::gpio::Gpio10,
        address: u8,
    ) -> Result<Self> {
        let driver = QMI8658Driver::new(i2c0, sda, scl, address)?;

        Ok(Self { driver })
    }

    pub fn read_sensor_data(&mut self) -> Result<SensorData> {
        self.driver.read_sensor_data()
    }

    pub fn driver(&mut self) -> &mut QMI8658Driver<'a> {
        &mut self.driver
    }
}
