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
        unsafe {
            let mut mouseX = GetMouseX();
            let mut mouseY = GetMouseY();

            draw.draw_text((&mouseX.to_string()), 20, 5, 20, Color::BLACK);
            draw.draw_text((&mouseY.to_string()), 70, 5, 20, Color::BLACK);
        }

        // TODO: Render stuff
        //Initial Option placeholder words in the main menu
        draw.draw_text("Game Title", 100, 90, 60, Color::BLACK);
        draw.draw_text("Start Game", 100, 190, 34, Color::BLACK);
        draw.draw_text("Options", 100, 250, 34, Color::BLACK);
        draw.draw_text("Credits", 100, 410, 34, Color::BLACK);
        draw.draw_text("Leaderboard", 100, 470, 34, Color::BLACK);
        draw.draw_text("Exit", 100, 550, 34, Color::BLACK);

        //Unsafe block??
        unsafe {
            //First two are starting X and Y position, last two finishing X and Y. Made to resemble a box
            if GetMouseX() >= 100 && GetMouseY() >= 193 && GetMouseX() <= 290 && GetMouseY() <= 216
            {
                //Insides while make a lil shade for it to look cool
                draw.draw_text("Start Game", 103, 191, 34, Color::GRAY);
                draw.draw_text("Start Game", 100, 190, 34, Color::BLACK);
                if IsMouseButtonDown(0) {
                    return MenuStateSignal::StartGame;
                }
            }

            if GetMouseX() >= 100 && GetMouseY() >= 250 && GetMouseX() <= 222 && GetMouseY() <= 275
            {
                draw.draw_text("Options", 103, 251, 34, Color::GRAY);
                draw.draw_text("Options", 100, 250, 34, Color::BLACK);
            }

            if GetMouseX() >= 100 && GetMouseY() >= 410 && GetMouseX() <= 222 && GetMouseY() <= 437
            {
                draw.draw_text("Credits", 103, 411, 34, Color::GRAY);
                draw.draw_text("Credits", 100, 410, 34, Color::BLACK);
            }
            if GetMouseX() >= 100 && GetMouseY() >= 470 && GetMouseX() <= 316 && GetMouseY() <= 496
            {
                draw.draw_text("Leaderboard", 103, 471, 34, Color::GRAY);
                draw.draw_text("Leaderboard", 100, 470, 34, Color::BLACK);
            }
            if GetMouseX() >= 100 && GetMouseY() >= 550 && GetMouseX() <= 162 && GetMouseY() <= 575
            {
                draw.draw_text("Exit", 103, 551, 34, Color::GRAY);
                draw.draw_text("Exit", 100, 550, 34, Color::BLACK);
            }
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
