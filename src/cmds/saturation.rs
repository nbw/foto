use crate::utils::color;

use std::simd::Simd;

use anyhow::Result;
use image::{DynamicImage, ImageBuffer, RgbImage};

#[derive(Clone)]
pub enum SaturationType {
    Hsv,
    Luminance,
    LuminanceSimd,
}

impl std::str::FromStr for SaturationType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "hsv" => Ok(SaturationType::Hsv),
            "lum" => Ok(SaturationType::Luminance),
            "lumsimd" => Ok(SaturationType::LuminanceSimd),
            _ => anyhow::bail!("Invalid saturation type. Use 'hsv' or 'lum' or 'lumsimd'"),
        }
    }
}

fn apply_luminance_saturation(img: DynamicImage, saturation: f32) -> Result<DynamicImage> {
    let mut out_img: RgbImage = ImageBuffer::new(img.width(), img.height());

    for (x, y, pixel) in img.to_rgb8().enumerate_pixels() {
        let [r, g, b] = pixel.0;
        let r_f = r as f32;
        let g_f = g as f32;
        let b_f = b as f32;
        let luminance = 0.299 * r_f + 0.587 * g_f + 0.114 * b_f;

        let new_pixel = image::Rgb([
            (luminance + (r_f - luminance) * saturation) as u8,
            (luminance + (g_f - luminance) * saturation) as u8,
            (luminance + (b_f - luminance) * saturation) as u8,
        ]);
        out_img.put_pixel(x, y, new_pixel);
    }

    Ok(DynamicImage::ImageRgb8(out_img))
}

fn apply_luminance_saturation_simd(img: DynamicImage, saturation: f32) -> Result<DynamicImage> {
    let mut out_img_raw = vec![0u8; img.to_rgb8().into_raw().len()];

    let lanes = 4; // simd::simd_lane_count();

    for (chunk_idx, rgb_chunk) in img
        .to_rgb32f()
        .into_raw()
        .chunks_exact(lanes * 3)
        .enumerate()
    {
        let chunk_start = chunk_idx * lanes * 3;
        let mut r_vals = [0f32; 4];
        let mut g_vals = [0f32; 4];
        let mut b_vals = [0f32; 4];

        for i in 0..lanes {
            r_vals[i] = rgb_chunk[i * 3] as f32;
            g_vals[i] = rgb_chunk[i * 3 + 1] as f32;
            b_vals[i] = rgb_chunk[i * 3 + 2] as f32;
        }

        let r_f = Simd::from_array(r_vals);
        let g_f = Simd::from_array(g_vals);
        let b_f = Simd::from_array(b_vals);

        let luminance =
            Simd::splat(0.299) * r_f + Simd::splat(0.587) * g_f + Simd::splat(0.114) * b_f;

        let sat: Simd<f32, 4> = Simd::splat(saturation);

        let r_out = luminance + (r_f - luminance) * sat;
        let g_out = luminance + (g_f - luminance) * sat;
        let b_out = luminance + (b_f - luminance) * sat;

        for i in 0..lanes {
            out_img_raw[chunk_start + i * 3] = (r_out[i].clamp(0.0, 1.0) * 255.0) as u8;
            out_img_raw[chunk_start + i * 3 + 1] = (g_out[i].clamp(0.0, 1.0) * 255.0) as u8;
            out_img_raw[chunk_start + i * 3 + 2] = (b_out[i].clamp(0.0, 1.0) * 255.0) as u8;
        }
    }

    let out_img = image::RgbImage::from_raw(img.width(), img.height(), out_img_raw)
        .expect("Failed to create image");

    Ok(image::DynamicImage::ImageRgb8(out_img))
}

fn apply_hsv_saturation(img: DynamicImage, saturation: f32) -> Result<DynamicImage> {
    let mut out_img: RgbImage = ImageBuffer::new(img.width(), img.height());

    for (x, y, pixel) in img.to_rgb8().enumerate_pixels() {
        let [r, g, b] = pixel.0;
        let (h, s_base, v) = color::rgb_to_hsv(r, g, b);
        let s = (s_base * saturation).clamp(0.0, 1.0);

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
    saturation: f32,
    sat_type: SaturationType,
) -> Result<DynamicImage> {
    if saturation <= 0.0 {
        anyhow::bail!("Saturation amount must be greater than 0");
    }

    match sat_type {
        SaturationType::Hsv => apply_hsv_saturation(img, saturation),
        SaturationType::Luminance => apply_luminance_saturation(img, saturation),
        SaturationType::LuminanceSimd => apply_luminance_saturation_simd(img, saturation),
    }
}
