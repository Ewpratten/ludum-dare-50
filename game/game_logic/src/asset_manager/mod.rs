//! Embedded asset management.
//!
//! ## Overview
//!
//! `asset_manager` is one of the most important modules in this project. It handles loading and packaging of in-game resources.
//! Generally in a game, you might distribute an executable along with a zip of everything needed to run, 
//! but we have had some issues with this before on systems with restrictive file permissions (cough OSX).
//! 
//! To make the game distribution process easier, we embed our resources directly into the executable's data section using 
//! [`rust-embed`](https://github.com/pyros2097/rust-embed). This means we only have to distribute one single file to players.
//! 
//! ## Debug vs. Release mode
//! 
//! When the game is built in debug mode (with `cargo build`), the resources are *not* packaged into the game. 
//! Instead, they are read from disk, allowing us to modify them while the game is running, and speeding up the compile times.
//! 
//! When the game is built in release mode (with `cargo build --release`), the resources are packaged into the game as described above.
//! This means the game will load faster, but also use more RAM.

mod datastore;
pub use datastore::InternalData;
mod json;
pub use json::{InternalJsonLoadError, load_json_structure};