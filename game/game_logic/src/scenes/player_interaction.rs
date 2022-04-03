//! This scene encompasses all of the game where the player can walk around.

use nalgebra as na;
use raylib::prelude::*;
use std::time::SystemTime;

use crate::{
    asset_manager::{load_music_from_internal_data, load_sound_from_internal_data},
    discord::{DiscordChannel, DiscordRpcSignal},
    global_resource_package::GlobalResources,
    model::{
        player::Player,
        world_object::{ObjectCollider, WorldSpaceObjectCollider},
    },
    project_constants::ProjectConstants,
    rendering::utilities::{anim_texture::AnimatedTexture, map_render::MapRenderer},
};

#[derive(Debug)]
pub struct PlayableScene {
    pub has_updated_discord_rpc: bool,
    player: Player,
    world_map: MapRenderer,
    camera: raylib::camera::Camera2D,
    last_update: SystemTime,
    game_soundtrack: Music,
    world_colliders: Vec<WorldSpaceObjectCollider>,
}

impl PlayableScene {
    /// Construct a new `PlayableScene`
    pub fn new(
        raylib_handle: &mut raylib::RaylibHandle,
        thread: &raylib::RaylibThread,
        constants: &ProjectConstants,
    ) -> Self {
        let map_renderer = MapRenderer::new(
            "map_gameMap.tmx",
            "map_gameMap.objects.json",
            raylib_handle,
            thread,
        )
        .unwrap();
        let world_colliders = map_renderer.get_world_colliders();

        // Load the game music
        let game_soundtrack =
            load_music_from_internal_data(thread, "assets/audio/gameSoundtrack.mp3").unwrap();

        Self {
            has_updated_discord_rpc: false,
            player: Player::new(na::Vector2::new(
                10.0 * constants.tile_size as f32,
                -10.0 * constants.tile_size as f32,
            )),
            world_map: map_renderer,
            camera: raylib::camera::Camera2D {
                target: raylib::math::Vector2 { x: 0.0, y: 0.0 },
                offset: raylib::math::Vector2 { x: 0.0, y: 0.0 },
                rotation: 0.0,
                zoom: 1.0,
            },
            last_update: SystemTime::UNIX_EPOCH,
            game_soundtrack,
            world_colliders,
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
        audio_subsystem: &mut RaylibAudio,
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

        // Ensure the game soundtrack is playing
        if !audio_subsystem.is_music_playing(&self.game_soundtrack) {
            debug!("Playing game soundtrack");
            audio_subsystem.play_music_stream(&mut self.game_soundtrack);
        } else {
            audio_subsystem.update_music_stream(&mut self.game_soundtrack);
        }

        // Get a drawing handle
        let mut draw = raylib.begin_drawing(rl_thread);

        // Clear the screen
        draw.clear_background(Color::WHITE);

        self.draw_world(&mut draw, constants);

        self.draw_ui(&mut draw, constants);
    }

    pub fn draw_world(&mut self, draw: &mut RaylibDrawHandle, constants: &ProjectConstants) {
        // Begin camera mode
        let mut ctx2d = draw.begin_mode2D(self.camera);

        // Render the map
        self.world_map
            .render_map(&mut ctx2d, &self.camera, true, self.player.position);

        let player_size =
            (constants.tile_size as f32 * constants.player.start_size * self.player.size) as i32;

        ctx2d.draw_rectangle(
            self.player.position[0] as i32 - player_size / 2,
            self.player.position[1] as i32 * -1 - player_size / 2,
            player_size,
            player_size,
            Color::LIGHTBLUE,
        );
    }

    pub fn draw_ui(&mut self, draw: &mut RaylibDrawHandle, constants: &ProjectConstants) {
        draw.draw_rectangle(draw.get_screen_width() / 2 - 225, 0, 450, 40, Color::WHITE);
        draw.draw_text(
            "Unregistered HyperCam 2",
            draw.get_screen_width() / 2 - 215,
            0,
            32,
            Color::BLACK,
        );
    }

    // Physics
    pub async fn update_physics(
        &mut self,
        raylib: &raylib::RaylibHandle,
        constants: &ProjectConstants,
    ) {
        // Get time since last physics update
        let time = SystemTime::now();
        let elapsed = time
            .duration_since(self.last_update)
            .expect("Time Appears to Have Moved Backwards!");
        self.last_update = time;
        let delta_time = elapsed.as_millis() as f32 / 1000.0; // Physics will be scaled by this value

        let player = &mut self.player;

        // NOTE: This is how to check friction and temperature
        let current_friction = self.world_map.sample_friction_at(player.position);
        let current_temperature = self.world_map.sample_temperature_at(player.position);
        let map_size = self.world_map.get_map_size();
        // TODO: You can access the colission list with: self.world_colliders

        // Get input direction components
        let h_axis = raylib.is_key_down(KeyboardKey::KEY_D) as i8
            - raylib.is_key_down(KeyboardKey::KEY_A) as i8;
        let v_axis = raylib.is_key_down(KeyboardKey::KEY_W) as i8
            - raylib.is_key_down(KeyboardKey::KEY_S) as i8;
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
            if player.velocity.magnitude() < 1.0 {
                player.velocity.set_magnitude(0.0);
            }
        }

        if ((constants.player.max_velocity * constants.tile_size) as f32)
            < player.velocity.magnitude()
        {
            player
                .velocity
                .set_magnitude((constants.player.max_velocity * constants.tile_size) as f32);
        }

        let velocity_modifier = &player.velocity * delta_time;

        player.position.x += velocity_modifier.x;

        for i in &self.world_colliders {
            if player.position.x <= i.position.x + i.size.x
                && player.position.x + player.size >= i.position.x
                && player.position.y <= i.position.y + i.size.y
                && player.position.y + player.size >= i.position.y
            {
                player.position.x -= velocity_modifier.x;
                player.velocity.x = 0;
                break;
            }
        }

        if player.position.x < 0.0 || next_player_position.x > self.world_map.get_map_size().x  * 128.0 {
            player.position.x -= velocity_modifier.x;
            player_velocity.x = 0.0;
        }

        player.position.y += velocity_modifier.y;

        for i in &self.world_colliders {
            if player.position.x <= i.position.x + i.size.x
                && player.position.x + player.size >= i.position.x
                && player.position.y <= i.position.y + i.size.y
                && player.position.y + player.size >= i.position.y
            {
                player.position.y -= velocity_modifier.y;
                break;
            }
        }

        if player.position.y < 0.0 || next_player_position.y > self.world_map.get_map_size().y  * 128.0 {
            player.position.y -= velocity_modifier.y;
            player_velocity.y = 0.0;
        }

        self.update_camera(raylib);
    }

