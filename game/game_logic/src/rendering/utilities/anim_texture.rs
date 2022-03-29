//! This module handles the code for rendering framerate-locked animations from textures

use chrono::{DateTime, Utc};
use nalgebra::Vector2;
use raylib::{
    color::Color,
    math::Rectangle,
    prelude::{RaylibDraw, RaylibDrawHandle},
    texture::Texture2D,
    RaylibHandle, RaylibThread,
};
use serde::Deserialize;

use crate::asset_manager::{
    load_json_structure, load_known_sprite_types, load_texture_from_internal_data,
    InternalJsonLoadError,
};

/// Possible errors to be thrown during the animation texture loading process
#[derive(Debug, thiserror::Error)]
pub enum AnimatedTextureLoadError {
    #[error(transparent)]
    MetadataLoadError(#[from] InternalJsonLoadError),
    #[error(transparent)]
    KnownSpriteTypesLoadError(#[from] serde_json::Error),
    #[error("Invalid Sprite Type: {0}")]
    InvalidSpriteType(String),
    #[error(transparent)]
    TextureLoadError(#[from] crate::asset_manager::ResourceLoadError),
}

/// Definition for the structure describing a frame's size and position in a texture
#[derive(Debug, Clone, Deserialize)]
struct FrameTextureDescriptor {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Into<Rectangle> for FrameTextureDescriptor {
    fn into(self) -> Rectangle {
        Rectangle::new(self.x, self.y, self.width, self.height)
    }
}

/// Definition for the metadata structure attached to each spritesheet
#[derive(Debug, Clone, Deserialize)]
struct AnimatedTextureMetadata {
    pub sheet_height: u64,
    pub sheet_width: u64,
    pub fps: f32,
    pub frames: Vec<FrameTextureDescriptor>,
}

#[derive(Debug)]
pub struct AnimatedTexture {
    /// The whole internal spritesheet
    texture: Texture2D,
    /// The metadata describing the spritesheet
    texture_metadata: AnimatedTextureMetadata,
    /// a list of source rects to reduce memory allocation needs during render time
    texture_source_rects: Vec<Rectangle>,
    /// The animation start timestamp
    start_time: Option<DateTime<Utc>>,
}

impl AnimatedTexture {
    /// Construct a new `AnimatedTexture`
    ///
    /// This will load all resources from RAM or disk. May take a while.
    #[profiling::function]
    pub fn new(
        raylib_handle: &mut RaylibHandle,
        thread: &RaylibThread,
        sprite_type: &str,
        sprite_name: &str,
    ) -> Result<Self, AnimatedTextureLoadError> {
        // Try to convert the sprite type string to a real type
        let known_sprite_types = load_known_sprite_types()?;
        let sprite_type_obj = known_sprite_types.iter().find(|known_sprite_type| {
            known_sprite_type.short_name == sprite_type
                || known_sprite_type.friendly_name == sprite_type
        });
        if let None = sprite_type_obj {
            error!("Invalid sprite type supplied: {}", sprite_type);
            return Err(AnimatedTextureLoadError::InvalidSpriteType(
                sprite_type.to_string(),
            ));
        }
        let sprite_type_obj = sprite_type_obj.unwrap();

        // Now, we can construct the paths to the texture and metadata
        let parent_dir_path = format!(
            "assets/anm/{}/{}_{}",
            sprite_type_obj.short_name, sprite_type_obj.short_name, sprite_name
        );
        let metadata_file_path = format!(
            "{}/{}_{}.anim_meta.json",
            parent_dir_path, sprite_type_obj.short_name, sprite_name
        );
        let texture_file_path = format!(
            "{}/{}_{}.png",
            parent_dir_path, sprite_type_obj.short_name, sprite_name
        );

        // Attempt to load the metadata
        let texture_metadata: AnimatedTextureMetadata = load_json_structure(&metadata_file_path)?;
        let source_rects = texture_metadata
            .frames
            .iter()
            .map(|frame_descriptor| frame_descriptor.clone().into())
            .collect();

        // Attempt to load the texture itself
        let texture = load_texture_from_internal_data(raylib_handle, thread, &texture_file_path)?;

        Ok(Self {
            texture,
            texture_metadata,
            texture_source_rects: source_rects,
            start_time: None,
        })
    }

    /// Render a single frame to the screen
    #[profiling::function]
    pub fn render_frame_by_index(
        &self,
        draw_handle: &mut RaylibDrawHandle,
        index: usize,
        position: Vector2<f32>,
        percent_scale: Option<Vector2<f32>>,
        origin: Option<Vector2<f32>>,
        rotation: Option<f32>,
        tint: Option<Color>,
    ) {
        // Get the frame-specific metadata
        let metadata = &self.texture_metadata.frames[index];

        // Build a source rectangle
        let source = self.texture_source_rects[index];

        // Build a destination rectangle
        let scaler = percent_scale.unwrap_or(Vector2::new(1.0, 1.0));
        let destination = Rectangle::new(
            position.x,
            position.y,
            metadata.width * scaler.x,
            metadata.height * scaler.y,
        );
        let origin: raylib::core::math::Vector2 =
            origin.unwrap_or_else(|| Vector2::<f32>::zeros()).into();
        // debug!("{:?} -> {:?}", source, destination);

        // Render the frame
        draw_handle.draw_texture_pro(
            &self.texture,
            source,
            destination,
            origin,
            rotation.unwrap_or(0.0),
            tint.unwrap_or(Color::WHITE),
        );
    }

    /// Clear the internal tracker for when the animation started
    ///
    /// This will bring the animation back to frame 1. Useful for non-looping rendering
    pub fn reset_animation(&mut self) {
        self.start_time = None;
    }

    /// Get the current frame index
    pub fn get_current_frame_index(&self) -> Option<usize> {
        self.start_time.map(|start_time| {
            let elapsed_time_ms = Utc::now()
                .signed_duration_since(start_time)
                .num_milliseconds() as f32;
            let elapsed_time_s = elapsed_time_ms / 1000.0;
            let frame_index = (elapsed_time_s * self.texture_metadata.fps) as usize;
            frame_index % self.texture_metadata.frames.len()
        })
    }

    /// Render the animation based on timestamp
    pub fn render_automatic(
        &mut self,
        draw_handle: &mut RaylibDrawHandle,
        position: Vector2<f32>,
        percent_scale: Option<Vector2<f32>>,
        origin: Option<Vector2<f32>>,
        rotation: Option<f32>,
        tint: Option<Color>,
    ) {
        // If this is the first time we're rendering, set the start time
        if self.start_time.is_none() {
            self.start_time = Some(Utc::now());
        }

        // Get the current frame index
        let current_frame_index = self.get_current_frame_index();

        // If we have a valid index, render it
        if let Some(current_frame_index) = current_frame_index {
            self.render_frame_by_index(
                draw_handle,
                current_frame_index,
                position,
                percent_scale,
                origin,
                rotation,
                tint,
            );
        } else {
            warn!("We somehow got a frame index of None");
        }
    }
}
