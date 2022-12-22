use super::imports::prelude::*;

pub struct TextCommand<'a> {
    python: Python<'a>,
    inner:  PyObject,
}

impl<'a> TextCommand<'a> {
    pub fn new(
        python: Python<'a>,
        object: PyObject,
    ) -> Self {
        Self {
            python,
            inner: object,
        }
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
