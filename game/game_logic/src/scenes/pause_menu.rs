//! This scene encompasses the main menu system

use na::Vector1;
use nalgebra as na;
use raylib::{
    ffi::{GetMouseX, GetMouseY, IsMouseButtonDown, Texture},
    prelude::*,
};

use crate::{
    discord::{DiscordChannel, DiscordRpcSignal},
    global_resource_package::GlobalResources,
    persistent::settings::PersistentGameSettings,
    project_constants::ProjectConstants,
};

use super::main_menu::MenuStateSignal;

#[derive(Debug)]
pub struct PauseMenu {}

impl PauseMenu {
    /// Construct a new `PauseMenu`
    pub fn new(
        raylib_handle: &mut RaylibHandle,
        thread: &RaylibThread,
        constants: &ProjectConstants,
        game_settings: &mut PersistentGameSettings,
    ) -> Self {
        Self {}
    }

    pub async fn render_pause_menu_frame(
        &mut self,
        raylib: &mut RaylibHandle,
        rl_thread: &RaylibThread,
        discord: &DiscordChannel,
        global_resources: &GlobalResources,
        constants: &ProjectConstants,
        audio_subsystem: &mut RaylibAudio,
    ) -> MenuStateSignal {
        // Get a drawing handle
        let mut draw = raylib.begin_drawing(rl_thread);

        // Clear the screen
        draw.clear_background(Color::WHITE);

        // Title
        draw.draw_text("Paused", 100, 90, 60, Color::BLACK);

        // Let the user leave this menu by pressing escape
        if draw.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
            return MenuStateSignal::StartGame;
        }

        // Return MenuStateSignal::DoMainMenu if you want to return to the main menu
        // Return MenuStateSignal::StartGame if you want the game to start.
        // Return MenuStateSignal::QuitGame if you want the game to quit.
        // Otherwise, keep returning MenuStateSignal::DoPauseMenu until the player clicks the start button
        return MenuStateSignal::DoPauseMenu;
    }
}
