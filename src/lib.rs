//! Barebones driver for the 13.3" Inky Impression e-ink display (EL133UF1).
//!
//! This library provides a minimal interface to the EL133UF1 driver for the
//! 1600×1200 pixel, 6-color Spectra 6 display.
//!
//! # Example
//! ```no_run
//! use inky_el133::{InkyDisplay, colors};
//!
//! let mut display = InkyDisplay::new()?;
//! display.fill(colors::WHITE)?;
//! display.set_pixel(100, 100, colors::RED)?;
//! display.show()?;
//! # Ok::<(), inky_el133::InkyError>(())
//! ```

mod buffer;
mod constants;
mod controller;
pub mod error;

use buffer::PixelBuffer;
use constants::*;
use controller::{ChipSelect, DisplayController};
pub use error::{InkyError, Result};

/// Color indices for the 6-color Spectra 6 display.
///
/// Note: Color index 4 is invalid and skipped by the display hardware.
pub mod colors {
    pub const BLACK: u8 = 0;
    pub const WHITE: u8 = 1;
    pub const YELLOW: u8 = 2;
    pub const RED: u8 = 3;
    pub const BLUE: u8 = 5;
    pub const GREEN: u8 = 6;
}

/// Main interface for the Inky Impression 13.3" display
pub struct InkyDisplay {
    controller: DisplayController,
    buffer: PixelBuffer,
}

impl InkyDisplay {
    /// Initialize the display
    ///
    /// This will set up GPIO pins, SPI communication, reset the display,
    /// and send the initialization sequence.
    pub fn new() -> Result<Self> {
        let mut controller = DisplayController::new()?;
        controller.reset()?;

        let mut display = Self {
            controller,
            buffer: PixelBuffer::new(WIDTH, HEIGHT),
        };

        display.initialize()?;
        Ok(display)
    }

    /// Send initialization command sequence to the display (inky_el133uf1.py:236-255)
    fn initialize(&mut self) -> Result<()> {
        self.controller.wait_busy(300)?;

        self.controller.send_command(
            ChipSelect::CS0,
            CMD_ANTM,
            &[0xC0, 0x1C, 0x1C, 0xCC, 0xCC, 0xCC, 0x15, 0x15, 0x55],
        )?;

        self.controller.send_command(
            ChipSelect::Both,
            CMD_CMD66,
            &[0x49, 0x55, 0x13, 0x5D, 0x05, 0x10],
        )?;

        self.controller
            .send_command(ChipSelect::Both, CMD_PSR, &[0xDF, 0x69])?;

        self.controller
            .send_command(ChipSelect::Both, CMD_PLL, &[0x08])?;

        self.controller
            .send_command(ChipSelect::Both, CMD_CDI, &[0xF7])?;

        self.controller
            .send_command(ChipSelect::Both, CMD_TCON, &[0x03, 0x03])?;

        self.controller
            .send_command(ChipSelect::Both, CMD_AGID, &[0x10])?;

        self.controller
            .send_command(ChipSelect::Both, CMD_PWS, &[0x22])?;

        self.controller
            .send_command(ChipSelect::Both, CMD_CCSET, &[0x01])?;

        self.controller
            .send_command(ChipSelect::Both, CMD_TRES, &[0x04, 0xB0, 0x03, 0x20])?;

        self.controller.send_command(
            ChipSelect::CS0,
            CMD_PWR,
            &[0x0F, 0x00, 0x28, 0x2C, 0x28, 0x38],
        )?;

        self.controller
            .send_command(ChipSelect::CS0, CMD_EN_BUF, &[0x07])?;

        self.controller
            .send_command(ChipSelect::CS0, CMD_BTST_P, &[0xD8, 0x18])?;

        self.controller
            .send_command(ChipSelect::CS0, CMD_BOOST_VDDP_EN, &[0x01])?;

        self.controller
            .send_command(ChipSelect::CS0, CMD_BTST_N, &[0xD8, 0x18])?;

        self.controller
            .send_command(ChipSelect::CS0, CMD_BUCK_BOOST_VDDN, &[0x01])?;

        self.controller
            .send_command(ChipSelect::CS0, CMD_TFT_VCOM_POWER, &[0x02])?;

        Ok(())
    }

    /// Set a single pixel
    ///
    /// # Arguments
    /// * `x` - X coordinate (0-1599)
    /// * `y` - Y coordinate (0-1199)
    /// * `color` - Color index (0, 1, 2, 3, 5, or 6)
    ///
    /// # Example
    /// ```no_run
    /// # use inky_el133::{InkyDisplay, colors};
    /// # let mut display = InkyDisplay::new()?;
    /// display.set_pixel(100, 200, colors::RED)?;
    /// # Ok::<(), inky_el133::InkyError>(())
    /// ```
    pub fn set_pixel(&mut self, x: usize, y: usize, color: u8) -> Result<()> {
        self.buffer.set_pixel(x, y, color)
    }

    /// Fill the entire buffer with a single color
    ///
    /// # Example
    /// ```no_run
    /// # use inky_el133::{InkyDisplay, colors};
    /// # let mut display = InkyDisplay::new()?;
    /// display.fill(colors::WHITE)?;
    /// # Ok::<(), inky_el133::InkyError>(())
    /// ```
    pub fn fill(&mut self, color: u8) -> Result<()> {
        self.buffer.fill(color)
    }

    /// Update the display with the current buffer contents
    ///
    /// This operation takes approximately 32 seconds due to hardware limitations.
    /// The display will rotate the buffer, split it between the two controllers,
    /// and refresh the screen.
    ///
    /// # Example
    /// ```no_run
    /// # use inky_el133::{InkyDisplay, colors};
    /// # let mut display = InkyDisplay::new()?;
    /// display.fill(colors::BLACK)?;
    /// display.show()?;  // Takes ~32 seconds
    /// # Ok::<(), inky_el133::InkyError>(())
    /// ```
    pub fn show(&mut self) -> Result<()> {
        let (buf_a, buf_b) = self.buffer.rotate_and_split();

        self.controller
            .send_command(ChipSelect::CS0, CMD_DTM, &buf_a)?;
        self.controller
            .send_command(ChipSelect::CS1, CMD_DTM, &buf_b)?;

        self.controller
            .send_command(ChipSelect::Both, CMD_PON, &[])?;
        self.controller.wait_busy(200)?;

        self.controller
            .send_command(ChipSelect::Both, CMD_DRF, &[0x00])?;
        self.controller.wait_busy(32000)?;

        self.controller
            .send_command(ChipSelect::Both, CMD_POF, &[0x00])?;
        self.controller.wait_busy(200)?;

        Ok(())
    }

    /// Clear the display to white
    ///
    /// This is equivalent to calling `fill(colors::WHITE)` followed by `show()`.
    ///
    /// # Example
    /// ```no_run
    /// # use inky_el133::InkyDisplay;
    /// # let mut display = InkyDisplay::new()?;
    /// display.clear()?;  // Takes ~32 seconds
    /// # Ok::<(), inky_el133::InkyError>(())
    /// ```
    pub fn clear(&mut self) -> Result<()> {
        self.fill(colors::WHITE)?;
        self.show()
    }
}
