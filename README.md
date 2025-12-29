# inky-el133

A barebones Rust driver for the 13.3" Inky Impression e-ink display (EL133UF1).

This is a simple port of Pimoroni's [Python library](https://github.com/pimoroni/inky), providing minimal hardware control for the 1600×1200 pixel, 6-color Spectra 6 display.

## Features

- Direct pixel manipulation with 6-color support (Black, White, Yellow, Red, Blue, Green)
- Hardware SPI/GPIO communication
- Cross-compilation support for Raspberry Pi (ARM)

## Usage

```rust
use inky_el133::{InkyDisplay, colors};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut display = InkyDisplay::new()?;

    display.fill(colors::WHITE)?;
    display.set_pixel(100, 100, colors::RED)?;
    display.show()?; // Takes ~32 seconds

    Ok(())
}
```

## Examples

- `simple_display` - Draws vertical color stripes
- `web_server` - HTTP server with `/display` endpoint to receive PNG images (1600×1200, automatically maps colors)
- `web_client` - Client to preprocess and send images to the web server (resizes, quantizes to 6 colors)

**Web server workflow:**
```bash
# On Pi: start the server
cargo run --example web_server

# From any machine: send an image
cargo run --example web_client -- my-image.jpg --server http://pi-ip:3000
```

## Development

A Nix flake is provided that manages all dependencies. If you prefer manual installation, you'll need:

- Rust toolchain with `armv7-unknown-linux-gnueabihf` target
- Zig (for cross-compilation)
- cargo-zigbuild
- just (build automation)

Build for Raspberry Pi:
```bash
cargo zigbuild --release --target armv7-unknown-linux-gnueabihf
```

## Platform Support

This library only runs on Linux (requires GPIO and SPI access). It will compile on other platforms but return `UnsupportedPlatform` errors.

**Note:** Only tested on Raspberry Pi Zero 2 W.

## License

MIT
