mod cli;

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let cli = cli::Cli::parse();

    match cli.command {
        cli::Commands::Contrast {
            input,
            output,
            ratio,
            threshold,
        } => cli::process_contrast_command(input, output, ratio, threshold),
        cli::Commands::Saturation {
            input,
            output,
            saturation,
            sat_type,
        } => cli::process_saturation_command(input, output, saturation, sat_type),
        cli::Commands::CliReadme {} => cli::add_cli_cmd_to_readme(),
    }
}
