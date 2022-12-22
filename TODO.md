

# TODOs

- [x] determine how to pick the latest compiled rust package from the wheels folder `vim-visual-line/rust-api/target/wheels/...`


- [x] create entire plugin in rust and just copy the shared object into the User folder  (Doesnt work, i tried, the file cached wont be reloaded, even manually, only when i restart sublime, which is inconvenient)


- [x] create wrapper py file like so

    # it works, just with the functions
    # i cant extend classes in rust so i must create a wrapper from python
    # plugin.py
    ```python
    from rust_app import *
    ```
    and so rust_app contains all classes expanded with the help of
    ```python
    let sublime_py = py.import(PyString::new(py, "sublime"))?;
    let sublime_plugin_py =
        py.import(PyString::new(py, "sublime_plugin"))?;
    ```
    importing the sublime module

- [x] api importable from python that will be wrapped in python
    just create rust entrypoints to the rust run functions to be called from python, like this
    ```
    class SomeRandomCommand(sublime_plugin.TextCommand):
        rust_ffi = rust_app.SomeSublimeCommand()

        def run(self, edit, *args, **kwargs):
            print(rust_app)
            result = self.rust_ffi.run(self, edit, *args, **kwargs)
            print(f"result: {result}")
    ```

- [x] why i dont have autocomplete on this project from rust-analyzer ? (now i have, i dont understand how that was fixed)

- [x] auto reload sublime plugin dependencies, because sublime doesnt reload them, unless you restart sublime app ( doesnt work i couldnt find a way to reload the shared object, even manually)

- [ ] https://rustrepo.com/repo/mcandre-tinyrick-rust-development-tools
- [ ] put project on github
- [ ]
- [ ]
- [ ]
- [ ]
- [ ]
- [ ]
- [ ]
- [ ]
- [ ]
- [ ]
- [ ]
- [ ]
- [ ]
- [ ]
- [ ]
- [ ]