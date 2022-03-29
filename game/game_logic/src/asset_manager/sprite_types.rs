use serde::Deserialize;

use super::InternalData;

/// The structure backing the `dist/known-sprite-types.json` file
#[derive(Debug, Clone, Deserialize)]
pub struct KnownSpriteType {
    /// Sprite short name (used in filenames)
    #[serde(rename = "short")]
    pub short_name: String,
    /// Sprite long name
    #[serde(rename = "friendly")]
    pub friendly_name: String,
}

/// Loads a list of all known sprite types from the definitions file
pub fn load_known_sprite_types() -> Result<Vec<KnownSpriteType>, serde_json::Error> {
    // Load the json file from the embedded data as a string
    let data = InternalData::get("known-sprite-types.json").unwrap().data;

    // Deserialize the json string into a rust structure
    let json_structure: Vec<KnownSpriteType> = serde_json::from_slice(&data)?;
    Ok(json_structure)
}
