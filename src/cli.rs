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
    /// Move target workspace to active monitor. Use the postion of the mouse
    /// to determine the active monitor.
    ///
    /// If the target is not on the active monitor, it will be moved to the
    /// active monitor. If the target workspace is the primary active, it will
    /// be swapped with the active workspace.
    Move { target: i32 },
}

/// Parse command line arguments
pub fn parse() -> Args {
    Args::parse()
}
