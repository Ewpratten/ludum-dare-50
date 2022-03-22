use serde::Deserialize;

/// This structure is filled with the contents of `dist/project-constants.json` at runtime
#[derive(Debug, Deserialize)]
pub struct ProjectConstants {
    /// The name of the game
    pub game_name: String,

    /// The window size to use on launch
    pub base_window_size: (u32, u32),

    /// The Discord application ID
    pub discord_app_id: i64,

    /// The target framerate of the game
    pub target_fps: u32,
}
