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

// Returns GrayImage based on the provided target image
fn fetch_image(args: Args) -> Result<image::GrayImage, Box<dyn std::error::Error>> {
    let img = image::ImageReader::open(args.target)?.decode()?;
    let gray = img.to_luma8();
    Ok(gray)
}

// Returns target dimensions for ASCII art based on aspect ratio of given image unless specific target dimensions are specified
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

fn sample_image(img: image::GrayImage, ascii_dimensions: AsciiDimensions) -> Vec<u8> {
    let ascii_w = ascii_dimensions.width;
    let ascii_h = ascii_dimensions.height;

    let (img_w, img_h) = img.dimensions();

    let mut raw_ascii_chroma: Vec<u8> = Vec::with_capacity(ascii_w as usize * ascii_h as usize);

    for grid_y in 0..ascii_h {
        let y_start = grid_y * img_h / ascii_h; // start pixel index for cell on y axis
        let y_end = (grid_y + 1) * img_h / ascii_h; // end pixel index for cell on y axis
        let centre_y = (y_start + (y_end - y_start) / 2).min(img_h.saturating_sub(1));

        for grid_x in 0..ascii_w {
            let x_start = grid_x * img_w / ascii_w;
            let x_end = (grid_x + 1) * img_w / ascii_w;
            let centre_x = (x_start + (x_end - x_start) / 2).min(img_w.saturating_sub(1));

            raw_ascii_chroma.push(img.get_pixel(centre_x, centre_y).0[0]);
        }
    }

    raw_ascii_chroma
}

fn brightness_to_char(brightness: u8) -> char {
    const CHARSET: &[char] = &['@', '%', '#', '*', '+', '=', '-', ':', '.', ' '];
    let bucket_width = 256 / CHARSET.len();
    let index = (brightness as usize / bucket_width).min(CHARSET.len() - 1);
    CHARSET[index]
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
    #[test]
    fn sample_image_test() {
        let img = image::GrayImage::from_fn(256, 256, |x, y| {
            if x < 128 {
                image::Luma([0u8])
            } else {
                image::Luma([255u8])
            }
        });
        let dimensions = AsciiDimensions {
            width: 32,
            height: 16,
        };
        let samples = sample_image(img, dimensions);

        let mut row = vec![0u8; 16];
        row.extend(vec![255u8; 16]);
        let expected = row.repeat(16);

        assert_eq!(samples, expected);
    }
}
