//! Global resources
//!
//! ## Overview
//!
//! This module contains a structure for all resources that are needed through the whole game (sounds, fonts, etc.).
//! These are automatically loaded during the first loading screen, and are then passed around the game as needed.
//!
//! ## How this is loaded
//!
//! The resources are loaded via [`asset_manager`](./asset_manager/index.html) in their own thread so we do not block the renderer.

use poll_promise::Promise;
use raylib::{RaylibHandle, RaylibThread, audio::Sound};

use crate::asset_manager::load_sound_from_internal_data;

/// Global resource package
#[derive(Debug)]
pub struct GlobalResources {
    pub button_click_sound: Sound
}

impl GlobalResources {
    /// Load the resources (**blocking**)
    ///
    /// This should not be called more than once.
    pub async fn load(
        raylib: &mut RaylibHandle,
        rl_thread: &RaylibThread,
    ) -> Self {

        // Load the button click sound
        let button_click_sound = load_sound_from_internal_data("assets/audio/button_click.ogg").unwrap();

        Self {
            button_click_sound
        }
    }
}
