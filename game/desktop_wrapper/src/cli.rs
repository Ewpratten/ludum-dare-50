//! This module contains some code for handling CLI flags
use clap::Parser;

/// Ludum Dare 50 game
#[derive(Parser, Debug)]
#[clap( version, about, long_about = None)]
pub struct Args {
    /// Use verbose logging
    #[clap(short, long)]
    pub verbose: bool,
}
