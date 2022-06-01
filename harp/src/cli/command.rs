use std::path::PathBuf;

use structopt::StructOpt;

/// # Command-line action
///
#[derive(Debug, StructOpt)]
pub enum Action {
    /// Write a memver to the journal file.
    Convert(TargetMermaid),
}

/// # Target and output name
///
#[derive(StructOpt, Debug)]
pub struct TargetMermaid {
    #[structopt()]
    pub input: PathBuf,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "harp",
    about = "This application issues DDL from the ER diagram described in `mermaid`."
)]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,

    #[structopt(parse(from_os_str), short, long)]
    pub output: Option<PathBuf>,
}
