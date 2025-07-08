use anyhow::Result;
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
            // QSPI 四线 + 时钟
            let bus_cfg = spi_bus_config_t {
                __bindgen_anon_1: spi_bus_config_t__bindgen_ty_1 { mosi_io_num: 41 },
                __bindgen_anon_2: spi_bus_config_t__bindgen_ty_2 { miso_io_num: -1 },
                sclk_io_num: 40,
                __bindgen_anon_3: spi_bus_config_t__bindgen_ty_3 { quadwp_io_num: 46 }, // SD0
                __bindgen_anon_4: spi_bus_config_t__bindgen_ty_4 { quadhd_io_num: 45 }, // SD1
                data4_io_num: -1,
                data5_io_num: -1,
                data6_io_num: -1,
                data7_io_num: -1,
                max_transfer_sz: LCD_WIDTH * LCD_HEIGHT * 2,
                flags: 0,
                intr_flags: 0,
                isr_cpu_id: 0,
            };

            esp!(spi_bus_initialize(
                spi_host_device_t_SPI2_HOST as _,
                &bus_cfg,
                spi_common_dma_t_SPI_DMA_CH_AUTO
            ))?;
        }

        // 创建 LCD IO（4-线 QSPI）
        let mut io_handle: esp_lcd_panel_io_handle_t = core::ptr::null_mut();
        let io_cfg = esp_lcd_panel_io_spi_config_t {
            cs_gpio_num: 21,
            dc_gpio_num: -1,     // ST77916 QSPI 模式无需 DC
            pclk_hz: 40_000_000, // 40 MHz
            spi_mode: 0,
            trans_queue_depth: 10,
            flags: esp_lcd_panel_io_spi_config_t__bindgen_ty_1 {
                _bitfield_align_1: [],
                _bitfield_1: Default::default(),
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
                _bitfield_1: st77916_vendor_config_t__bindgen_ty_1::new_bitfield_1(1),
                __bindgen_padding_0: [0; 3],
            },
            init_cmds: core::ptr::null(),
            init_cmds_size: 0,
        };
        let panel_cfg = esp_lcd_panel_dev_config_t {
            reset_gpio_num: -1, // 如果你接了 RST，请填实际引脚
            __bindgen_anon_1: esp_lcd_panel_dev_config_t__bindgen_ty_1 {
                rgb_ele_order: lcd_rgb_element_order_t_LCD_RGB_ELEMENT_ORDER_RGB,
            },
            data_endian: lcd_rgb_data_endian_t_LCD_RGB_DATA_ENDIAN_BIG,
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
            esp!(esp_lcd_panel_reset(panel))?;
            esp!(esp_lcd_panel_init(panel))?;

            // 设置显示方向
            esp!(esp_lcd_panel_mirror(panel, false, false))?;
            esp!(esp_lcd_panel_swap_xy(panel, false))?;

            // 开启显示
            esp!(esp_lcd_panel_disp_on_off(panel, true))?;
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
                x,
                y,
                width,
                height,
                data as *const _
            ))?;
        }
        Ok(())
    }

    pub fn clear(&self, color: u16) -> Result<()> {
        static mut COLOR_BUFFER: [u16; (360 * 360) as usize] = [0; 360 * 360];
        unsafe {
            COLOR_BUFFER.fill(color);
            self.draw_bitmap(0, 0, LCD_WIDTH, LCD_HEIGHT, COLOR_BUFFER.as_ptr())?;
        }
        Ok(())
    }

    pub fn draw_test_pattern(&self) -> Result<()> {
        static mut COLOR_BUFFER: [u16; (360 * 360) as usize] = [0; 360 * 360];
        unsafe {
            // 创建彩色条纹图案
            for y in 0..360 {
                for x in 0..360 {
                    let color = if x < 120 {
                        0xF800 // 红色
                    } else if x < 240 {
                        0x07E0 // 绿色
                    } else {
                        0x001F // 蓝色
                    };
                    COLOR_BUFFER[y * 360 + x] = color;
                }
            }

            self.draw_bitmap(0, 0, LCD_WIDTH, LCD_HEIGHT, COLOR_BUFFER.as_ptr())?;
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
