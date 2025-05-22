use std::path::PathBuf;
use std::str::FromStr;

use anyhow::Result;
use clap::{Parser, Subcommand};

use foto::cmds;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Adjust the contrast of an image
    Contrast {
        /// Path to the input image
        #[arg(short, long)]
        input: PathBuf,

        /// Path to the output image
        #[arg(short, long)]
        output: PathBuf,

        /// Contrast ratio (must be greater than 0)
        #[arg(short, long)]
        ratio: f32,

        /// Threshold value (between 0 and 256)
        #[arg(short, long, default_value = "128.0")]
        threshold: f32,
    },
    Saturation {
        /// Path to the input image
        #[arg(short, long)]
        input: PathBuf,

        /// Path to the output image
        #[arg(short, long)]
        output: PathBuf,

        /// Saturation amount (must be greater than 0)
        #[arg(short, long)]
        amount: f32,

        /// Saturation type (hsv or luma)
        #[arg(short = 't', long = "type", default_value = "hsv")]
        sat_type: String,
    },
}

pub fn process_contrast_command(
    input: PathBuf,
    output: PathBuf,
    ratio: f32,
    threshold: f32,
) -> Result<()> {
    if threshold <= 0.0 || threshold >= 256.0 {
        anyhow::bail!("Threshold must be between 1 and 255");
    }

    // Read the input image
    let img = image::open(&input)?;

    // Process the image
    let processed_img = cmds::contrast::apply_contrast(img, ratio, threshold)?;

    // Save the processed image
    processed_img.save(&output)?;

    println!("Image processed successfully!");
    println!("Input: {}", input.display());
    println!("Output: {}", output.display());
    println!("Contrast ratio: {}", ratio);
    println!("Threshold: {}", threshold);

    Ok(())
}

pub fn process_saturation_command(
    input: PathBuf,
    output: PathBuf,
    amount: f32,
    sat_type: String,
) -> Result<()> {
    // Read the input image
    let img = image::open(&input)?;

    // Convert string to SaturationType
    let sat_type = cmds::saturation::SaturationType::from_str(&sat_type)?;

    // Process the image
    let processed_img = cmds::saturation::apply_saturation(img, amount, sat_type.clone())?;

    // Save the processed image
    processed_img.save(&output)?;

    println!("Image processed successfully!");
    println!("Input: {}", input.display());
    println!("Output: {}", output.display());
    println!("Saturation amount: {}", amount);
    println!(
        "Saturation type: {}",
        match sat_type {
            cmds::saturation::SaturationType::Hsv => "hsv",
            cmds::saturation::SaturationType::Luminance => "luma",
        }
    );

    Ok(())
}
