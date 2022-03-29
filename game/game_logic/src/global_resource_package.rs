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
use raylib::{RaylibHandle, RaylibThread};

/// Global resource package
#[derive(Debug)]
pub struct GlobalResources {}

impl GlobalResources {
    /// Load the resources (**blocking**)
    ///
    /// This should not be called more than once.
    pub async fn load(
        raylib: &mut RaylibHandle,
        rl_thread: &RaylibThread,
    ) -> Self {
        Self {}
    }
}
