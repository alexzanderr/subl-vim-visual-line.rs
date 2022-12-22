use pyo3::prelude::*;
use pyo3::types::*;
use pyo3::*;

use crate::sublime::prelude::*;
use crate::utils::_print;

#[pyclass]
pub struct VimVisualLine {}

#[pymethods]
impl VimVisualLine {
    #[new]
    fn new(python: Python<'_>) -> Self {
        // println!("{:?}", python);

        Self {}
    }

    pub fn by_wholelines_forward_extend<'a>(
        &'a self,
        py: Python<'a>,
        forward: bool,
        extend: bool,
    ) -> PyResult<&'a PyDict> {
        let kwargs = PyDict::new(py);
        kwargs.set_item("by", "wholelines")?;
        kwargs.set_item("forward", forward)?;
        kwargs.set_item("extend", extend)?;

        Ok(kwargs)
    }

    #[args(args = "*", kwargs = "**")]
    pub fn run(
        &self,
        py: Python<'_>,
        sublime_module: &PyModule,
        sublime_plugin_module: &PyModule,
        raw_text_command: PyObject,
        edit: PyObject,
        args: &PyTuple,
        kwargs: Option<&PyDict>,
    ) -> PyResult<()> {
        // let sublime_py = py.import(PyString::new(py, "sublime"))?;
        // let sublime_plugin_py =
        //     py.import(PyString::new(py, "sublime_plugin"))?;

        // println!("{}", args);
        // println!("{:?}", kwargs);
        // _print(py, "here");

        let kwargs = kwargs.unwrap();
        let extend = kwargs
            .get_item("extend".to_object(py))
            .unwrap()
            .extract::<bool>()?;
        let forward = kwargs
            .get_item("forward".to_object(py))
            .unwrap()
            .extract::<bool>()?;

        let text_command = TextCommand::new(py, raw_text_command);
        let raw_view = text_command.get_attr("view")?;
        let view = View::from_raw(py, raw_view);
        // selection_area = self.view.sel()
        let selection_area = view.sel()?;

        // let selection = sublime::Selection::new(py, selection_area);
        // println!("{:?}", selection);

        // println!("selection: {}", selection_area);

        // // if selection_area.__len__() != 0:
        let length = selection_area.len()?;
        // println!("len: {}", length);

        if length == 0 {
            return Ok(());
        }

        // start, end = selection_area[0]
        let region = selection_area.to_region()?;
        // println!("pair: {}", region);
        let (start, stop) = region.to_rs_tuple()?;
        // println!("start: {}", start);
        // println!("end: {}", end);
        // if start == end:
        if start == stop {
            //         let sequence =
            //             [("to", "hardbol"), ("extend", &extend.to_string())];
            // let kwargs =
            // PyDict::from_sequence(py, sequence.to_object(py))?;
            let kwargs = PyDict::new(py);
            kwargs.set_item("to", "hardbol")?;
            kwargs.set_item("extend", extend)?;

            // self.view.run_command("move_to", {"to": "hardbol", "extend": extend})
            view.run_command("move_to", Some(&kwargs))?;

            let kwargs =
                self.by_wholelines_forward_extend(py, forward, extend)?;
            for _ in 0..2 {
                // self.view.run_command("move", {"by": "wholelines", "forward": forward, "extend": extend})
                view.run_command("move", Some(&kwargs))?;
            }
        } else {
            let kwargs =
                self.by_wholelines_forward_extend(py, forward, extend)?;
            // self.view.run_command("move", {"by": "wholelines", "forward": forward, "extend": extend})
            view.run_command("move", Some(&kwargs))?;
            // new_selection = self.view.sel()
            // here we are getting a new selection because it changed
            // after the command move ran
            let new_selection = view.sel()?;
            // if new_selection.__len__() != 0:
            if new_selection.len()? == 0 {
                return Ok(());
            }

            let region = selection_area.to_region()?;
            // println!("pair: {}", region);
            let (start, stop) = region.to_rs_tuple()?;
            if start != stop {
                return Ok(());
            }

            // self.view.run_command("move", {"by": "wholelines", "forward": forward, "extend": extend})
            view.run_command("move", Some(&kwargs))?;
        }

        Ok(())
    }
}
