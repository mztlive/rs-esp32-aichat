use esp_idf_sys::st77916::st77916_lcd_init_cmd_t;

macro_rules! lcd_init_cmd {
    ($cmd:expr, $data:expr, $delay:expr) => {
        st77916_lcd_init_cmd_t {
            cmd: $cmd,
            data: $data.as_ptr() as *const ::core::ffi::c_void,
            data_bytes: $data.len(),
            delay_ms: $delay,
        }
    };
}

static DATA_28: [u8; 1] = [0x28];
static DATA_28_2: [u8; 1] = [0x28];
static DATA_F0: [u8; 1] = [0xF0];
static DATA_D1: [u8; 1] = [0xD1];
static DATA_E0: [u8; 1] = [0xE0];
static DATA_61: [u8; 1] = [0x61];
static DATA_82: [u8; 1] = [0x82];
static DATA_00: [u8; 1] = [0x00];
static DATA_01: [u8; 1] = [0x01];
static DATA_01_2: [u8; 1] = [0x01];
static DATA_56: [u8; 1] = [0x56];
static DATA_4D: [u8; 1] = [0x4D];
static DATA_24: [u8; 1] = [0x24];
static DATA_87: [u8; 1] = [0x87];
static DATA_44: [u8; 1] = [0x44];
static DATA_8B: [u8; 1] = [0x8B];
static DATA_40: [u8; 1] = [0x40];
static DATA_86: [u8; 1] = [0x86];
static DATA_00_2: [u8; 1] = [0x00];
static DATA_08: [u8; 1] = [0x08];
static DATA_08_2: [u8; 1] = [0x08];
static DATA_00_3: [u8; 1] = [0x00];
static DATA_80: [u8; 1] = [0x80];
static DATA_10: [u8; 1] = [0x10];
static DATA_37: [u8; 1] = [0x37];
static DATA_80_2: [u8; 1] = [0x80];
static DATA_10_2: [u8; 1] = [0x10];
static DATA_37_2: [u8; 1] = [0x37];
static DATA_A9: [u8; 1] = [0xA9];
static DATA_41: [u8; 1] = [0x41];
static DATA_01_3: [u8; 1] = [0x01];
static DATA_A9_2: [u8; 1] = [0xA9];
static DATA_41_2: [u8; 1] = [0x41];
static DATA_01_4: [u8; 1] = [0x01];
static DATA_91: [u8; 1] = [0x91];
static DATA_68: [u8; 1] = [0x68];
static DATA_68_2: [u8; 1] = [0x68];
static DATA_00_A5: [u8; 2] = [0x00, 0xA5];
static DATA_4F: [u8; 1] = [0x4F];
static DATA_4F_2: [u8; 1] = [0x4F];
static DATA_10_3: [u8; 1] = [0x10];
static DATA_00_4: [u8; 1] = [0x00];
static DATA_02: [u8; 1] = [0x02];
static DATA_E0_GAMMA: [u8; 14] = [
    0xF0, 0x0A, 0x10, 0x09, 0x09, 0x36, 0x35, 0x33, 0x4A, 0x29, 0x15, 0x15, 0x2E, 0x34,
];
static DATA_E1_GAMMA: [u8; 14] = [
    0xF0, 0x0A, 0x0F, 0x08, 0x08, 0x05, 0x34, 0x33, 0x4A, 0x39, 0x15, 0x15, 0x2D, 0x33,
];
static DATA_10_4: [u8; 1] = [0x10];
static DATA_10_5: [u8; 1] = [0x10];
static DATA_07: [u8; 1] = [0x07];
static DATA_00_5: [u8; 1] = [0x00];
static DATA_00_6: [u8; 1] = [0x00];
static DATA_00_7: [u8; 1] = [0x00];
static DATA_E0_2: [u8; 1] = [0xE0];
static DATA_06: [u8; 1] = [0x06];
static DATA_21: [u8; 1] = [0x21];
static DATA_01_5: [u8; 1] = [0x01];
static DATA_05: [u8; 1] = [0x05];
static DATA_02_2: [u8; 1] = [0x02];
static DATA_DA: [u8; 1] = [0xDA];
static DATA_00_8: [u8; 1] = [0x00];
static DATA_00_9: [u8; 1] = [0x00];
static DATA_0F: [u8; 1] = [0x0F];
static DATA_00_10: [u8; 1] = [0x00];
static DATA_00_11: [u8; 1] = [0x00];
static DATA_00_12: [u8; 1] = [0x00];
static DATA_00_13: [u8; 1] = [0x00];
static DATA_00_14: [u8; 1] = [0x00];
static DATA_00_15: [u8; 1] = [0x00];
static DATA_00_16: [u8; 1] = [0x00];
static DATA_00_17: [u8; 1] = [0x00];
static DATA_00_18: [u8; 1] = [0x00];
static DATA_00_19: [u8; 1] = [0x00];
static DATA_40_2: [u8; 1] = [0x40];
static DATA_04: [u8; 1] = [0x04];
static DATA_00_20: [u8; 1] = [0x00];
static DATA_42: [u8; 1] = [0x42];
static DATA_D9: [u8; 1] = [0xD9];
static DATA_00_21: [u8; 1] = [0x00];
static DATA_00_22: [u8; 1] = [0x00];
static DATA_00_23: [u8; 1] = [0x00];
static DATA_00_24: [u8; 1] = [0x00];
static DATA_00_25: [u8; 1] = [0x00];
static DATA_00_26: [u8; 1] = [0x00];
static DATA_00_27: [u8; 1] = [0x00];
static DATA_40_3: [u8; 1] = [0x40];
static DATA_03: [u8; 1] = [0x03];
static DATA_00_28: [u8; 1] = [0x00];
static DATA_42_2: [u8; 1] = [0x42];
static DATA_D8: [u8; 1] = [0xD8];
static DATA_00_29: [u8; 1] = [0x00];
static DATA_00_30: [u8; 1] = [0x00];
static DATA_00_31: [u8; 1] = [0x00];
static DATA_00_32: [u8; 1] = [0x00];
static DATA_00_33: [u8; 1] = [0x00];
static DATA_00_34: [u8; 1] = [0x00];
static DATA_00_35: [u8; 1] = [0x00];
static DATA_48: [u8; 1] = [0x48];
static DATA_00_36: [u8; 1] = [0x00];
static DATA_06_2: [u8; 1] = [0x06];
static DATA_02_3: [u8; 1] = [0x02];
static DATA_D6: [u8; 1] = [0xD6];
static DATA_04_2: [u8; 1] = [0x04];
static DATA_00_37: [u8; 1] = [0x00];
static DATA_00_38: [u8; 1] = [0x00];
static DATA_48_2: [u8; 1] = [0x48];
static DATA_00_39: [u8; 1] = [0x00];
static DATA_08_3: [u8; 1] = [0x08];
static DATA_02_4: [u8; 1] = [0x02];
static DATA_D8_2: [u8; 1] = [0xD8];
static DATA_04_3: [u8; 1] = [0x04];
static DATA_00_40: [u8; 1] = [0x00];
static DATA_00_41: [u8; 1] = [0x00];
static DATA_48_3: [u8; 1] = [0x48];
static DATA_00_42: [u8; 1] = [0x00];
static DATA_0A: [u8; 1] = [0x0A];
static DATA_02_5: [u8; 1] = [0x02];
static DATA_DA_2: [u8; 1] = [0xDA];
static DATA_04_4: [u8; 1] = [0x04];
static DATA_00_43: [u8; 1] = [0x00];
static DATA_00_44: [u8; 1] = [0x00];
static DATA_48_4: [u8; 1] = [0x48];
static DATA_00_45: [u8; 1] = [0x00];
static DATA_0C: [u8; 1] = [0x0C];
static DATA_02_6: [u8; 1] = [0x02];
static DATA_DC: [u8; 1] = [0xDC];
static DATA_04_5: [u8; 1] = [0x04];
static DATA_00_46: [u8; 1] = [0x00];
static DATA_00_47: [u8; 1] = [0x00];
static DATA_48_5: [u8; 1] = [0x48];
static DATA_00_48: [u8; 1] = [0x00];
static DATA_05_2: [u8; 1] = [0x05];
static DATA_02_7: [u8; 1] = [0x02];
static DATA_D5: [u8; 1] = [0xD5];
static DATA_04_6: [u8; 1] = [0x04];
static DATA_00_49: [u8; 1] = [0x00];
static DATA_00_50: [u8; 1] = [0x00];
static DATA_48_6: [u8; 1] = [0x48];
static DATA_00_51: [u8; 1] = [0x00];
static DATA_07_2: [u8; 1] = [0x07];
static DATA_02_8: [u8; 1] = [0x02];
static DATA_D7: [u8; 1] = [0xD7];
static DATA_04_7: [u8; 1] = [0x04];
static DATA_00_52: [u8; 1] = [0x00];
static DATA_00_53: [u8; 1] = [0x00];
static DATA_48_7: [u8; 1] = [0x48];
static DATA_00_54: [u8; 1] = [0x00];
static DATA_09: [u8; 1] = [0x09];
static DATA_02_9: [u8; 1] = [0x02];
static DATA_D9_2: [u8; 1] = [0xD9];
static DATA_04_8: [u8; 1] = [0x04];
static DATA_00_55: [u8; 1] = [0x00];
static DATA_00_56: [u8; 1] = [0x00];
static DATA_48_8: [u8; 1] = [0x48];
static DATA_00_57: [u8; 1] = [0x00];
static DATA_0B: [u8; 1] = [0x0B];
static DATA_02_10: [u8; 1] = [0x02];
static DATA_DB: [u8; 1] = [0xDB];
static DATA_04_9: [u8; 1] = [0x04];
static DATA_00_58: [u8; 1] = [0x00];
static DATA_00_59: [u8; 1] = [0x00];
static DATA_10_6: [u8; 1] = [0x10];
static DATA_47: [u8; 1] = [0x47];
static DATA_56_2: [u8; 1] = [0x56];
static DATA_65: [u8; 1] = [0x65];
static DATA_74: [u8; 1] = [0x74];
static DATA_88: [u8; 1] = [0x88];
static DATA_99: [u8; 1] = [0x99];
static DATA_01_6: [u8; 1] = [0x01];
static DATA_BB: [u8; 1] = [0xBB];
static DATA_AA: [u8; 1] = [0xAA];
static DATA_10_7: [u8; 1] = [0x10];
static DATA_47_2: [u8; 1] = [0x47];
static DATA_56_3: [u8; 1] = [0x56];
static DATA_65_2: [u8; 1] = [0x65];
static DATA_74_2: [u8; 1] = [0x74];
static DATA_88_2: [u8; 1] = [0x88];
static DATA_99_2: [u8; 1] = [0x99];
static DATA_01_7: [u8; 1] = [0x01];
static DATA_BB_2: [u8; 1] = [0xBB];
static DATA_AA_2: [u8; 1] = [0xAA];
static DATA_01_8: [u8; 1] = [0x01];
static DATA_00_60: [u8; 1] = [0x00];
static DATA_00_61: [u8; 1] = [0x00];
static DATA_00_62: [u8; 1] = [0x00];
static DATA_00_63: [u8; 1] = [0x00];

