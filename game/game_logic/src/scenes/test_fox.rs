//! This "scene" is used only for testing animation and resource loading
//! It should be removed once the game is being worked on

use raylib::prelude::*;
use nalgebra as na;

use crate::{
    discord::DiscordChannel, global_resource_package::GlobalResources,
    rendering::utilities::{anim_texture::AnimatedTexture, map_render::MapRenderer},
};

#[derive(Debug)]
pub struct TestFoxScene {
    fox_animation: AnimatedTexture,
    world_map: MapRenderer
}

impl TestFoxScene {
    /// Construct a new `TestFoxScene`
    pub fn new(raylib_handle: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        // Load the fox texture
        let fox = AnimatedTexture::new(raylib_handle, thread, "chr", "testFox").unwrap();

        // Load the map
        let map_renderer = MapRenderer::new("map_gameMap.tmx").unwrap();

        Self { fox_animation: fox, world_map: map_renderer }
    }

    /// Handler for each frame
    pub async fn render_frame(
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
        self.fox_animation.render_automatic(
            &mut draw,
            na::Vector2::new(0.0, 0.0),
            None,
            None,
            None,
            None,
        );
    }
}
