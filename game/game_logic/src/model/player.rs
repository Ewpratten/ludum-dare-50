use nalgebra as na;

use raylib::prelude::*;

#[derive(Debug)]
pub struct Player {
    pub position: na::Vector2<f32>,
    pub velocity: na::Vector2<f32>,
    pub size: f32,
    pub active_texture: i32,
    pub textures: [String; 3],
}

impl Player {
    
    /// Construct a new player.
    pub fn new(position: na::Vector2<f32>) -> Self {
        Self {
            position,
            velocity: na::Vector2::zeros(),
            size: 1.0, 
            active_texture: 0, 
            textures: [
                "assets/chr/chr_cubee/chr_cubeeLarge.png".to_string(),
                "assets/chr/chr_cubee/chr_cubeeMedium.png".to_string(),
                "assets/chr/chr_cubee/chr_cubeeSmall.png".to_string()
            ]
        }
    }

}
