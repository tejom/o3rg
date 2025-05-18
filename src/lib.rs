use {
    grep::{matcher::Matcher, regex::RegexMatcher, searcher::sinks::UTF8, searcher::Searcher},
    ignore::{WalkBuilder, WalkState},
    pyo3::prelude::*,
    std::path::Path,
    std::sync::Mutex,
};

mod error;
mod search;

use crate::error::Error;
use crate::search::{do_search, SearchMatch};

/// Search one file for the regex string
#[pyfunction]
#[pyo3(name = "search")]
fn py_search(file_name: String, search_regex: String) -> PyResult<Vec<SearchMatch>> {
    let f_handle = Path::new(&file_name);
    do_search(f_handle, &search_regex)
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
    let matches: Mutex<Vec<(SearchMatch, String)>> = Mutex::new(vec![]);

    let mut walk_builder = WalkBuilder::new(dir_path);

    if let Some(false) = hidden {
        walk_builder.hidden(false);
    }

    let w = walk_builder.build_parallel();

    w.run(|| {
        Box::new(|entry_result| {
            if let Ok(entry) = entry_result {
                let path = entry.path();
                if let Ok(captured) = do_search(path, &search_regex) {
                    let with_fname = captured.into_iter().filter_map(|res| {
                        let file_name = path.to_str()?.to_owned();
                        Some((res, file_name))
                    });
                    matches
                        .lock()
                        .expect("Issue while getting mutex lock")
                        .extend(with_fname.collect::<Vec<_>>());
                }
            }

            WalkState::Continue
        })
    });

    Ok(matches.into_inner().expect("Unable to get inner value"))
}

/// A Python module implemented in Rust.
#[pymodule]
fn o3rg(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_search, m)?)?;
    m.add_function(wrap_pyfunction!(py_search_dir, m)?)?;
    Ok(())
}
