



SUBLIME_ENV = False
try:
    import sublime
    import sublime_plugin
    SUBLIME_ENV = True
except ImportError:
    print("sublime environment not defined")
    from rich.traceback import install
    install(show_locals=False)

import importlib
import sys



# try:
#     from . import rust_api
#     # def plugin_loaded(): is not need anymore
#     # its imported from rust api and used as it is
#     from .rust_api import plugin_loaded
# except ImportError as ie:
#     import rust_api

# TODO import this config from yaml file
DEBUG_MODE = False




def _debug_print(*args, **kwargs):
    if DEBUG_MODE:
        print(*args, **kwargs)


from . import depend
import importlib

from . import vim_visual_line as rust_api
# reload doesnt work
rust_api = importlib.reload(rust_api)


vim_visual_line_runner = rust_api.VimVisualLine()

class VimVisualLine(sublime_plugin.TextCommand):

    def run(self, edit, *args, **kwargs):
        # depend.depender()
        _debug_print(f"edit: {edit}")
        _debug_print(f"args: {args}")
        _debug_print(f"kwargs: {kwargs}")
        _debug_print(rust_api)
        _debug_print("da its working")

        # i also tried here
        # reload doesnt work
        # runner = rust_api.VimVisualLine()
        vim_visual_line_runner.run(sublime, sublime_plugin, self, edit, *args, **kwargs)


        _debug_print("da its working 3")


cancel_selection_runner = rust_api.CancelSelection()

class CancelSelection(sublime_plugin.TextCommand):

    def run(self, edit, *args, **kwargs):
        _debug_print(f"edit: {edit}")
        _debug_print(f"args: {args}")
        _debug_print(f"kwargs: {kwargs}")
        _debug_print(rust_api)
        _debug_print("cancel selection works: 1")

        cancel_selection_runner.run(sublime, sublime_plugin, self, edit, *args, **kwargs)

        _debug_print("cancel selection works: 2")
