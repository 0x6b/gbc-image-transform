# gbc-image-transform

A CLI to generate Game Boy Color lookalike images from an image.

The Game Boy Color can display any combination of 32,768 different colors (15-bit RGB), but at any given time, it can display only up to 56 different colors (ref.: [Game Boy Color - Wikipedia](https://en.wikipedia.org/wiki/Game_Boy_Color#Technical_specifications)).

- Palette colors available: 32,768 (15-bit)
- Colors on screen: Supports 10, 32 or 56

## Installation

```console
$ cargo install --git https://github.com/0x6b/gbc-image-transform
```

## Usage

```console
$ gbc-image-transform --help
Generate Game Boy Color lookalike image from an image.

Usage: gbc-image-transform [OPTIONS] <INPUT>

Arguments:
  <INPUT>  Path to the image to be processed

Options:
  -o, --output <OUTPUT>
          Path to the output image [default: output.png]
  -p, --pixelation-factor <PIXELATION_FACTOR>
          Pixelation factor. Larger values result in more pixelation [default: 4]
  -n, --num-colors <NUM_COLORS>
          Number of colors to use [default: 56]
  -t, --transparent
          Whether to include transparent pixels in the color palette
  -h, --help
          Print help
  -V, --version
          Print version
```

### Supported Image Formats

The CLI should support any image format supported by the [image](https://crates.io/crates/image) crate, but tested with JPEG and PNG. The format is determined from the extension of your input file (`<INPUT>`).

## License

MIT. See [LICENSE](LICENSE) for details.
