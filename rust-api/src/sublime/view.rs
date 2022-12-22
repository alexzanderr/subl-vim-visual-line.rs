use super::imports::prelude::*;
use super::Selection;

pub struct View<'a> {
    python: Python<'a>,
    inner:  PyObject,
}
impl<'a> View<'a> {
    pub fn from_raw(
        python: Python<'a>,
        object: PyObject,
    ) -> Self {
        Self {
            python,
            inner: object,
        }
    }

    /// https://www.sublimetext.com/docs/api_reference.html#sublime.View.run_command
    pub fn run_command(
        &self,
        command: &str,
        kwargs: Option<&PyDict>,
    ) -> PyResult<()> {
        // let run_command = self.get_attr("run_command")?;
        // py_run!(
        //     self.python,
        //     run_command,
        //     r#"print(f'run command: {run_command}')"#
        // );
        // // now its working, without any keys and values ...
        // let kwargs = PyDict::new(self.python);
        // // kwargs.set_item("to", "hardbol")?;
        // // kwargs.set_item("extend", false)?;
        // let args = (command,);
        // // this is also not working
        // // TypeError: run_command() got an unexpected keyword argument 'to'
        // run_command.call(self.python, args, Some(kwargs))?;

        // // self.view.run_command("move_to", {"to": "hardbol", "extend": extend})
        // let args = PyTuple::new(self.python, vec![command]);
        // py_run!(self.python, args, r#"print(args)"#);
        // py_run!(self.python, kwargs, r#"print(kwargs)"#);
        // let view = self.inner.clone();
        // // this is working
        // py_run!(self.python, view, "view.run_command('move_to', {'to': 'hardbol', 'extend': False})");

        // this is not working something its broken cuz its saying
        // unexpected keyword argument 'to' sau 'by'
        // it was working, but i was stupid
        // the run_command API is
        // run_command(cmd: str, args: CommandArgs = None)
        // where CommandArgs = Optional[dict[str, Value]]
        // there are no **kwargs, i thought there were kwargs
        // its just `cmd` and a `dict`
        // run_command(cmd: str, args: dict); not even *args
        self.inner.call_method(
            self.python,
            PyString::new(self.python, "run_command"),
            (command, kwargs.unwrap()),
            None,
        )?;

        Ok(())
    }

    pub fn get_attr(
        &self,
        name: &str,
    ) -> PyResult<Py<PyAny>> {
        let attribute = self
            .inner
            .getattr(self.python, PyString::new(self.python, name));
        attribute
    }

    pub fn sel(&self) -> PyResult<Selection> {
        let selection = self.inner.call_method(
            self.python,
            PyString::new(self.python, "sel"),
            (),
            None,
        )?;
        let selection = Selection::from_raw(self.python, selection);
        Ok(selection)
    }
}
