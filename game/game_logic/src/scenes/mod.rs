//! The render code for various scenes
//!
//! ## Overview
//!
//! This will probably become a messy module over time. Stick your rendering code here
use raylib::prelude::*;

use crate::{discord::DiscordChannel, global_resource_package::GlobalResources};

/// This is called every frame once the game has started.
/// 
/// Keep in mind everything you do here will block the main thread (no loading files plz)
pub fn process_ingame_frame(
    raylib: &mut RaylibHandle,
    rl_thread: &RaylibThread,
    discord: &DiscordChannel,
    global_resources: &GlobalResources,
) {
    let mut d = raylib.begin_drawing(&rl_thread);

    d.clear_background(raylib::color::Color::WHITE);
}
