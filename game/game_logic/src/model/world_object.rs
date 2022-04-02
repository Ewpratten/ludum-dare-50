use std::collections::HashMap;

use nalgebra as na;
use serde::Deserialize;

use crate::asset_manager::{load_json_structure, InternalJsonLoadError};

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
    pub footprint: Vec<ObjectCollider>,
    /// Colliders for physics
    pub physics_colliders: Vec<ObjectCollider>,
    /// Temperature
    pub temperature: Option<f32>,
}

/// Used to reference an object in the world definition
#[derive(Debug, Clone, Deserialize)]
pub struct WorldObjectRef {
    /// Object type
    #[serde(rename = "type")]
    pub kind: String,
    /// Object name
    pub name: String,
    /// Object position. 1,1 being up and to the right
    pub position: na::Vector2<f32>,
    /// Object rotation, positive is clockwise
    pub rotation_radians: f32,
}

/// A simply interface for the madness
#[derive(Debug, Clone)]
pub struct WorldObjectPackage {
    /// The object definitions
    pub object_definitions: HashMap<String, WorldObject>,
    /// The object references
    pub object_references: Vec<WorldObjectRef>,
    /// Bottom static textures
    pub bottom_static_textures: HashMap<String, AnimatedTexture>,
    /// Top static textures
    pub top_static_textures: HashMap<String, AnimatedTexture>,
    /// Bottom animated textures
    pub bottom_animated_textures: HashMap<String, AnimatedTexture>,
    /// Top animated textures
    pub top_animated_textures: HashMap<String, AnimatedTexture>,
}

impl WorldObjectPackage {
    pub fn load(map_objects_file_path: &str) -> Result<Self, InternalJsonLoadError> {
        // Attempt to load the object reference list
        let object_references: Vec<WorldObjectRef> = load_json_structure(map_objects_file_path)?;

        // We also need to load the object definitions
        let mut object_definitions = HashMap::new();
        for reference in &object_references {
            // If this is a new object, load it.
            let object_key = format!("{}:{}", reference.kind, reference.name);
            if !object_definitions.contains_key(object_key.as_str()) {
                // Construct the file path from the data we know about the reference
                let path = format!(
                    "assets/{}/{}/{}.json",
                    reference.kind, reference.name, reference.name
                );

                // Attempt to load the object definition
                let object_definition: WorldObject = load_json_structure(&path)?;

                // Store the object definition
                object_definitions.insert(object_key.to_string(), object_definition);
            }
        }

        Ok(Self {
            object_definitions,
            object_references,
        })
    }


}
