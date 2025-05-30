use o3rg::search;
use std::fs::File;
use std::io::Write;
use tempfile::TempDir;

#[test]
fn test_search_single() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.toml");
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, "[package]\nname = \"test\"\nversion = \"0.1.0\"").unwrap();

    let res = search::search_file(file_path.to_str().unwrap(), "name").unwrap();
    assert_eq!(res.len(), 1);
}

#[test]
fn test_search_dir() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create test files
    let file_a = temp_dir.path().join("a.txt");
    let mut file_a_handle = File::create(&file_a).unwrap();
    writeln!(file_a_handle, "this is a\n").unwrap();

    let file_b = temp_dir.path().join("b.txt");
    let mut file_b_handle = File::create(&file_b).unwrap();
    writeln!(file_b_handle, "this is b\n").unwrap();

    let res = search::search_dir(temp_dir.path().to_str().unwrap(), "this is a", None).unwrap();
    assert_eq!(res.len(), 1);

    // Test searching for a pattern that exists in both files
    let res = search::search_dir(temp_dir.path().to_str().unwrap(), "this is", None).unwrap();
    assert_eq!(res.len(), 2);
}

#[test]
fn test_search_dir_with_hidden() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create visible and hidden files
    let visible_file = temp_dir.path().join("visible.txt");
    let mut visible_handle = File::create(&visible_file).unwrap();
    writeln!(visible_handle, "test content").unwrap();

    let hidden_file = temp_dir.path().join(".hidden.txt");
    let mut hidden_handle = File::create(&hidden_file).unwrap();
    writeln!(hidden_handle, "test content").unwrap();

    // Test with hidden files excluded (default)
    let res = search::search_dir(temp_dir.path().to_str().unwrap(), "test", Some(true)).unwrap();
    assert_eq!(res.len(), 1);

    // Test with hidden files included
    let res = search::search_dir(temp_dir.path().to_str().unwrap(), "test", Some(false)).unwrap();
    assert_eq!(res.len(), 2);
}
