//! The render code for various scenes
//!
//! ## Overview
//!
//! This will probably become a messy module over time. Stick your rendering code here
use raylib::prelude::*;

use crate::{
    discord::DiscordChannel, global_resource_package::GlobalResources,
    project_constants::ProjectConstants,
};

use self::{player_interaction::PlayableScene, test_fox::TestFoxScene, main_menu::MainMenu};
mod player_interaction;
mod test_fox;
mod main_menu;

/// Delegate for handling rendering.
/// This is a struct to allow for stateful data (like sub-screens) to be set up
pub struct SceneRenderDelegate {
    is_in_main_menu: bool,
    /* Scenes */
    scene_test_fox: TestFoxScene,
    scene_playable: PlayableScene,
    scene_main_menu: MainMenu
}

impl SceneRenderDelegate {
    /// This is called when the game first loads
    pub fn on_game_start(
        raylib: &mut RaylibHandle,
        rl_thread: &RaylibThread,
        constants: &ProjectConstants,
    ) -> Self {
        // TODO: Stick any init code you want here.

        // Init some scenes
        let scene_test_fox = TestFoxScene::new(raylib, rl_thread);
        let scene_playable = PlayableScene::new(raylib, rl_thread, constants);
        let scene_main_menu = MainMenu::new(raylib, rl_thread, constants);

        Self {
            is_in_main_menu: true,
            scene_test_fox,
            scene_playable,
            scene_main_menu,
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
    ) {
        // Render the main menu if in it, otherwise, render the game
        if self.is_in_main_menu {
            self.is_in_main_menu = !self.scene_main_menu
                .render_frame(raylib, rl_thread, discord, global_resources, constants)
                .await;
        }else {
            self.scene_playable
                .render_frame(raylib, rl_thread, &discord, global_resources, constants)
                .await;
        }
    }
}

impl Drop for SceneRenderDelegate {
    /// If you need anything to happen when the game closes, stick it here.
    fn drop(&mut self) {}
}
