use std::collections::HashMap;

use raylib::{texture::Texture2D, RaylibHandle, RaylibThread};

use crate::{
    asset_manager::{load_json_structure, load_texture_from_internal_data},
    rendering::utilities::anim_texture::AnimatedTexture,
};

use super::world_object::{WorldObject, WorldObjectRef, WorldSpaceObjectCollider};

#[derive(Debug, thiserror::Error)]
pub enum WorldObjectPackageLoadError {
    #[error(transparent)]
    JsonError(#[from] crate::asset_manager::InternalJsonLoadError),
    #[error(transparent)]
    ResourceError(#[from] crate::asset_manager::ResourceLoadError),
}

/// A simply interface for the madness
#[derive(Debug)]
pub struct WorldObjectPackage {
    /// The object definitions
    pub object_definitions: HashMap<String, WorldObject>,
    /// The object references
    pub object_references: Vec<WorldObjectRef>,
    /// Bottom static textures
    pub bottom_static_textures: HashMap<String, Texture2D>,
    /// Top static textures
    pub top_static_textures: HashMap<String, Texture2D>,
    /// Bottom animated textures
    pub bottom_animated_textures: HashMap<String, AnimatedTexture>,
    /// Top animated textures
    pub top_animated_textures: HashMap<String, AnimatedTexture>,
    /// A list of colliders in the world. We pre-solve these to make comput happy :)
    pub world_space_colliders: Vec<WorldSpaceObjectCollider>,
}

impl WorldObjectPackage {
    pub fn load(
        raylib_handle: &mut RaylibHandle,
        thread: &RaylibThread,
        map_objects_file_path: &str,
    ) -> Result<Self, WorldObjectPackageLoadError> {
        // Attempt to load the object reference list
        let object_references: Vec<WorldObjectRef> = load_json_structure(map_objects_file_path)?;

        // We also need to load the object definitions
        let mut object_definitions = HashMap::new();
        let mut bottom_static_textures = HashMap::new();
        let mut top_static_textures = HashMap::new();
        let mut bottom_animated_textures = HashMap::new();
        let mut top_animated_textures = HashMap::new();
        let mut world_space_colliders: Vec<WorldSpaceObjectCollider> = Vec::new();
        for reference in &object_references {
            // If this is a new object, load it.
            let object_key = reference.into_key();
            if !object_definitions.contains_key(object_key.as_str()) {
                // Construct the file path from the data we know about the reference
                let path = format!(
                    "assets/{}/{}/{}{}.json",
                    reference.kind,
                    reference.name,
                    reference.name,
                    reference.variant.as_ref().unwrap_or(&String::new())
                );

                // Attempt to load the object definition
                let object_definition: WorldObject = load_json_structure(&path)?;

                // If this object has a static bottom texture, load it
                if object_definition.bottom_texture.animated.unwrap_or(false) {
                    panic!("Animated bottom textures are not supported yet")
                } else {
                    // Load the bottom texture and save it
                    bottom_static_textures.insert(
                        object_key.to_string(),
                        load_texture_from_internal_data(
                            raylib_handle,
                            thread,
                            &object_definition.bottom_texture.file_path,
                        )?,
                    );
                }

                // If there is a top texture, load it
                if let Some(top_texture) = &object_definition.top_texture {
                    if top_texture.animated.unwrap_or(false) {
                        panic!("Animated top textures are not supported yet")
                    } else {
                        // Load the top texture and save it
                        top_static_textures.insert(
                            object_key.to_string(),
                            load_texture_from_internal_data(
                                raylib_handle,
                                thread,
                                &top_texture.file_path,
                            )?,
                        );
                    }
                }

                // Keep track of all the colliders in the world
                for collider in &object_definition.physics_colliders {
                    // Get the object's position
                    let object_position = reference.get_world_space_position();

                    // Convert the collider's position to world space
                    let world_space_collider = WorldSpaceObjectCollider {
                        position: object_position + collider.position,
                        size: collider.size,
                    };

                    // Add the collider to the list
                    world_space_colliders.push(world_space_collider);
                }

                // Store the object definition
                object_definitions.insert(object_key.to_string(), object_definition);
            }
        }

        Ok(Self {
            object_definitions,
            object_references,
            bottom_static_textures,
            top_static_textures,
            bottom_animated_textures,
            top_animated_textures,
            world_space_colliders,
        })
    }
}
