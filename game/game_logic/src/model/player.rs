use nalgebra as na;

use raylib::prelude::*;

use crate::asset_manager::load_texture_from_internal_data;

#[derive(Debug)]
pub struct Player {
    pub position: na::Vector2<f32>,
    pub velocity: na::Vector2<f32>,
    pub size: f32,
    pub active_texture: i32,
    pub textures: Vec<Texture2D>,
}

impl Player {
    /// Construct a new player.
    pub fn new(
        raylib_handle: &mut raylib::RaylibHandle,
        thread: &raylib::RaylibThread,
        position: na::Vector2<f32>,
    ) -> Self {
        // Load all the textures
        let textures = vec![
            load_texture_from_internal_data(
                raylib_handle,
                thread,
                "assets/chr/chr_cubee/chr_cubeeLarge.png",
            )
            .unwrap(),
            load_texture_from_internal_data(
                raylib_handle,
                thread,
                "assets/chr/chr_cubee/chr_cubeeMedium.png",
            )
            .unwrap(),
            load_texture_from_internal_data(
                raylib_handle,
                thread,
                "assets/chr/chr_cubee/chr_cubeeSmall.png",
            )
            .unwrap(),
        ];

        Self {
            position,
            velocity: na::Vector2::zeros(),
            size: 1.0,
            active_texture: 0,
            textures,
        }
    }
}
