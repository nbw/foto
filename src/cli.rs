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
        #[arg(short, long = "sat")]
        saturation: f32,

        /// Saturation type (hsv or luma)
        #[arg(short = 't', long = "type", default_value = "hsv")]
        sat_type: String,
    },
    #[command(hide = true)]
    CliReadme {},
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
    saturation: f32,
    sat_type: String,
) -> Result<()> {
    // Read the input image
    let img = image::open(&input)?;

    // Convert string to SaturationType
    let sat_type = cmds::saturation::SaturationType::from_str(&sat_type)?;

    // Process the image
    let processed_img = cmds::saturation::apply_saturation(img, saturation, sat_type.clone())?;

    // Save the processed image
    processed_img.save(&output)?;

    println!("Image processed successfully!");
    println!("Input: {}", input.display());
    println!("Output: {}", output.display());
    println!("Saturation amount: {}", saturation);
    println!(
        "Saturation type: {}",
        match sat_type {
            cmds::saturation::SaturationType::Hsv => "hsv",
            cmds::saturation::SaturationType::Luminance => "luma",
            cmds::saturation::SaturationType::LuminanceSimd => "luma simd",
        }
    );

    Ok(())
}

pub fn add_cli_cmd_to_readme() -> Result<()> {
    let md = clap_markdown::help_markdown::<Cli>();

    // Read the README.md file
    let mut content = std::fs::read_to_string("README.md")?;

    // Find the start and end markers
    let start = "<!-- start: CLI USAGE -->";
    let end = "<!-- end: CLI USAGE -->";

    // Replace content between markers
    let start_idx = content
        .find(start)
        .ok_or_else(|| anyhow::anyhow!("Could not find start marker"))?;
    let end_idx = content
        .find(end)
        .ok_or_else(|| anyhow::anyhow!("Could not find end marker"))?;

    content.replace_range((start_idx + start.len())..end_idx, &format!("\n\n{}\n", md));

    // Write back to README.md
    std::fs::write("README.md", content)?;

    println!("Markdown help written to README.md");

    Ok(())
}
