use std::collections::HashMap;

use nalgebra as na;
use serde::Deserialize;

use crate::{
    asset_manager::{load_json_structure, InternalJsonLoadError},
    rendering::utilities::anim_texture::AnimatedTexture,
};

#[derive(Debug, Clone, Deserialize)]
pub struct PossiblyAnimatedTexture {
    /// Signal if the texture is animated or static
    pub animated: Option<bool>,
    /// Relative file path from `dist` to the texture
    pub file_path: String,
}

/// Defines a collider in object space.
#[derive(Debug, Clone, Deserialize)]
pub struct ObjectCollider {
    /// Position, relative to the object's center (north east is 1,1 south west is -1,-1)
    pub position: na::Vector2<f32>,
    /// Possible sizing
    pub size: na::Vector2<f32>,
}

// Handy aliases
pub type ObjectSpaceObjectCollider = ObjectCollider;
pub type WorldSpaceObjectCollider = ObjectCollider;

/// Definition of an object. Only one of these should exist *per object*, and they will be GPU instanced.
#[derive(Debug, Clone, Deserialize)]
pub struct WorldObject {
    /// Object name. Must match the name of the texture
    pub name: String,
    /// The object's bottom texture
    pub bottom_texture: PossiblyAnimatedTexture,
    /// The object's top texture
    pub top_texture: Option<PossiblyAnimatedTexture>,
    /// colliders describing the object's footprint
    pub footprint: Vec<ObjectSpaceObjectCollider>,
    /// A "sphere of influence" for the object. This is used for showing under the roof
    pub visualization_radius: Option<f32>,
    /// Colliders for physics
    pub physics_colliders: Vec<ObjectSpaceObjectCollider>,
    /// Temperature
    pub temperature: Option<f32>,
    /// Friction
    pub friction: Option<f32>,
}

/// Used to reference an object in the world definition
#[derive(Debug, Clone, Deserialize)]
pub struct WorldObjectRef {
    /// Object type
    #[serde(rename = "type")]
    pub kind: String,
    /// Object name
    pub name: String,
    /// Variant name
    pub variant: Option<String>,
    /// Object position (tile-space *not* pixel-space). 1,1 being up and to the right
    position: na::Vector2<f32>,
    /// Object rotation, positive is clockwise
    pub rotation_degrees: f32,
}

impl WorldObjectRef {
    pub fn into_key(&self) -> String {
        format!(
            "{}:{}:{}",
            self.kind,
            self.name,
            self.variant.as_ref().unwrap_or(&"default".to_string())
        )
    }
    pub fn get_world_space_position(&self)-> na::Vector2<f32> {
        self.position * 128.0
    }
    pub fn get_tile_space_position(&self)-> na::Vector2<f32> {
        self.position
    }
}
