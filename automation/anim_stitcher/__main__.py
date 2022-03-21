import argparse
import sys
import logging
from qt_common.qt_app_wrapper import QtAppWrapper
from . import ui

def main() -> int:
    # Handle program arguments
    ap = argparse.ArgumentParser(
        prog='anim_stitcher', description='A tool for stitching PNG sequences into sprite sheets')

    args = ap.parse_args()

    # Setup logging
    logging.basicConfig(level=logging.DEBUG)

    # Run the application
    with QtAppWrapper():
        # Create and show the window
        w = ui.AnimStitcherWindow()
        w.show()

    return 0


if __name__ == "__main__":
    sys.exit(main())
