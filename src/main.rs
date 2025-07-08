// src/main.rs
use anyhow::Result;
use esp_idf_hal::{delay::FreeRtos, peripherals::Peripherals};
use esp_idf_sys::st77916::{
    esp_lcd_new_panel_st77916, st77916_vendor_config_t, st77916_vendor_config_t__bindgen_ty_1,
};
use esp_idf_sys::*;

const LCD_WIDTH: i32 = 360;
const LCD_HEIGHT: i32 = 360;

fn main() -> Result<()> {
    // 必须先调用，打补丁
    esp_idf_sys::link_patches();

    // 1. 取得外设
    let p = Peripherals::take().unwrap();
    let pins = p.pins;

    // 2. 初始化 SPI2 总线 (HSPI)
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

    // 3. 创建 LCD IO（4-线 QSPI）
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

    // 4. 创建 panel 句柄
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

    // 5. 打开背光
    let mut bl = esp_idf_hal::gpio::PinDriver::output(pins.gpio5)?;
    bl.set_high()?;

    // 6. 填充测试图案
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

        esp!(esp_lcd_panel_draw_bitmap(
            panel,
            0,
            0,
            LCD_WIDTH,
            LCD_HEIGHT,
            COLOR_BUFFER.as_ptr() as *const _
        ))?;
    }

    // 7. 保持运行
    FreeRtos::delay_ms(3_600_000);
    Ok(())
}
