use inky_el133::{InkyDisplay, colors};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Initializing Inky Impression 13.3\" display...");
    let mut display = InkyDisplay::new()?;

    println!("Drawing vertical color stripes...");
    // Draw 6 vertical stripes, one for each color
    for x in 0..1600 {
        let color = match x / 266 {
            0 => colors::BLACK,
            1 => colors::WHITE,
            2 => colors::YELLOW,
            3 => colors::RED,
            4 => colors::BLUE,
            _ => colors::GREEN,
        };

        for y in 0..1200 {
            display.set_pixel(x, y, color)?;
        }
    }

    println!("Updating display...");
    display.show()?;

    println!("Display updated successfully.");
    Ok(())
}
