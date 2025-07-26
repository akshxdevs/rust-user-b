#!/bin/bash
# Helper script to run cargo commands without Cursor IDE environment interference

# Clear Cursor-related environment variables
unset CURSOR_TRACE_ID
unset APPDIR
unset LD_LIBRARY_PATH
unset PERLLIB
unset GSETTINGS_SCHEMA_DIR
unset XDG_DATA_DIRS
unset QT_PLUGIN_PATH
unset GIO_LAUNCHED_DESKTOP_FILE
unset CHROME_DESKTOP
unset GIT_ASKPASS
unset VSCODE_GIT_ASKPASS_NODE
unset VSCODE_GIT_ASKPASS_MAIN

# Run the cargo command with all arguments passed to this script
cargo "$@" 