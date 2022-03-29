"""anim_stitcher GUI"""

from PySide2 import QtWidgets
from PySide2.QtCore import Qt
import pkgutil
from qt_common import qt_window_center, qt_lines, qt_dialog_style
import sprite_types
import re
import os
from . import stitcher
from project_root import get_project_root

import logging
logger = logging.getLogger("anim_stitcher.ui")

SPRITE_NAMING_VALIDATION = r"^[a-z][a-zA-Z\d]+$"


class AnimStitcherWindow(QtWidgets.QWidget):

    selected_files = None

    def __init__(self):
        super(AnimStitcherWindow, self).__init__()

        # Configure the window
        self.setWindowFlags(
            self.windowFlags() ^ Qt.Window)
        self.setWindowTitle("Anim Stitcher")
        self.resize(280, 200)
        qt_window_center.center_window(self)

        # Set the root of the application to be a vertical list
        self.setLayout(QtWidgets.QVBoxLayout())

        # Load the stylesheet for this app
        self.setStyleSheet(qt_dialog_style.STYLESHEET)

        # Configure the title at the top of the window
        self.label = QtWidgets.QLabel("Anim Stitcher")
        self.label.setProperty('labelClass', 'label-title')
        self.layout().addWidget(self.label)
        self.description = QtWidgets.QLabel(
            "Stitch PNG sequences into a sprite sheet")
        self.description.setProperty('labelClass', 'label-description')
        self.layout().addWidget(self.description)
        self.layout().addWidget(qt_lines.QHLine())

        # Add an import button
        self.import_button = QtWidgets.QPushButton("Select PNGs")
        self.import_button.clicked.connect(self.load_png_dialog)
        self.layout().addWidget(self.import_button)
        self.layout().addWidget(qt_lines.QHLine())

        # Add a selection option for the sprite type
        known_sprite_types = sprite_types.get_known_sprite_types().values()
        self.sprite_type_layout = QtWidgets.QHBoxLayout()
        self.sprite_type_label = QtWidgets.QLabel("Sprite Type")
        self.sprite_type_layout.addWidget(self.sprite_type_label)
        self.sprite_type_dropdown = QtWidgets.QComboBox()
        for ty in known_sprite_types:
            self.sprite_type_dropdown.addItem(ty)
        self.sprite_type_dropdown.setEnabled(False)
        self.sprite_type_layout.addWidget(self.sprite_type_dropdown)
        self.layout().addLayout(self.sprite_type_layout)

        # Add a box to accept a sprite name
        self.sprite_name_layout = QtWidgets.QHBoxLayout()
        self.sprite_name_label = QtWidgets.QLabel("Sprite Name")
        self.sprite_name_layout.addWidget(self.sprite_name_label)
        self.sprite_name_input = QtWidgets.QLineEdit()
        self.sprite_name_input.setText("unnamedSprite")
        self.sprite_name_input.setEnabled(False)
        self.sprite_name_layout.addWidget(self.sprite_name_input)
        self.layout().addLayout(self.sprite_name_layout)

        # Add a selection option for the sprite optimization
        self.optimization_layout = QtWidgets.QHBoxLayout()
        self.optimization_label = QtWidgets.QLabel("Optimize For")
        self.optimization_layout.addWidget(self.optimization_label)
        self.optimization_dropdown = QtWidgets.QComboBox()
        self.optimization_dropdown.addItem("Size")
        self.optimization_dropdown.addItem("Quality")
        self.optimization_dropdown.setEnabled(False)
        self.optimization_layout.addWidget(self.optimization_dropdown)
        self.layout().addLayout(self.optimization_layout)

        # Add a number input for the target FPS
        self.fps_layout = QtWidgets.QHBoxLayout()
        self.fps_label = QtWidgets.QLabel("Target FPS")
        self.fps_layout.addWidget(self.fps_label)
        self.fps_input = QtWidgets.QLineEdit()
        self.fps_input.setText("24")
        self.fps_input.setEnabled(False)
        self.fps_layout.addWidget(self.fps_input)
        self.layout().addLayout(self.fps_layout)

        # Add a seperator
        self.layout().addWidget(qt_lines.QHLine())

        # Add a button to start the stitching process
        self.finishing_layout = QtWidgets.QHBoxLayout()
        self.close_button = QtWidgets.QPushButton("Cancel")
        self.close_button.clicked.connect(self.close)
        self.finishing_layout.addWidget(self.close_button)
        self.stitch_button = QtWidgets.QPushButton("Stitch")
        self.stitch_button.clicked.connect(self.stitch_images)
        self.stitch_button.setEnabled(False)
        self.finishing_layout.addWidget(self.stitch_button)
        self.layout().addLayout(self.finishing_layout)

        # Add space at the bottom in case window size is wrong
        self.layout().addStretch()

    def load_png_dialog(self):

        # Open a file picker to search for the desired image
        file_dialog = QtWidgets.QFileDialog()
        file_dialog.setFileMode(QtWidgets.QFileDialog.ExistingFiles)
        file_dialog.setNameFilter("Image Files (*.png *.jpg *.jpeg)")
        file_dialog.setViewMode(QtWidgets.QFileDialog.Detail)
        file_dialog.setLabelText(QtWidgets.QFileDialog.Accept, "Import")
        file_dialog.setLabelText(QtWidgets.QFileDialog.Reject, "Cancel")
        file_dialog.setWindowTitle("Import PNG Sequence")
        file_dialog.setAcceptMode(QtWidgets.QFileDialog.AcceptOpen)
        file_dialog.setDirectory(os.path.join(get_project_root(), "assets"))

        # If the user selected an image, import it
        if file_dialog.exec_():
            # Enable all the disabled fields
            self.sprite_type_dropdown.setEnabled(True)
            self.sprite_name_input.setEnabled(True)
            self.stitch_button.setEnabled(True)
            self.optimization_dropdown.setEnabled(True)
            self.fps_input.setEnabled(True)

            # Save the selected files
            self.selected_files = file_dialog.selectedFiles()

        else:
            logger.warning("No image selected")
            return

    def stitch_images(self):

        # Check the naming convention
        if not re.match(SPRITE_NAMING_VALIDATION, self.sprite_name_input.text()):

            # Pop up a warning
            warning_dialog = QtWidgets.QMessageBox()
            warning_dialog.setIcon(QtWidgets.QMessageBox.Warning)
            warning_dialog.setText("Invalid Sprite Name")
            warning_dialog.setInformativeText(
                "The sprite name must be lower camel case\nExample: myShinySprite")
            warning_dialog.setWindowTitle("Invalid Sprite Name")
            warning_dialog.exec_()

            return

        # Check if we are about to overwrite an existing sprite
        known_sprite_types = sprite_types.get_known_sprite_types()
        ty_long_to_short_map = dict(map(reversed, known_sprite_types.items()))
        sprite_type = ty_long_to_short_map[self.sprite_type_dropdown.currentText(
        )]
        sprite_name = self.sprite_name_input.text()
        if stitcher.check_sprite_exists(sprite_type, sprite_name):

            # Pop up confirmation box
            warning_dialog = QtWidgets.QMessageBox()
            warning_dialog.setIcon(QtWidgets.QMessageBox.Warning)
            warning_dialog.setText("Overwrite Sprite?")
            warning_dialog.setInformativeText(
                "A sprite with the name {}_{} already exists.\nDo you want to overwrite it?".format(sprite_type, sprite_name))
            warning_dialog.setWindowTitle("Overwrite Sprite?")
            warning_dialog.setStandardButtons(
                QtWidgets.QMessageBox.Yes | QtWidgets.QMessageBox.No)
            warning_dialog.setDefaultButton(QtWidgets.QMessageBox.No)
            if warning_dialog.exec_() == QtWidgets.QMessageBox.No:
                return


        # Pop up an error if the inputted FPS is not a number
        try:
            fps = float(self.fps_input.text())
        except ValueError:
            warning_dialog = QtWidgets.QMessageBox()
            warning_dialog.setIcon(QtWidgets.QMessageBox.Warning)
            warning_dialog.setText("Invalid FPS")
            warning_dialog.setInformativeText(
                "The FPS must be a number")
            warning_dialog.setWindowTitle("Invalid FPS")
            warning_dialog.exec_()
            return

        # Perform the actual stitching action
        stitcher.stitch_images_and_write_to_disk(
            sprite_type, sprite_name, self.selected_files, self.optimization_dropdown.currentText() == "Size", float(self.fps_input.text()))

        # Close the window
        self.close()
