#![allow(
    dead_code,
    unused_imports,
    unused_variables,
    unused_macros,
    unused_assignments,
    unused_mut,
    non_snake_case,
    unused_must_use,
    non_upper_case_globals,
    non_camel_case_types,
    semicolon_in_expressions_from_macros,
    redundant_semicolons,
    unused_macros
)]

use pyo3::prelude::*;
use pyo3::types::*;
use pyo3::*;

mod sublime;
mod commands;
mod utils;

/// plugin_loaded
#[pyfunction]
pub fn plugin_loaded(python: Python<'_>) -> PyResult<()> {
    utils::_print(python, "vim-visual-line-rs loaded (note: this messages is printed from rust)");
    Ok(())
}

#[pymodule]
fn vim_visual_line(
    python: Python,
    module: &PyModule,
) -> PyResult<()> {
    module.add_class::<commands::VimVisualLine>()?;
    module.add_class::<commands::CancelSelection>()?;
    module.add_function(wrap_pyfunction!(plugin_loaded, module)?)?;

    Ok(())
}
