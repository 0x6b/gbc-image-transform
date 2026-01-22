use clap::Parser;

#[derive(Debug, Parser)]
#[clap(about, version)]
pub struct Args {
    /// Path to the image to be processed
    #[clap()]
    pub input: String,

    /// Path to the output image
    #[clap(short, long, default_value = "output.png")]
    pub output: String,

    /// Pixelation factor. Larger values result in more pixelation
    #[clap(short, long, default_value = "4")]
    pub pixelation_factor: u32,

    /// Number of colors to use
    #[clap(short, long, default_value = "56")]
    pub num_colors: usize,

    /// Whether to include transparent pixels in the color palette
    #[clap(short, long)]
    pub transparent: bool,

    /// Width of the output image. If only width is specified, height is calculated to maintain
    /// aspect ratio.
    #[clap(short = 'W', long)]
    pub width: Option<u32>,

    /// Height of the output image. If only height is specified, width is calculated to maintain
    /// aspect ratio.
    #[clap(short = 'H', long)]
    pub height: Option<u32>,
}
