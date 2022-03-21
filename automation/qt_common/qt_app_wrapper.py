"""A wrapper that handles bootstrapping QT applications in environments that do not have QT support."""

# Load the logging system
import logging
logger = logging.getLogger("qt_common.utils")

# We need to have a global to keep track of the QApplication
_qt_app = None

class QtAppWrapper:

    def __init__(self, parent=None):
        self.parent = parent

    def __enter__(self):
        global _qt_app

        # If there is no parent, we must make a QApplication
        if self.parent is None:
            logger.info("No parent specified. Creating QApplication")
            from PySide2 import QtWidgets
            try:
                if not _qt_app:
                    _qt_app = QtWidgets.QApplication([])
            except RuntimeError:
                logger.error(
                    "Could not create QApplication. Is it already running?")
                raise

    def __exit__(self, type, value, traceback):
        global _qt_app

        # If there is no parent, we must run the QApplication ourselves
        if self.parent is None:
            logger.info("Running QApplication")
            if _qt_app:
                _qt_app.exec_()