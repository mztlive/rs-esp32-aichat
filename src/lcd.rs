use anyhow::Result;
use esp_idf_hal::i2c::I2cDriver;
use esp_idf_hal::{gpio::PinDriver, peripherals::Peripherals};
use esp_idf_sys::st77916::{
    esp_lcd_new_panel_st77916, st77916_vendor_config_t, st77916_vendor_config_t__bindgen_ty_1,
};
use esp_idf_sys::*;

pub const LCD_WIDTH: i32 = 360;
pub const LCD_HEIGHT: i32 = 360;

pub struct LcdController {
    panel: esp_lcd_panel_handle_t,
    io_handle: esp_lcd_panel_io_handle_t,
    backlight: PinDriver<'static, esp_idf_hal::gpio::Gpio5, esp_idf_hal::gpio::Output>,
}

impl LcdController {
    pub fn new(peripherals: Peripherals) -> Result<Self> {
        // 1. 初始化 SPI2 总线 (HSPI)
        let io_handle = Self::init_spi_bus()?;

        // 2. 创建 LCD 面板
        let panel = Self::create_panel(io_handle)?;

        // 3. 初始化背光
        let backlight = Self::init_backlight(peripherals)?;

        Ok(LcdController {
            panel,
            io_handle,
            backlight,
        })
    }

    fn init_spi_bus() -> Result<esp_lcd_panel_io_handle_t> {
        unsafe {
            // QSPI 四线 + 时钟 (根据引脚文档修正)
            let bus_cfg = spi_bus_config_t {
                __bindgen_anon_1: spi_bus_config_t__bindgen_ty_1 { mosi_io_num: 46 }, // SDA0
                __bindgen_anon_2: spi_bus_config_t__bindgen_ty_2 { miso_io_num: -1 },
                sclk_io_num: 40, // LCD_SCK
                __bindgen_anon_3: spi_bus_config_t__bindgen_ty_3 { quadwp_io_num: 42 }, // SDA2
                __bindgen_anon_4: spi_bus_config_t__bindgen_ty_4 { quadhd_io_num: 45 }, // SDA1
                data4_io_num: 41, // SDA3
                data5_io_num: -1,
                data6_io_num: -1,
                data7_io_num: -1,
                max_transfer_sz: LCD_WIDTH * 80 * 2,
                flags: SPICOMMON_BUSFLAG_QUAD,
                intr_flags: 0,
                isr_cpu_id: 0,
            };

            esp!(spi_bus_initialize(
                spi_host_device_t_SPI2_HOST as _,
                &bus_cfg,
                spi_common_dma_t_SPI_DMA_CH_AUTO // 使用自动DMA通道
            ))?;
        }

        // 创建 LCD IO（4-线 QSPI）
        let mut io_handle: esp_lcd_panel_io_handle_t = core::ptr::null_mut();
        let io_cfg = esp_lcd_panel_io_spi_config_t {
            cs_gpio_num: 21,
            dc_gpio_num: -1,     // ST77916 QSPI 模式无需 DC
            pclk_hz: 20_000_000, // 降低到 20 MHz 提高稳定性
            spi_mode: 0,
            trans_queue_depth: 10,
            flags: esp_lcd_panel_io_spi_config_t__bindgen_ty_1 {
                _bitfield_align_1: [],
                _bitfield_1: esp_lcd_panel_io_spi_config_t__bindgen_ty_1::new_bitfield_1(
                    0, 0, 0, 0, 1, 0, 0, 0, // quad_mode设为1，其他为0
                ),
                __bindgen_padding_0: [0; 3],
            },
            lcd_cmd_bits: 8,
            lcd_param_bits: 8,
            on_color_trans_done: None,
            user_ctx: core::ptr::null_mut(),
        };

        unsafe {
            esp!(esp_lcd_new_panel_io_spi(
                spi_host_device_t_SPI2_HOST as _,
                &io_cfg,
                &mut io_handle
            ))?;
        }

        Ok(io_handle)
    }

