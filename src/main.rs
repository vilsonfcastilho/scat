use clap::{arg, Parser};
use image::{DynamicImage, GenericImageView};
use std::path::Path;

const ASCII_CHARS: &[u8] = b"@%#*+=-:. "; // Gradient from dark to light

#[derive(Parser)]
struct Args {
    path: String,

    #[arg(short, long, default_value_t = 80)]
    width: u32,
}

fn main() {
    let args: Args = Args::parse();
    let img_path: &Path = Path::new(&args.path);

    let img: DynamicImage = match image::open(&img_path) {
        Ok(img) => img,
        Err(e) => {
            eprintln!("Error loading image: {}", e);
            return;
        }
    };

    let ascii_art: String = image_to_ascii(img, args.width);

    println!("{}", ascii_art);
}

/// Converts an image to ASCII art
fn image_to_ascii(img: DynamicImage, width: u32) -> String {
    let (orig_width, orig_height) = img.dimensions();
    let aspect_ratio: f32 = orig_height as f32 / orig_width as f32;
    let height: u32 = (width as f32 * aspect_ratio) as u32 / 2; // Adjust for terminal aspect ratio

    let resized_img: DynamicImage =
        img.resize_exact(width, height, image::imageops::FilterType::Nearest);
    let grayscale: image::ImageBuffer<image::Luma<u8>, Vec<u8>> = resized_img.to_luma8();

    let mut ascii_art: String = String::new();
    for y in 0..grayscale.height() {
        for x in 0..grayscale.width() {
            let pixel: &image::Luma<u8> = grayscale.get_pixel(x, y);
            let intensity: f32 = pixel[0] as f32 / 255.0;
            let char_index: usize = (intensity * (ASCII_CHARS.len() - 1) as f32).round() as usize;
            ascii_art.push(ASCII_CHARS[char_index] as char);
        }
        ascii_art.push('\n');
    }

    ascii_art
}
