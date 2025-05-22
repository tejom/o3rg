use pyo3::prelude::*;

mod error;
pub mod search;

use crate::search::{search_dir, search_file, SearchMatch};

/// Search one file for the regex string
#[pyfunction]
#[pyo3(name = "search")]
fn py_search(file_name: String, search_regex: String) -> PyResult<Vec<SearchMatch>> {
    search_file(&file_name, &search_regex).map_err(PyErr::from)
}

/// Search through a directory for the regex string
#[pyfunction]
#[pyo3(name = "search_dir")]
#[pyo3(signature = (dir_path, search_regex, hidden=false))]
fn py_search_dir(
    dir_path: String,
    search_regex: String,
    hidden: Option<bool>,
) -> PyResult<Vec<(SearchMatch, String)>> {
    search_dir(&dir_path, &search_regex, hidden).map_err(PyErr::from)
}

/// o3rg module provides some wrappers for the searching and filesystem traversing libraries
/// used in ripgrep.
#[pymodule]
fn o3rg(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_search, m)?)?;
    m.add_function(wrap_pyfunction!(py_search_dir, m)?)?;
    Ok(())
}
