mod cli;
mod closing_thread;
mod colors;
mod desktop;
mod drawer;
mod image_converter;

use desktop::Desktop;
use drawer::Drawer;
use image::{DynamicImage, ImageFormat};
use std::process::{self, Command};

fn main() {
    let (drawing_area, color_box, dither, checkerboard, batch_colors, delay, timeout) =
        cli::get_cli();

    closing_thread::start();

    let desktop = Desktop::new();
    let mut drawer = Drawer::new(
        drawing_area.x,
        drawing_area.y,
        checkerboard,
        batch_colors,
        delay,
        timeout,
        color_box,
    );

    let image = image_from_clipboard();
    let converted =
        image_converter::convert(image, dither, drawing_area.width, drawing_area.height);

    drawer.draw(&desktop, &converted);
}

fn image_from_clipboard() -> DynamicImage {
    let xclip = Command::new("sh")
        .arg("-c")
        .arg("xclip -o -target image/png -selection clipboard")
        .output()
        .expect("failed to execute process");

    match image::load_from_memory_with_format(&xclip.stdout, ImageFormat::PNG) {
        Ok(image) => image,
        _ => {
            println!(
                "Failed to load image from clipboard: {}",
                String::from_utf8_lossy(&xclip.stderr)
            );
            process::exit(1);
        }
    }
}
