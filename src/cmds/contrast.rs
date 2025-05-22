use anyhow::Result;

use image::{DynamicImage, ImageBuffer, RgbImage};

fn adjust_pixel_contrast(value: u8, factor: f32, threshold: f32) -> u8 {
    let val = (value as f32 - threshold) * factor + threshold;
    val.clamp(0.0, 255.0) as u8
}

/// Adjusts the contrast of an image
///
/// # Arguments
/// * `img` - The input image
/// * `ratio` - The contrast ratio (must be greater than 0)
/// * `threshold` - The threshold value (between 0 and 256)
///
/// # Returns
/// A new image with adjusted contrast
pub fn apply_contrast(img: DynamicImage, ratio: f32, threshold: f32) -> Result<DynamicImage> {
    if ratio <= 0.0 {
        anyhow::bail!("Contrast ratio must be greater than 0");
    }

    if threshold <= 0.0 || threshold >= 256.0 {
        anyhow::bail!("Threshold must be between 1 and 255");
    }

    let mut out_img: RgbImage = ImageBuffer::new(img.width(), img.height());

    for (x, y, pixel) in img.to_rgb8().enumerate_pixels() {
        let [r, g, b] = pixel.0;
        let new_pixel = image::Rgb([
            adjust_pixel_contrast(r, ratio, threshold),
            adjust_pixel_contrast(g, ratio, threshold),
            adjust_pixel_contrast(b, ratio, threshold),
        ]);
        out_img.put_pixel(x, y, new_pixel);
    }

    Ok(DynamicImage::ImageRgb8(out_img))
}
