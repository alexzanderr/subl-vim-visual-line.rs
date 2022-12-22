https://stackoverflow.com/questions/437589/how-do-i-unload-reload-a-python-module/438845#438845

https://docs.python.org/3/library/importlib.html#importlib.reload








#
- https://pyo3.rs/latest/faq.html#i-cant-run-cargo-test-or-i-cant-build-in-a-cargo-workspace-im-having-linker-issues-like-symbol-not-found-or-undefined-reference-to-_pyexc_systemerror


# dict derive
https://github.com/gperinazzo/dict-derive

#
- https://github.com/davidhewitt/pythonize


# exampels
https://github.com/PyO3/pyo3#examples

# maturin
https://github.com/PyO3/maturin
https://www.maturin.rs/develop.html

#

https://stackoverflow.com/questions/24736146/how-to-use-virtualenv-in-makefile
https://gist.github.com/bbl/bf4bf5875d0c705c4cd78d264f98a8b1

https://stackoverflow.com/questions/33839018/activate-virtualenv-in-makefile

# sublime plugins
- https://www.sublimetext.com/docs/1/plugin-examples
- https://www.sublimetext.com/docs/1/plugin-basics
- https://www.sublimetext.com/docs/1/api-reference

https://www.sublimetext.com/docs/api_reference.html#sublime.View

https://docs.sublimetext.io/reference/settings.html#whitespace-and-indentation

# to check
- https://github.com/gerardroche/sublime-api-completions`
- https://sublime-text-unofficial-documentation.readthedocs.io/en/sublime-text-2/reference/api.html#module-sublime_plugin




- https://forum.sublimetext.com/t/how-to-bind-multiple-commands-for-a-keybinding/20745/2

- https://stackoverflow.com/questions/13388477/multiple-commands-in-a-single-sublime-text-2-user-keymap-shortcut

https://gist.github.com/khoanguyen26/7216553

# sublime API
https://www.sublimetext.com/docs/api_reference.html
https://sublime-text-unofficial-documentation.readthedocs.io/en/sublime-text-2/reference/api.html#module-sublime_plugin

- [TextCommand](https://www.sublimetext.com/docs/api_reference.html#sublime_plugin.TextCommand)
- [View](https://www.sublimetext.com/docs/api_reference.html#sublime.View)
- [Selection](https://www.sublimetext.com/docs/api_reference.html#sublime.Selection)
- [Region](https://www.sublimetext.com/docs/api_reference.html#sublime.Region)
- [Edit](https://www.sublimetext.com/docs/api_reference.html#sublime.Edit)
- [Commands(old)](https://www.sublimetext.com/docs/1/commands)
- []()
- []()
- []()
- []()
- []()
- []()

# reloading
https://stackoverflow.com/questions/437589/how-do-i-unload-reload-a-python-module/438845#438845

https://docs.python.org/3/library/importlib.html#importlib.reload


https://forum.sublimetext.com/t/hot-update-without-restarting-sublime/14965/2

https://github.com/wbond/package_control/blob/d3bc04fd5c0c4bdb476a3ea798e6815508dc9bac/package_control/reloader.py

https://github.com/wbond/package_control/blob/d3bc04fd5c0c4bdb476a3ea798e6815508dc9bac/Package%20Control.py#L98-L108

https://github.com/timbrel/GitSavvy/blob/599ba3cdb539875568a96a53fafb033b01708a67/common/util/reload.py

https://forum.sublimetext.com/t/auto-reloading-of-python-module-files-used-by-plugin/5321?u=kingkeith

https://forum.sublimetext.com/t/reloading-external-modules-st3/8990?u=kingkeith

```python
class ReloadCommand(sublime_plugin.WindowCommand):
    def run(self):
        logger.log.info("reload")
        import sys
        from imp import reload
        for module in sys.modules.keys():
            if "MY_PLUGIN_DIR" in module:
                reload(sys.modules[module])
        logger.log.info("reload DONE")
```
???

# scheduling apps without containers using linux orchestrator
- https://medium.com/hashicorp-engineering/hashicorp-nomad-from-zero-to-wow-1615345aa539#8e85

- https://developer.hashicorp.com/nomad/tutorials/get-started/get-started-jobs

- https://github.com/hashicorp/nomad-guides/blob/master/application-deployment/go-blue-green/README.md

- https://github.com/hashicorp/http-echo

- https://blog.knoldus.com/how-to-run-the-binary-job-in-hashicrop-nomad/
### example using binary/command
```shell
job "binary" {
  datacenters = ["dc1"]

  group "binary-example" {
    network {
      port "http" {
        static = "9090"
      }
    }

    task "server" {
      driver = "exec"

      config {
        command = "/bin/http-echo"

        args = [
          "-listen",
          ":9090",
          "-text",
          "Hello",
        ]
      }
    }
  }
}
```

# debugging plugins
https://stackoverflow.com/questions/16384626/how-to-debug-sublime-plugins-during-development
https://packagecontrol.io/packages/Plugin%20Debugger


# docs
https://sublime-text-unofficial-documentation.readthedocs.io/en/latest/reference/commands.html#id1

https://pyo3.rs/v0.17.3/exception.html

#
https://gist.github.com/gerardroche/6e46cbdf8da19a39f9da

# discussions
https://gitter.im/PyO3/Lobby?at=5dca047750010612b2a08311



# try this
https://rustrepo.com/repo/mcandre-tinyrick-rust-development-tools

https://stackoverflow.com/questions/4585929/how-to-use-cp-command-to-exclude-a-specific-directory

# dont even manually move a virtual environment cuz will crash

# makefile
https://www.gnu.org/software/make/manual/html_node/Functions.html

https://www.gnu.org/software/make/manual/html_node/Call-Function.html

# rsync

if you dont want the src folder to be copied put a slash at the end of src like so: `src/` (this way you only want to copy the contents of the folder)