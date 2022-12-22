use pyo3::prelude::*;
use pyo3::types::*;
use pyo3::*;

pub fn _print(
    python: Python<'_>,
    message: impl AsRef<str>,
) {
    let msg = message.as_ref();
    let message = PyString::new(python, msg);
    py_run!(python, message, r#"print(message)"#);
}

pub fn _printa(
    python: Python<'_>,
    anything: &PyObject,
) {
    py_run!(python, anything, r#"print(anything)"#);
}
