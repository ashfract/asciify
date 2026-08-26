use clap::Parser;
use image::{self, GenericImageView};
use std::path::PathBuf;

/// Lightweight image to ASCII art conversion tool
#[derive(Parser, Debug)]
#[command(name = "ascii_art_generator", version, about)]
struct Args {
    #[arg(short = 'W', long)]
    width: u32,
    #[arg(short = 'H', long)]
    height: Option<u32>,
    target: PathBuf,
}

fn main() {
    let args = Args::parse();
}

fn fetch_image(args: Args) -> Result<image::GrayImage, Box<dyn std::error::Error>> {
    let img = image::ImageReader::open(args.target)?.decode()?;
    let grey = img.to_luma8();
    Ok(grey)
}

mod tests {
    use super::*;

    #[test]
    fn parse_args() {
        let args = Args::parse_from(["ascii_art_generator", "-W", "100", "-H", "50", "image.png"]);
        println!("{:?}", args);

        assert_eq!(args.width, 100);
        assert_eq!(args.height, Some(50));
        assert_eq!(args.target, PathBuf::from("image.png"));
    }
}
