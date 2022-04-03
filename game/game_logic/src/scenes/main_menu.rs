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

#[derive(Debug, Clone)]
pub enum MenuStateSignal {
    StartGame,
    QuitGame,
    DoMainMenu,
    DoOptions,
    DoCredits,
    DoLeaderboard,
    DoPauseMenu,
}

#[derive(Debug)]
pub struct MainMenu {
    pub has_updated_discord_rpc: bool,
    volume_percentage: f32,
}

impl MainMenu {
    /// Construct a new `MainMenu`
    pub fn new(
        raylib_handle: &mut RaylibHandle,
        thread: &RaylibThread,
        constants: &ProjectConstants,
        game_settings: &mut PersistentGameSettings,
    ) -> Self {
        Self {
            has_updated_discord_rpc: false,
            volume_percentage: game_settings.volume.unwrap_or(0.5),
        }
    }

    pub async fn render_main_menu_frame(
        &mut self,
        raylib: &mut RaylibHandle,
        rl_thread: &RaylibThread,
        discord: &DiscordChannel,
        global_resources: &GlobalResources,
        constants: &ProjectConstants,
        audio_subsystem: &mut RaylibAudio,
        game_settings: &mut PersistentGameSettings,
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

        //TODO Errase later
        draw.draw_text(&mouse_x.to_string(), 20, 5, 20, Color::BLACK);
        draw.draw_text(&mouse_y.to_string(), 70, 5, 20, Color::BLACK);

        //Screen Size
        let window_height = draw.get_screen_height();
        let window_width = draw.get_screen_width();

        // TODO: Render stuff
        //Label Colors
        let label_colors = Color::BLACK;
        let label_shadow_colors = Color::GRAY;

        //Initial Option placeholder words in the main menu
        draw.draw_text(&constants.game_name, 100, 90, 60, label_colors);
        draw.draw_text("Start Game", 100, 190, 34, label_colors);
        draw.draw_text("Credits", 100, 410, 34, label_colors);
        draw.draw_text("Leaderboard", 100, 470, 34, label_colors);
        draw.draw_text("Exit", 100, 550, 34, label_colors);

        //First two are starting X and Y position, last two finishing X and Y. Made to resemble a box
        if mouse_x >= 100 && mouse_y >= 193 && mouse_x <= 290 && mouse_y <= 216 {
            //Insides while make a lil shade for it to look cool
            draw.draw_text("Start Game", 103, 191, 34, label_shadow_colors);
            draw.draw_text("Start Game", 100, 190, 34, label_colors);
            if draw.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
                audio_subsystem.play_sound(&global_resources.button_click_sound);
                return MenuStateSignal::StartGame;
            }
        }

