//! This module handles enabling the remote-attach profiler when running in debug mode.

/// When in debug mode, this will set up profiling. (note: this will cause very slight lag)
#[must_use]
pub fn init_profiling() -> Option<puffin_http::Server> {
    if cfg!(debug_assertions) {
        // Enable the puffin HTTP service
        let server =
            puffin_http::Server::new(&format!("0.0.0.0:{}", puffin_http::DEFAULT_PORT)).unwrap();

        // Enable puffin itself
        puffin::set_scopes_on(true);

        Some(server)
    } else {
        None
    }
}
