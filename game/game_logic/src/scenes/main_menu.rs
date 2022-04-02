//! This scene encompasses the main menu system

use nalgebra as na;
use raylib::prelude::*;

use crate::{
    discord::{DiscordChannel, DiscordRpcSignal},
    global_resource_package::GlobalResources,
    project_constants::ProjectConstants,
};

#[derive(Debug)]
pub struct MainMenu {
    has_updated_discord_rpc: bool,
}

impl MainMenu {
    /// Construct a new `MainMenu`
    pub fn new(
        raylib_handle: &mut RaylibHandle,
        thread: &RaylibThread,
        constants: &ProjectConstants,
    ) -> Self {
        Self {
            has_updated_discord_rpc: false,
        }
    }

    /// Handler for each frame
    pub async fn render_frame(
        &mut self,
        raylib: &mut RaylibHandle,
        rl_thread: &RaylibThread,
        discord: &DiscordChannel,
        global_resources: &GlobalResources,
        constants: &ProjectConstants,
    ) -> bool {
        // Handle updating discord RPC
        if !self.has_updated_discord_rpc {
            discord
                .send(DiscordRpcSignal::EndGameTimer)
                .await
                .unwrap();
            discord
                .send(DiscordRpcSignal::ChangeDetails {
                    details: "Looking at a menu".to_string(),
                    party_status: None,
                })
                .await
                .unwrap();
            self.has_updated_discord_rpc = true;
        }

        // Get a drawing handle
        let mut draw = raylib.begin_drawing(rl_thread);

        // Clear the screen
        draw.clear_background(Color::WHITE);

        // TODO: Render stuff


        // Return true if you want the game to start. 
        // Otherwise, keep returning false until the player clicks the start button
        return false;
    }
}
