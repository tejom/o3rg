use {
    grep::{matcher::Matcher, regex::RegexMatcher, searcher::sinks::UTF8, searcher::Searcher},
    ignore::{WalkBuilder, WalkState},
    pyo3::prelude::*,
    std::fs::File,
    std::path::Path,
    std::sync::Mutex,
};

use crate::error::{Error, SearchResult};

/// Class for a match result in a file returned to Python users.
/// Line is the line number of the match.
/// Match result is the text of the match.
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
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_search_single_file() {
        let f = "name abc\n123 xyz\nteststring\n\nname"
            .to_owned()
            .into_bytes();
        let c = std::io::Cursor::new(f);

        let res = search_single_file(c, "name").unwrap();
        assert_eq!(res.len(), 2)
    }

    #[test]
    fn test_search_single_file_no_matches() {
        let f = "test abc\n123 xyz\nteststring".to_owned().into_bytes();
        let c = std::io::Cursor::new(f);

        let res = search_single_file(c, "nonexistent").unwrap();
        assert_eq!(res.len(), 0);
    }

    #[test]
    fn test_search_single_file_with_regex() {
        let f = "test123\ntest456\nnotmatch\ntest789"
            .to_owned()
            .into_bytes();
        let c = std::io::Cursor::new(f);

        let res = search_single_file(c, r"test\d+").unwrap();
        assert_eq!(res.len(), 3);
    }

    #[test]
    fn test_search_single_path_and_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "line1 test\nline2\nline3 test").unwrap();

        let res = search_single_path(&file_path, "test").unwrap();
        assert_eq!(res.len(), 2);

        // Test search_file wrapper as well
        let res = search_file(file_path.to_str().unwrap(), "test").unwrap();
        assert_eq!(res.len(), 2);
    }

    #[test]
    fn test_search_dir_basic() {
        let temp_dir = TempDir::new().unwrap();

        // Create a few test files
        let file1_path = temp_dir.path().join("file1.txt");
        let mut file1 = File::create(&file1_path).unwrap();
        writeln!(file1, "test content\nmore content").unwrap();

        let file2_path = temp_dir.path().join("file2.txt");
        let mut file2 = File::create(&file2_path).unwrap();
        writeln!(file2, "test another\ntest final").unwrap();

        let res = search_dir(temp_dir.path().to_str().unwrap(), "test", Some(false)).unwrap();

        assert_eq!(res.len(), 3); // Should find 3 matches across both files
    }

    #[test]
    fn test_search_dir_with_hidden() {
        let temp_dir = TempDir::new().unwrap();

        // Create a regular file
        let file_path = temp_dir.path().join("visible.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "test content").unwrap();

        // Create a hidden file
        let hidden_path = temp_dir.path().join(".hidden.txt");
        let mut hidden_file = File::create(&hidden_path).unwrap();
        writeln!(hidden_file, "test content").unwrap();

        // Test with hidden files excluded (default behavior)
        let res_without_hidden = search_dir(
            temp_dir.path().to_str().unwrap(),
            "test",
            Some(true), // true means ignore hidden files
        )
        .unwrap();
        assert_eq!(res_without_hidden.len(), 1);

        // Test with hidden files included
        let res_with_hidden = search_dir(
            temp_dir.path().to_str().unwrap(),
            "test",
            Some(false), // false means don't ignore hidden files
        )
        .unwrap();
        assert_eq!(res_with_hidden.len(), 2);
    }

    #[test]
    fn test_invalid_regex() {
        let f = "test content".to_owned().into_bytes();
        let c = std::io::Cursor::new(f);

        let res = search_single_file(c, "[invalid regex(");
        assert!(res.is_err());
    }
}
