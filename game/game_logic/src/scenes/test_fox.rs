//! This "scene" is used only for testing animation and resource loading
//! It should be removed once the game is being worked on

use nalgebra as na;
use raylib::prelude::*;

use crate::{
    discord::DiscordChannel,
    global_resource_package::GlobalResources,
    rendering::utilities::{anim_texture::AnimatedTexture, map_render::MapRenderer},
};

#[derive(Debug)]
pub struct TestFoxScene {
    fox_animation: AnimatedTexture,
    world_map: MapRenderer,
    camera: Camera2D,
}

impl TestFoxScene {
    /// Construct a new `TestFoxScene`
    pub fn new(raylib_handle: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        // Load the fox texture
        let fox = AnimatedTexture::new(raylib_handle, thread, "chr", "testFox").unwrap();

        // Load the map
        let map_renderer = MapRenderer::new(
            "map_gameMap.tmx",
            "map_gameMap.objects.json",
            raylib_handle,
            thread,
        )
        .unwrap();

        // Create a camera
        let camera = Camera2D {
            target: Vector2 { x: 0.0, y: 0.0 },
            offset: Vector2 {
                x: raylib_handle.get_screen_width() as f32 * 0.5,
                y: (raylib_handle.get_screen_height() as f32) * 0.5,
            },
            rotation: 0.0,
            zoom: 1.0,
        };

        Self {
            fox_animation: fox,
            world_map: map_renderer,
            camera,
        }
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

        // // Render the fox
        // self.fox_animation.render_automatic(
        //     &mut draw,
        //     na::Vector2::new(0.0, 0.0),
        //     None,
        //     None,
        //     None,
        //     None,
        // );

        // Allow the camera to be moved with wasd
        if draw.is_key_down(KeyboardKey::KEY_W) {
            self.camera.target.y -= 5.0;
        }
        if draw.is_key_down(KeyboardKey::KEY_S) {
            self.camera.target.y += 5.0;
        }
        if draw.is_key_down(KeyboardKey::KEY_A) {
            self.camera.target.x -= 5.0;
        }
        if draw.is_key_down(KeyboardKey::KEY_D) {
            self.camera.target.x += 5.0;
        }

        {
            // Begin camera mode
            let mut ctx2d = draw.begin_mode2D(self.camera);

            // Render the map
            self.world_map.render_map(
                &mut ctx2d,
                &self.camera,
                true,
                na::Vector2::new(self.camera.target.x, self.camera.target.y).into(),
            );
        }

        draw.draw_circle(
            draw.get_screen_width() / 2,
            draw.get_screen_height() / 2,
            4.0,
            Color::RED,
        );
    }
}
