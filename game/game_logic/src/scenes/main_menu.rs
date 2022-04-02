//! This scene encompasses the main menu system

use nalgebra as na;
use raylib::{
    ffi::{GetMouseX, GetMouseY, IsMouseButtonDown, Texture},
    prelude::*,
};

use crate::{
    discord::{DiscordChannel, DiscordRpcSignal},
    global_resource_package::GlobalResources,
    project_constants::ProjectConstants,
};

#[derive(Debug, Clone)]
pub enum MenuStateSignal {
    StartGame,
    QuitGame,
    DoMainMenu,
    DoOptions,
    DoCredits,
    DoLeaderboard,
}

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

    pub async fn render_main_menu_frame(
        &mut self,
        raylib: &mut RaylibHandle,
        rl_thread: &RaylibThread,
        discord: &DiscordChannel,
        global_resources: &GlobalResources,
        constants: &ProjectConstants,
    ) -> MenuStateSignal {
        // Handle updating discord RPC
        if !self.has_updated_discord_rpc {
            discord.send(DiscordRpcSignal::EndGameTimer).await.unwrap();
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

        //I wanna see where mouseeee
        let mouse_x = draw.get_mouse_x();
        let mouse_y = draw.get_mouse_y();

        draw.draw_text(&mouse_x.to_string(), 20, 5, 20, Color::BLACK);
        draw.draw_text(&mouse_y.to_string(), 70, 5, 20, Color::BLACK);

        // TODO: Render stuff
        //Initial Option placeholder words in the main menu
        draw.draw_text("Game Title", 100, 90, 60, Color::BLACK);
        draw.draw_text("Start Game", 100, 190, 34, Color::BLACK);
        draw.draw_text("Options", 100, 250, 34, Color::BLACK);
        draw.draw_text("Credits", 100, 410, 34, Color::BLACK);
        draw.draw_text("Leaderboard", 100, 470, 34, Color::BLACK);
        draw.draw_text("Exit", 100, 550, 34, Color::BLACK);

        //First two are starting X and Y position, last two finishing X and Y. Made to resemble a box

        if mouse_x >= 100 && mouse_y >= 193 && mouse_x <= 290 && mouse_y <= 216 {
            //Insides while make a lil shade for it to look cool
            draw.draw_text("Start Game", 103, 191, 34, Color::GRAY);
            draw.draw_text("Start Game", 100, 190, 34, Color::BLACK);
            if draw.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) {
                return MenuStateSignal::StartGame;
            }
        }

        if mouse_x >= 100 && mouse_y >= 250 && mouse_x <= 222 && mouse_y <= 275 {
            draw.draw_text("Options", 103, 251, 34, Color::GRAY);
            draw.draw_text("Options", 100, 250, 34, Color::BLACK);
        }

        if mouse_x >= 100 && mouse_y >= 410 && mouse_x <= 222 && mouse_y <= 437 {
            draw.draw_text("Credits", 103, 411, 34, Color::GRAY);
            draw.draw_text("Credits", 100, 410, 34, Color::BLACK);
        }
        if mouse_x >= 100 && mouse_y >= 470 && mouse_x <= 316 && mouse_y <= 496 {
            draw.draw_text("Leaderboard", 103, 471, 34, Color::GRAY);
            draw.draw_text("Leaderboard", 100, 470, 34, Color::BLACK);
        }

        if mouse_x >= 100 && mouse_y >= 550 && mouse_x <= 162 && mouse_y <= 575 {
            draw.draw_text("Exit", 103, 551, 34, Color::GRAY);
            draw.draw_text("Exit", 100, 550, 34, Color::BLACK);
        }

        // Return MenuStateSignal::StartGame if you want the game to start.
        // Otherwise, keep returning MenuStateSignal::DoMainMenu until the player clicks the start button
        return MenuStateSignal::DoMainMenu;
    }

    pub async fn render_options_frame(
        &mut self,
        raylib: &mut RaylibHandle,
        rl_thread: &RaylibThread,
        discord: &DiscordChannel,
        global_resources: &GlobalResources,
        constants: &ProjectConstants,
    ) -> MenuStateSignal {
        return MenuStateSignal::DoOptions;
    }

    pub async fn render_credits_frame(
        &mut self,
        raylib: &mut RaylibHandle,
        rl_thread: &RaylibThread,
        discord: &DiscordChannel,
        global_resources: &GlobalResources,
        constants: &ProjectConstants,
    ) -> MenuStateSignal {
        return MenuStateSignal::DoCredits;
    }

    pub async fn render_leaderboard_frame(
        &mut self,
        raylib: &mut RaylibHandle,
        rl_thread: &RaylibThread,
        discord: &DiscordChannel,
        global_resources: &GlobalResources,
        constants: &ProjectConstants,
    ) -> MenuStateSignal {
        return MenuStateSignal::DoLeaderboard;
    }
}
