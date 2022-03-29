//! This "scene" is used only for testing animation and resource loading
//! It should be removed once the game is being worked on

use raylib::{RaylibHandle, RaylibThread};

use crate::{
    discord::DiscordChannel, global_resource_package::GlobalResources,
    rendering::utilities::anim_texture::AnimatedTexture,
};

#[derive(Debug)]
pub struct TestFoxScene {
    fox_animation: AnimatedTexture,
}

impl TestFoxScene {
    /// Construct a new `TestFoxScene`
    pub fn new() -> Self {
        // Load the fox texture
        let fox = AnimatedTexture::new();

        Self { fox_animation: fox }
    }

    /// Handler for each frame
    pub fn render_frame(
        &mut self,
        raylib: &mut RaylibHandle,
        rl_thread: &RaylibThread,
        discord: &DiscordChannel,
        global_resources: &GlobalResources,
    ) {
    }
}
