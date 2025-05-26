use o3rg::search;

#[test]
fn test_search_single() {
    let res = search::search_file("Cargo.toml", "name").unwrap();
    assert_eq!(res.len(), 2)
}

#[test]
fn test_search_dir() {
    let res = search::search_dir("./tests/fake_dir", "this is a", None).unwrap();
    assert_eq!(res.len(), 1)
}
