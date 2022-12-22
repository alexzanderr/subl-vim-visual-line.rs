
import sublime
from sublime import *

import sublime_plugin
from sublime_plugin import *

from rich.traceback import install
install(show_locals=False)




import vim_visual_line as rust_api

def plugin_loaded():
    print("loaded")


class VimVisualLine(TextCommand):
    runner = rust_api.VimVisualLine()

    def run(self, edit, *args, **kwargs):
        self.runner.run(sublime, sublime_plugin, self, edit, *args, **kwargs)



class CancelSelection(TextCommand):
    runner = rust_api.CancelSelection()

    def run(self, edit, *args, **kwargs):
        self.runner.run(sublime, sublime_plugin, self, edit, *args, **kwargs)



def _main():
    edit = "some source code"
    command_args = {"extend": True, "forward": False}





    view = View()
    selection = view.sel()

    region = Region(a=123, b=124)
    print(region.to_tuple())

    selection.add(region)
    print(selection)

    command = VimVisualLine()
    result = command.run(edit, **command_args)
    print(f"result: {result}")

    command = CancelSelection()
    result = command.run(edit)
    print(f"result: {result}")

if __name__ == '__main__':
    _main()
