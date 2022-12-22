use colored::*;

use super::imports::prelude::*;
// use super::Selection;
use crate::utils::*;

pub struct Region<'a> {
    python: Python<'a>,
    inner:  PyObject,
}
impl<'a> Region<'a> {
    pub fn from_raw(
        python: Python<'a>,
        object: PyObject,
    ) -> Self {
        Self {
            python,
            inner: object,
        }
    }

    pub fn to_rs_tuple(&self) -> PyResult<(i32, i32)> {
        // https://www.sublimetext.com/docs/api_reference.html#sublime.Region.to_tuple
        // function `to_tuple` returns tuple[Point, Point]
        // but since Point = int as alias
        // it can be converted easily below with extract
        let tuple_result = self.inner.call_method(
            self.python,
            PyString::new(self.python, "to_tuple"),
            (),
            None,
        );
        let tuple_raw = match tuple_result {
            Ok(rt) => rt,
            Err(error) => {
                let msg = error.to_string().yellow().to_string();
                _print(self.python, msg);
                // error.print(self.python);
                // _print(self.python, error);
                todo!();
            },
        };
        // println!("{}", "---".yellow());
        // _printa(self.python, &tuple_raw);
        // println!("{}", "---".red());

        let tuple = tuple_raw.extract::<(i32, i32)>(self.python)?;

        Ok(tuple)
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
}
