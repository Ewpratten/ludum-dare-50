use raylib::RaylibBuilder;

pub fn handle_graphics_blocking<ConfigBuilder>(config: ConfigBuilder, target_frames_per_second: u32)
where
    ConfigBuilder: FnOnce(&mut RaylibBuilder),
{
    // Let the caller configure Raylib's internal window stuff
    let (mut raylib_handle, raylib_thread) = {
        let mut builder = raylib::init();
        config(&mut builder);
        builder.build()
    };

    // Set some important settings on the window
    raylib_handle.set_exit_key(None);
    raylib_handle.set_target_fps(target_frames_per_second);

    // Run the event loop
    while !raylib_handle.window_should_close() {}
}
