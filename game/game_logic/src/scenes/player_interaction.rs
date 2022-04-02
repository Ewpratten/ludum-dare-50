//! This scene encompasses all of the game where the player can walk around.

use nalgebra as na;
use raylib::prelude::*;
use std::time::SystemTime;

use crate::{
    discord::{DiscordChannel, DiscordRpcSignal},
    global_resource_package::GlobalResources,
    model::player::Player,
    project_constants::ProjectConstants,
    rendering::utilities::{anim_texture::AnimatedTexture, map_render::MapRenderer},
};

#[derive(Debug)]
pub struct PlayableScene {
    has_updated_discord_rpc: bool,
    player: Player,
    world_map: MapRenderer,
    camera: raylib::camera::Camera2D,
    last_update: SystemTime,
}

impl PlayableScene {
    /// Construct a new `PlayableScene`
    pub fn new(
        raylib_handle: &mut raylib::RaylibHandle,
        thread: & raylib::RaylibThread,
        constants: &ProjectConstants,
    ) -> Self {

        let map_renderer = MapRenderer::new("map_gameMap.tmx", raylib_handle, thread).unwrap();

        Self {
            has_updated_discord_rpc: false,
            player: Player::new(na::Vector2::new(10.0, 10.0)),
            world_map: map_renderer,
            camera: raylib::camera::Camera2D {
                target: raylib::math::Vector2 { 
                    x: 0.0, 
                    y: 0.0,
                },
                offset: raylib::math::Vector2 { 
                    x: (constants.base_window_size.0 as f32 / 2.0),
                    y: (constants.base_window_size.1 as f32 / 2.0) 
                },
                rotation: 0.0,
                zoom: 1.0
            },
            last_update: SystemTime::UNIX_EPOCH
        }
    }

    /// Handler for each frame
    pub async fn render_frame(
        &mut self,
        raylib: &mut raylib::RaylibHandle,
        rl_thread: &raylib::RaylibThread,
        discord: &DiscordChannel,
        global_resources: &GlobalResources,
        constants: &ProjectConstants,
    ) {
        // Handle updating discord RPC
        if !self.has_updated_discord_rpc {
            discord
                .send(DiscordRpcSignal::BeginGameTimer)
                .await
                .unwrap();
            discord
                .send(DiscordRpcSignal::ChangeDetails {
                    details: "Playing the game".to_string(),
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

        {
            // Begin camera mode
            let mut ctx2d = draw.begin_mode2D(self.camera);

            // Render the map
            self.world_map.render_map(&mut ctx2d, &self.camera, true);
        }       

        for i in 0..100 {
            for j in 0..100 {
                draw.draw_rectangle(
                    constants.tile_size as i32 * (i * 2),
                    constants.tile_size as i32 * (j * 2) * -1,
                    constants.tile_size as i32,
                    constants.tile_size as i32,
                    Color::RED
                )
            }
        }    

        draw.draw_rectangle(
            self.player.position[0] as i32, 
            self.player.position[1] as i32 * -1,
            (constants.tile_size as f32 * self.player.size) as i32, 
            (constants.tile_size as f32 * self.player.size) as i32, 
            Color::GREEN
        );
    }
        
    // Physics
    pub async fn update_physics(
        &mut self,
        raylib: & raylib::RaylibHandle,
        constants: &ProjectConstants,
    ) {

        // Get time since last physics update
        let time = SystemTime::now();
        let elapsed = time.duration_since(self.last_update).expect("Time Appears to Have Moved Backwards!");
        self.last_update = time;
        let delta_time = elapsed.as_millis() as f32 / 1000.0; // Physics will be scaled by this value

        let player = &mut self.player;

        // Get input direction components
        let h_axis = raylib.is_key_down(KeyboardKey::KEY_D) as i8 - raylib.is_key_down(KeyboardKey::KEY_A) as i8;
        let v_axis = raylib.is_key_down(KeyboardKey::KEY_W) as i8 - raylib.is_key_down(KeyboardKey::KEY_S) as i8;
        if h_axis != 0 || v_axis != 0 {
            // Normalize input and accelerate in desired direction
            let direction = na::Vector2::new(h_axis as f32, v_axis as f32).normalize();
            player.velocity += &direction.xy() 
                * constants.player.acceleration as f32
                * constants.tile_size as f32 
                * delta_time;
        }

        if player.velocity.magnitude() != 0.0 {
            player.velocity -= player.velocity.normalize() 
                * constants.player.deceleration as f32 
                * constants.tile_size as f32 
                * delta_time;
            if player.velocity.magnitude() < 0.01 {
                player.velocity.set_magnitude(0.0);
            }
        }

        if ((constants.player.max_velocity * constants.tile_size) as f32) 
            < player.velocity.magnitude() {
            player.velocity.set_magnitude((constants.player.max_velocity * constants.tile_size) as f32);
        }

        player.position += &player.velocity * delta_time;
    }
}


