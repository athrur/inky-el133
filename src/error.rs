use thiserror::Error;

/// Errors that can occur when using the Inky display.
#[derive(Debug, Error)]
pub enum InkyError {
    #[cfg(target_os = "linux")]
    #[error("GPIO error: {0}")]
    Gpio(#[from] gpio_cdev::Error),

    #[cfg(target_os = "linux")]
    #[error("SPI error: {0}")]
    Spi(#[from] std::io::Error),

    #[error("Invalid color index: {0} (valid: 0, 1, 2, 3, 5, 6)")]
    InvalidColor(u8),

    #[error("Coordinates out of bounds: ({0}, {1})")]
    OutOfBounds(usize, usize),

    #[error("Display busy timeout")]
    BusyTimeout,

    #[error("Invalid pixel buffer size")]
    InvalidBufferSize,

    #[cfg(not(target_os = "linux"))]
    #[error("This library only works on Linux")]
    UnsupportedPlatform,
}

/// Convenience type alias for Results with [`InkyError`].
pub type Result<T> = std::result::Result<T, InkyError>;
