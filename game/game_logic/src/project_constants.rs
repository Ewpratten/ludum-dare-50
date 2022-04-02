//! The rust side of the `dist/project-constants.json` file
//! 
//! ## Overview
//! 
//! This file contains a structure defining all data we want to load from the project constants file.
//! Feel free to add anything you'd like here, just make sure the relavant data is also written in the JSON file so the game doesn't crash.
//! You can treat these as constants. I just prefer storing this kind of data in JSON rather than hard-coding it in the program.
//! 
//! ## How this is loaded
//! 
//! Somewhere in `lib.rs`, a call is made to load this through the `asset_manager`. 
//! Its all already set up, so you shouldn't have to worry about the logistics.

use std::collections::HashMap;

use serde::Deserialize;

/// Constants relating to Discord
#[derive(Debug, Deserialize)]
pub struct DiscordConstants {
    /// The Discord application ID
    pub app_id: i64,

    /// Artwork name mapping
    pub artwork: HashMap<String, String>,
    
    /// Strings
    pub strings: HashMap<String, String>,
}

/// Constants relating to the Player
#[derive(Debug, Deserialize)]
pub struct PlayerConstants {

    /// Maximum velocity, tiles per second
    pub max_velocity: u32,

    /// Acceleration, tiles per second per second
    pub acceleration: u32,

    /// Deceleration, tiles per second per second
    pub deceleration: u32,

    /// Starting size of player in tiles
    pub start_size: f32,
}

/// This structure is filled with the contents of `dist/project-constants.json` at runtime
#[derive(Debug, Deserialize)]
pub struct ProjectConstants {
    /// The name of the game
    pub game_name: String,

    /// The window size to use on launch
    pub base_window_size: (u32, u32),

    /// The Discord constants
    pub discord: DiscordConstants,

    /// The Player constants
    pub player: PlayerConstants,

    /// The target framerate of the game
    pub target_fps: u32,

    /// The size of the game tiles
    pub tile_size: u32,
}
