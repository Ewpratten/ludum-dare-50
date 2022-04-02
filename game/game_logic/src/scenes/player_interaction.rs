//! This scene encompasses all of the game where the player can walk around.

use nalgebra as na;
use raylib::prelude::*;

use crate::{
    discord::{DiscordChannel, DiscordRpcSignal}, global_resource_package::GlobalResources,
    rendering::utilities::anim_texture::AnimatedTexture, model::player::Player,
};

#[derive(Debug)]
pub struct PlayableScene {
    has_updated_discord_rpc: bool,
    player: Player
}

impl PlayableScene {
    /// Construct a new `PlayableScene`
    pub fn new(raylib_handle: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        Self {
            has_updated_discord_rpc: false,
            player: Player::new(na::Vector2::new(10.0, 10.0))
        }
    }

    /// Handler for each frame
    pub async fn render_frame(
        &mut self,
        raylib: &mut RaylibHandle,
        rl_thread: &RaylibThread,
        discord: &DiscordChannel,
        global_resources: &GlobalResources,
    ) {

        // Handle updating discord RPC
        if !self.has_updated_discord_rpc {
            discord.send(DiscordRpcSignal::BeginGameTimer).await.unwrap();
            discord.send(DiscordRpcSignal::ChangeDetails { details: "Playing the game".to_string(), party_status: None }).await.unwrap();
            self.has_updated_discord_rpc = true;
        }

        // Get a drawing handle
        let mut draw = raylib.begin_drawing(rl_thread);

        // Clear the screen
        draw.clear_background(Color::WHITE);

        // TODO: Render stuff
        // self.player. <whatever>
    }
}
