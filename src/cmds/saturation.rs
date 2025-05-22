use crate::utils::color;
use anyhow::Result;
use image::{DynamicImage, ImageBuffer, RgbImage};

#[derive(Clone)]
pub enum SaturationType {
    Hsv,
    Luminance,
}

impl std::str::FromStr for SaturationType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "hsv" => Ok(SaturationType::Hsv),
            "luma" => Ok(SaturationType::Luminance),
            _ => anyhow::bail!("Invalid saturation type. Use 'hsv' or 'luma'"),
        }
    }
}

fn apply_hsv_saturation(img: DynamicImage, amount: f32) -> Result<DynamicImage> {
    let mut out_img: RgbImage = ImageBuffer::new(img.width(), img.height());

    for (x, y, pixel) in img.to_rgb8().enumerate_pixels() {
        let [r, g, b] = pixel.0;
        let (h, s_base, v) = color::rgb_to_hsv(r, g, b);
        let s = (s_base * amount).clamp(0.0, 1.0);

        let (r, g, b) = color::hsv_to_rgb(h, s, v);
        let new_pixel = image::Rgb([r, g, b]);
        out_img.put_pixel(x, y, new_pixel);
    }

    Ok(DynamicImage::ImageRgb8(out_img))
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
pub fn apply_saturation(
    img: DynamicImage,
    amount: f32,
    sat_type: SaturationType,
) -> Result<DynamicImage> {
    if amount <= 0.0 {
        anyhow::bail!("Saturation amount must be greater than 0");
    }

    match sat_type {
        SaturationType::Hsv => apply_hsv_saturation(img, amount),
        SaturationType::Luminance => apply_hsv_saturation(img, amount),
    }
}
