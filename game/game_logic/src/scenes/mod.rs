//! The render code for various scenes
//!
//! ## Overview
//!
//! This will probably become a messy module over time. Stick your rendering code here
use raylib::prelude::*;

use crate::{discord::DiscordChannel, global_resource_package::GlobalResources};

use self::test_fox::TestFoxScene;
mod test_fox;

/// Delegate for handling rendering.
/// This is a struct to allow for stateful data (like sub-screens) to be set up
pub struct SceneRenderDelegate {
    /* Scenes */
    scene_test_fox: TestFoxScene,
}

impl SceneRenderDelegate {
    /// This is called when the game first loads
    pub fn on_game_start() -> Self {
        // TODO: Stick any init code you want here.

        // Init some scenes
        let scene_test_fox = TestFoxScene::new();

        Self { scene_test_fox }
    }

    /// This is called every frame once the game has started.
    ///
    /// Keep in mind everything you do here will block the main thread (no loading files plz)
    pub fn process_ingame_frame(
        &mut self,
        raylib: &mut RaylibHandle,
        rl_thread: &RaylibThread,
        discord: &DiscordChannel,
        global_resources: &GlobalResources,
    ) {
        // For now, we will just render the test fox scene
        self.scene_test_fox
            .render_frame(raylib, rl_thread, &discord, global_resources);
    }
}

impl Drop for SceneRenderDelegate {
    /// If you need anything to happen when the game closes, stick it here.
    fn drop(&mut self) {}
}
