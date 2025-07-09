use anyhow::Result;
use embedded_graphics::mono_font::jis_x0201::FONT_10X20;
use esp_idf_hal::gpio::PinDriver;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_sys::st77916::{
    esp_lcd_new_panel_st77916, st77916_lcd_init_cmd_t, st77916_vendor_config_t,
    st77916_vendor_config_t__bindgen_ty_1,
};
use esp_idf_sys::*;
use std::ptr;

use crate::lcd_cmds::get_vendor_specific_init_new;

// embedded-graphics相关导入
use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Point, Size},
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::{Rgb565, RgbColor},
    text::{Text, TextStyleBuilder},
    Drawable, Pixel,
};

// ===================== 常量区 =====================
// 分辨率 & 像素格式
pub const LCD_WIDTH: i32 = 360;
pub const LCD_HEIGHT: i32 = 360;
pub const LCD_BIT_PER_PIXEL: u8 = 16; // RGB565

// QSPI 引脚映射（根据硬件连接）
pub const QSPI_LCD_HOST: i32 = spi_host_device_t_SPI2_HOST as i32;
pub const QSPI_PIN_NUM_LCD_SCK: i32 = gpio_num_t_GPIO_NUM_40; // LCD_SCK
pub const QSPI_PIN_NUM_LCD_CS: i32 = gpio_num_t_GPIO_NUM_21; // LCD_CS
pub const QSPI_PIN_NUM_LCD_SDA0: i32 = gpio_num_t_GPIO_NUM_46; // LCD_SDA0 (DATA0)
pub const QSPI_PIN_NUM_LCD_SDA1: i32 = gpio_num_t_GPIO_NUM_45; // LCD_SDA1 (DATA1)
pub const QSPI_PIN_NUM_LCD_SDA2: i32 = gpio_num_t_GPIO_NUM_42; // LCD_SDA2 (DATA2)
pub const QSPI_PIN_NUM_LCD_SDA3: i32 = gpio_num_t_GPIO_NUM_41; // LCD_SDA3 (DATA3)
pub const QSPI_PIN_NUM_LCD_TE: i32 = gpio_num_t_GPIO_NUM_18; // LCD_TE (Tearing Effect)
pub const QSPI_PIN_NUM_LCD_BL: i32 = gpio_num_t_GPIO_NUM_5; // LCD_BL (背光)
pub const QSPI_PIN_NUM_LCD_RST: i32 = gpio_num_t_GPIO_NUM_NC; // LCD_RST

// 预定义颜色（RGB565）
pub const COLOR_BLACK: u16 = 0x0000;
pub const COLOR_WHITE: u16 = 0xFFFF;
pub const COLOR_RED: u16 = 0xF800;
pub const COLOR_GREEN: u16 = 0x07E0;
pub const COLOR_BLUE: u16 = 0x001F;
pub const COLOR_YELLOW: u16 = 0xFFE0;
pub const COLOR_CYAN: u16 = 0x07FF;
pub const COLOR_MAGENTA: u16 = 0xF81F;

// =================================================

pub struct LcdController {
    panel: esp_lcd_panel_handle_t,
    io_handle: esp_lcd_panel_io_handle_t,
    backlight: PinDriver<'static, esp_idf_hal::gpio::Gpio5, esp_idf_hal::gpio::Output>,
}

impl LcdController {
    /// 创建新的LCD控制器实例
    pub fn new(peripherals: Peripherals) -> Result<Self> {
        // 步骤1：初始化SPI总线
        let io_handle = Self::init_spi_bus()?;

        // 步骤2：创建LCD面板
        let panel = Self::create_panel(io_handle)?;

        // 步骤3：初始化背光控制
        let backlight = Self::init_backlight(peripherals)?;

        // 步骤4：启动显示器
        let controller = LcdController {
            panel,
            io_handle,
            backlight,
        };

        controller.start_display()?;

        Ok(controller)
    }

