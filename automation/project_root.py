"""Module to fetch the project root directory."""

import os

def get_project_root() -> str:
    """Gets the project root directory.
    
    Returns:
        str: The project root directory.
    """
    return os.environ.get("LD50_PROJECT_ROOT")