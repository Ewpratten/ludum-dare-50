use raylib::prelude::*;

use crate::{discord::{DiscordChannel, DiscordRpcSignal}, project_constants::ProjectConstants};

#[derive(Debug)]
pub struct SmFailureScreen {
    has_updated_discord_status: bool,
}

impl SmFailureScreen {
    /// Construct a new `SmFailureScreen`
    pub fn new() -> Self {
        Self {
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
                        .get("details.sm_failure")
                        .unwrap()
                        .to_owned(),
                    party_status: None,
                })
                .await
                .unwrap();
            self.has_updated_discord_status = true;
        }

        // Render the error message
        let mut d = raylib.begin_drawing(&rl_thread);
        d.clear_background(raylib::color::Color::RED);
        d.draw_text(
            "Backend Rendering Broke.\nYou should not be seeing this!",
            10,
            10,
            40,
            raylib::color::Color::WHITE,
        );

        false
    }
}
