use std::{collections::HashMap, path::PathBuf, sync::Arc};

use crate::{
    asset_manager::{load_texture_from_internal_data, InternalData},
    model::world_object_package::WorldObjectPackage,
};
use nalgebra as na;
use raylib::{
    camera::Camera2D,
    color::Color,
    math::{Rectangle, Vector2},
    prelude::{RaylibDraw, RaylibDrawHandle, RaylibMode2D},
    texture::Texture2D,
    RaylibHandle, RaylibThread,
};
use tiled::{Loader, Map, PropertyValue, ResourceCache, ResourcePath, ResourcePathBuf, Tileset};

/// Possible errors generated by the map loading process
#[derive(Debug, thiserror::Error)]
pub enum MapRenderError {
    #[error("Could not load embedded asset: {0}")]
    AssetNotFound(String),
    #[error(transparent)]
    TiledError(#[from] tiled::Error),
}

#[derive(Debug)]
struct ProgramDataTileCache {
    tilesets: HashMap<ResourcePathBuf, Arc<Tileset>>,
    internal_loader: Loader,
}

impl ProgramDataTileCache {
    fn new() -> Self {
        Self {
            tilesets: HashMap::new(),
            internal_loader: Loader::new(),
        }
    }
}

impl ResourceCache for ProgramDataTileCache {
    /// Load the tileset. First attempts to pull from an in-RAM cache, otherwise attempts to load from disk.
    fn get_tileset(&self, path: impl AsRef<ResourcePath>) -> Option<Arc<Tileset>> {
        let possibly_cached_tileset = self.tilesets.get(path.as_ref()).map(Clone::clone);
        if let Some(tileset) = possibly_cached_tileset {
            return Some(tileset);
        } else {
            // Pull the TSX from storage and parse it
            InternalData::get(path.as_ref().to_str().unwrap()).map(|file| {
                let data = file.data.into_owned();
                Arc::new(
                    self.internal_loader
                        .load_tsx_tileset_from(data.as_slice(), path)
                        .unwrap(),
                )
            })
        }
    }

    fn get_or_try_insert_tileset_with<F, E>(
        &mut self,
        path: ResourcePathBuf,
        f: F,
    ) -> Result<Arc<Tileset>, E>
    where
        F: FnOnce() -> Result<Tileset, E>,
    {
        Ok(match self.tilesets.entry(path) {
            std::collections::hash_map::Entry::Occupied(o) => o.into_mut(),
            std::collections::hash_map::Entry::Vacant(v) => v.insert(Arc::new(f()?)),
        }
        .clone())
    }
}

#[derive(Debug)]
pub struct MapRenderer {
    map: Map,
    tile_textures: HashMap<PathBuf, Texture2D>,
    world_objects: WorldObjectPackage,
}

impl MapRenderer {
    /// Construct a new MapRenderer.
    pub fn new(
        tmx_path: &str,
        objects_path: &str,
        raylib: &mut RaylibHandle,
        raylib_thread: &RaylibThread,
    ) -> Result<Self, MapRenderError> {
        // Pull the TMX from storage
        let data = InternalData::get(tmx_path)
            .ok_or(MapRenderError::AssetNotFound(tmx_path.to_string()))?
            .data
            .into_owned();

        // Attempt to parse the TMX file
        let mut loader = Loader::with_cache(ProgramDataTileCache::new());
        let map = loader.load_tmx_map_from(data.as_slice(), tmx_path)?;

        // Iterate over all images in the map
        let mut tile_textures = HashMap::new();
        for tileset in map.tilesets() {
            for (idx, tile) in tileset.tiles() {
                if let Some(image) = &tile.data.image {
                    // We now have a path to an image
                    let image_path = image.source.clone();

                    // Load the texture
                    let texture = load_texture_from_internal_data(
                        raylib,
                        raylib_thread,
                        image_path.to_str().unwrap(),
                    )
                    .unwrap();

                    // Store the texture in the cache
                    tile_textures.insert(image_path, texture);
                }
            }
        }

        // Load the world objects
        let world_objects = WorldObjectPackage::load(raylib, raylib_thread, objects_path).unwrap();

        Ok(Self {
            map,
            tile_textures,
            world_objects,
        })
    }

