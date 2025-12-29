use crate::constants::*;
use crate::error::{InkyError, Result};

/// Pixel buffer for the display
pub(crate) struct PixelBuffer {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

impl PixelBuffer {
    /// Create new pixel buffer initialized to white
    pub fn new(width: usize, height: usize) -> Self {
        let data = vec![WHITE; width * height];
        Self {
            data,
            width,
            height,
        }
    }

    /// Set a single pixel
    pub fn set_pixel(&mut self, x: usize, y: usize, color: u8) -> Result<()> {
        if x >= self.width || y >= self.height {
            return Err(InkyError::OutOfBounds(x, y));
        }

        if !is_valid_color(color) {
            return Err(InkyError::InvalidColor(color));
        }

        self.data[y * self.width + x] = color;
        Ok(())
    }

    /// Fill entire buffer with a single color
    pub fn fill(&mut self, color: u8) -> Result<()> {
        if !is_valid_color(color) {
            return Err(InkyError::InvalidColor(color));
        }

        self.data.fill(color);
        Ok(())
    }

    /// Rotate buffer -90 degrees, split at column 600, and pack pixels.
    ///
    /// Returns (buf_a, buf_b) for CS0 and CS1 respectively. After rotation,
    /// the 1600×1200 buffer becomes 1200×1600 and is split at column 600.
    pub fn rotate_and_split(&self) -> (Vec<u8>, Vec<u8>) {
        let rotated_width = self.height;
        let rotated_height = self.width;
        let mut rotated = vec![0u8; rotated_width * rotated_height];

        // Rotate -90 degrees: rotated[y][1599-x] = original[x][y]
        for y in 0..self.height {
            for x in 0..self.width {
                let original_pixel = self.data[y * self.width + x];
                let new_x = y;
                let new_y = self.width - 1 - x;
                rotated[new_y * rotated_width + new_x] = original_pixel;
            }
        }

        // Split at column 600: buf_a (0-599) for CS0, buf_b (600-1199) for CS1
        let mut pixels_a = Vec::new();
        let mut pixels_b = Vec::new();

        for row in 0..rotated_height {
            for col in 0..SPLIT_COL {
                pixels_a.push(rotated[row * rotated_width + col]);
            }
            for col in SPLIT_COL..rotated_width {
                pixels_b.push(rotated[row * rotated_width + col]);
            }
        }

        (pack_pixels(&pixels_a), pack_pixels(&pixels_b))
    }
}

/// Pack pixels into bytes (2 pixels per byte, 4 bits each).
fn pack_pixels(pixels: &[u8]) -> Vec<u8> {
    let mut packed = Vec::with_capacity((pixels.len() + 1) / 2);

    for chunk in pixels.chunks(2) {
        let byte = if chunk.len() == 2 {
            (chunk[0] << 4) | (chunk[1] & 0x0F)
        } else {
            chunk[0] << 4
        };
        packed.push(byte);
    }

    packed
}
