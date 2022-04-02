//! This file is the main entry point for the game logic.
//!
//! ## Overview
//!
//! The main function in this module is `entrypoint()`. This is called from `desktop_wrapper` to start the game.
//!
//! This module also includes all the other sub-modules of the game. If you are viewing this document from the web, click on the modules below to see more info.
//!
//! ## Programming Guide
//!
//! The game code is split into two parts: the core code, and the actual game logic.
//!
//! [@ewpratten](https://github.com/ewpratten) has written most of the core code to bootstrap the game, and provide convenience functions.
//! This stuff probably won't need to be touched.
//! Most of the game logic is expected to live in `src/scenes` and `src/model` (rendering and data).
//!
//! ## Important Functions and Files
//!
//! - If you are wanting to write rendering code, check out [`process_ingame_frame`](scenes/fn.process_ingame_frame.html).
//! - If you want to have something load at the start of the game and stay in memory, check out [`GlobalResources`](global_resource_package/struct.GlobalResources.html).
//! - If you want to add data to the save state file or settings file, check out the [`persistent`](persistent/index.html) module.
#![doc(issue_tracker_base_url = "https://github.com/Ewpratten/ludum-dare-50/issues/")]

use crate::{
    asset_manager::load_json_structure,
    discord::{DiscordRpcSignal, DiscordRpcThreadHandle},
    project_constants::ProjectConstants,
};

#[macro_use]
extern crate approx; // For the macro `relative_eq!`
#[macro_use]
extern crate log; // For the `info!`, `warn!`, etc. macros

pub(crate) mod asset_manager;
pub(crate) mod discord;
pub(crate) mod global_resource_package;
pub(crate) mod persistent;
pub(crate) mod project_constants;
pub(crate) mod rendering;
pub(crate) mod scenes;
pub(crate) mod model;

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

    // Connect to Discord
    let discord = DiscordRpcThreadHandle::new(project_constants.discord.app_id)
        .await
        .expect("Failed to connect to Discord RPC");
    let event_loop_discord_tx = discord.get_channel();
    let discord_task_handle = discord.begin_thread_non_blocking();

    // Blocking call to the graphics rendering loop.
    rendering::event_loop::handle_graphics_blocking(
        |builder| {
            builder
                .msaa_4x()
                // .vsync()
                .title(project_constants.game_name.as_str())
                .height(project_constants.base_window_size.1 as i32)
                .width(project_constants.base_window_size.0 as i32);
        },
        project_constants.target_fps,
        &project_constants,
        event_loop_discord_tx,
    )
    .await;

    // Clean up any resources
    settings
        .save()
        .expect("Could not save game settings to disk.");
    save_state
        .save()
        .expect("Could not save game save state to disk.");
    discord_task_handle.abort();
}