    /// 初始化QSPI总线（使用官方推荐的配置）
    fn init_spi_bus() -> Result<esp_lcd_panel_io_handle_t> {
        unsafe {
            // 步骤1：修复QSPI引脚映射（标准QSPI配置）
            let bus_config = spi_bus_config_t {
                sclk_io_num: QSPI_PIN_NUM_LCD_SCK, // 时钟线 GPIO40
                __bindgen_anon_1: spi_bus_config_t__bindgen_ty_1 {
                    data0_io_num: QSPI_PIN_NUM_LCD_SDA0,
                },
                __bindgen_anon_2: spi_bus_config_t__bindgen_ty_2 {
                    data1_io_num: QSPI_PIN_NUM_LCD_SDA1,
                },
                __bindgen_anon_3: spi_bus_config_t__bindgen_ty_3 {
                    data2_io_num: QSPI_PIN_NUM_LCD_SDA2,
                },
                __bindgen_anon_4: spi_bus_config_t__bindgen_ty_4 {
                    data3_io_num: QSPI_PIN_NUM_LCD_SDA3,
                },
                max_transfer_sz: LCD_WIDTH * LCD_HEIGHT * 2,
                ..Default::default()
            };

            // 初始化SPI总线
            esp!(spi_bus_initialize(
                QSPI_LCD_HOST as _,
                &bus_config,
                spi_common_dma_t_SPI_DMA_CH_AUTO // 自动分配DMA通道
            ))?;
        }

        // 步骤2：创建Panel IO
        let mut io_handle: esp_lcd_panel_io_handle_t = ptr::null_mut();
        let mut flags = esp_lcd_panel_io_spi_config_t__bindgen_ty_1::default();
        flags.set_quad_mode(1);
        flags.set_dc_low_on_data(0);
        flags.set_octal_mode(0);
        flags.set_sio_mode(0);
        flags.set_lsb_first(0);
        flags.set_cs_high_active(0);

        let io_config = esp_lcd_panel_io_spi_config_t {
            cs_gpio_num: QSPI_PIN_NUM_LCD_CS,
            dc_gpio_num: -1, // QSPI模式不需要DC引脚
            spi_mode: 0,
            pclk_hz: 40 * 1000 * 1000,
            trans_queue_depth: 10,
            on_color_trans_done: None,
            user_ctx: ptr::null_mut(),
            lcd_cmd_bits: 32,  // QSPI使用32位命令
            lcd_param_bits: 8, // 8位参数
            flags,
        };

        unsafe {
            esp!(esp_lcd_new_panel_io_spi(
                spi_host_device_t_SPI2_HOST as _,
                &io_config,
                &mut io_handle
            ))?;
        }

        Ok(io_handle)
    }

    /// 创建LCD面板
    fn create_panel(io_handle: esp_lcd_panel_io_handle_t) -> Result<esp_lcd_panel_handle_t> {
        let mut panel: esp_lcd_panel_handle_t = ptr::null_mut();

        let st77916_init_cmds = get_vendor_specific_init_new();
        let mut vendor_config = st77916_vendor_config_t::default();
        vendor_config.flags.set_use_qspi_interface(1);
        vendor_config.init_cmds = st77916_init_cmds.as_ptr() as *const _;
        vendor_config.init_cmds_size = st77916_init_cmds.len() as u16;

        // 配置面板参数（修复条纹问题）
        let panel_config = esp_lcd_panel_dev_config_t {
            reset_gpio_num: QSPI_PIN_NUM_LCD_RST, // LCD_RST连接到TCA9554扩展IO，暂时不使用
            __bindgen_anon_1: esp_lcd_panel_dev_config_t__bindgen_ty_1 {
                rgb_ele_order: lcd_rgb_element_order_t_LCD_RGB_ELEMENT_ORDER_RGB,
            },
            data_endian: lcd_rgb_data_endian_t_LCD_RGB_DATA_ENDIAN_BIG,
            bits_per_pixel: LCD_BIT_PER_PIXEL as u32,
            flags: esp_lcd_panel_dev_config_t__bindgen_ty_2::default(),
            vendor_config: &vendor_config as *const _ as *mut _,
        };

        unsafe {
            esp!(esp_lcd_new_panel_st77916(
                io_handle as *mut esp_idf_sys::st77916::esp_lcd_panel_io_t,
                &panel_config as *const esp_lcd_panel_dev_config_t
                    as *const esp_idf_sys::st77916::esp_lcd_panel_dev_config_t,
                &mut panel as *mut esp_lcd_panel_handle_t
                    as *mut *mut esp_idf_sys::st77916::esp_lcd_panel_t
            ))?;
        }

        Ok(panel)
    }

