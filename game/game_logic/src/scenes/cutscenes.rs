//! This scene encompasses the main menu system

use chrono::Duration;
use na::Vector1;
use nalgebra as na;
use raylib::{
    ffi::{GetMouseX, GetMouseY, IsMouseButtonDown, Texture},
    prelude::*,
};

use crate::{
    asset_manager::load_texture_from_internal_data,
    discord::{DiscordChannel, DiscordRpcSignal},
    global_resource_package::GlobalResources,
    persistent::settings::PersistentGameSettings,
    project_constants::ProjectConstants,
};

use super::main_menu::MenuStateSignal;

const MIWU_WHITE: Color = Color {
    r: 247,
    g: 239,
    b: 231,
    a: 255,
};
const MIWU_WHITE_V2: Color = Color {
    r: 255,
    g: 245,
    b: 228,
    a: 255,
};

#[derive(Debug)]
pub struct CutScenes {
    show_debug_info: bool,
    intro_art: Texture2D,
    melted_art: Texture2D,
}

impl CutScenes {
    /// Construct a new `CutScenes`
    pub fn new(
        raylib_handle: &mut RaylibHandle,
        thread: &RaylibThread,
        constants: &ProjectConstants,
        game_settings: &mut PersistentGameSettings,
    ) -> Self {
        // Load art
        let intro_art = load_texture_from_internal_data(
            raylib_handle,
            thread,
            "assets/cut/cut_intro/cut_intro.png",
        )
        .unwrap();
        let melted_art = load_texture_from_internal_data(
            raylib_handle,
            thread,
            "assets/cut/cut_melty/cut_melty.png",
        )
        .unwrap();

        Self {
            show_debug_info: false,
            intro_art,
            melted_art,
        }
    }

    pub async fn render_bartender_cutscene_frame(
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
        draw.clear_background(MIWU_WHITE);

        //Obtain mouse position
        let mouse_x = draw.get_mouse_x();
        let mouse_y = draw.get_mouse_y();

        // Optionally display debug info
        if draw.is_key_pressed(KeyboardKey::KEY_F3) {
            self.show_debug_info = !self.show_debug_info;
        }
        if self.show_debug_info {
            // Draw FPS and mouse location
            draw.draw_fps(10, 10);
            draw.draw_text(
                format!("Mouse position: ({}, {})", mouse_x, mouse_y).as_str(),
                10,
                30,
                20,
                Color::GREEN,
            );
        }

        // Title
        // draw.draw_text("INTRO CUTSCENE GOES HERE", 100, 90, 60, Color::BLACK);
        // draw.draw_text("Press SPACE to skip", 100, 600, 20, Color::BLACK);

        let screen_height = draw.get_screen_height();
        let screen_width = draw.get_screen_width();

        // Build a rect for the texture
        let tex_rect = Rectangle::new(
            0.0,
            0.0,
            self.intro_art.width as f32,
            self.intro_art.height as f32,
        );

        // Draw the texture to the center of the screen.
        // Keep in mind, textures are drawn from the top left
        // corner, so we need to offset the rect by half the
        // texture's width and height.
        let dest_rect = Rectangle::new(
            (screen_width / 2) as f32 - (tex_rect.width / 2.0),
            (screen_height / 2) as f32 - (tex_rect.height / 2.0),
            tex_rect.width,
            tex_rect.height,
        );

        // Draw the texture
        draw.draw_texture_pro(
            &self.intro_art,
            &tex_rect,
            &dest_rect,
            Vector2::zero(),
            0.0,
            Color::WHITE,
        );

        // Let the user leave this cutscene by pressing space
        if draw.is_key_pressed(KeyboardKey::KEY_SPACE) {
            return MenuStateSignal::StartGame;
        }

        // Return MenuStateSignal::DoMainMenu if you want to return to the main menu
        // Return MenuStateSignal::StartGame if you want the game to start.
        // Otherwise, keep returning MenuStateSignal::DoIntroCutscene
        return MenuStateSignal::DoIntroCutscene;
    }

