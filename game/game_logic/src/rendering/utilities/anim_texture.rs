//! This module handles the code for rendering framerate-locked animations from textures

use raylib::texture::Texture2D;

#[derive(Debug)]
pub struct AnimatedTexture {
    texture: Texture2D,
    target_fps: f32,
}

impl AnimatedTexture {
    /// Construct a new `AnimatedTexture`
    pub fn new(texture: Texture2D, target_frames_per_second: f32) -> Self {
        Self {
            texture,
            target_fps: target_frames_per_second,
        }
    }

    pub fn render_frame_by_index(&self, index: usize) {

    }
}
