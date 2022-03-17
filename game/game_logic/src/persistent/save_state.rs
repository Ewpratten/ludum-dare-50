use std::path::PathBuf;

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

/// Game save state.
///
/// This can be used for health, coins, inventory, progress, high scores, etc.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameSaveState {
    // TODO: Add data here.
}

// Add any default values here.
impl Default for GameSaveState {
    fn default() -> Self {
        Self {}
    }
}

/* ----------------------- You likely are looking for code above this line ----------------------- */

// This is the code for actually saving and loading the file from disk.
impl GameSaveState {
    /// Returns the optimal path for storing settings data.
    #[profiling::function]
    fn get_save_location() -> PathBuf {
        // We should allow this path to be overridden through an environment variable.
        let preferences_dir = match std::env::var("OVERRIDE_GAME_SAVE_STATE_LOCATION") {
            Ok(path) => PathBuf::from(path),
            Err(_) => {
                // If there is no override, we shall ask `directories` for the appropriate location.
                ProjectDirs::from("com", "va3zza", "ludum-dare-50")
                    .unwrap()
                    .data_local_dir()
                    .to_path_buf()
            }
        };

        return preferences_dir.join("progress.json");
    }

    /// Loads the savestate from disk.
    #[profiling::function]
    pub fn load_or_create() -> Result<Self, serde_json::Error> {
        // Attempt to load the savestate from the save location.
        let save_location = Self::get_save_location();
        log::debug!(
            "Attempting to load game savestate from: {}",
            save_location.display()
        );

        if save_location.is_file() {
            log::debug!("Found existing savestate file.");
            return serde_json::from_str(std::fs::read_to_string(&save_location).unwrap().as_str());
        }

        // If we got here, we need to create a new savestate file. In this case, we can just init the default savestate.
        log::debug!("No existing savestate file found.");
        return Ok(Self::default());
    }

    /// Saves the savestate to disk.
    #[profiling::function]
    pub fn save(&self) -> Result<(), serde_json::Error> {
        // Get the save location
        let save_location = Self::get_save_location();
        log::debug!("Saving game savestate to: {}", save_location.display());

        // Write the savestate to disk.
        std::fs::write(save_location, serde_json::to_string(self).unwrap()).unwrap();

        return Ok(());
    }
}
