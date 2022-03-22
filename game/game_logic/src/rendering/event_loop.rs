//! The Event Loop module
//!
//! ## Overview
//!
//! This is the code that handles beginning each frame and ending it. Do not try to add your own game logic in here.
//! The event loop function has its own statemachine (`core_renderer_sm.rs`) that handles the current action.
//!
//! You can think of this as a bit of bootstrap code for the game. All that happens directly here is rendering of the loading screen and a bit of error handling.

use crate::discord::DiscordChannel;
use crate::rendering::core_renderer_sm::{PreloadState, RenderBackendStates};
use crate::rendering::screens::sm_failure_screen;
use raylib::RaylibBuilder;

/// Will begin rendering graphics. Returns when the window closes
pub fn handle_graphics_blocking<ConfigBuilder>(
    config: ConfigBuilder,
    target_frames_per_second: u32,
    discord_signaling: DiscordChannel,
) where
    ConfigBuilder: FnOnce(&mut RaylibBuilder),
{
    // Set up the backend rendering state machine
    let mut backend_sm = RenderBackendStates::preload();

    // Let the caller configure Raylib's internal window stuff
    let (mut raylib_handle, raylib_thread) = {
        log::trace!("Configuring Raylib");
        let mut builder = raylib::init();
        config(&mut builder);
        builder.build()
    };

    // Set some important settings on the window
    raylib_handle.set_exit_key(None);
    raylib_handle.set_target_fps(target_frames_per_second);

    // Set up the internal screens
    let mut loading_screen = crate::rendering::screens::loading_screen::LoadingScreen::new();
    let mut sm_failure_screen = sm_failure_screen::SmFailureScreen::new();

    // Run the event loop
    log::trace!("Running event loop");
    while !raylib_handle.window_should_close() {
        // Handle state machine updates
        match backend_sm {
            RenderBackendStates::Preload(m @ PreloadState::FromInit) => {
                backend_sm = m.finish_preload();
            }
            RenderBackendStates::Loading(ref m) => {
                if loading_screen.render(&mut raylib_handle, &raylib_thread, &discord_signaling) {
                    backend_sm = m.finish_loading();
                }
            }
            RenderBackendStates::SmFailed(ref m) => {
                sm_failure_screen.render(&mut raylib_handle, &raylib_thread, &discord_signaling);
            }
        };

        // Tell the profiler that we ended the frame
        profiling::finish_frame!();
    }
    log::trace!("Event loop ended");
}