pub fn get_vendor_specific_init_new() -> &'static [st77916_lcd_init_cmd_t] {
    use std::sync::Once;
    static INIT: Once = Once::new();
    static mut ARRAY: Option<&'static [st77916_lcd_init_cmd_t]> = None;

    unsafe {
        INIT.call_once(|| {
            let boxed = Box::new([
                lcd_init_cmd!(0xF0, DATA_28, 0),
                lcd_init_cmd!(0xF2, DATA_28_2, 0),
                lcd_init_cmd!(0x73, DATA_F0, 0),
                lcd_init_cmd!(0x7C, DATA_D1, 0),
                lcd_init_cmd!(0x83, DATA_E0, 0),
                lcd_init_cmd!(0x84, DATA_61, 0),
                lcd_init_cmd!(0xF2, DATA_82, 0),
                lcd_init_cmd!(0xF0, DATA_00, 0),
                lcd_init_cmd!(0xF0, DATA_01, 0),
                lcd_init_cmd!(0xF1, DATA_01_2, 0),
                lcd_init_cmd!(0xB0, DATA_56, 0),
                lcd_init_cmd!(0xB1, DATA_4D, 0),
                lcd_init_cmd!(0xB2, DATA_24, 0),
                lcd_init_cmd!(0xB4, DATA_87, 0),
                lcd_init_cmd!(0xB5, DATA_44, 0),
                lcd_init_cmd!(0xB6, DATA_8B, 0),
                lcd_init_cmd!(0xB7, DATA_40, 0),
                lcd_init_cmd!(0xB8, DATA_86, 0),
                lcd_init_cmd!(0xBA, DATA_00_2, 0),
                lcd_init_cmd!(0xBB, DATA_08, 0),
                lcd_init_cmd!(0xBC, DATA_08_2, 0),
                lcd_init_cmd!(0xBD, DATA_00_3, 0),
                lcd_init_cmd!(0xC0, DATA_80, 0),
                lcd_init_cmd!(0xC1, DATA_10, 0),
                lcd_init_cmd!(0xC2, DATA_37, 0),
                lcd_init_cmd!(0xC3, DATA_80_2, 0),
                lcd_init_cmd!(0xC4, DATA_10_2, 0),
                lcd_init_cmd!(0xC5, DATA_37_2, 0),
                lcd_init_cmd!(0xC6, DATA_A9, 0),
                lcd_init_cmd!(0xC7, DATA_41, 0),
                lcd_init_cmd!(0xC8, DATA_01_3, 0),
                lcd_init_cmd!(0xC9, DATA_A9_2, 0),
                lcd_init_cmd!(0xCA, DATA_41_2, 0),
                lcd_init_cmd!(0xCB, DATA_01_4, 0),
                lcd_init_cmd!(0xD0, DATA_91, 0),
                lcd_init_cmd!(0xD1, DATA_68, 0),
                lcd_init_cmd!(0xD2, DATA_68_2, 0),
                lcd_init_cmd!(0xF5, DATA_00_A5, 0),
                lcd_init_cmd!(0xDD, DATA_4F, 0),
                lcd_init_cmd!(0xDE, DATA_4F_2, 0),
                lcd_init_cmd!(0xF1, DATA_10_3, 0),
                lcd_init_cmd!(0xF0, DATA_00_4, 0),
                lcd_init_cmd!(0xF0, DATA_02, 0),
                lcd_init_cmd!(0xE0, DATA_E0_GAMMA, 0),
                lcd_init_cmd!(0xE1, DATA_E1_GAMMA, 0),
                lcd_init_cmd!(0xF0, DATA_10_4, 0),
                lcd_init_cmd!(0xF3, DATA_10_5, 0),
                lcd_init_cmd!(0xE0, DATA_07, 0),
                lcd_init_cmd!(0xE1, DATA_00_5, 0),
                lcd_init_cmd!(0xE2, DATA_00_6, 0),
                lcd_init_cmd!(0xE3, DATA_00_7, 0),
                lcd_init_cmd!(0xE4, DATA_E0_2, 0),
                lcd_init_cmd!(0xE5, DATA_06, 0),
                lcd_init_cmd!(0xE6, DATA_21, 0),
                lcd_init_cmd!(0xE7, DATA_01_5, 0),
                lcd_init_cmd!(0xE8, DATA_05, 0),
                lcd_init_cmd!(0xE9, DATA_02_2, 0),
                lcd_init_cmd!(0xEA, DATA_DA, 0),
                lcd_init_cmd!(0xEB, DATA_00_8, 0),
                lcd_init_cmd!(0xEC, DATA_00_9, 0),
                lcd_init_cmd!(0xED, DATA_0F, 0),
                lcd_init_cmd!(0xEE, DATA_00_10, 0),
                lcd_init_cmd!(0xEF, DATA_00_11, 0),
                lcd_init_cmd!(0xF8, DATA_00_12, 0),
                lcd_init_cmd!(0xF9, DATA_00_13, 0),
                lcd_init_cmd!(0xFA, DATA_00_14, 0),
                lcd_init_cmd!(0xFB, DATA_00_15, 0),
                lcd_init_cmd!(0xFC, DATA_00_16, 0),
                lcd_init_cmd!(0xFD, DATA_00_17, 0),
                lcd_init_cmd!(0xFE, DATA_00_18, 0),
                lcd_init_cmd!(0xFF, DATA_00_19, 0),
                lcd_init_cmd!(0x60, DATA_40_2, 0),
                lcd_init_cmd!(0x61, DATA_04, 0),
                lcd_init_cmd!(0x62, DATA_00_20, 0),
                lcd_init_cmd!(0x63, DATA_42, 0),
                lcd_init_cmd!(0x64, DATA_D9, 0),
                lcd_init_cmd!(0x65, DATA_00_21, 0),
                lcd_init_cmd!(0x66, DATA_00_22, 0),
                lcd_init_cmd!(0x67, DATA_00_23, 0),
                lcd_init_cmd!(0x68, DATA_00_24, 0),
                lcd_init_cmd!(0x69, DATA_00_25, 0),
                lcd_init_cmd!(0x6A, DATA_00_26, 0),
                lcd_init_cmd!(0x6B, DATA_00_27, 0),
                lcd_init_cmd!(0x70, DATA_40_3, 0),
                lcd_init_cmd!(0x71, DATA_03, 0),
                lcd_init_cmd!(0x72, DATA_00_28, 0),
                lcd_init_cmd!(0x73, DATA_42_2, 0),
                lcd_init_cmd!(0x74, DATA_D8, 0),
                lcd_init_cmd!(0x75, DATA_00_29, 0),
                lcd_init_cmd!(0x76, DATA_00_30, 0),
                lcd_init_cmd!(0x77, DATA_00_31, 0),
                lcd_init_cmd!(0x78, DATA_00_32, 0),
                lcd_init_cmd!(0x79, DATA_00_33, 0),
                lcd_init_cmd!(0x7A, DATA_00_34, 0),
                lcd_init_cmd!(0x7B, DATA_00_35, 0),
                lcd_init_cmd!(0x80, DATA_48, 0),
                lcd_init_cmd!(0x81, DATA_00_36, 0),
                lcd_init_cmd!(0x82, DATA_06_2, 0),
                lcd_init_cmd!(0x83, DATA_02_3, 0),
                lcd_init_cmd!(0x84, DATA_D6, 0),
                lcd_init_cmd!(0x85, DATA_04_2, 0),
                lcd_init_cmd!(0x86, DATA_00_37, 0),
                lcd_init_cmd!(0x87, DATA_00_38, 0),
                lcd_init_cmd!(0x88, DATA_48_2, 0),
                lcd_init_cmd!(0x89, DATA_00_39, 0),
                lcd_init_cmd!(0x8A, DATA_08_3, 0),
                lcd_init_cmd!(0x8B, DATA_02_4, 0),
                lcd_init_cmd!(0x8C, DATA_D8_2, 0),
                lcd_init_cmd!(0x8D, DATA_04_3, 0),
                lcd_init_cmd!(0x8E, DATA_00_40, 0),
                lcd_init_cmd!(0x8F, DATA_00_41, 0),
                lcd_init_cmd!(0x90, DATA_48_3, 0),
                lcd_init_cmd!(0x91, DATA_00_42, 0),
                lcd_init_cmd!(0x92, DATA_0A, 0),
                lcd_init_cmd!(0x93, DATA_02_5, 0),
                lcd_init_cmd!(0x94, DATA_DA_2, 0),
                lcd_init_cmd!(0x95, DATA_04_4, 0),
                lcd_init_cmd!(0x96, DATA_00_43, 0),
                lcd_init_cmd!(0x97, DATA_00_44, 0),
                lcd_init_cmd!(0x98, DATA_48_4, 0),
                lcd_init_cmd!(0x99, DATA_00_45, 0),
                lcd_init_cmd!(0x9A, DATA_0C, 0),
                lcd_init_cmd!(0x9B, DATA_02_6, 0),
                lcd_init_cmd!(0x9C, DATA_DC, 0),
                lcd_init_cmd!(0x9D, DATA_04_5, 0),
                lcd_init_cmd!(0x9E, DATA_00_46, 0),
                lcd_init_cmd!(0x9F, DATA_00_47, 0),
                lcd_init_cmd!(0xA0, DATA_48_5, 0),
                lcd_init_cmd!(0xA1, DATA_00_48, 0),
                lcd_init_cmd!(0xA2, DATA_05_2, 0),
                lcd_init_cmd!(0xA3, DATA_02_7, 0),
                lcd_init_cmd!(0xA4, DATA_D5, 0),
                lcd_init_cmd!(0xA5, DATA_04_6, 0),
                lcd_init_cmd!(0xA6, DATA_00_49, 0),
                lcd_init_cmd!(0xA7, DATA_00_50, 0),
                lcd_init_cmd!(0xA8, DATA_48_6, 0),
                lcd_init_cmd!(0xA9, DATA_00_51, 0),
                lcd_init_cmd!(0xAA, DATA_07_2, 0),
                lcd_init_cmd!(0xAB, DATA_02_8, 0),
                lcd_init_cmd!(0xAC, DATA_D7, 0),
                lcd_init_cmd!(0xAD, DATA_04_7, 0),
                lcd_init_cmd!(0xAE, DATA_00_52, 0),
                lcd_init_cmd!(0xAF, DATA_00_53, 0),
                lcd_init_cmd!(0xB0, DATA_48_7, 0),
                lcd_init_cmd!(0xB1, DATA_00_54, 0),
                lcd_init_cmd!(0xB2, DATA_09, 0),
                lcd_init_cmd!(0xB3, DATA_02_9, 0),
                lcd_init_cmd!(0xB4, DATA_D9_2, 0),
                lcd_init_cmd!(0xB5, DATA_04_8, 0),
                lcd_init_cmd!(0xB6, DATA_00_55, 0),
                lcd_init_cmd!(0xB7, DATA_00_56, 0),
                lcd_init_cmd!(0xB8, DATA_48_8, 0),
                lcd_init_cmd!(0xB9, DATA_00_57, 0),
                lcd_init_cmd!(0xBA, DATA_0B, 0),
                lcd_init_cmd!(0xBB, DATA_02_10, 0),
                lcd_init_cmd!(0xBC, DATA_DB, 0),
                lcd_init_cmd!(0xBD, DATA_04_9, 0),
                lcd_init_cmd!(0xBE, DATA_00_58, 0),
                lcd_init_cmd!(0xBF, DATA_00_59, 0),
                lcd_init_cmd!(0xC0, DATA_10_6, 0),
                lcd_init_cmd!(0xC1, DATA_47, 0),
                lcd_init_cmd!(0xC2, DATA_56_2, 0),
                lcd_init_cmd!(0xC3, DATA_65, 0),
                lcd_init_cmd!(0xC4, DATA_74, 0),
                lcd_init_cmd!(0xC5, DATA_88, 0),
                lcd_init_cmd!(0xC6, DATA_99, 0),
                lcd_init_cmd!(0xC7, DATA_01_6, 0),
                lcd_init_cmd!(0xC8, DATA_BB, 0),
                lcd_init_cmd!(0xC9, DATA_AA, 0),
                lcd_init_cmd!(0xD0, DATA_10_7, 0),
                lcd_init_cmd!(0xD1, DATA_47_2, 0),
                lcd_init_cmd!(0xD2, DATA_56_3, 0),
                lcd_init_cmd!(0xD3, DATA_65_2, 0),
                lcd_init_cmd!(0xD4, DATA_74_2, 0),
                lcd_init_cmd!(0xD5, DATA_88_2, 0),
                lcd_init_cmd!(0xD6, DATA_99_2, 0),
                lcd_init_cmd!(0xD7, DATA_01_7, 0),
                lcd_init_cmd!(0xD8, DATA_BB_2, 0),
                lcd_init_cmd!(0xD9, DATA_AA_2, 0),
                lcd_init_cmd!(0xF3, DATA_01_8, 0),
                lcd_init_cmd!(0xF0, DATA_00_60, 0),
                lcd_init_cmd!(0x21, DATA_00_61, 0),
                lcd_init_cmd!(0x11, DATA_00_62, 120),
                lcd_init_cmd!(0x29, DATA_00_63, 0),
            ]);
            ARRAY = Some(Box::leak(boxed));
        });
        ARRAY.unwrap()
    }
}