    fn create_panel(io_handle: esp_lcd_panel_io_handle_t) -> Result<esp_lcd_panel_handle_t> {
        let mut panel: esp_lcd_panel_handle_t = core::ptr::null_mut();
        let vendor_cfg = st77916_vendor_config_t {
            flags: st77916_vendor_config_t__bindgen_ty_1 {
                _bitfield_align_1: [],
                _bitfield_1: st77916_vendor_config_t__bindgen_ty_1::new_bitfield_1(1), // use_qspi_interface = 1
                __bindgen_padding_0: [0; 3],
            },
            init_cmds: core::ptr::null(),
            init_cmds_size: 0,
        };
        let panel_cfg = esp_lcd_panel_dev_config_t {
            reset_gpio_num: -1, // 如果你接了 RST，请填实际引脚
            __bindgen_anon_1: esp_lcd_panel_dev_config_t__bindgen_ty_1 {
                rgb_ele_order: lcd_rgb_element_order_t_LCD_RGB_ELEMENT_ORDER_RGB, // 恢复RGB顺序
            },
            data_endian: lcd_rgb_data_endian_t_LCD_RGB_DATA_ENDIAN_LITTLE, // 恢复大端
            bits_per_pixel: 16,
            flags: esp_lcd_panel_dev_config_t__bindgen_ty_2 {
                _bitfield_align_1: [],
                _bitfield_1: Default::default(),
                __bindgen_padding_0: [0; 3],
            },
            vendor_config: &vendor_cfg as *const _ as *mut _,
        };

        unsafe {
            esp!(esp_lcd_new_panel_st77916(
                io_handle as *mut esp_idf_sys::st77916::esp_lcd_panel_io_t,
                &panel_cfg as *const _ as *const esp_idf_sys::st77916::esp_lcd_panel_dev_config_t,
                &mut panel as *mut _ as *mut *mut esp_idf_sys::st77916::esp_lcd_panel_t
            ))?;
            println!("开始面板复位和初始化...");
            esp!(esp_lcd_panel_reset(panel))?;
            println!("面板复位完成");

            // 等待复位稳定
            std::thread::sleep(std::time::Duration::from_millis(200));

            esp!(esp_lcd_panel_init(panel))?;
            println!("面板初始化完成");

            // 等待初始化稳定
            std::thread::sleep(std::time::Duration::from_millis(200));

            // 设置显示方向
            esp!(esp_lcd_panel_mirror(panel, false, false))?;
            esp!(esp_lcd_panel_swap_xy(panel, false))?;
            println!("显示方向设置完成");

            // 尝试颜色反转以测试显示
            esp!(esp_lcd_panel_invert_color(panel, false))?;
            println!("颜色反转设置完成");

            // 尝试不同的颜色反转
            std::thread::sleep(std::time::Duration::from_millis(100));
            esp!(esp_lcd_panel_invert_color(panel, true))?;
            println!("颜色反转开启");
            std::thread::sleep(std::time::Duration::from_millis(100));
            esp!(esp_lcd_panel_invert_color(panel, false))?;
            println!("颜色反转关闭");

            // 开启显示
            esp!(esp_lcd_panel_disp_on_off(panel, true))?;
            println!("显示开启完成");

            // 等待显示稳定
            std::thread::sleep(std::time::Duration::from_millis(500));
        }

        Ok(panel)
    }

    fn init_backlight(
        peripherals: Peripherals,
    ) -> Result<PinDriver<'static, esp_idf_hal::gpio::Gpio5, esp_idf_hal::gpio::Output>> {
        let mut bl = esp_idf_hal::gpio::PinDriver::output(peripherals.pins.gpio5)?;
        bl.set_high()?;
        Ok(bl)
    }

    pub fn set_mirror(&self, mirror_x: bool, mirror_y: bool) -> Result<()> {
        unsafe {
            esp!(esp_lcd_panel_mirror(self.panel, mirror_x, mirror_y))?;
        }
        Ok(())
    }

    pub fn set_swap_xy(&self, swap_xy: bool) -> Result<()> {
        unsafe {
            esp!(esp_lcd_panel_swap_xy(self.panel, swap_xy))?;
        }

        Ok(())
    }

    pub fn draw_bitmap(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        data: *const u16,
    ) -> Result<()> {
        unsafe {
            esp!(esp_lcd_panel_draw_bitmap(
                self.panel,
                x,          // x_start
                y,          // y_start
                x + width,  // x_end (exclusive)
                y + height, // y_end (exclusive)
                data as *const _
            ))?;
        }
        Ok(())
    }

    pub fn clear(&self, color: u16) -> Result<()> {
        // 分块清屏，每次处理一行，避免栈溢出
        let line_buffer = vec![color; LCD_WIDTH as usize];
        for y in 0..LCD_HEIGHT {
            self.draw_bitmap(0, y, LCD_WIDTH, 1, line_buffer.as_ptr())?;
        }
        Ok(())
    }

    pub fn draw_test_pattern(&self) -> Result<()> {
        // 分行绘制测试图案，避免栈溢出
        let mut line_buffer = vec![0u16; LCD_WIDTH as usize];

        for y in 0..LCD_HEIGHT {
            for x in 0..LCD_WIDTH {
                let color = if x < 120 {
                    0xF800 // 红色
                } else if x < 240 {
                    0x07E0 // 绿色
                } else {
                    0x001F // 蓝色
                };
                line_buffer[x as usize] = color;
            }
            self.draw_bitmap(0, y, LCD_WIDTH, 1, line_buffer.as_ptr())?;
        }
        Ok(())
    }

    pub fn set_backlight(&mut self, on: bool) -> Result<()> {
        if on {
            self.backlight.set_high()?;
        } else {
            self.backlight.set_low()?;
        }
        Ok(())
    }
}
