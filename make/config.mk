

VENV_ACTIVATE = venv-python38/bin/activate

RUST_PKG = rust-api
PLUGIN_PY_NAME = vim_visual_line
PLUGIN_VENV_PATH = venv-python38/lib/python3.8/site-packages/${PLUGIN_PY_NAME}

PLUGIN_NAME = vim-visual-line
SUBLIME_PACKAGES = ~/.config/sublime-text/Packages
PLUGIN_SRC = sublime-plugin
SUBLIME_PLUGIN_RUST_API = ${PLUGIN_SRC}/rust_api



VENV_ACTIVATE = venv-python38/bin/activate

RUST_API_NAME = "vim_visual_line"

# this is not scalable
# if the version changes this variable is RIP
RUST_API_WHEEL_RELEASE = rust-api/release
RUST_API_DEST = ${PLUGIN_SRC}/rust_api
COLORS_SCRIPT = scripts/colors.sh


WATCHEXEC_FORCE_POLL = 100
WATCHEXEC_DEBOUCE = 500

# with this you can import the exported variables from the env.sh
# ifneq (,$(wildcard ./scripts/env.sh))
# 	include ./scripts/env.sh
# 	export
# endif