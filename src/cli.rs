use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Show version & git build revision
    #[arg(long, short, default_value_t = false)]
    pub version: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Move current window to a workspace
    Move { target: i32 },
}

/// Parse command line arguments
pub fn parse() -> Args {
    Args::parse()
}