        if mouse_x >= 100 && mouse_y >= 410 && mouse_x <= 222 && mouse_y <= 437 {
            draw.draw_text("Credits", 103, 411, 34, label_shadow_colors);
            draw.draw_text("Credits", 100, 410, 34, label_colors);
            if draw.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
                audio_subsystem.play_sound(&global_resources.button_click_sound);
                return MenuStateSignal::DoCredits;
            }
        }
        if mouse_x >= 100 && mouse_y >= 470 && mouse_x <= 316 && mouse_y <= 496 {
            draw.draw_text("Leaderboard", 103, 471, 34, label_shadow_colors);
            draw.draw_text("Leaderboard", 100, 470, 34, label_colors);
            if draw.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
                audio_subsystem.play_sound(&global_resources.button_click_sound);
                return MenuStateSignal::DoLeaderboard;
            }
        }

        //Volume Controller
        //Color Pallete Variables
        let tile_color = Color::new(158, 93, 65, 255);
        let outer_ring_color = Color::new(255, 191, 113, 255);
        let inner_ring_color = Color::new(244, 203, 184, 255);
        let button_color = Color::new(82, 135, 195, 255);
        let button_shadow_color = Color::new(123, 201, 244, 255);

        //Inner pieces of the controller
        draw.draw_ring(
            Vector2::new((window_width as f32) - 90.0, (window_height as f32) - 140.0),
            50.0,
            15.0,
            275.0,
            235.0,
            0,
            tile_color,
        ); //tile1
        draw.draw_ring(
            Vector2::new((window_width as f32) - 90.0, (window_height as f32) - 140.0),
            50.0,
            15.0,
            225.0,
            185.0,
            0,
            tile_color,
        ); //tile2

        //- button
        draw.draw_rectangle(window_width - 133, window_height - 128, 21, 5, button_color);
        //+ button
        draw.draw_rectangle(window_width - 62, window_height - 135, 5, 20, button_color); // vertical line
        draw.draw_rectangle(window_width - 70, window_height - 128, 21, 5, button_color); //horizontal line

        //Drawing external ring and internal ring
        draw.draw_ring_lines(
            Vector2::new((window_width as f32) - 90.0, (window_height as f32) - 140.0),
            50.0,
            15.0,
            315.0,
            45.0,
            1,
            outer_ring_color,
        ); //Outer
        draw.draw_ring(
            Vector2::new((window_width as f32) - 90.0, (window_height as f32) - 140.0),
            50.0,
            15.0,
            275.0,
            85.0,
            1,
            inner_ring_color,
        ); //Inner

        //Tiles shown depending on volume_percentage's value
        if self.volume_percentage == 1.0 {
            draw.draw_ring(
                Vector2::new((window_width as f32) - 90.0, (window_height as f32) - 140.0),
                50.0,
                15.0,
                125.0,
                85.0,
                0,
                tile_color,
            ); //tile4
            draw.draw_ring(
                Vector2::new((window_width as f32) - 90.0, (window_height as f32) - 140.0),
                50.0,
                15.0,
                175.0,
                135.0,
                0,
                tile_color,
            ); //tile3
            draw.draw_ring(
                Vector2::new((window_width as f32) - 90.0, (window_height as f32) - 140.0),
                50.0,
                15.0,
                225.0,
                185.0,
                0,
                tile_color,
            ); //tile2
            draw.draw_ring(
                Vector2::new((window_width as f32) - 90.0, (window_height as f32) - 140.0),
                50.0,
                15.0,
                275.0,
                235.0,
                0,
                tile_color,
            ); //tile1
        } else if self.volume_percentage == 0.75 {
            draw.draw_ring(
                Vector2::new((window_width as f32) - 90.0, (window_height as f32) - 140.0),
                50.0,
                15.0,
                175.0,
                135.0,
                0,
                tile_color,
            ); //tile3
            draw.draw_ring(
                Vector2::new((window_width as f32) - 90.0, (window_height as f32) - 140.0),
                50.0,
                15.0,
                225.0,
                185.0,
                0,
                tile_color,
            ); //tile2
            draw.draw_ring(
                Vector2::new((window_width as f32) - 90.0, (window_height as f32) - 140.0),
                50.0,
                15.0,
                275.0,
                235.0,
                0,
                tile_color,
            ); //tile1
        } else if self.volume_percentage == 0.5 {
            draw.draw_ring(
                Vector2::new((window_width as f32) - 90.0, (window_height as f32) - 140.0),
                50.0,
                15.0,
                225.0,
                185.0,
                0,
                tile_color,
            ); //tile2
            draw.draw_ring(
                Vector2::new((window_width as f32) - 90.0, (window_height as f32) - 140.0),
                50.0,
                15.0,
                275.0,
                235.0,
                0,
                tile_color,
            ); //tile1
        } else if self.volume_percentage == 0.25 {
            draw.draw_ring(
                Vector2::new((window_width as f32) - 90.0, (window_height as f32) - 140.0),
                50.0,
                15.0,
                275.0,
                235.0,
                0,
                tile_color,
            ); //tile1
        } else if self.volume_percentage == 0.0 {
        }

        //- Button functionality
        if mouse_x >= (window_width - 133)
            && mouse_y >= (window_height - 135)
            && mouse_x <= (window_width - 112)
            && mouse_y <= (window_height - 115)
        {
            draw.draw_rectangle(
                window_width - 130,
                window_height - 127,
                21,
                5,
                button_shadow_color,
            );
            draw.draw_rectangle(window_width - 133, window_height - 128, 21, 5, button_color);

            if draw.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
                audio_subsystem.play_sound(&global_resources.button_click_sound);
                if self.volume_percentage <= 1.0 && self.volume_percentage > 0.0 {
                    self.volume_percentage = self.volume_percentage - 0.25
                } else if self.volume_percentage <= 0.0 {
                    self.volume_percentage = 0.0;
                }
                audio_subsystem.set_master_volume(self.volume_percentage);
                game_settings.volume = Some(self.volume_percentage);
            }
        }

        // + Button functionallity
        if mouse_x >= (window_width - 70)
            && mouse_y >= (window_height - 135)
            && mouse_x <= (window_width - 49)
            && mouse_y <= (window_height - 115)
        {
            draw.draw_rectangle(
                window_width - 59,
                window_height - 134,
                5,
                20,
                button_shadow_color,
            ); //Vertical Line
            draw.draw_rectangle(
                window_width - 67,
                window_height - 127,
                21,
                5,
                button_shadow_color,
            );

            draw.draw_rectangle(window_width - 62, window_height - 135, 5, 20, button_color); // vertical line
            draw.draw_rectangle(window_width - 70, window_height - 128, 21, 5, button_color); //horizontal line

            if draw.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
                audio_subsystem.play_sound(&global_resources.button_click_sound);
                if self.volume_percentage < 1.0 && self.volume_percentage >= 0.0 {
                    self.volume_percentage = self.volume_percentage + 0.25
                } else if self.volume_percentage <= 0.0 {
                    self.volume_percentage = 0.0;
                }
                audio_subsystem.set_master_volume(self.volume_percentage);
                game_settings.volume = Some(self.volume_percentage);
            }
        }

        //Exit button
        if mouse_x >= 100 && mouse_y >= 550 && mouse_x <= 162 && mouse_y <= 575 {
            draw.draw_text("Exit", 103, 551, 34, label_shadow_colors);
            draw.draw_text("Exit", 100, 550, 34, label_colors);
            if draw.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
                audio_subsystem.play_sound(&global_resources.button_click_sound);
                return MenuStateSignal::QuitGame;
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
        //Options Errased, Block of code left for precaution
        return MenuStateSignal::DoMainMenu;
    }

    pub async fn render_credits_frame(
        &mut self,
        raylib: &mut RaylibHandle,
        rl_thread: &RaylibThread,
        discord: &DiscordChannel,
        global_resources: &GlobalResources,
        constants: &ProjectConstants,
        audio_subsystem: &mut RaylibAudio,
    ) -> MenuStateSignal {
        //Colors
        let label_colors = Color::BLACK;
        let label_shadow_colors = Color::GRAY;
        let credits_colours = Color::new(82, 135, 195, 255);

        let mut draw = raylib.begin_drawing(rl_thread);
        draw.clear_background(Color::WHITE);
        //Mouse Position
        let mouse_x = draw.get_mouse_x();
        let mouse_y = draw.get_mouse_y();

        //TODO Errase in the end Show mouse position
        draw.draw_text(&mouse_x.to_string(), 20, 5, 20, Color::BLACK);
        draw.draw_text(&mouse_y.to_string(), 70, 5, 20, Color::BLACK);

        //Screen Size
        let window_height = draw.get_screen_height();
        let window_width = draw.get_screen_width();

        draw.draw_text("Credits", (window_width / 2) - 100, 30, 55, label_colors);

        draw.draw_text(
            "Carter Tomlenovich",
            (window_width / 2) - 170,
            120,
            40,
            credits_colours,
        );
        draw.draw_text(
            "Emilia Firas",
            (window_width / 2) - 170,
            160,
            40,
            credits_colours,
        );
        draw.draw_text(
            "Emmet Logue",
            (window_width / 2) - 170,
            200,
            40,
            credits_colours,
        );
        draw.draw_text(
            "Evan Pratten",
            (window_width / 2) - 170,
            240,
            40,
            credits_colours,
        );
        draw.draw_text(
            "James Nickoli",
            (window_width / 2) - 170,
            280,
            40,
            credits_colours,
        );
        draw.draw_text(
            "Marcelo Geldres",
            (window_width / 2) - 170,
            320,
            40,
            credits_colours,
        );
        draw.draw_text("Percy", (window_width / 2) - 170, 360, 40, credits_colours);
        draw.draw_text(
            "Silas Bartha",
            (window_width / 2) - 170,
            400,
            40,
            credits_colours,
        );
        draw.draw_text(
            "Taya Armstrong",
            (window_width / 2) - 170,
            440,
            40,
            credits_colours,
        );

        //Return button variables
        let button_pos_x = 100; //116 Wide
        let button_pos_y = window_height - (window_height / 5); //26 height

        draw.draw_text("Return", button_pos_x, button_pos_y, 34, label_colors);
        if mouse_x >= 100
            && mouse_y >= button_pos_y
            && mouse_x <= 216
            && mouse_y <= (window_height - (window_height / 5)) + 26
        {
            draw.draw_text(
                "Return",
                button_pos_x + 3,
                button_pos_y + 1,
                34,
                label_shadow_colors,
            );
            draw.draw_text("Return", button_pos_x, button_pos_y, 34, label_colors);
            if draw.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) {
                audio_subsystem.play_sound(&global_resources.button_click_sound);
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
        audio_subsystem: &mut RaylibAudio,
    ) -> MenuStateSignal {
        //Colors
        let label_colors = Color::BLACK;
        let label_shadow_colors = Color::GRAY;

        let mut draw = raylib.begin_drawing(rl_thread);
        draw.clear_background(Color::WHITE);
        //Mouse Position
        let mouse_x = draw.get_mouse_x();
        let mouse_y = draw.get_mouse_y();

        //Window size storing variables
        let window_height = draw.get_screen_height();
        let window_width = draw.get_screen_width();

        //TODO errase later
        draw.draw_text(&mouse_x.to_string(), 20, 5, 20, Color::BLACK);
        draw.draw_text(&mouse_y.to_string(), 70, 5, 20, Color::BLACK);

        let window_width = draw.get_screen_width();
        draw.draw_text(
            "Leaderboard",
            (window_width / 2) - 176,
            30,
            55,
            label_colors,
        );

        //Return button variables
        let button_pos_x = 100; //116 Wide
        let button_pos_y = window_height - (window_height / 5); //26 height

        draw.draw_text("Return", button_pos_x, button_pos_y, 34, label_colors);
        if mouse_x >= 100
            && mouse_y >= button_pos_y
            && mouse_x <= 216
            && mouse_y <= (window_height - (window_height / 5)) + 26
        {
            draw.draw_text(
                "Return",
                button_pos_x + 3,
                button_pos_y + 1,
                34,
                label_shadow_colors,
            );
            draw.draw_text("Return", button_pos_x, button_pos_y, 34, label_colors);
            if draw.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) {
                audio_subsystem.play_sound(&global_resources.button_click_sound);
                return MenuStateSignal::DoMainMenu; //Goes back to main menu
            }
        }

        return MenuStateSignal::DoLeaderboard;
    }
}
