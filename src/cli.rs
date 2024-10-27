use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Print debugging information
    #[arg(long, default_value_t = false, hide = true)]
    verbose: bool,

    /// Show version & git build revision
    #[arg(long, short, default_value_t = false)]
    pub version: bool
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Move current window to a workspace
    Move { target: i32 },
}

/// Log information with --verbose
pub fn log(text: &str) {
    let Args { verbose, .. } = parse();

    if verbose {
        println!("{text}");
    }
}

/// Parse command line arguments
pub fn parse() -> Args {
    Args::parse()
}
