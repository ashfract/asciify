use clap::Parser;
use image::{self, GenericImageView};
use std::path::PathBuf;

/// Lightweight image to ASCII art conversion tool
#[derive(Parser, Debug, Clone)]
#[command(name = "ascii_art_generator", version, about)]
struct Args {
    #[arg(short = 'W', long)]
    width: Option<u32>,
    #[arg(short = 'H', long)]
    height: Option<u32>,
    target: PathBuf,
}

#[derive(Clone)]
struct AsciiDimensions {
    width: u32,
    height: u32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let image = fetch_image(args.clone())?;
    let ascii_dimensions = calculate_ascii_dimensions(args.clone(), image.width(), image.height())?;
    let samples = sample_image(image, ascii_dimensions.clone());
    let ascii = convert_to_ascii(samples, ascii_dimensions);

    let result: String = ascii.iter().collect();
    println!("{}", result);

    Ok(())
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
            const MAX_ASCII_WIDTH: u32 = 100;

            let mut ascii_width = img_width / PIXEL_WIDTH_PER_CHAR;
            let mut ascii_height = img_height / PIXEL_HEIGHT_PER_CHAR;
            if ascii_width > MAX_ASCII_WIDTH {
                let scale_factor: f32 = MAX_ASCII_WIDTH as f32 / ascii_width as f32;
                ascii_height = (ascii_height as f32 * scale_factor) as u32;
                ascii_width = (ascii_width as f32 * scale_factor) as u32;
            }

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

fn convert_to_ascii(luma_values: Vec<u8>, ascii_dimensions: AsciiDimensions) -> Vec<char> {
    let chars: Vec<char> = luma_values
        .into_iter()
        .map(|brightness| brightness_to_char(brightness))
        .collect();
    let mut result: Vec<char> = Vec::new();
    for (index, char) in chars.into_iter().enumerate() {
        if index > 0 && index % ascii_dimensions.width as usize == 0 {
            result.push('\n');
        }
        result.push(char);
    }
    result
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
    #[test]
    fn brightness_to_char_test() {
        assert_eq!(brightness_to_char(0), '@');
        assert_eq!(brightness_to_char(24), '@');
        assert_eq!(brightness_to_char(25), '%');
        assert_eq!(brightness_to_char(26), '%');
        assert_eq!(brightness_to_char(250), ' ');
        assert_eq!(brightness_to_char(255), ' ');
    }
    #[test]
    fn convert_to_ascii_test() {
        let ascii_dimensions = AsciiDimensions {
            width: 32,
            height: 16,
        };
        let mut row = vec![0u8; 16];
        row.extend(vec![255u8; 16]);
        let luma = row.repeat(16);

        let result = convert_to_ascii(luma, ascii_dimensions);

        let mut row = vec!['@'; 16];
        row.extend(vec![' '; 16]);
        row.extend(vec!['\n']);
        let mut expected = row.repeat(16);
        expected.pop();

        assert_eq!(result, expected);
    }
}
