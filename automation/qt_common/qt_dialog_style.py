"""This module has an embedded CSS file defining the style of the dialogs."""

import pkgutil

STYLESHEET =  pkgutil.get_data(__name__, "./dialog.css").decode("utf-8")