//! This "scene" is used only for testing animation and resource loading
//! It should be removed once the game is being worked on

use raylib::prelude::*;
use nalgebra as na;

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
    pub fn new(raylib_handle: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        // Load the fox texture
        let fox = AnimatedTexture::new(raylib_handle, thread, "test", "debugTexture").unwrap();

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
        // Get a drawing handle
        let mut draw = raylib.begin_drawing(rl_thread);

        // Clear the screen
        draw.clear_background(Color::WHITE);

        // Render the fox
        self.fox_animation.render_frame_by_index(
            &mut draw,
            0,
            na::Vector2::new(0.0, 0.0),
            None,
            None,
            None,
            None,
        );
    }
}
