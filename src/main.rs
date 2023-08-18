mod args;
use crate::args::Args;

use anyhow::Result;
use clap::Parser;
use image::{
    imageops::{resize, FilterType},
    ImageBuffer, Pixel, Rgb, Rgba,
};
use kmeans_colors::get_kmeans;
use palette::{cast::ComponentsAs, FromColor, Srgb, Srgba};
use tracing::info;
use tracing_subscriber::FmtSubscriber;

type Image = ImageBuffer<Rgba<u8>, Vec<u8>>;

fn main() -> Result<()> {
    let Args {
        input,
        output,
        pixelation_factor,
        num_colors,
        transparent,
    } = Args::parse();

    let subscriber = FmtSubscriber::builder().finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    info!("loading image from {}", input);
    let mut image = get_pixelated_image(&input, pixelation_factor)?;
    info!("finding palette");
    let palette = find_palette(&image, num_colors, transparent)?;
    info!("reducing colors");
    reduce_colors(&mut image, &palette);
    info!("saving image to {}", output);
    image.save(output)?;

    Ok(())
}

/// Returns a pixelated version of an image.
///
/// This function opens an image file from the given path, scales it down using the given pixelation
/// factor, and then scales it back up to the original size to create a pixelated effect.
///
/// # Arguments
///
/// - `image_path` - A str representation of the path to the image file to be pixelated.
/// - `pixelation_factor` - A u32 that represents the factor by which the image will be downscaled.
///   Larger values result in more pixelation.
///
/// # Returns
///
/// - `Result<Image>` - A Result wrapping an Image type. On success, contains the pixelated Image.
/// On failure, contains an Error detailing what went wrong.
fn get_pixelated_image(image_path: &str, pixelation_factor: u32) -> Result<Image> {
    let image = image::open(image_path)?.into_rgba8();
    let (width, height) = (image.width(), image.height());

    // Downscale the image to a smaller size
    let small = resize(
        &image,
        width / pixelation_factor,
        height / pixelation_factor,
        FilterType::Nearest,
    );

    // Then upscale it back to the original size to get the pixelated effect
    Ok(resize(&small, width, height, FilterType::Nearest))
}

/// This function aims to find a color palette in an image according to input conditions.
///
/// # Arguments
///
/// - `image` - A reference to the image for which the color palette is to be obtained.
/// - `num_colors` - The desired number of colors in the resulting color palette.
/// - `transparent` - A boolean value that indicates whether transparent pixels should be included
///   in the color palette.
///
/// # Returns
///
/// A `Result` which is `Ok` when the palette could be found successfully. The `Ok` variant wraps a
/// `Vec` of `Rgb`. Each `Rgb` instance represents a color from the palette. In case of an error,
/// the `Err` variant is returned.
fn find_palette(image: &Image, num_colors: usize, transparent: bool) -> Result<Vec<Rgb<u8>>> {
    let img_vec: &[Srgba<u8>] = image.as_raw().components_as();

    let rgb_pixels = img_vec
        .iter()
        .filter(|&pixel| !transparent || pixel.alpha == 255)
        .map(|pixel| Srgb::<f32>::from_color(pixel.into_format::<_, f32>()))
        .collect::<Vec<_>>();

    Ok(get_kmeans(num_colors, 1, 5.0, false, &rgb_pixels, 0)
        .centroids
        .iter()
        .map(|&color| {
            Rgb([
                (color.red * 255f32) as u8,
                (color.green * 255f32) as u8,
                (color.blue * 255f32) as u8,
            ])
        })
        .map(|color| {
            // reduce the color to 5 bits per channel, means 15-bit color
            Rgb([
                (color[0] >> 3) << 3,
                (color[1] >> 3) << 3,
                (color[2] >> 3) << 3,
            ])
        })
        .collect())
}

/// Reduces the colors of an image based on a provided color palette. The pixels of the image
/// are changed in place to the closest color available in the palette.
///
/// # Arguments
///
/// - `image` - A mutable reference to the image that will be reduced in colors.
/// - `palette` - A slice of `Rgb<u8>` color values that will serve as the palette for color
///   reduction.
///
/// # Algorithm
///
/// Each pixel of the image is compared to each color in the palette by calculating the squared
/// distance between the pixel color and the palette color. The color with the minimum distance
/// squared is considered the closest and therefore used as the new color for the pixel.
///
/// If the palette is empty, all pixel colors will become black (`Rgb([0, 0, 0])`).
fn reduce_colors(image: &mut Image, palette: &[Rgb<u8>]) {
    image.enumerate_pixels_mut().for_each(|(_, _, pixel)| {
        let closest_color = palette
            .iter()
            .copied()
            .min_by_key(|&color| compute_squared_distance(&color, &pixel.to_rgb()))
            .unwrap_or_else(|| Rgb([0, 0, 0]));

        *pixel = Rgba([
            closest_color[0],
            closest_color[1],
            closest_color[2],
            pixel[3],
        ]);
    });
}

/// Computes the squared Euclidean distance between two colors.
///
/// It computes the distance using the formula `(dr * dr + dg * dg + db * db)`
/// where `dr`, `dg`, and `db` are the differences of the RGB values of the two colors
///
/// # Arguments
///
/// * `first_color` - An Rgb<u8> color.
/// * `second_color` - An Rgb<u8> color.
///
/// # Returns
///
/// * An `u32` - The computed squared Euclidean distance.
fn compute_squared_distance(first_color: &Rgb<u8>, second_color: &Rgb<u8>) -> u32 {
    // cast to i32 to avoid subtraction overflow
    let red_diff = first_color[0] as i32 - second_color[0] as i32;
    let green_diff = first_color[1] as i32 - second_color[1] as i32;
    let blue_diff = first_color[2] as i32 - second_color[2] as i32;

    (red_diff.pow(2) + green_diff.pow(2) + blue_diff.pow(2)) as u32
}
