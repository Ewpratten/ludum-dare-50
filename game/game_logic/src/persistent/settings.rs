use std::path::PathBuf;

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

/// Settings for the game.
///
/// You can put whatever you want in here.
/// Please don't add anything relating to gameplay though (no coins, health, etc.).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistentGameSettings {
    // TODO: Add data here.
    pub volume: Option<f32>,
}

// Add any default values here.
impl Default for PersistentGameSettings {
    fn default() -> Self {
        Self {
            volume: Some(0.5),
        }
    }
}

/* ----------------------- You likely are looking for code above this line ----------------------- */

// This is the code for actually saving and loading the file from disk.
impl PersistentGameSettings {
    /// Returns the optimal path for storing settings data.
    #[profiling::function]
    fn get_save_location() -> PathBuf {
        // We should allow this path to be overridden through an environment variable.
        let preferences_dir = match std::env::var("OVERRIDE_GAME_SETTINGS_SAVE_LOCATION") {
            Ok(path) => PathBuf::from(path),
            Err(_) => {
                // If there is no override, we shall ask `directories` for the appropriate location.
                ProjectDirs::from("com", "va3zza", "ludum-dare-50")
                    .unwrap()
                    .preference_dir()
                    .to_path_buf()
            }
        };

        return preferences_dir.join("settings.json");
    }

    /// Loads the settings from disk.
    #[profiling::function]
    pub fn load_or_create(force_create: bool) -> Result<Self, serde_json::Error> {
        // Attempt to load the settings from the save location.
        let save_location = Self::get_save_location();
        log::debug!(
            "Attempting to load game settings from: {}",
            save_location.display()
        );

        if save_location.is_file() && !force_create {
            log::debug!("Found existing settings file.");
            return serde_json::from_str(std::fs::read_to_string(&save_location).unwrap().as_str());
        }

        // If we got here, we need to create a new settings file. In this case, we can just init the default settings.
        log::debug!("No existing settings file found.");
        return Ok(Self::default());
    }

    /// Saves the settings to disk.
    #[profiling::function]
    pub fn save(&self) -> Result<(), serde_json::Error> {
        // Get the save location
        let save_location = Self::get_save_location();
        log::debug!("Saving game settings to: {}", save_location.display());

        // Create the directory if it doesn't exist.
        if !save_location.parent().unwrap().is_dir() {
            log::debug!(
                "Creating directory: {}",
                save_location.parent().unwrap().display()
            );
            std::fs::create_dir_all(save_location.parent().unwrap()).unwrap();
        }

        // Write the settings to disk.
        std::fs::write(save_location, serde_json::to_string(self).unwrap()).unwrap();

        return Ok(());
    }
}
