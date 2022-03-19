use serde::de::DeserializeOwned;

use super::datastore::InternalData;

/// Possible errors generated when deserializing JSON data from memory
#[derive(Debug, thiserror::Error)]
pub enum InternalJsonLoadError {
    /// An error occurred with the JSON data itself
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),

    /// The JSON data was not found in the internal data store
    #[error("Could not load embedded asset: {0}")]
    AssetNotFound(String),
}

/// Load an embedded JSON file
pub fn load_json_structure<'a, T: DeserializeOwned>(
    dist_path: &str,
) -> Result<T, InternalJsonLoadError> {
    // Load the json file from the embedded data as a string
    let data = InternalData::get(dist_path)
        .ok_or(InternalJsonLoadError::AssetNotFound(dist_path.to_string()))?
        .data;

    // Deserialize the json string into a rust structure
    let json_structure: T = serde_json::from_slice(&data)?;
    Ok(json_structure)
}
