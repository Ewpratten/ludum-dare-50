use std::path::{Path, PathBuf};

use image::{GenericImage, ImageResult};
use regex::Regex;

const SPRITE_TYPE_RE: &str = r"^([a-z]+)_";

fn sprite_name_to_output_dir(sprite_name: &str) -> PathBuf {
    // Determine the sprite type
    let sprite_type: String = Regex::new(SPRITE_TYPE_RE)
        .unwrap()
        .captures(sprite_name)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .to_string()
        .into();

    // Build the output directory
    Path::new("../dist/gen/anm/")
        .join(sprite_type)
        .join(sprite_name)
}

pub fn stitch_sprites(sprite_name: &str, ordered_image_files: Vec<PathBuf>) -> ImageResult<()> {
    build_helper::warning!(
        "Generating sprite `{}` from {} images",
        sprite_name,
        ordered_image_files.len()
    );

    // Collect all images into a vec of actual image data
    let mut images = Vec::new();
    for image_file in ordered_image_files {
        let image_data = image::open(image_file.as_path()).unwrap();
        images.push(image_data);
    }

    // Calculate the final width and height of the sprite
    let img_width_out: u32 = images.iter().map(|im| im.width()).sum();
    let img_height_out: u32 = images.iter().map(|im| im.height()).max().unwrap_or(0);

    // Initialize an image buffer with the appropriate size.
    let mut imgbuf = image::ImageBuffer::new(img_width_out, img_height_out);
    let mut accumulated_width = 0;

    // Copy each input image at the correct location in the output image.
    for img in images {
        imgbuf.copy_from(&img, accumulated_width, 0).unwrap();
        accumulated_width += img.width();
    }

    // Get the output directory
    let output_dir = sprite_name_to_output_dir(sprite_name);

    // Create the output directory if it doesn't exist
    if !output_dir.exists() {
        std::fs::create_dir_all(output_dir.as_path()).unwrap();
    }

    // Write the output image to disk.
    imgbuf.save(output_dir.join(format!("{}.png", sprite_name)))?;

    Ok(())
}
