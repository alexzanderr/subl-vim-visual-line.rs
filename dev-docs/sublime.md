



### this sys path has python3.3 why??
```python
import sys
# is it because plugins from here `sublime-text/Packages` are used with python3.3
print(sys.path)
# if site_packages_py38 not in sys.path:
#     sys.path.append(site_packages_py38)
```

answer: because sublime runs some plugins (most of them) with python33 and when user developer creates 38 plugins, they are run with 38


### reload

its imported successfully, but
when i make a change to the code and recompile it
changes are not reflected here, only if i restart sublime
however, changes are reflected inside local modules
so: i guess that the cache is not updated on this module at all
i tried deleting the module from the virtual env path and recompile it, still doesnt work
because sublime text doesnt reload all the modules this module uses
it reloads only this one on file change and keeps the other modules cached

answer:
    python modules are reloaded, but not the rust compiled modules used from python

# plugin placement
if the plugin is placed in
~/.config/sublime-text/Packages/User/full_rust/
doesnt work; its not loaded
By default all plugins are run using Python 3.3.6 (i.e. plugins that are inside `~/config/sublime-text/Packages`) , except inside the User package which always uses the latest python. - from sublime docs

# traceback is only inside here (classes that extend sublime_plugin.classes)

#

```python
import os
full_rust_folder = "~/.config/sublime-text/Packages/full_rust"
if full_rust_folder not in sys.path:
    sys.path.append(full_rust_folder)
# now its working -
# i dont have to reload everything from full_rust folder
# if the shared object is rewritten (force copy) then plugin_host crashes
# the overcome that i have to delete the .so and recopy it
```

# reloading doesnt work for the rust .so module
```python
class Jumper2(sublime_plugin.TextCommand):
    # rust_ffi = rust_app.SomeSublimeCommand()
    def run(self, edit, *args, **kwargs):
        from . import rust_app
        from importlib import reload as __reload
        # not working at all
        # not even like this
        # you still need to restart sublime to reload the rust_app.so file
        # even if im calling reload manually
        # __reload()
        # print(rust_app)

        # print(sys.modules.values())
        rust_app = __reload(rust_app.rust_app)
        print(rust_app)
        print("da")

        x = rust_app.some_random_command_run()
        print(f"x: {x}")
        # error: plugin_host-3.8 has exited unexpectedly, some plugin functionality won't be available until Sublime Text has been restarted
        # how to reproduce:
        # start sublime
        # run this function
        # edit this file
        # the plugin will be re-copied, but this one not reloaded, vim-visual will be reloaded
        # run this function again
        # and while running this below function crashes second time after restart
        # question: am i not allowed to put the dependency inside plugin source code? until now i was appending to sys.path

```