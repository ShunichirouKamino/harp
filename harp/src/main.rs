mod cli;

use std::path::PathBuf;

use anyhow::anyhow;
use cli::command::{CommandLineArgs, TargetMermaid};
use structopt::StructOpt;

/// # main entry
fn main() -> anyhow::Result<()> {
    let CommandLineArgs { action, output } = CommandLineArgs::from_args();

    let output_file = output
        .or_else(find_default_out_path)
        .ok_or(anyhow!("Failed to find default path."))?;

    match action {
        Convert(TargetMermaid { input }) => todo!("impl"),
    }?;

    Ok(())
}

/// # Default output file path
///
fn find_default_out_path() -> Option<PathBuf> {
    let output_file = "harp-out";
    let pusher = |mut path: PathBuf| {
        path.push(output_file);
        println!("Target persist file: {:?}", path);
        path
    };
    home::home_dir().map(pusher)
}
