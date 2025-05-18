use {
    grep::{matcher::Matcher, regex::RegexMatcher, searcher::sinks::UTF8, searcher::Searcher},
    ignore::{WalkBuilder, WalkState},
    pyo3::prelude::*,
    std::path::Path,
    std::sync::Mutex,
};

//mod error;
use crate::error::Error;


#[pyclass]
pub struct SearchMatch {
    line: u64,
    match_result: String,
}

impl SearchMatch {
    fn new(line: u64, match_result: String) -> SearchMatch {
        SearchMatch { line, match_result }
    }
}

#[pymethods]
impl SearchMatch {
    fn __repr__(&self) -> String {
        format!("SearchMatch({}, {})", self.line, self.match_result)
    }
}

/// Performs the search on one file
pub fn do_search(file_path: &Path, search_regex: &str) -> PyResult<Vec<SearchMatch>> {
    let matcher = RegexMatcher::new(search_regex).map_err(Error::from)?;
    let mut matches: Vec<SearchMatch> = vec![];

    let _ = Searcher::new().search_path(
        &matcher,
        file_path,
        UTF8(|lnum, line| {
            let mymatch = matcher.find(line.as_bytes())?;
            if let Some(m) = mymatch {
                matches.push(SearchMatch::new(lnum, line[m].to_string()));
            }
            Ok(true)
        }),
    );
    Ok(matches)
}
