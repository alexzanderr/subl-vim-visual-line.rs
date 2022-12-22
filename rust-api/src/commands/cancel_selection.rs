use pyo3::prelude::*;
use pyo3::types::*;
use pyo3::*;

use crate::sublime::prelude::*;
use crate::utils::*;

fn new_function(x: i32) -> u32 {
    x.count_ones()
}

#[pyclass]
pub struct CancelSelection {}

#[pymethods]
impl CancelSelection {
    #[new]
    fn new() -> Self {
        Self {}
    }

    fn __log(
        &self,
        python: Python<'_>,
        msg: &str,
    ) {
        let msg = format!("[CancelSelection] [INFO]: {msg}");
        _print(python, msg);
    }

    #[args(args = "*", kwargs = "**")]
    pub fn run(
        &self,
        python: Python<'_>,
        sublime_module: &PyModule,
        sublime_plugin_module: &PyModule,
        raw_text_command: PyObject,
        edit: PyObject,
        args: &PyTuple,
        kwargs: Option<&PyDict>,
    ) -> PyResult<()> {
        // _printa(python, &sublime_module);
        // _printa(python, &sublime_plugin_module);

        // let sublime_py =
        //     python.import(PyString::new(python, "sublime"))?;
        // let sublime_plugin_py =
        //     python.import(PyString::new(python, "sublime_plugin"))?;

        let text_command = TextCommand::new(python, raw_text_command);
        let raw_view = text_command.get_attr("view")?;
        let view = View::from_raw(python, raw_view);
        // selection_area = self.view.sel()
        let mut selection_area = view.sel()?;

        // // if selection_area.__len__() != 0:
        let length = selection_area.len()?;
        self.__log(python, &format!("len: {length}"));

        if length == 0 {
            return Ok(());
        }

        // start, end = selection_area[0]
        let region = selection_area.to_region()?;
        // println!("pair: {}", region);
        let (start, end) = region.to_rs_tuple()?;
        self.__log(python, &format!("start: {start}"));
        self.__log(python, &format!("start: {end}"));
        // println!("start: {}", start);
        // println!("end: {}", end);
        // if start == end:

        // there is no selection
        if start == end {
            return Ok(());
        }

        // selector.clear()
        // TODO: the plugin doesnt crash, but this line doesnt work
        // code runs after this with no problem but this is not ran
        // selection is not canceled
        selection_area.clear();

        // i guess this object is Copy
        let selection_inner = selection_area.inner.clone();
        // python.eval("selector.add(sublime.Region(stop))")
        // this is working
        // thanks to this
        // https://pyo3.rs/latest/python_from_rust.html#want-to-run-statements-then-use-run
        py_run!(python,
            end selection_inner,
            "selection_inner.add(sublime.Region(end))");

        // END

        let locals = [("end", end.to_object(python))].into_py_dict(python);
        let run_result =
            python.run("result = sublime.Region(end)", None, Some(locals));
        match run_result {
            Err(some_error) => {
                let msg = format!("{:?}", some_error);
                _print(python, msg);
                return Ok(());
            },
            _ => {},
        };
        let _new_region = locals.get_item("result").unwrap();
        py_run!(python, _new_region, "print(_new_region)");

        // let _new_region = python.eval("sublime.Region()", None, None)?;

        // TODO
        // selector.add(sublime.Region(stop))
        // the class
        let sublime_Region =
            sublime_module.getattr(PyString::new(python, "Region"))?;
        // this is not None

        py_run!(python, sublime_Region, "print(sublime_Region)");
        // _printa(python, sublime_Region.into_ref(python));

        // let kwargs = PyDict::new(python);
        // kwargs.set_item("a", 123)?;
        // kwargs.set_item("b", 234)?;

        // class Region:
        //     __slots__ = ['a', 'b', 'xpos']

        //     def __init__(self, a, b=None, xpos=-1):
        //         if b is None:
        //             b = a
        //         self.a = a
        //         self.b = b
        //         self.xpos = xpos

        // __new__ called with the class name
        // new_Region is not None
        let new_Region = sublime_Region.call_method(
            PyString::new(python, "__new__"),
            (sublime_Region,),
            None,
        )?;

        // pyo3_runtime.PanicException: print(new_Region)
        // https://docs.rs/pyo3/0.17.3/pyo3/marker/struct.Python.html#method.run

        let locals = [("new_region", new_Region.to_object(python))]
            .into_py_dict(python);
        let run_result = python.run(
            "print(f'new region from rust: {new_region}')",
            None,
            Some(locals),
        );

        match run_result {
            Err(some_error) => {
                // PyErr {
                // type: <class 'AttributeError'>,
                // value: AttributeError('a'),
                // traceback: Some(<traceback object at 0x7fa081424680>)
                // }
                let t = some_error.traceback(python).unwrap();
                // <traceback object at 0x7f3b918b7a00>
                let msg = format!("{}", t.format()?);
                // Traceback (most recent call last):
                //   File "<string>", line 1, in <module>
                //   File "/opt/sublime_text/Lib/python38/sublime.py", line 938, in __str__
                //     return "(" + str(self.a) + ", " + str(self.b) + ")"
                _print(python, msg);

                return Ok(());
            },
            _ => {},
        };

        // __init__ called with the __new__ object
        // well this is None
        let region = new_Region.call_method(
            PyString::new(python, "__init__"),
            (new_Region, end),
            // (end,),
            None,
        )?;
        py_run!(python, region, "print(region)");

        // this is needed when i need to access a new

        // ORIGINAL ERROR: here region was None
        // sublime_api.view_selection_add_point(self.view_id, x)
        // TypeError: an integer is required (got type NoneType)
        self.__log(python, "selection canceled with ESC");
        selection_area.inner.call_method(
            python,
            PyString::new(python, "add"),
            (region,),
            None,
        )?;

        Ok(())
    }
}
