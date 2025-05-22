use {
    grep::{matcher::Matcher, regex::RegexMatcher, searcher::sinks::UTF8, searcher::Searcher},
    ignore::{WalkBuilder, WalkState},
    pyo3::prelude::*,
    std::fs::File,
    std::path::Path,
    std::sync::Mutex,
};

//mod error;
use crate::error::{Error, SearchResult};

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

fn search_single_file<R>(file: R, search_regex: &str) -> SearchResult<Vec<SearchMatch>>
where
    R: std::io::Read,
{
    let matcher = RegexMatcher::new(search_regex).map_err(Error::from)?;
    let mut matches: Vec<SearchMatch> = vec![];

    let _ = Searcher::new()
        .search_reader(
            &matcher,
            file,
            UTF8(|lnum, line| {
                let mymatch = matcher.find(line.as_bytes())?;
                if let Some(m) = mymatch {
                    matches.push(SearchMatch::new(lnum, line[m].to_string()));
                }
                Ok(true)
            }),
        )
        .map_err(Error::from);
    Ok(matches)
}

/// Performs the search on one file
pub(crate) fn search_single_path(
    file_path: &Path,
    search_regex: &str,
) -> SearchResult<Vec<SearchMatch>> {
    let file = File::open(file_path)?;
    search_single_file(&file, search_regex)
}

/// implementation for py_search
pub fn search_file(file_name: &str, search_regex: &str) -> SearchResult<Vec<SearchMatch>> {
    let f_handle = Path::new(&file_name);
    search_single_path(f_handle, search_regex)
}

/// implementation for py_search_dir
pub fn search_dir(
    dir_path: &str,
    search_regex: &str,
    hidden: Option<bool>,
) -> SearchResult<Vec<(SearchMatch, String)>> {
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
                if let Ok(captured) = search_single_path(path, search_regex) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_single_file() {
        let f = "name abc\n123 xyz\nteststring\n\nname"
            .to_owned()
            .into_bytes();
        let c = std::io::Cursor::new(f);

        let res = search_single_file(c, "name").unwrap();
        assert_eq!(res.len(), 2)
    }
}
