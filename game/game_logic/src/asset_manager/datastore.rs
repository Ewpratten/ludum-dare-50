
/// This structure is dynamically packed with the contents of `dist` at compile time
/// 
/// This process allows us to only distribute a single binary, and have all the game assets stored in memory automatically.
/// The downside of this process is that the game will require a decent amount of RAM on the client's machine (and x64).
#[derive(rust_embed::RustEmbed)]
#[folder = "../dist"]
pub struct InternalData;