use axum::{Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};
use inky_el133::{InkyDisplay, colors};
use std::sync::{Arc, Mutex};

const WIDTH: u32 = 1600;
const HEIGHT: u32 = 1200;

const COLOR_MAP: [([u8; 3], u8); 6] = [
    ([0, 0, 0], colors::BLACK),
    ([255, 255, 255], colors::WHITE),
    ([255, 255, 0], colors::YELLOW),
    ([255, 0, 0], colors::RED),
    ([0, 0, 255], colors::BLUE),
    ([0, 255, 0], colors::GREEN),
];

struct AppState {
    display: Mutex<InkyDisplay>,
}

fn map_color(rgb: [u8; 3]) -> u8 {
    COLOR_MAP
        .iter()
        .min_by_key(|(color_rgb, _)| {
            let dr = (rgb[0] as i32 - color_rgb[0] as i32).abs();
            let dg = (rgb[1] as i32 - color_rgb[1] as i32).abs();
            let db = (rgb[2] as i32 - color_rgb[2] as i32).abs();
            dr + dg + db
        })
        .map(|(_, idx)| *idx)
        .unwrap_or(colors::WHITE)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Initializing Inky Impression display...");
    let display = InkyDisplay::new()?;

    let state = Arc::new(AppState {
        display: Mutex::new(display),
    });

    let app = Router::new()
        .route("/display", post(update_display))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Server listening on http://0.0.0.0:3000");
    println!("POST a PNG image to /display (1600x1200, indexed/palette mode)");

    axum::serve(listener, app).await?;

    Ok(())
}

async fn update_display(
    State(state): State<Arc<AppState>>,
    body: axum::body::Bytes,
) -> impl IntoResponse {
    // Decode PNG
    let img = match image::load_from_memory(&body) {
        Ok(img) => img,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                format!("Failed to decode image: {}", e),
            );
        }
    };

    // Validate dimensions
    if img.width() != WIDTH || img.height() != HEIGHT {
        return (
            StatusCode::BAD_REQUEST,
            format!(
                "Invalid dimensions: expected {}x{}, got {}x{}",
                WIDTH,
                HEIGHT,
                img.width(),
                img.height()
            ),
        );
    }

    let rgb_img = img.to_rgb8();
    let mut display = match state.display.lock() {
        Ok(d) => d,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to acquire display lock: {}", e),
            );
        }
    };

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let pixel = rgb_img.get_pixel(x, y);
            let color = map_color([pixel[0], pixel[1], pixel[2]]);
            if let Err(e) = display.set_pixel(x as usize, y as usize, color) {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to set pixel: {}", e),
                );
            }
        }
    }

    println!("Updating display (~32 seconds)...");
    if let Err(e) = display.show() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to update display: {}", e),
        );
    }

    println!("Display updated successfully");
    (StatusCode::OK, "Display updated successfully".to_string())
}