    /// 初始化背光控制
    fn init_backlight(
        peripherals: Peripherals,
    ) -> Result<PinDriver<'static, esp_idf_hal::gpio::Gpio5, esp_idf_hal::gpio::Output>> {
        let mut backlight = PinDriver::output(peripherals.pins.gpio5)?;
        backlight.set_high()?; // 默认开启背光
        Ok(backlight)
    }

    /// 启动显示器
    fn start_display(&self) -> Result<()> {
        unsafe {
            esp!(esp_lcd_panel_reset(self.panel))?;

            // 等待重置完成
            std::thread::sleep(std::time::Duration::from_millis(120));

            // 步骤2：初始化面板
            esp!(esp_lcd_panel_init(self.panel))?;

            // 步骤3：设置显示方向（尝试不同配置）
            esp!(esp_lcd_panel_swap_xy(self.panel, false))?; // 不交换XY轴
            esp!(esp_lcd_panel_mirror(self.panel, false, false))?; // 不镜像

            // 步骤4：先关闭显示，清除GRAM，再开启显示
            esp!(esp_lcd_panel_disp_on_off(self.panel, false))?;
            std::thread::sleep(std::time::Duration::from_millis(50));

            // 清除显示器内容，确保干净的显示
            self.fill_screen(COLOR_BLACK)?;

            esp!(esp_lcd_panel_disp_on_off(self.panel, true))?;
        }

        Ok(())
    }

    /// 绘制位图到指定区域
    pub fn draw_bitmap(
        &self,
        x_start: i32,
        y_start: i32,
        x_end: i32,
        y_end: i32,
        color_data: &[u16],
    ) -> Result<()> {
        if x_start < 0 || y_start < 0 || x_end > LCD_WIDTH || y_end > LCD_HEIGHT {
            return Err(anyhow::anyhow!("坐标超出屏幕范围"));
        }

        let expected_len = ((x_end - x_start) * (y_end - y_start)) as usize;
        if color_data.len() != expected_len {
            return Err(anyhow::anyhow!("颜色数据长度不匹配"));
        }

        unsafe {
            esp!(esp_lcd_panel_draw_bitmap(
                self.panel,
                x_start,
                y_start,
                x_end,
                y_end,
                color_data.as_ptr() as *const _
            ))?;
        }

        Ok(())
    }

    /// 填充整个屏幕（分块传输）
    pub fn fill_screen(&self, color: u16) -> Result<()> {
        // 使用分块传输以减少内存使用并提高稳定性
        const CHUNK_HEIGHT: i32 = 40;

        for y in (0..LCD_HEIGHT).step_by(CHUNK_HEIGHT as usize) {
            let chunk_height = (CHUNK_HEIGHT).min(LCD_HEIGHT - y);
            let chunk_size = (LCD_WIDTH * chunk_height) as usize;
            let buffer = vec![color; chunk_size];

            self.draw_bitmap(0, y, LCD_WIDTH, y + chunk_height, &buffer)?;
        }

        println!("fill_screen: 填充完成");
        Ok(())
    }

    /// 设置背光状态
    pub fn set_backlight(&mut self, on: bool) -> Result<()> {
        if on {
            self.backlight.set_high()?;
        } else {
            self.backlight.set_low()?;
        }
        Ok(())
    }

    /// 绘制单个像素
    pub fn draw_pixel(&self, x: i32, y: i32, color: u16) -> Result<()> {
        if x < 0 || y < 0 || x >= LCD_WIDTH || y >= LCD_HEIGHT {
            return Ok(()); // 超出边界直接返回
        }

        let buffer = [color];
        self.draw_bitmap(x, y, x + 1, y + 1, &buffer)?;
        Ok(())
    }

    /// 绘制圆形（使用Bresenham算法）
    pub fn draw_circle(&self, center_x: i32, center_y: i32, radius: i32, color: u16) -> Result<()> {
        if radius <= 0 {
            return Ok(());
        }

        let mut x = 0;
        let mut y = radius;
        let mut decision = 1 - radius;

        // 绘制中心点
        self.draw_pixel(center_x, center_y, color)?;

        while x <= y {
            // 绘制八个对称点
            self.draw_pixel(center_x + x, center_y + y, color)?;
            self.draw_pixel(center_x - x, center_y + y, color)?;
            self.draw_pixel(center_x + x, center_y - y, color)?;
            self.draw_pixel(center_x - x, center_y - y, color)?;
            self.draw_pixel(center_x + y, center_y + x, color)?;
            self.draw_pixel(center_x - y, center_y + x, color)?;
            self.draw_pixel(center_x + y, center_y - x, color)?;
            self.draw_pixel(center_x - y, center_y - x, color)?;

            x += 1;
            if decision < 0 {
                decision += 2 * x + 1;
            } else {
                y -= 1;
                decision += 2 * (x - y) + 1;
            }
        }

        Ok(())
    }

    /// 绘制实心圆形（填充）
    pub fn draw_filled_circle(
        &self,
        center_x: i32,
        center_y: i32,
        radius: i32,
        color: u16,
    ) -> Result<()> {
        if radius <= 0 {
            return Ok(());
        }

        for y in -radius..=radius {
            let y_coord = center_y + y;
            if !(0..LCD_HEIGHT).contains(&y_coord) {
                continue;
            }

            // 计算当前行的半宽
            let half_width = ((radius * radius - y * y) as f32).sqrt() as i32;

            let x_start = (center_x - half_width).max(0);
            let x_end = (center_x + half_width + 1).min(LCD_WIDTH);

            if x_start < x_end {
                let line_width = (x_end - x_start) as usize;
                let line_buffer = vec![color; line_width];
                self.draw_bitmap(x_start, y_coord, x_end, y_coord + 1, &line_buffer)?;
            }
        }

        Ok(())
    }

    /// 使用embedded-graphics绘制文本
    pub fn draw_text(&mut self, text: &str, x: i32, y: i32, color: Rgb565) -> Result<()> {
        let character_style = MonoTextStyle::new(&FONT_10X20, color);
        let text_style = TextStyleBuilder::new().build();

        let text_obj = Text::with_text_style(text, Point::new(x, y), character_style, text_style);
        text_obj.draw(self)?;
        Ok(())
    }

    /// 绘制平滑文本（使用背景色进行简单的抗锯齿）
    pub fn draw_smooth_text(
        &mut self,
        text: &str,
        x: i32,
        y: i32,
        fg_color: Rgb565,
        bg_color: Rgb565,
    ) -> Result<()> {
        // 先绘制背景色的文本作为阴影（偏移1像素）
        let shadow_color = Rgb565::new(
            (fg_color.r() + bg_color.r()) / 2,
            (fg_color.g() + bg_color.g()) / 2,
            (fg_color.b() + bg_color.b()) / 2,
        );

        self.draw_text(text, x + 1, y + 1, shadow_color)?;

        // 再绘制前景色的文本
        self.draw_text(text, x, y, fg_color)?;
        Ok(())
    }

    /// 使用embedded-graphics绘制彩色文本（方便方法）
    pub fn draw_colored_text(
        &mut self,
        text: &str,
        x: i32,
        y: i32,
        r: u8,
        g: u8,
        b: u8,
    ) -> Result<()> {
        let color = Rgb565::new(r >> 3, g >> 2, b >> 3);
        self.draw_text(text, x, y, color)
    }

    /// 绘制一个眼睛
    pub fn draw_eye(&self, center_x: i32, center_y: i32, eye_size: i32) -> Result<()> {
        // 眼球半径
        let eyeball_radius = eye_size;
        // 瞳孔半径
        let pupil_radius = eye_size / 2;
        // 高光半径
        let highlight_radius = eye_size / 4;

        // 绘制眼球（蓝色）
        self.draw_filled_circle(center_x, center_y, eyeball_radius, COLOR_BLUE)?;

        // 绘制瞳孔（黑色，稍微偏右下）
        let pupil_x = center_x + eye_size / 6;
        let pupil_y = center_y + eye_size / 6;
        self.draw_filled_circle(pupil_x, pupil_y, pupil_radius, COLOR_BLACK)?;

        // 绘制高光（白色，在瞳孔左上角）
        let highlight_x = pupil_x - pupil_radius / 3;
        let highlight_y = pupil_y - pupil_radius / 3;
        self.draw_filled_circle(highlight_x, highlight_y, highlight_radius, COLOR_WHITE)?;

        Ok(())
    }

    /// 绘制两个眼睛
    pub fn draw_eyes(&self) -> Result<()> {
        // 360x360屏幕，眼睛大小为40像素半径
        let eye_size = 40;
        let eye_spacing = 120; // 眼睛之间的距离

        // 屏幕中心
        let center_x = LCD_WIDTH / 2;
        let center_y = LCD_HEIGHT / 2;

        // 左眼位置
        let left_eye_x = center_x - eye_spacing / 2;
        let left_eye_y = center_y;

        // 右眼位置
        let right_eye_x = center_x + eye_spacing / 2;
        let right_eye_y = center_y;

        // 绘制左眼
        self.draw_eye(left_eye_x, left_eye_y, eye_size)?;

        // 绘制右眼
        self.draw_eye(right_eye_x, right_eye_y, eye_size)?;

        Ok(())
    }

    /// 绘制一个眼睛，支持瞳孔位置偏移
    pub fn draw_eye_with_pupil_offset(
        &self,
        center_x: i32,
        center_y: i32,
        eye_size: i32,
        pupil_offset_x: i32,
        pupil_offset_y: i32,
    ) -> Result<()> {
        // 眼球半径
        let eyeball_radius = eye_size;
        // 瞳孔半径
        let pupil_radius = eye_size / 2;
        // 高光半径
        let highlight_radius = eye_size / 4;

        // 绘制眼球（蓝色）
        self.draw_filled_circle(center_x, center_y, eyeball_radius, COLOR_BLUE)?;

        // 绘制瞳孔（黑色，可以偏移）
        let pupil_x = center_x + pupil_offset_x;
        let pupil_y = center_y + pupil_offset_y;
        self.draw_filled_circle(pupil_x, pupil_y, pupil_radius, COLOR_BLACK)?;

        // 绘制高光（白色，在瞳孔左上角）
        let highlight_x = pupil_x - pupil_radius / 3;
        let highlight_y = pupil_y - pupil_radius / 3;
        self.draw_filled_circle(highlight_x, highlight_y, highlight_radius, COLOR_WHITE)?;

        Ok(())
    }

    /// 绘制椭圆形眼睛（用于眨眼效果）
    pub fn draw_eye_blink(
        &self,
        center_x: i32,
        center_y: i32,
        eye_size: i32,
        blink_ratio: f32,
    ) -> Result<()> {
        let eyeball_radius = eye_size;
        let pupil_radius = eye_size / 2;
        let highlight_radius = eye_size / 4;

        // 根据眨眼比例调整眼睛高度
        let eye_height = (eyeball_radius as f32 * blink_ratio) as i32;

        if eye_height <= 2 {
            // 完全闭眼，绘制一条线
            let line_buffer = vec![COLOR_BLUE; (eyeball_radius * 2) as usize];
            self.draw_bitmap(
                center_x - eyeball_radius,
                center_y - 1,
                center_x + eyeball_radius,
                center_y + 1,
                &line_buffer,
            )?;
            return Ok(());
        }

        // 绘制压缩的眼球
        for y in -eye_height..=eye_height {
            let y_coord = center_y + y;
            if !(0..LCD_HEIGHT).contains(&y_coord) {
                continue;
            }

            let half_width = ((eyeball_radius * eyeball_radius
                - (y * eyeball_radius / eye_height) * (y * eyeball_radius / eye_height))
                as f32)
                .sqrt() as i32;
            let x_start = (center_x - half_width).max(0);
            let x_end = (center_x + half_width + 1).min(LCD_WIDTH);

            if x_start < x_end {
                let line_width = (x_end - x_start) as usize;
                let line_buffer = vec![COLOR_BLUE; line_width];
                self.draw_bitmap(x_start, y_coord, x_end, y_coord + 1, &line_buffer)?;
            }
        }

        // 如果眼睛开度足够，绘制瞳孔和高光
        if eye_height > eyeball_radius / 2 {
            let pupil_x = center_x + eye_size / 6;
            let pupil_y = center_y + eye_size / 6;
            let compressed_pupil_radius = (pupil_radius as f32 * blink_ratio) as i32;

            if compressed_pupil_radius > 0 {
                self.draw_filled_circle(pupil_x, pupil_y, compressed_pupil_radius, COLOR_BLACK)?;

                let highlight_x = pupil_x - compressed_pupil_radius / 3;
                let highlight_y = pupil_y - compressed_pupil_radius / 3;
                let compressed_highlight_radius = (highlight_radius as f32 * blink_ratio) as i32;

                if compressed_highlight_radius > 0 {
                    self.draw_filled_circle(
                        highlight_x,
                        highlight_y,
                        compressed_highlight_radius,
                        COLOR_WHITE,
                    )?;
                }
            }
        }

        Ok(())
    }

    /// 绘制眼睛看向左边
    pub fn draw_eyes_look_left(&self) -> Result<()> {
        let eye_size = 40;
        let eye_spacing = 120;
        let center_x = LCD_WIDTH / 2;
        let center_y = LCD_HEIGHT / 2;

        let left_eye_x = center_x - eye_spacing / 2;
        let left_eye_y = center_y;
        let right_eye_x = center_x + eye_spacing / 2;
        let right_eye_y = center_y;

        // 瞳孔向左偏移
        let pupil_offset_x = -eye_size / 4;
        let pupil_offset_y = 0;

        self.draw_eye_with_pupil_offset(
            left_eye_x,
            left_eye_y,
            eye_size,
            pupil_offset_x,
            pupil_offset_y,
        )?;
        self.draw_eye_with_pupil_offset(
            right_eye_x,
            right_eye_y,
            eye_size,
            pupil_offset_x,
            pupil_offset_y,
        )?;

        Ok(())
    }

    /// 绘制眼睛看向右边
    pub fn draw_eyes_look_right(&self) -> Result<()> {
        let eye_size = 40;
        let eye_spacing = 120;
        let center_x = LCD_WIDTH / 2;
        let center_y = LCD_HEIGHT / 2;

        let left_eye_x = center_x - eye_spacing / 2;
        let left_eye_y = center_y;
        let right_eye_x = center_x + eye_spacing / 2;
        let right_eye_y = center_y;

        // 瞳孔向右偏移
        let pupil_offset_x = eye_size / 4;
        let pupil_offset_y = 0;

        self.draw_eye_with_pupil_offset(
            left_eye_x,
            left_eye_y,
            eye_size,
            pupil_offset_x,
            pupil_offset_y,
        )?;
        self.draw_eye_with_pupil_offset(
            right_eye_x,
            right_eye_y,
            eye_size,
            pupil_offset_x,
            pupil_offset_y,
        )?;

        Ok(())
    }

    /// 绘制眼睛看向上方
    pub fn draw_eyes_look_up(&self) -> Result<()> {
        let eye_size = 40;
        let eye_spacing = 120;
        let center_x = LCD_WIDTH / 2;
        let center_y = LCD_HEIGHT / 2;

        let left_eye_x = center_x - eye_spacing / 2;
        let left_eye_y = center_y;
        let right_eye_x = center_x + eye_spacing / 2;
        let right_eye_y = center_y;

        // 瞳孔向上偏移
        let pupil_offset_x = 0;
        let pupil_offset_y = -eye_size / 4;

        self.draw_eye_with_pupil_offset(
            left_eye_x,
            left_eye_y,
            eye_size,
            pupil_offset_x,
            pupil_offset_y,
        )?;
        self.draw_eye_with_pupil_offset(
            right_eye_x,
            right_eye_y,
            eye_size,
            pupil_offset_x,
            pupil_offset_y,
        )?;

        Ok(())
    }

    /// 绘制眼睛看向下方
    pub fn draw_eyes_look_down(&self) -> Result<()> {
        let eye_size = 40;
        let eye_spacing = 120;
        let center_x = LCD_WIDTH / 2;
        let center_y = LCD_HEIGHT / 2;

        let left_eye_x = center_x - eye_spacing / 2;
        let left_eye_y = center_y;
        let right_eye_x = center_x + eye_spacing / 2;
        let right_eye_y = center_y;

        // 瞳孔向下偏移
        let pupil_offset_x = 0;
        let pupil_offset_y = eye_size / 4;

        self.draw_eye_with_pupil_offset(
            left_eye_x,
            left_eye_y,
            eye_size,
            pupil_offset_x,
            pupil_offset_y,
        )?;
        self.draw_eye_with_pupil_offset(
            right_eye_x,
            right_eye_y,
            eye_size,
            pupil_offset_x,
            pupil_offset_y,
        )?;

        Ok(())
    }

    /// 绘制眨眼动画
    pub fn draw_eyes_blink(&self, blink_ratio: f32) -> Result<()> {
        let eye_size = 40;
        let eye_spacing = 120;
        let center_x = LCD_WIDTH / 2;
        let center_y = LCD_HEIGHT / 2;

        let left_eye_x = center_x - eye_spacing / 2;
        let left_eye_y = center_y;
        let right_eye_x = center_x + eye_spacing / 2;
        let right_eye_y = center_y;

        self.draw_eye_blink(left_eye_x, left_eye_y, eye_size, blink_ratio)?;
        self.draw_eye_blink(right_eye_x, right_eye_y, eye_size, blink_ratio)?;

        Ok(())
    }

    /// 播放眼睛动画序列
    pub fn play_eye_animation(&self) -> Result<()> {
        use std::thread;
        use std::time::Duration;

        let frame_duration = Duration::from_millis(500);

        // 1. 正常眼睛
        println!("动画: 正常眼睛");
        self.fill_screen(COLOR_BLACK)?;
        self.draw_eyes()?;
        thread::sleep(frame_duration);

        // 2. 眨眼动画序列
        println!("动画: 眨眼");
        let blink_frames = [1.0, 0.7, 0.4, 0.1, 0.4, 0.7, 1.0];
        for &blink_ratio in &blink_frames {
            self.fill_screen(COLOR_BLACK)?;
            self.draw_eyes_blink(blink_ratio)?;
            thread::sleep(Duration::from_millis(100));
        }

        // 3. 看左边
        println!("动画: 看左边");
        self.fill_screen(COLOR_BLACK)?;
        self.draw_eyes_look_left()?;
        thread::sleep(frame_duration);

        // 4. 回到中间
        self.fill_screen(COLOR_BLACK)?;
        self.draw_eyes()?;
        thread::sleep(Duration::from_millis(300));

        // 5. 看右边
        println!("动画: 看右边");
        self.fill_screen(COLOR_BLACK)?;
        self.draw_eyes_look_right()?;
        thread::sleep(frame_duration);

        // 6. 回到中间
        self.fill_screen(COLOR_BLACK)?;
        self.draw_eyes()?;
        thread::sleep(Duration::from_millis(300));

        // 7. 看上面
        println!("动画: 看上面");
        self.fill_screen(COLOR_BLACK)?;
        self.draw_eyes_look_up()?;
        thread::sleep(frame_duration);

        // 8. 回到中间
        self.fill_screen(COLOR_BLACK)?;
        self.draw_eyes()?;
        thread::sleep(Duration::from_millis(300));

        // 9. 看下面
        println!("动画: 看下面");
        self.fill_screen(COLOR_BLACK)?;
        self.draw_eyes_look_down()?;
        thread::sleep(frame_duration);

        // 10. 回到中间
        self.fill_screen(COLOR_BLACK)?;
        self.draw_eyes()?;
        thread::sleep(Duration::from_millis(300));

        // 11. 最后再眨一次眼
        println!("动画: 最后眨眼");
        for &blink_ratio in &blink_frames {
            self.fill_screen(COLOR_BLACK)?;
            self.draw_eyes_blink(blink_ratio)?;
            thread::sleep(Duration::from_millis(80));
        }

        Ok(())
    }
}

// 为LcdController实现embedded-graphics的DrawTarget trait
impl DrawTarget for LcdController {
    type Color = Rgb565;
    type Error = anyhow::Error;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels {
            // 将Rgb565转换为RGB565格式的u16值
            let color_u16 =
                ((color.r() as u16) << 11) | ((color.g() as u16) << 5) | (color.b() as u16);
            self.draw_pixel(coord.x, coord.y, color_u16)?;
        }
        Ok(())
    }
}

impl OriginDimensions for LcdController {
    fn size(&self) -> Size {
        Size::new(LCD_WIDTH as u32, LCD_HEIGHT as u32)
    }
}

impl Drop for LcdController {
    fn drop(&mut self) {
        // 清理资源
        unsafe {
            if !self.panel.is_null() {
                esp_lcd_panel_del(self.panel);
            }
            if !self.io_handle.is_null() {
                esp_lcd_panel_io_del(self.io_handle);
            }
            spi_bus_free(spi_host_device_t_SPI2_HOST as _);
        }
    }
}
