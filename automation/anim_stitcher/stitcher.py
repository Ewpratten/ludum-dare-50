"""This file contains the actual stitcher logic."""

import os
from PIL import Image
import json
import logging
import time
import getpass
from typing import List
from project_root import get_project_root
logger = logging.getLogger(__name__)


def check_sprite_exists(sprite_type: str, sprite_name: str) -> bool:
    """Checks if a sprite directory exists for the given sprite type and name.

    Args:
        sprite_type (str): Sprite type (short name)
        sprite_name (str): Sprite name

    Returns:
        bool: Does it exist?
    """

    # Get the project root
    project_root = get_project_root()
    logger.debug(f"Project root: {project_root}")

    # Build the path the sprite should exist in
    sprite_path = os.path.join(
        project_root, "game", "dist", "assets", "anm", sprite_type, f"{sprite_type}_{sprite_name}")

    return os.path.isdir(sprite_path)


def stitch_images_and_write_to_disk(sprite_type: str, sprite_name: str, images: List[str], quantize: bool, fps: float) -> None:

    # Load all the images
    images_to_stitch = []
    for image_path in images:
        images_to_stitch.append(Image.open(image_path))

    # Collect the total width and maximum height of the images while building a list of the sizes
    total_width = 0
    max_height = 0
    for image in images_to_stitch:
        total_width += image.size[0]
        max_height = max(max_height, image.size[1])

    # Create a new image with the total width and maximum height
    new_image = Image.new("RGBA", (total_width, max_height))

    # Paste each image into the new image
    x_offset = 0
    for image in images_to_stitch:
        new_image.paste(image, (x_offset, 0))
        x_offset += image.size[0]

    # Save the new image
    project_root = get_project_root()
    logger.debug(f"Project root: {project_root}")
    if quantize:
        new_image = new_image.quantize(method=2)
    new_image.save(os.path.join(project_root, "game", "dist", "assets", "anm", sprite_type,
                   f"{sprite_type}_{sprite_name}", f"{sprite_type}_{sprite_name}.png"))

    # Build some JSON metadata
    metadata = {
        "sheet_height": max_height,
        "sheet_width": total_width,
        "published_at": time.strftime("%Y-%m-%d %H:%M:%S", time.gmtime()),
        "published_by": getpass.getuser(),
        "frames": []
    }

    # Add the metadata for each image
    x_offset = 0
    for image in images_to_stitch:
        metadata["frames"].append({
            "x": x_offset,
            "y": 0,
            "width": image.size[0],
            "height": image.size[1]
        })
        x_offset += image.size[0]

    # Write the metadata to disk
    with open(os.path.join(project_root, "game", "dist", "assets", "anm", sprite_type,
                           f"{sprite_type}_{sprite_name}", f"{sprite_type}_{sprite_name}.anim_meta.json"), "w") as f:
        json.dump(metadata, f, indent=4)
