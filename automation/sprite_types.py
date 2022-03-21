"""This file contains functions for reading known sprite types."""

from typing import Dict
import json
import os
from project_root import get_project_root

import logging
logger = logging.getLogger(__name__)

def get_known_sprite_types() -> Dict[str, str]:
    """Gets a dictionary of known sprite types as a mapping from short name to friendly name

    Returns:
        Dict[str, str]: Short name -> Friendly name
    """

    # Load our JSON file containing known sprite types
    project_root = get_project_root()
    logger.debug(f"Project root: {project_root}")
    with open(os.path.join(project_root, "game", "dist", "known-sprite-types.json"), "r") as f:
        known_sprite_types = json.load(f)

    # We need to re-shape the data
    sprite_types = {}
    for item in known_sprite_types:
        sprite_types[item["short"]] = item["friendly"]


    return sprite_types
