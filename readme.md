# Asciify

![Rust](https://img.shields.io/badge/rust-1.94.0%2B-red)
![License](https://img.shields.io/badge/license-MIT-green)

### What is it?

Asciify is a rust-based tool that takes any image and converts it to an ASCII art format.

> **Compatibility:** Should support any file format handled by the image crate, however only PNG has been tested.

### Status

> **Functional:** Asciify is fully functional.

### How does it work?

1. **Image Loading:** The provided target image is decoded and converted to a grayscale format, reducing each pixel to a single value representing its brightness - the result image is determined purely based on luminance.
2. **Dimension Calculation:** Output grid size is determined either using user-specified dimensions (--width and --height flags) or automatically from the image's aspect ratio using a fixed pixels-per-character ratio (8px width, 16px height per character). Images exceeding 100 characters in width are automatically scaled down maintaining their original proportions.
3. **Sampling:** The image is divided into a grid matching the calculated output dimensions, and the centre pixel of each cell is sampled to represent the brightness of that cell which is then mapped to a character.
4. **Character Mapping:** Each cell brightness sample is mapped to a character from a fixed gradient (@%#*+=-:. ).
5. **Assembly:** Mapped characters are joined row by row with line breaks inserted at each boundary, producing the final printable ASCII art.

### Stack

- **Clap:** Command line argument handling.
- **image (Rust Crate):** Decoding image files and converting to grayscale pixel data.

### Limitations

- Only PNG images have been tested
- Image downscaling has not been edge-case tested
- Character gradient is fixed and not easily configurable
- Sampling uses a single centre pixel per cell rather than calculating an average, which in high-resolution images can result in a loss of fine detail
- The 8:16 pixel per character ratio is fixed and may not accurately match all terminal fonts