    pub fn sample_friction_at(&self, world_position: na::Vector2<f32>) -> Option<f32> {
        // Convert to a tile position
        let tile_position = na::Vector2::new(
            (world_position.x / 128.0).floor() as i32,
            (world_position.y / 128.0).floor() as i32,
        );

        // If there is an object here, let it override the output
        for obj_ref in &self.world_objects.object_references {
            if obj_ref.position.x == tile_position.x as f32
                && obj_ref.position.y == tile_position.y as f32
            {
                // Get access to the actual object definition
                let object_key = format!("{}:{}", obj_ref.kind, obj_ref.name);
                let obj_def = self
                    .world_objects
                    .object_definitions
                    .get(&object_key)
                    .unwrap();

                // Check if there is a friction property
                if let Some(friction) = obj_def.friction {
                    return Some(friction);
                }
            }
        }

        // Get the first layer
        let layer = self.map.layers().next().unwrap();

        // Handle the layer type
        match layer.layer_type() {
            tiled::LayerType::TileLayer(layer) => {
                // Get the tile
                if let Some(tile) = layer.get_tile(tile_position.x, tile_position.y) {
                    if let Some(tile) = tile.get_tile() {
                        if let Some(data) = tile.data.properties.get("friction") {
                            match data {
                                PropertyValue::FloatValue(f) => Some(*f),
                                _ => None,
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn sample_temperature_at(&self, world_position: na::Vector2<f32>) -> Option<f32> {
        // Convert to a tile position
        let tile_position = na::Vector2::new(
            (world_position.x / 128.0).floor() as i32,
            (world_position.y / 128.0).floor() as i32,
        );

        // If there is an object here, let it override the output
        for obj_ref in &self.world_objects.object_references {
            if obj_ref.position.x == tile_position.x as f32
                && obj_ref.position.y == tile_position.y as f32
            {
                // Get access to the actual object definition
                let object_key = format!("{}:{}", obj_ref.kind, obj_ref.name);
                let obj_def = self
                    .world_objects
                    .object_definitions
                    .get(&object_key)
                    .unwrap();

                // Check if there is a temperature property
                if let Some(temperature) = obj_def.temperature {
                    return Some(temperature);
                }
            }
        }

        // Get the first layer
        let layer = self.map.layers().next().unwrap();

        // Handle the layer type
        match layer.layer_type() {
            tiled::LayerType::TileLayer(layer) => {
                // Get the tile
                if let Some(tile) = layer.get_tile(tile_position.x, tile_position.y) {
                    if let Some(tile) = tile.get_tile() {
                        if let Some(data) = tile.data.properties.get("temperature") {
                            match data {
                                PropertyValue::FloatValue(f) => Some(*f),
                                _ => None,
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn render_map(
        &mut self,
        draw_handle: &mut RaylibMode2D<RaylibDrawHandle>,
        camera: &Camera2D,
        show_debug_grid: bool,
        player_position: na::Vector2<f32>,
    ) {
        // Get the window corners in world space
        let screen_width = draw_handle.get_screen_width();
        let screen_height = draw_handle.get_screen_height();
        let world_win_top_left = draw_handle.get_screen_to_world2D(Vector2::new(0.0, 0.0), camera);
        let world_win_bottom_right = draw_handle.get_screen_to_world2D(
            Vector2::new(screen_width as f32, screen_height as f32),
            camera,
        );

        // Handle each layer from the bottom up
        for layer in self.map.layers() {
            // Handle different layer types
            match layer.layer_type() {
                tiled::LayerType::TileLayer(layer) => {
                    // Keep track of our sampler X and Y values
                    let mut sampler_x = 0;
                    let mut sampler_y = 0;

                    // Get the tile width and height
                    let tile_width = 128;
                    let tile_height = 128;

                    // Loop until we have covered all tiles on the screen
                    for y in (world_win_top_left.y as i64)..(world_win_bottom_right.y as i64) {
                        // Convert the pixel coordinates to tile coordinates
                        let tile_y = (y as f32 / tile_height as f32).floor() as i32;

                        // If we are looking at a new tile, update the sampler
                        if sampler_y != tile_y {
                            sampler_y = tile_y;

                            for x in
                                (world_win_top_left.x as i64)..(world_win_bottom_right.x as i64)
                            {
                                // Convert the pixel coordinates to tile coordinates
                                let tile_x = (x as f32 / tile_width as f32).floor() as i32;
                                // debug!("Tile: ({}, {})", tile_x, tile_y);

                                // If we are looking at a new tile, update the sampler
                                if sampler_x != tile_x {
                                    sampler_x = tile_x;

                                    // Get the tile at this coordinate
                                    if let Some(tile) = layer.get_tile(sampler_x, sampler_y) {
                                        // debug!("Tile: ({}, {})", tile_x, tile_y);
                                        // Fetch the texture for this tile
                                        let real_tile = tile.get_tile().unwrap();
                                        let texture = self
                                            .tile_textures
                                            .get(&real_tile.image.as_ref().unwrap().source)
                                            .unwrap();

                                        // Draw the tile
                                        draw_handle.draw_texture(
                                            texture,
                                            tile_x * tile_width as i32,
                                            tile_y * tile_height as i32,
                                            Color::WHITE,
                                        );
                                    }

                                    // Check if there is an object at this tile
                                    for obj_ref in &self.world_objects.object_references {
                                        if obj_ref.position.x == sampler_x as f32
                                            && obj_ref.position.y == sampler_y as f32
                                        {
                                            // Get access to the actual object definition
                                            let object_key =
                                                format!("{}:{}", obj_ref.kind, obj_ref.name);
                                            let obj_def = self
                                                .world_objects
                                                .object_definitions
                                                .get(&object_key)
                                                .unwrap();

                                            // We need to render the base layer of the object
                                            if obj_def.bottom_texture.animated.unwrap_or(false) {
                                                let tex = self
                                                    .world_objects
                                                    .bottom_animated_textures
                                                    .get_mut(&object_key)
                                                    .unwrap();
                                                tex.render_automatic(
                                                    draw_handle,
                                                    obj_ref.position - (tex.size() / 2.0),
                                                    None,
                                                    Some(tex.size() / 2.0),
                                                    Some(obj_ref.rotation_radians.to_degrees()),
                                                    None,
                                                );
                                            } else {
                                                let tex = self
                                                    .world_objects
                                                    .bottom_static_textures
                                                    .get_mut(&object_key)
                                                    .unwrap();
                                                let p: Vector2 = obj_ref.position.into();
                                                let r1 = Rectangle {
                                                    x: 0.0,
                                                    y: 0.0,
                                                    width: tex.width as f32,
                                                    height: tex.height as f32,
                                                };
                                                let r2 = Rectangle {
                                                    x: p.x,
                                                    y: p.y,
                                                    width: tex.width as f32,
                                                    height: tex.height as f32,
                                                };

                                                draw_handle.draw_texture_pro(
                                                    &tex,
                                                    r1,
                                                    r2,
                                                    Vector2::new(
                                                        tex.width as f32 / 2.0,
                                                        tex.height as f32 / 2.0,
                                                    ),
                                                    obj_ref.rotation_radians.to_degrees(),
                                                    Color::WHITE,
                                                );
                                            }

                                            // If needed we can render the top layer of the object
                                            if let Some(top_texture) = &obj_def.top_texture {
                                                // We need to detect if the player is in the footprint of the object
                                                let mut tint = Color::WHITE;
                                                if let Some(footprint_radius) =
                                                    obj_def.footprint_radius
                                                {
                                                    let player_dist_to_object =
                                                        (obj_ref.position - player_position).norm();
                                                    // debug!(
                                                    //     "Player dist to object: {}",
                                                    //     player_dist_to_object
                                                    // );
                                                    if player_dist_to_object <= footprint_radius {
                                                        tint.a = 128;
                                                    }
                                                }

                                                if top_texture.animated.unwrap_or(false) {
                                                    let tex = self
                                                        .world_objects
                                                        .top_animated_textures
                                                        .get_mut(&object_key)
                                                        .unwrap();
                                                    tex.render_automatic(
                                                        draw_handle,
                                                        obj_ref.position - (tex.size() / 2.0),
                                                        None,
                                                        Some(tex.size() / 2.0),
                                                        Some(obj_ref.rotation_radians.to_degrees()),
                                                        Some(tint),
                                                    );
                                                } else {
                                                    let tex = self
                                                        .world_objects
                                                        .top_static_textures
                                                        .get_mut(&object_key)
                                                        .unwrap();
                                                    let p: Vector2 = obj_ref.position.into();
                                                    let r1 = Rectangle {
                                                        x: 0.0,
                                                        y: 0.0,
                                                        width: tex.width as f32,
                                                        height: tex.height as f32,
                                                    };
                                                    let r2 = Rectangle {
                                                        x: p.x,
                                                        y: p.y,
                                                        width: tex.width as f32,
                                                        height: tex.height as f32,
                                                    };

                                                    draw_handle.draw_texture_pro(
                                                        &tex,
                                                        r1,
                                                        r2,
                                                        Vector2::new(
                                                            tex.width as f32 / 2.0,
                                                            tex.height as f32 / 2.0,
                                                        ),
                                                        obj_ref.rotation_radians.to_degrees(),
                                                        tint,
                                                    );
                                                }
                                            }
                                        }
                                    }

                                    if show_debug_grid {
                                        draw_handle.draw_rectangle_lines(
                                            tile_x * tile_width as i32,
                                            tile_y * tile_height as i32,
                                            self.map.tile_width as i32,
                                            self.map.tile_height as i32,
                                            Color::RED,
                                        );
                                        draw_handle.draw_pixel(x as i32, y as i32, Color::BLUE);
                                    }
                                }
                            }
                        }
                    }
                }
                tiled::LayerType::ObjectLayer(_) => todo!(),
                tiled::LayerType::ImageLayer(_) => todo!(),
                tiled::LayerType::GroupLayer(_) => todo!(),
            }
        }
    }
}