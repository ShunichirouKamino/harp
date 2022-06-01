mod cli;

use cli::command::CommandLineArgs;
use structopt::StructOpt;

/// # main entry
fn main() {
    let CommandLineArgs { action, input_file } = CommandLineArgs::from_args();
}