    // Update the camera
    pub fn update_camera(&mut self, raylib: &raylib::RaylibHandle) {
        // Bounding box
        let bbox = na::Vector2::new(0.2, 0.2);

        // Get bounding box dimensions on the screen
        let bbox_screen_min: raylib::math::Vector2 = (((na::Vector2::new(1.0, 1.0) - bbox) * 0.5)
            .component_mul(&na::Vector2::new(
                raylib.get_screen_width() as f32,
                raylib.get_screen_height() as f32,
            )))
        .into();
        let bbox_screen_max: raylib::math::Vector2 = (((na::Vector2::new(1.0, 1.0) + bbox) * 0.5)
            .component_mul(&na::Vector2::new(
                raylib.get_screen_width() as f32,
                raylib.get_screen_height() as f32,
            )))
        .into();

        // Get bounding box in world space
        let mut bbox_world_min = raylib.get_screen_to_world2D(bbox_screen_min, self.camera);
        let mut bbox_world_max = raylib.get_screen_to_world2D(bbox_screen_max, self.camera);

        // Invert y
        bbox_world_min.y *= -1.0;
        bbox_world_max.y *= -1.0;

        self.camera.offset = bbox_screen_min;

        if self.player.position.x < bbox_world_min.x {
            self.camera.target.x = self.player.position.x;
        }

        if self.player.position.y > bbox_world_min.y {
            self.camera.target.y = -self.player.position.y;
        }

        if self.player.position.x > bbox_world_max.x {
            self.camera.target.x = bbox_world_min.x + (self.player.position.x - bbox_world_max.x);
        }

        if self.player.position.y < bbox_world_max.y {
            self.camera.target.y = bbox_world_max.y - (self.player.position.y + bbox_world_min.y);
        }
    }
}
