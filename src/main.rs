use clap::Parser;
use image::{self, GenericImageView};
use std::path::PathBuf;

/// Lightweight image to ASCII art conversion tool
#[derive(Parser, Debug)]
#[command(name = "ascii_art_generator", version, about)]
struct Args {
    #[arg(short = 'W', long)]
    width: Option<u32>,
    #[arg(short = 'H', long)]
    height: Option<u32>,
    target: PathBuf,
}

struct AsciiDimensions {
    width: u32,
    height: u32,
}

fn main() {
    let args = Args::parse();
}

fn fetch_image(args: Args) -> Result<image::GrayImage, Box<dyn std::error::Error>> {
    let img = image::ImageReader::open(args.target)?.decode()?;
    let gray = img.to_luma8();
    Ok(gray)
}

fn calculate_aspect_ratio(width: u32, height: u32) -> f32 {
    let ratio = height as f32 / width as f32;
    ratio
}

fn calculate_ascii_dimensions(
    args: Args,
    img_width: u32,
    img_height: u32,
) -> Result<AsciiDimensions, Box<dyn std::error::Error>> {
    match (args.width, args.height) {
        (Some(w), Some(h)) => Ok(AsciiDimensions {
            width: w,
            height: h,
        }),
        (None, None) => {
            // defaults
            const PIXEL_WIDTH_PER_CHAR: u32 = 8;
            const PIXEL_HEIGHT_PER_CHAR: u32 = 16;

            let ascii_width = img_width / PIXEL_WIDTH_PER_CHAR;
            let ascii_height = img_height / PIXEL_HEIGHT_PER_CHAR;
            Ok(AsciiDimensions {
                width: ascii_width,
                height: ascii_height,
            })
        }
        (Some(_), None) | (None, Some(_)) => Err("Mismatched arguments provided".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_format_test() {
        let args = Args::parse_from(["ascii_art_generator", "-W", "256", "-H", "256", "image.png"]);
        println!("{:?}", args);

        assert_eq!(args.width, Some(256));
        assert_eq!(args.height, Some(256));
        assert_eq!(args.target, PathBuf::from("image.png"));
    }

    #[test]
    fn fetch_image_test() {
        let args = Args::parse_from([
            "ascii_art_generator",
            "-W",
            "256",
            "-H",
            "256",
            "/home/callum/dev/ascii_art_generator/tests/fixtures/sample1.png",
        ]);
        let gray_img = fetch_image(args).unwrap();
        let raw = gray_img.into_raw();

        assert_eq!(
            raw,
            Vec::from([
                0, 0, 255, 255, 0, 0, 255, 255, 0, 0, 255, 255, 0, 0, 255, 255
            ])
        );
    }
}
