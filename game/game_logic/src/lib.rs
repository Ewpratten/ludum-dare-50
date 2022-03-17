//! This file is the main entry point for the game logic.

pub(crate) mod persistent;

/// This is the game logic entrypoint. Despite being async,
/// this is expected to block the main thread for rendering and stuff.
pub async fn entrypoint() {
    log::info!("Game main thread handed off to logic crate.");

    // Load the game settings
    let mut settings = persistent::settings::PersistentGameSettings::load_or_create()
        .expect("Failed to parse game settings from disk. Possibly corrupt file?");
    
    // Load the game save state
    let mut save_state = persistent::save_state::GameSaveState::load_or_create()
        .expect("Failed to parse game save state from disk. Possibly corrupt file?");
}
