use clap::Parser;
use std::path::PathBuf;

/// Lightweight image to ASCII art conversion tool
#[derive(Parser, Debug)]
#[command(name = "ascii_art_generator", version, about)]
struct Args {
    #[arg(short = 'W', long)]
    width: u32,
    #[arg(short = 'H', long)]
    height: u32,
    target: PathBuf,
}

fn main() {}
