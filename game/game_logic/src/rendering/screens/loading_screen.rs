//! Handles loading the global resources and playing an intro animation
//!
//! ## Overview
//!
//! This module contains `LoadingScreen` which will perform multi-threaded resource loading while rendering a loading animation.
//!
//! ## Whats happening
//!
//! - Discord RPC is set
//! - Resources are loaded
//! - Animation is rendered

use poll_promise::Promise;
use raylib::prelude::*;

use crate::{
    discord::{DiscordChannel, DiscordRpcSignal},
    global_resource_package::GlobalResources,
    project_constants::ProjectConstants,
};

pub struct LoadingScreen {
    pub resources: Option<GlobalResources>,
    has_updated_discord_status: bool,
}

impl LoadingScreen {
    /// Construct a new `LoadingScreen`
    pub fn new() -> Self {
        Self {
            resources: None,
            has_updated_discord_status: false,
        }
    }

    pub async fn render(
        &mut self,
        raylib: &mut RaylibHandle,
        rl_thread: &RaylibThread,
        discord: &DiscordChannel,
        constants: &ProjectConstants,
    ) -> bool {
        // Handle updating the Discord status
        if !self.has_updated_discord_status {
            discord
                .send(DiscordRpcSignal::ChangeDetails {
                    details: constants
                        .discord
                        .strings
                        .get("details.loading")
                        .unwrap()
                        .to_owned(),
                    party_status: None,
                })
                .await
                .unwrap();
            self.has_updated_discord_status = true;
        }

        // Begin loading resources if we haven't already
        if let None = self.resources {
            self.resources = Some(GlobalResources::load(raylib, rl_thread).await);
        }

        // Draw some graphics
        let mut d = raylib.begin_drawing(&rl_thread);
        d.clear_background(raylib::color::Color::BLACK);


        true
    }
}
