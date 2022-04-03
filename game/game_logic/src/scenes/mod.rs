//! The render code for various scenes
//!
//! ## Overview
//!
//! This will probably become a messy module over time. Stick your rendering code here
use raylib::prelude::*;

use crate::{
    discord::DiscordChannel,
    global_resource_package::GlobalResources,
    persistent::{save_state::GameSaveState, settings::PersistentGameSettings},
    project_constants::ProjectConstants,
};

use self::{
    main_menu::{MainMenu, MenuStateSignal},
    pause_menu::PauseMenu,
    player_interaction::PlayableScene,
    test_fox::TestFoxScene,
};
mod main_menu;
mod pause_menu;
mod player_interaction;
mod test_fox;

/// Delegate for handling rendering.
/// This is a struct to allow for stateful data (like sub-screens) to be set up
pub struct SceneRenderDelegate {
    menu_control_signal: MenuStateSignal,
    pub needs_exit: bool,
    audio_subsystem: RaylibAudio,
    /* Scenes */
    scene_test_fox: TestFoxScene,
    scene_playable: PlayableScene,
    scene_main_menu: MainMenu,
    scene_pause_menu: PauseMenu,
}

impl SceneRenderDelegate {
    /// This is called when the game first loads
    pub fn on_game_start(
        raylib: &mut RaylibHandle,
        rl_thread: &RaylibThread,
        constants: &ProjectConstants,
        audio_subsystem: RaylibAudio,
        game_settings: &mut PersistentGameSettings,
        save_state: &mut GameSaveState,
    ) -> Self {
        // Init some scenes
        let scene_test_fox = TestFoxScene::new(raylib, rl_thread);
        let scene_playable = PlayableScene::new(raylib, rl_thread, constants);
        let scene_main_menu = MainMenu::new(raylib, rl_thread, constants, game_settings);
        let scene_pause_menu = PauseMenu::new(raylib, rl_thread, constants, game_settings);

        Self {
            menu_control_signal: MenuStateSignal::DoMainMenu,
            needs_exit: false,
            audio_subsystem,
            scene_test_fox,
            scene_playable,
            scene_main_menu,
            scene_pause_menu,
        }
    }

    /// This is called every frame once the game has started.
    ///
    /// Keep in mind everything you do here will block the main thread (no loading files plz)
    pub async fn process_ingame_frame(
        &mut self,
        raylib: &mut RaylibHandle,
        rl_thread: &RaylibThread,
        discord: &DiscordChannel,
        global_resources: &GlobalResources,
        constants: &ProjectConstants,
        game_settings: &mut PersistentGameSettings,
        save_state: &mut GameSaveState,
    ) {
        // Render the main menu if in it, otherwise, render the game
        match self.menu_control_signal {
            MenuStateSignal::StartGame => {
                self.menu_control_signal = self.scene_playable
                    .render_frame(
                        raylib,
                        rl_thread,
                        &discord,
                        global_resources,
                        constants,
                        &mut self.audio_subsystem,
                    )
                    .await;
                self.scene_playable.update_physics(raylib, constants).await;

                // Clear the menu system discord status
                self.scene_main_menu.has_updated_discord_rpc = false;
            }
            MenuStateSignal::QuitGame => {
                self.needs_exit = true;
            }
            MenuStateSignal::DoMainMenu => {
                self.menu_control_signal = self
                    .scene_main_menu
                    .render_main_menu_frame(
                        raylib,
                        rl_thread,
                        discord,
                        global_resources,
                        constants,
                        &mut self.audio_subsystem,
                        game_settings,
                    )
                    .await;

                // Clear the ingame discord status
                self.scene_playable.has_updated_discord_rpc = false;
            }
            MenuStateSignal::DoOptions => {
                self.menu_control_signal = self
                    .scene_main_menu
                    .render_options_frame(raylib, rl_thread, discord, global_resources, constants)
                    .await
            }
            MenuStateSignal::DoCredits => {
                self.menu_control_signal = self
                    .scene_main_menu
                    .render_credits_frame(
                        raylib,
                        rl_thread,
                        discord,
                        global_resources,
                        constants,
                        &mut self.audio_subsystem,
                    )
                    .await
            }
            MenuStateSignal::DoLeaderboard => {
                self.menu_control_signal = self
                    .scene_main_menu
                    .render_leaderboard_frame(
                        raylib,
                        rl_thread,
                        discord,
                        global_resources,
                        constants,
                        &mut self.audio_subsystem,
                    )
                    .await
            }
            MenuStateSignal::DoPauseMenu => {
                self.menu_control_signal = self
                    .scene_pause_menu
                    .render_pause_menu_frame(
                        raylib,
                        rl_thread,
                        discord,
                        global_resources,
                        constants,
                        &mut self.audio_subsystem,
                    )
                    .await;
            }
        }
    }
}

impl Drop for SceneRenderDelegate {
    /// If you need anything to happen when the game closes, stick it here.
    fn drop(&mut self) {}
}
