//! This script contains code that generates more code.
//!
//! The idea being that we can have assets auto-pack and optimize themselves at build time.

fn main() {
    // We want to re-build if the assets change
    println!("cargo:rerun-if-changed=../auto_stitch");

    // Search for all direct children of the auto_stitch directory
    for entry in std::fs::read_dir("../auto_stitch").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        // Get all children of the current entry
        if let Ok(children) = std::fs::read_dir(&path) {
            let children_paths = children.map(|e| e.unwrap().path()).collect::<Vec<_>>();

            // Process into a sprite
            anim_stitcher::generator::stitch_sprites(
                path.file_name().unwrap().to_str().unwrap(),
                children_paths,
            )
            .unwrap();
        }
    }
}
