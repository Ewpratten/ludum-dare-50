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
    pub size: Option<na::Vector2<f32>>,
    /// Possible radius
    pub radius: Option<f32>,
}

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
    // pub footprint: Vec<ObjectCollider>,
    pub footprint_radius: Option<f32>,
    /// Colliders for physics
    pub physics_colliders: Vec<ObjectCollider>,
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
    /// Object position (tile-space *not* pixel-space). 1,1 being up and to the right
    pub position: na::Vector2<f32>,
    /// Object rotation, positive is clockwise
    pub rotation_radians: f32,
}


