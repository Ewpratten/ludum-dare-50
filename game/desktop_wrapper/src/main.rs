//! # This file is probably not what you are looking for.
//!
//! In order to keep compile times reasonable, this crate is split into a `bin` and `lib` part, this being `bin`.
//! All this crate is designed to do is bootstrap the game, and call `game_logic::entrypoint()` to *actually* start the game.

use clap::StructOpt;
use log::LevelFilter;

mod cli;
mod debug_profiling;
mod logging;

#[tokio::main]
async fn main() {
    // Set up CLI args
    let args = cli::Args::parse();

    // Enable profiling
    let _profile_handle = debug_profiling::init_profiling();

    // Set up logging
    logging::init_logging_system(
        "ldjam50",
        match args.verbose {
            true => Some(LevelFilter::Debug),
            false => None,
        },
    )
    .expect("Failed to initialize logging system");

    // Start the game
    log::info!("Starting game");
    game_logic::entrypoint(args.force_recreate_savefiles).await;
    log::info!("Goodbye!");
}
