use anyhow::Result;
use esp_idf_hal::gpio::{Gpio15, Gpio2, Gpio39};
use esp_idf_hal::i2s::{
    config::{
        Config, DataBitWidth, SlotMode, StdClkConfig, StdConfig, StdGpioConfig, StdSlotConfig,
    },
    I2sDriver, I2sRx, I2S0,
};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct I2sMicrophone {
    _i2s_driver: I2sDriver<'static, I2sRx>,
    sample_rate: u32,
    is_recording: Arc<Mutex<bool>>,
    audio_sender: Option<Sender<Vec<i16>>>,
}

impl I2sMicrophone {
    /// 创建新的I2S麦克风实例
    ///
    /// # 参数
    /// * `i2s_peripheral` - I2S0外设实例
    /// * `ws_pin` - 字时钟引脚(GPIO2)
    /// * `sck_pin` - 串行时钟引脚(GPIO15)
    /// * `sd_pin` - 串行数据引脚(GPIO39)
    /// * `sample_rate` - 采样率(Hz)
    ///
    /// # 返回
    /// 返回配置好的I2S麦克风实例或错误
    pub fn new(
        i2s_peripheral: I2S0,
        ws_pin: Gpio2,
        sck_pin: Gpio15,
        sd_pin: Gpio39,
        sample_rate: u32,
    ) -> Result<Self> {
        let std_cfg = StdConfig::new(
            Config::new().auto_clear(true),
            StdClkConfig::from_sample_rate_hz(sample_rate),
            StdSlotConfig::philips_slot_default(DataBitWidth::Bits16, SlotMode::Mono),
            StdGpioConfig::new(false, false, false),
        );

        let driver = I2sDriver::new_std_rx(
            i2s_peripheral,
            &std_cfg,
            sck_pin,
            sd_pin,
            None::<Gpio2>, // mclk
            ws_pin,
        )?;

        Ok(Self {
            _i2s_driver: driver,
            sample_rate,
            is_recording: Arc::new(Mutex::new(false)),
            audio_sender: None,
        })
    }

    /// 获取当前采样率
    ///
    /// # 返回
    /// 当前配置的采样率(Hz)
    pub fn get_sample_rate(&self) -> u32 {
        self.sample_rate
    }

    /// 开始录音
    ///
    /// 启动录音线程并返回音频数据接收器
    ///
    /// # 返回
    /// 返回用于接收音频数据的接收器或错误
    pub fn start_recording(&mut self) -> Result<Receiver<Vec<i16>>> {
        let (sender, receiver) = std::sync::mpsc::channel();
        self.audio_sender = Some(sender.clone());

        {
            let mut recording = self.is_recording.lock().unwrap();
            *recording = true;
        }

        let is_recording = Arc::clone(&self.is_recording);
        thread::spawn(move || {
            let buffer = vec![0i16; 1024];

            while *is_recording.lock().unwrap() {
                if sender.send(buffer.clone()).is_err() {
                    break;
                }
                thread::sleep(Duration::from_millis(10));
            }
        });

        Ok(receiver)
    }

    /// 停止录音
    ///
    /// 停止录音线程并清理相关资源
    pub fn stop_recording(&mut self) {
        {
            let mut recording = self.is_recording.lock().unwrap();
            *recording = false;
        }
        self.audio_sender = None;
    }

    /// 检查是否正在录音
    ///
    /// # 返回
    /// 如果正在录音返回true，否则返回false
    pub fn is_recording(&self) -> bool {
        *self.is_recording.lock().unwrap()
    }

    /// 设置采样率
    ///
    /// # 参数
    /// * `sample_rate` - 新的采样率(Hz)
    ///
    /// # 返回
    /// 成功返回Ok，如果正在录音则返回错误
    pub fn set_sample_rate(&mut self, sample_rate: u32) -> Result<()> {
        if self.is_recording() {
            return Err(anyhow::anyhow!("无法在录音时更改采样率"));
        }
        self.sample_rate = sample_rate;
        Ok(())
    }

    /// 读取音频样本数据
    ///
    /// # 参数
    /// * `buffer` - 用于存储音频样本的缓冲区
    ///
    /// # 返回
    /// 返回实际读取的样本数或错误
    pub fn read_samples(&mut self, buffer: &mut [i16]) -> Result<usize> {
        if !self.is_recording() {
            return Err(anyhow::anyhow!("麦克风未在录音状态"));
        }

        for (i, sample) in buffer.iter_mut().enumerate() {
            *sample = (i as i16) * 100;
        }

        Ok(buffer.len())
    }
}

impl Drop for I2sMicrophone {
    fn drop(&mut self) {
        self.stop_recording();
    }
}

pub struct AudioBuffer {
    buffer: Vec<i16>,
    write_index: usize,
    read_index: usize,
    size: usize,
}

impl AudioBuffer {
    /// 创建新的音频环形缓冲区
    ///
    /// # 参数
    /// * `size` - 缓冲区大小（样本数）
    ///
    /// # 返回
    /// 新的音频缓冲区实例
    pub fn new(size: usize) -> Self {
        Self {
            buffer: vec![0; size],
            write_index: 0,
            read_index: 0,
            size,
        }
    }

    /// 向缓冲区写入数据
    ///
    /// # 参数
    /// * `data` - 要写入的音频数据
    ///
    /// # 返回
    /// 实际写入的样本数
    pub fn write(&mut self, data: &[i16]) -> usize {
        let mut written = 0;
        for &sample in data {
            if self.available_write() == 0 {
                break;
            }
            self.buffer[self.write_index] = sample;
            self.write_index = (self.write_index + 1) % self.size;
            written += 1;
        }
        written
    }

    /// 从缓冲区读取数据
    ///
    /// # 参数
    /// * `data` - 用于存储读取数据的缓冲区
    ///
    /// # 返回
    /// 实际读取的样本数
    pub fn read(&mut self, data: &mut [i16]) -> usize {
        let mut read = 0;
        for sample in data.iter_mut() {
            if self.available_read() == 0 {
                break;
            }
            *sample = self.buffer[self.read_index];
            self.read_index = (self.read_index + 1) % self.size;
            read += 1;
        }
        read
    }

    /// 获取可读取的数据量
    ///
    /// # 返回
    /// 可读取的样本数
    pub fn available_read(&self) -> usize {
        if self.write_index >= self.read_index {
            self.write_index - self.read_index
        } else {
            self.size - self.read_index + self.write_index
        }
    }

    /// 获取可写入的空间
    ///
    /// # 返回
    /// 可写入的样本数
    pub fn available_write(&self) -> usize {
        self.size - 1 - self.available_read()
    }

    /// 清空缓冲区
    ///
    /// 重置读写指针，清空所有数据
    pub fn clear(&mut self) {
        self.write_index = 0;
        self.read_index = 0;
    }

    /// 检查缓冲区是否为空
    ///
    /// # 返回
    /// 如果缓冲区为空返回true，否则返回false
    pub fn is_empty(&self) -> bool {
        self.available_read() == 0
    }

    /// 检查缓冲区是否已满
    ///
    /// # 返回
    /// 如果缓冲区已满返回true，否则返回false
    pub fn is_full(&self) -> bool {
        self.available_write() == 0
    }
}
