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

        //Obtain mouse position
        let mouse_x = draw.get_mouse_x();
        let mouse_y = draw.get_mouse_y();

        //I wanna see where mouseeee
        draw.draw_text(&mouse_x.to_string(), 20, 5, 20, Color::BLACK);
        draw.draw_text(&mouse_y.to_string(), 70, 5, 20, Color::BLACK);

        // TODO: Render stuff
        //Initial Option placeholder words in the main menu
        draw.draw_text("Game Title", 100, 90, 60, Color::BLACK);
        draw.draw_text("Start Game", 100, 190, 34, Color::BLACK);
        draw.draw_text("Options", 100, 250, 34, Color::BLACK);
        draw.draw_text("Volume", 100, 300, 34, Color::BLACK);  
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
            if draw.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) {
                return MenuStateSignal::DoOptions;
            }
        }

        if mouse_x >= 100 && mouse_y >= 410 && mouse_x <= 222 && mouse_y <= 437 {
            draw.draw_text("Credits", 103, 411, 34, Color::GRAY);
            draw.draw_text("Credits", 100, 410, 34, Color::BLACK);
            if draw.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) {
                return MenuStateSignal::DoCredits;
            }
        }
        if mouse_x >= 100 && mouse_y >= 470 && mouse_x <= 316 && mouse_y <= 496 {
            draw.draw_text("Leaderboard", 103, 471, 34, Color::GRAY);
            draw.draw_text("Leaderboard", 100, 470, 34, Color::BLACK);
            if draw.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) {
                return MenuStateSignal::DoLeaderboard;
            }
        }

        if mouse_x >= 100 && mouse_y >= 300 && mouse_x <= 215 && mouse_y <= 330 {
            draw.draw_text("Volume", 103, 301, 34, Color::GRAY);
            draw.draw_text("Volume", 100, 300, 34, Color::BLACK);
            if draw.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) {
                //Function for Volume here
            }
        }
        
        //Exit button has no function yet
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

        //Draw declared
        let mut draw = raylib.begin_drawing(rl_thread);
        draw.clear_background(Color::WHITE);
        //Mouse Position
        let mouse_x = draw.get_mouse_x();
        let mouse_y = draw.get_mouse_y();
        //Show mouse position
        draw.draw_text(&mouse_x.to_string(), 20, 5, 20, Color::BLACK);
        draw.draw_text(&mouse_y.to_string(), 70, 5, 20, Color::BLACK);

        //Top Label
        draw.draw_text("Options", 25, 30, 55, Color::BLACK);    

        //Window size storing variables
        let window_height = draw.get_screen_height();
        let window_width = draw.get_screen_width();

        //Return button variables
        let button_pos_x = 100; //116 Wide
        let button_pos_y = window_height - (window_height/5); //26 height

        draw.draw_text("Return", button_pos_x, button_pos_y, 34, Color::BLACK);
        if mouse_x >= 100 && mouse_y >= button_pos_y && mouse_x <= 216 && mouse_y <= (window_height - (window_height/5)) + 26 {
            draw.draw_text("Return", button_pos_x + 3, button_pos_y + 1, 34, Color::GRAY);
            draw.draw_text("Return", button_pos_x, button_pos_y, 34, Color::BLACK);
            if draw.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) {
                return MenuStateSignal::DoMainMenu; //Goes back to main menu
            }
        }
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
        let mut draw = raylib.begin_drawing(rl_thread);
        draw.clear_background(Color::WHITE);
        //Mouse Position
        let mouse_x = draw.get_mouse_x();
        let mouse_y = draw.get_mouse_y();
        //Show mouse position
        draw.draw_text(&mouse_x.to_string(), 20, 5, 20, Color::BLACK);
        draw.draw_text(&mouse_y.to_string(), 70, 5, 20, Color::BLACK);

        //Screen Size
        let window_height = draw.get_screen_height();
        let window_width = draw.get_screen_width();

        draw.draw_text("Credits", (window_width/2) - 100, 30, 55, Color::BLACK);

        draw.draw_text("Carter Tomlenovich", (window_width/2) - 170, 120, 40, Color::DARKBLUE);
        draw.draw_text("Emilia Firas", (window_width/2) - 170, 160, 40, Color::DARKBLUE);
        draw.draw_text("Emmet Logue", (window_width/2) - 170, 200, 40, Color::DARKBLUE);
        draw.draw_text("Evan Pratten", (window_width/2) - 170, 240, 40, Color::DARKBLUE);
        draw.draw_text("James Nickoli", (window_width/2) - 170, 280, 40, Color::DARKBLUE);
        draw.draw_text("Marcelo Geldres", (window_width/2) - 170, 320, 40, Color::DARKBLUE);
        draw.draw_text("Percy", (window_width/2) - 170, 360, 40, Color::DARKBLUE);
        draw.draw_text("Silas Bartha", (window_width/2) - 170, 400, 40, Color::DARKBLUE);
        draw.draw_text("Taya Armstrong", (window_width/2) - 170, 440, 40, Color::DARKBLUE);

        //Return button variables
        let button_pos_x = 100; //116 Wide
        let button_pos_y = window_height - (window_height/5); //26 height

        draw.draw_text("Return", button_pos_x, button_pos_y, 34, Color::BLACK);
        if mouse_x >= 100 && mouse_y >= button_pos_y && mouse_x <= 216 && mouse_y <= (window_height - (window_height/5)) + 26 {
            draw.draw_text("Return", button_pos_x + 3, button_pos_y + 1, 34, Color::GRAY);
            draw.draw_text("Return", button_pos_x, button_pos_y, 34, Color::BLACK);
            if draw.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) {
                return MenuStateSignal::DoMainMenu; //Goes back to main menu
            }
        }

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
        let mut draw = raylib.begin_drawing(rl_thread);
        draw.clear_background(Color::WHITE);
        //Mouse Position
        let mouse_x = draw.get_mouse_x();
        let mouse_y = draw.get_mouse_y();

        //Window size storing variables
        let window_height = draw.get_screen_height();
        let window_width = draw.get_screen_width();

        //Show mouse position
        draw.draw_text(&mouse_x.to_string(), 20, 5, 20, Color::BLACK);
        draw.draw_text(&mouse_y.to_string(), 70, 5, 20, Color::BLACK);

        let window_width = draw.get_screen_width();
        draw.draw_text("Leaderboard", (window_width/2) - 176, 30, 55, Color::BLACK);

        //Return button variables
        let button_pos_x = 100; //116 Wide
        let button_pos_y = window_height - (window_height/5); //26 height

        draw.draw_text("Return", button_pos_x, button_pos_y, 34, Color::BLACK);
        if mouse_x >= 100 && mouse_y >= button_pos_y && mouse_x <= 216 && mouse_y <= (window_height - (window_height/5)) + 26 {
            draw.draw_text("Return", button_pos_x + 3, button_pos_y + 1, 34, Color::GRAY);
            draw.draw_text("Return", button_pos_x, button_pos_y, 34, Color::BLACK);
            if draw.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) {
                return MenuStateSignal::DoMainMenu; //Goes back to main menu
            }
        }

        return MenuStateSignal::DoLeaderboard;
    }
}
