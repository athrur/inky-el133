//! Hardware configuration constants for the EL133UF1 display controller.

// GPIO Pin Configuration
#[cfg_attr(not(target_os = "linux"), allow(dead_code))]
pub const CS0_PIN: u8 = 26;
#[cfg_attr(not(target_os = "linux"), allow(dead_code))]
pub const CS1_PIN: u8 = 16;
#[cfg_attr(not(target_os = "linux"), allow(dead_code))]
pub const DC_PIN: u8 = 22;
#[cfg_attr(not(target_os = "linux"), allow(dead_code))]
pub const RESET_PIN: u8 = 27;
#[cfg_attr(not(target_os = "linux"), allow(dead_code))]
pub const BUSY_PIN: u8 = 17;

// Display Dimensions
pub const WIDTH: usize = 1600;
pub const HEIGHT: usize = 1200;
pub const SPLIT_COL: usize = 600;

// SPI Configuration
#[cfg_attr(not(target_os = "linux"), allow(dead_code))]
pub const SPI_SPEED_HZ: u32 = 10_000_000;

// EL133UF1 Commands
pub const CMD_PSR: u8 = 0x00; // Panel Setting Register
pub const CMD_PWR: u8 = 0x01; // Power Setting
pub const CMD_POF: u8 = 0x02; // Power Off
pub const CMD_PON: u8 = 0x04; // Power On
pub const CMD_BTST_N: u8 = 0x05; // Booster Soft Start VDDN
pub const CMD_BTST_P: u8 = 0x06; // Booster Soft Start VDDP
pub const CMD_DTM: u8 = 0x10; // Data Transmission
pub const CMD_DRF: u8 = 0x12; // Display Refresh
pub const CMD_PLL: u8 = 0x30; // PLL Control
pub const CMD_CDI: u8 = 0x50; // VCOM Data Interval
pub const CMD_TCON: u8 = 0x60; // TCON Setting
pub const CMD_TRES: u8 = 0x61; // Resolution Setting
pub const CMD_ANTM: u8 = 0x74; // Analog Block Control
pub const CMD_AGID: u8 = 0x86; // Gate ID
pub const CMD_BUCK_BOOST_VDDN: u8 = 0xB0; // Buck/Boost VDDN
pub const CMD_TFT_VCOM_POWER: u8 = 0xB1; // TFT VCOM Power
pub const CMD_EN_BUF: u8 = 0xB6; // Enable Buffer
pub const CMD_BOOST_VDDP_EN: u8 = 0xB7; // Boost VDDP Enable
pub const CMD_CCSET: u8 = 0xE0; // Cascade Setting
pub const CMD_PWS: u8 = 0xE3; // Power Saving
pub const CMD_CMD66: u8 = 0xF0; // Command 0x66 (undocumented)

// Color Constants
pub const BLACK: u8 = 0;
pub const WHITE: u8 = 1;
pub const YELLOW: u8 = 2;
pub const RED: u8 = 3;
// Note: Color 4 is invalid/skipped
pub const BLUE: u8 = 5;
pub const GREEN: u8 = 6;

// Validate color index
pub fn is_valid_color(color: u8) -> bool {
    matches!(color, BLACK | WHITE | YELLOW | RED | BLUE | GREEN)
}
