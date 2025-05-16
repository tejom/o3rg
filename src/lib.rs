use {
    grep::{matcher::Matcher, regex::RegexMatcher, searcher::sinks::UTF8, searcher::Searcher},
    ignore::{WalkBuilder, WalkState},
    pyo3::prelude::*,
    std::path::Path,
    walkdir::WalkDir,
};

mod error;

use crate::error::Error;

/// Performs the search on one file
fn do_search(file_path: &Path, search_regex: &str) -> PyResult<Vec<(u64, String)>> {
    let matcher = RegexMatcher::new(search_regex).map_err(Error::from)?;
    let mut matches: Vec<(u64, String)> = vec![];

    let _ = Searcher::new().search_path(
        &matcher,
        file_path,
        UTF8(|lnum, line| {
            let mymatch = matcher.find(line.as_bytes())?;
            if let Some(m) = mymatch {
                matches.push((lnum, line[m].to_string()));
            }
            Ok(true)
        }),
    );
    Ok(matches)
}

/// Search one file for the regex string
#[pyfunction]
fn search(file_name: String, search_regex: String) -> PyResult<Vec<(u64, String)>> {
    let f_handle = Path::new(&file_name);
    do_search(f_handle, &search_regex)
}

/// Search through a directory for the regex string
#[pyfunction]
fn search_dir(dir_path: String, search_regex: String) -> PyResult<Vec<(u64, String, String)>> {
    let mut matches: Vec<(u64, String, String)> = vec![];

    for entry in WalkDir::new(dir_path) {
        let entry = entry.unwrap();

        if let Ok(captured) = do_search(entry.path(), &search_regex) {
            let with_fname = captured.into_iter().filter_map(|res| {
                let file_name = entry.file_name().to_os_string().into_string().ok()?;
                Some((res.0, res.1, file_name))
            });
            matches.extend(with_fname.collect::<Vec<_>>())
        }
    }

    Ok(matches)
}

/// Search through a directory for the regex string
#[pyfunction]
fn search_dir_ignore(
    dir_path: String,
    search_regex: String,
) -> PyResult<Vec<(u64, String, String)>> {
    let matches: std::sync::Mutex<Vec<(u64, String, String)>> = std::sync::Mutex::new(vec![]);

    let walk_builder = WalkBuilder::new(dir_path);

    let w = walk_builder.build_parallel();

    w.run(|| {
        Box::new(|entry_result| {
            if let Ok(entry) = entry_result {
                let path = entry.path();
                if let Ok(captured) = do_search(path, &search_regex) {
                    let with_fname = captured.into_iter().filter_map(|res| {
                        let file_name = path.to_str()?.to_owned();
                        Some((res.0, res.1, file_name))
                    });
                    matches
                        .lock()
                        .expect("Issue whole getting mutex lock")
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
    m.add_function(wrap_pyfunction!(search, m)?)?;
    m.add_function(wrap_pyfunction!(search_dir, m)?)?;
    m.add_function(wrap_pyfunction!(search_dir_ignore, m)?)?;
    Ok(())
}
