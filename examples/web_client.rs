use clap::Parser;
use image::{ImageBuffer, Rgb, RgbImage};
use std::path::PathBuf;

const WIDTH: u32 = 1600;
const HEIGHT: u32 = 1200;

const DISPLAY_COLORS: [[u8; 3]; 6] = [
    [0, 0, 0],       // Black
    [255, 255, 255], // White
    [255, 255, 0],   // Yellow
    [255, 0, 0],     // Red
    [0, 0, 255],     // Blue
    [0, 255, 0],     // Green
];

#[derive(Parser)]
#[command(about = "Process and send images to Inky display server")]
struct Args {
    /// Input image path
    input: PathBuf,

    /// Server URL (default: http://localhost:3000)
    #[arg(short, long, default_value = "http://localhost:3000")]
    server: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!("Loading image: {}", args.input.display());
    let img = image::open(&args.input)?;

    println!("Resizing to {}x{}...", WIDTH, HEIGHT);
    let resized = img.resize_exact(WIDTH, HEIGHT, image::imageops::FilterType::Lanczos3);

    println!("Quantizing colors to 6-color palette...");
    let processed = quantize_image(resized.to_rgb8());

    println!("Encoding PNG...");
    let mut png_data = Vec::new();
    processed.write_to(
        &mut std::io::Cursor::new(&mut png_data),
        image::ImageFormat::Png,
    )?;

    println!("Sending to {}...", args.server);
    let client = reqwest::blocking::Client::new();
    let response = client
        .post(format!("{}/display", args.server))
        .body(png_data)
        .send()?;

    if response.status().is_success() {
        println!("✓ Display updated successfully");
    } else {
        eprintln!("✗ Server error: {}", response.text()?);
    }

    Ok(())
}

fn quantize_image(img: RgbImage) -> RgbImage {
    ImageBuffer::from_fn(WIDTH, HEIGHT, |x, y| {
        let pixel = img.get_pixel(x, y);
        let rgb = [pixel[0], pixel[1], pixel[2]];
        let closest = find_closest_color(rgb);
        Rgb(closest)
    })
}

fn find_closest_color(rgb: [u8; 3]) -> [u8; 3] {
    *DISPLAY_COLORS
        .iter()
        .min_by_key(|color| {
            let dr = (rgb[0] as i32 - color[0] as i32).abs();
            let dg = (rgb[1] as i32 - color[1] as i32).abs();
            let db = (rgb[2] as i32 - color[2] as i32).abs();
            dr + dg + db
        })
        .unwrap()
}
