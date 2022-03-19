//! This file is the main entry point for the game logic.

use crate::{asset_manager::json::load_json_structure, project_constants::ProjectConstants};

#[macro_use]
extern crate approx; // For the macro `relative_eq!`

pub mod asset_manager;
pub mod discord;
pub mod persistent;
pub mod project_constants;
pub mod rendering;

/// This is the game logic entrypoint. Despite being async,
/// this is expected to block the main thread for rendering and stuff.
///
/// Setting `force_recreate_savefiles` will cause the game to recreate its settings and savestate files.
pub async fn entrypoint(force_recreate_savefiles: bool) {
    log::info!("Game main thread handed off to logic crate.");

    // Load the project constants
    let project_constants: ProjectConstants =
        load_json_structure("project-constants.json").expect("Could not load project constants");

    // Load the game settings
    let mut settings =
        persistent::settings::PersistentGameSettings::load_or_create(force_recreate_savefiles)
            .expect("Failed to parse game settings from disk. Possibly corrupt file?");

    // Load the game save state
    let mut save_state =
        persistent::save_state::GameSaveState::load_or_create(force_recreate_savefiles)
            .expect("Failed to parse game save state from disk. Possibly corrupt file?");

    // Blocking call to the graphics rendering loop.
    rendering::event_loop::handle_graphics_blocking(
        |builder| {
            builder
                .msaa_4x()
                .vsync()
                .title(project_constants.game_name.as_str())
                .height(project_constants.base_window_size.1 as i32)
                .width(project_constants.base_window_size.0 as i32);
        },
        settings.target_fps,
    );

    // Clean up any resources
    settings
        .save()
        .expect("Could not save game settings to disk.");
    save_state
        .save()
        .expect("Could not save game save state to disk.");
}
