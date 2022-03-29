//! The Event Loop module
//!
//! ## Overview
//!
//! This is the code that handles beginning each frame and ending it. Do not try to add your own game logic in here.
//! The event loop function has its own statemachine (`core_renderer_sm.rs`) that handles the current action.
//!
//! You can think of this as a bit of bootstrap code for the game. All that happens directly here is rendering of the loading screen and a bit of error handling.

use std::cell::RefCell;

use crate::discord::DiscordChannel;
use crate::project_constants::ProjectConstants;
use crate::rendering::core_renderer_sm::{PreloadState, RenderBackendStates};
use crate::rendering::screens::sm_failure_screen;
use crate::scenes::SceneRenderDelegate;
use raylib::RaylibBuilder;
use raylib::consts::KeyboardKey;
use raylib::prelude::RaylibDraw;

/// Will begin rendering graphics. Returns when the window closes
pub async fn handle_graphics_blocking<ConfigBuilder>(
    config: ConfigBuilder,
    target_frames_per_second: u32,
    constants: &ProjectConstants,
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

    // Set up the main render delegate
    let mut render_delegate =
        SceneRenderDelegate::on_game_start(&mut raylib_handle, &raylib_thread);

    // Handle loading the resources and rendering the loading screen
    log::trace!("Running event loop");
    while !raylib_handle.window_should_close() {
        // Handle state machine updates
        match backend_sm {
            RenderBackendStates::Preload(m @ PreloadState::FromInit) => {
                backend_sm = m.finish_preload();
            }
            RenderBackendStates::Loading(ref m) => {
                if loading_screen
                    .render(
                        &mut raylib_handle,
                        &raylib_thread,
                        &discord_signaling,
                        &constants,
                    )
                    .await
                {
                    backend_sm = m.finish_loading();
                }
            }
            _ => break,
        };

        // Tell the profiler that we ended the frame
        profiling::finish_frame!();
    }
    log::info!("Finished loading game");

    // Get access to the global resources
    let global_resources = loading_screen
        .resources
        .expect("Failed to get global resources");

    // Tracker for if we are showing the FPS counter
    let mut show_fps_counter = false;

    // Run the event loop
    while !raylib_handle.window_should_close() {
        // Handle state machine updates
        match backend_sm {
            RenderBackendStates::SmFailed(ref m) => {
                sm_failure_screen
                    .render(
                        &mut raylib_handle,
                        &raylib_thread,
                        &discord_signaling,
                        &constants,
                    )
                    .await;
            }
            RenderBackendStates::RenderGame(ref m) => {
                render_delegate.process_ingame_frame(
                    &mut raylib_handle,
                    &raylib_thread,
                    &discord_signaling,
                    &global_resources,
                );
            }
            _ => backend_sm = RenderBackendStates::sm_failed(),
        };

        // Check for F3 being pressed
        if raylib_handle.is_key_pressed(KeyboardKey::KEY_F3) {
            show_fps_counter = !show_fps_counter;
        }

        // Show the FPS counter
        if show_fps_counter {
            raylib_handle.begin_drawing(&raylib_thread).draw_fps(10, 10);
        }

        // Tell the profiler that we ended the frame
        profiling::finish_frame!();
    }
    log::trace!("Event loop ended");
}