    pub async fn render_melted_cutscene_frame(
        &mut self,
        raylib: &mut RaylibHandle,
        rl_thread: &RaylibThread,
        discord: &DiscordChannel,
        global_resources: &GlobalResources,
        constants: &ProjectConstants,
        audio_subsystem: &mut RaylibAudio,
        playtime: &Duration,
    ) -> MenuStateSignal {
        // Get a drawing handle
        let mut draw = raylib.begin_drawing(rl_thread);

        // Clear the screen
        draw.clear_background(MIWU_WHITE_V2);

        //Obtain mouse position
        let mouse_x = draw.get_mouse_x();
        let mouse_y = draw.get_mouse_y();

        // Optionally display debug info
        if draw.is_key_pressed(KeyboardKey::KEY_F3) {
            self.show_debug_info = !self.show_debug_info;
        }
        if self.show_debug_info {
            // Draw FPS and mouse location
            draw.draw_fps(10, 10);
            draw.draw_text(
                format!("Mouse position: ({}, {})", mouse_x, mouse_y).as_str(),
                10,
                30,
                20,
                Color::GREEN,
            );
        }

        // // Title
        // draw.draw_text("MELTY CUTSCENE GOES HERE", 100, 90, 60, Color::BLACK);
        // draw.draw_text(
        //     &format!("This took you {} seconds", playtime.num_seconds()),
        //     100,
        //     600,
        //     20,
        //     Color::BLACK,
        // );
        // draw.draw_text("Press SPACE to skip", 100, 680, 20, Color::BLACK);

        let screen_height = draw.get_screen_height();
        let screen_width = draw.get_screen_width();

        // Build a rect for the texture
        let tex_rect = Rectangle::new(
            0.0,
            0.0,
            self.melted_art.width as f32,
            self.melted_art.height as f32,
        );

        // Draw the texture to the center of the screen.
        // Keep in mind, textures are drawn from the top left
        // corner, so we need to offset the rect by half the
        // texture's width and height.
        let dest_rect = Rectangle::new(
            (screen_width / 2) as f32 - (tex_rect.width / 2.0),
            (screen_height / 2) as f32 - (tex_rect.height / 2.0),
            tex_rect.width,
            tex_rect.height,
        );

        // Draw the texture
        draw.draw_texture_pro(
            &self.melted_art,
            &tex_rect,
            &dest_rect,
            Vector2::zero(),
            0.0,
            Color::WHITE,
        );

        // Let the user leave this cutscene by pressing space
        if draw.is_key_pressed(KeyboardKey::KEY_SPACE) {
            return MenuStateSignal::DoMainMenu;
        }

        // Return MenuStateSignal::DoMainMenu if you want to return to the main menu
        // Otherwise, keep returning MenuStateSignal::DoMeltedDeathCutscene
        return MenuStateSignal::DoMeltedDeathCutscene {
            playtime: playtime.clone(),
        };
    }

    pub async fn render_finished_cutscene_frame(
        &mut self,
        raylib: &mut RaylibHandle,
        rl_thread: &RaylibThread,
        discord: &DiscordChannel,
        global_resources: &GlobalResources,
        constants: &ProjectConstants,
        audio_subsystem: &mut RaylibAudio,
        playtime: &Duration,
    ) -> MenuStateSignal {
        // Get a drawing handle
        let mut draw = raylib.begin_drawing(rl_thread);

        // Clear the screen
        draw.clear_background(MIWU_WHITE);

        //Obtain mouse position
        let mouse_x = draw.get_mouse_x();
        let mouse_y = draw.get_mouse_y();

        // Optionally display debug info
        if draw.is_key_pressed(KeyboardKey::KEY_F3) {
            self.show_debug_info = !self.show_debug_info;
        }
        if self.show_debug_info {
            // Draw FPS and mouse location
            draw.draw_fps(10, 10);
            draw.draw_text(
                format!("Mouse position: ({}, {})", mouse_x, mouse_y).as_str(),
                10,
                30,
                20,
                Color::GREEN,
            );
        }

        // Title
        draw.draw_text("END CUTSCENE GOES HERE", 100, 90, 60, Color::BLACK);
        draw.draw_text(
            &format!("This took you {} seconds", playtime.num_seconds()),
            100,
            600,
            20,
            Color::BLACK,
        );
        draw.draw_text("Press SPACE to skip", 100, 680, 20, Color::BLACK);

        // Let the user leave this cutscene by pressing space
        if draw.is_key_pressed(KeyboardKey::KEY_SPACE) {
            return MenuStateSignal::DoMainMenu;
        }

        // Return MenuStateSignal::DoMainMenu if you want to return to the main menu
        // Otherwise, keep returning MenuStateSignal::DoFinishedCutscene
        return MenuStateSignal::DoFinishedCutscene {
            playtime: playtime.clone(),
        };
    }

    pub async fn render_ocean_cutscene_frame(
        &mut self,
        raylib: &mut RaylibHandle,
        rl_thread: &RaylibThread,
        discord: &DiscordChannel,
        global_resources: &GlobalResources,
        constants: &ProjectConstants,
        audio_subsystem: &mut RaylibAudio,
        playtime: &Duration,
    ) -> MenuStateSignal {
        // Get a drawing handle
        let mut draw = raylib.begin_drawing(rl_thread);

        // Clear the screen
        draw.clear_background(MIWU_WHITE);

        //Obtain mouse position
        let mouse_x = draw.get_mouse_x();
        let mouse_y = draw.get_mouse_y();

        // Optionally display debug info
        if draw.is_key_pressed(KeyboardKey::KEY_F3) {
            self.show_debug_info = !self.show_debug_info;
        }
        if self.show_debug_info {
            // Draw FPS and mouse location
            draw.draw_fps(10, 10);
            draw.draw_text(
                format!("Mouse position: ({}, {})", mouse_x, mouse_y).as_str(),
                10,
                30,
                20,
                Color::GREEN,
            );
        }

        // Title
        draw.draw_text("OCEAN CUTSCENE GOES HERE", 100, 90, 60, Color::BLACK);
        draw.draw_text(
            &format!("This took you {} seconds", playtime.num_seconds()),
            100,
            600,
            20,
            Color::BLACK,
        );
        draw.draw_text("Press SPACE to skip", 100, 680, 20, Color::BLACK);

        // Let the user leave this cutscene by pressing space
        if draw.is_key_pressed(KeyboardKey::KEY_SPACE) {
            return MenuStateSignal::DoMainMenu;
        }

        // Return MenuStateSignal::DoMainMenu if you want to return to the main menu
        // Otherwise, keep returning MenuStateSignal::DoOceanCutscene
        return MenuStateSignal::DoOceanCutscene {
            playtime: playtime.clone(),
        };
    }
}
