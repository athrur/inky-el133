use crate::error::Result;

#[cfg(target_os = "linux")]
use {
    crate::constants::*,
    gpio_cdev::{Chip, LineHandle, LineRequestFlags},
    spidev::{SpiModeFlags, Spidev, SpidevOptions},
    std::thread,
    std::time::{Duration, Instant},
};

/// Chip select options for dual-controller display
#[derive(Debug, Clone, Copy)]
pub(crate) enum ChipSelect {
    CS0,
    CS1,
    Both,
}

/// Low-level hardware controller for SPI and GPIO communication
#[cfg(target_os = "linux")]
pub(crate) struct DisplayController {
    spi: Spidev,
    cs0_pin: LineHandle,
    cs1_pin: LineHandle,
    dc_pin: LineHandle,
    reset_pin: LineHandle,
    busy_pin: LineHandle,
}

#[cfg(target_os = "linux")]
impl DisplayController {
    /// Initialize GPIO pins and SPI interface
    pub fn new() -> Result<Self> {
        let mut chip = Chip::new("/dev/gpiochip0")?;

        let cs0_pin =
            chip.get_line(CS0_PIN as u32)?
                .request(LineRequestFlags::OUTPUT, 1, "inky-cs0")?;

        let cs1_pin =
            chip.get_line(CS1_PIN as u32)?
                .request(LineRequestFlags::OUTPUT, 1, "inky-cs1")?;

        let dc_pin =
            chip.get_line(DC_PIN as u32)?
                .request(LineRequestFlags::OUTPUT, 0, "inky-dc")?;

        let reset_pin =
            chip.get_line(RESET_PIN as u32)?
                .request(LineRequestFlags::OUTPUT, 1, "inky-reset")?;

        let busy_pin =
            chip.get_line(BUSY_PIN as u32)?
                .request(LineRequestFlags::INPUT, 0, "inky-busy")?;

        let mut spi = Spidev::open("/dev/spidev0.0")?;
        let options = SpidevOptions::new()
            .bits_per_word(8)
            .max_speed_hz(SPI_SPEED_HZ)
            .mode(SpiModeFlags::SPI_MODE_0)
            .build();
        spi.configure(&options)?;

        Ok(Self {
            spi,
            cs0_pin,
            cs1_pin,
            dc_pin,
            reset_pin,
            busy_pin,
        })
    }

    /// Perform hardware reset of the display (inky_el133uf1.py:229-232)
    pub fn reset(&mut self) -> Result<()> {
        self.reset_pin.set_value(0)?;
        thread::sleep(Duration::from_millis(30));

        self.reset_pin.set_value(1)?;
        thread::sleep(Duration::from_millis(30));

        Ok(())
    }

    /// Send command and optional data to specified chip select
    pub fn send_command(&mut self, cs: ChipSelect, cmd: u8, data: &[u8]) -> Result<()> {
        match cs {
            ChipSelect::CS0 => self.cs0_pin.set_value(0)?,
            ChipSelect::CS1 => self.cs1_pin.set_value(0)?,
            ChipSelect::Both => {
                self.cs0_pin.set_value(0)?;
                self.cs1_pin.set_value(0)?;
            }
        }

        self.dc_pin.set_value(0)?;
        thread::sleep(Duration::from_millis(300)); // inky_el133uf1.py:384

        use std::io::Write;
        self.spi.write_all(&[cmd])?;

        // Chunk into 4KB blocks (inky_el133uf1.py:367-370)
        if !data.is_empty() {
            self.dc_pin.set_value(1)?;

            const CHUNK_SIZE: usize = 4096;
            for chunk in data.chunks(CHUNK_SIZE) {
                self.spi.write_all(chunk)?;
            }
        }

        self.cs0_pin.set_value(1)?;
        self.cs1_pin.set_value(1)?;
        self.dc_pin.set_value(0)?;

        Ok(())
    }

    /// Wait for busy pin to go low (display ready) - inky_el133uf1.py:261-270
    pub fn wait_busy(&mut self, timeout_ms: u64) -> Result<()> {
        let timeout = Duration::from_millis(timeout_ms);
        let start = Instant::now();

        // If busy_pin is HIGH initially, display isn't connected - just sleep
        if self.busy_pin.get_value()? == 1 {
            thread::sleep(timeout);
            return Ok(());
        }

        while self.busy_pin.get_value()? == 1 {
            if start.elapsed() > timeout {
                eprintln!("Warning: Busy wait timed out after {} ms", timeout_ms);
                return Ok(());
            }
            thread::sleep(Duration::from_millis(100));
        }

        Ok(())
    }
}

// Stub implementation for non-Linux platforms
#[cfg(not(target_os = "linux"))]
pub(crate) struct DisplayController;

#[cfg(not(target_os = "linux"))]
impl DisplayController {
    pub fn new() -> Result<Self> {
        Err(crate::error::InkyError::UnsupportedPlatform)
    }

    pub fn reset(&mut self) -> Result<()> {
        Err(crate::error::InkyError::UnsupportedPlatform)
    }

    pub fn send_command(&mut self, _cs: ChipSelect, _cmd: u8, _data: &[u8]) -> Result<()> {
        Err(crate::error::InkyError::UnsupportedPlatform)
    }

    pub fn wait_busy(&mut self, _timeout_ms: u64) -> Result<()> {
        Err(crate::error::InkyError::UnsupportedPlatform)
    }
}
