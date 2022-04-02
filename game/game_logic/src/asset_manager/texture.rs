//! Code for loading textures from RAM to VRAM
//!
//! Largely coppied from last year: https://github.com/Ewpratten/ludum-dare-49/blob/master/game/src/utilities/datastore.rs

use std::path::Path;

use raylib::{
    audio::{Music, RaylibAudio, Sound},
    texture::Texture2D,
    RaylibHandle, RaylibThread,
};
use tempfile::tempdir;

use crate::asset_manager::InternalData;

#[derive(Debug, thiserror::Error)]
pub enum ResourceLoadError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Could not load embedded asset: {0}")]
    AssetNotFound(String),
    #[error("Generic error: {0}")]
    Generic(String),
}

/// Loads an embedded texture into VRAM.
///
/// # Technical Info
/// In this application, we are using `rust_embed` to embed static assets directly inside the executable.
/// This has the limitation of none of the assets being "real files", which causes an issue with Raylib.
/// Raylib requires a "real file" in order to load data into VRAM (without digging into `unsafe` dark magic).
/// The solution is to temporarily write the assets to disk, and then load them from disk.
/// We must also preserve the file extension, so the Raylib file loader can parse them correctly.
pub fn load_texture_from_internal_data(
    raylib_handle: &mut RaylibHandle,
    thread: &RaylibThread,
    path: &str,
) -> Result<Texture2D, ResourceLoadError> {
    // Create a temp file path to work with
    let temp_dir = tempdir()?;
    debug!(
        "Created temporary directory for passing embedded data to Raylib: {}",
        temp_dir.path().display()
    );
    let tmp_path = temp_dir.path().join(Path::new(path).file_name().unwrap());

    // Unpack the raw image data to a real file on the local filesystem so raylib will read it correctly
    std::fs::write(
        &tmp_path,
        &InternalData::get(path)
            .ok_or(ResourceLoadError::AssetNotFound(path.to_string()))?
            .data,
    )?;

    // Call through via FFI to re-load the file
    let texture = raylib_handle
        .load_texture(thread, tmp_path.to_str().unwrap())
        .map_err(ResourceLoadError::Generic)?;

    // Close the file
    debug!(
        "Dropping temporary directory: {}",
        temp_dir.path().display()
    );
    temp_dir.close()?;

    Ok(texture)
}

pub fn load_music_from_internal_data(
    thread: &RaylibThread,
    path: &str,
) -> Result<Music, ResourceLoadError> {
    // Create a temp file path to work with
    let temp_dir = tempdir()?;
    debug!(
        "Created temporary directory for passing embedded data to Raylib: {}",
        temp_dir.path().display()
    );
    let tmp_path = temp_dir.path().join(Path::new(path).file_name().unwrap());

    // Unpack the raw sound data to a real file on the local filesystem so raylib will read it correctly
    std::fs::write(
        &tmp_path,
        &InternalData::get(path)
            .ok_or(ResourceLoadError::AssetNotFound(path.to_string()))?
            .data,
    )?;

    // Call through via FFI to re-load the file
    let texture = Music::load_music_stream(thread, tmp_path.to_str().unwrap())
        .map_err(ResourceLoadError::Generic)?;

    // Close the file
    debug!(
        "Dropping temporary directory: {}",
        temp_dir.path().display()
    );
    // temp_dir.close()?;

    Ok(texture)
}

pub fn load_sound_from_internal_data(
    path: &str,
) -> Result<Sound, ResourceLoadError> {
    // Create a temp file path to work with
    let temp_dir = tempdir()?;
    debug!(
        "Created temporary directory for passing embedded data to Raylib: {}",
        temp_dir.path().display()
    );
    let tmp_path = temp_dir.path().join(Path::new(path).file_name().unwrap());

    // Unpack the raw sound data to a real file on the local filesystem so raylib will read it correctly
    std::fs::write(
        &tmp_path,
        &InternalData::get(path)
            .ok_or(ResourceLoadError::AssetNotFound(path.to_string()))?
            .data,
    )?;

    // Call through via FFI to re-load the file
    let texture =
        Sound::load_sound(tmp_path.to_str().unwrap()).map_err(ResourceLoadError::Generic)?;

    // Close the file
    debug!(
        "Dropping temporary directory: {}",
        temp_dir.path().display()
    );
    temp_dir.close()?;

    Ok(texture)
}