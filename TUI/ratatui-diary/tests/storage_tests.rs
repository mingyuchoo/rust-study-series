use ratatui_diary::storage::Storage;
use tempfile::TempDir;

#[test]
fn test_new_creates_entries_directory() {
    let temp = TempDir::new().unwrap();
    let storage = Storage::with_dir(temp.path()).unwrap();

    let entries_dir = temp.path().join("entries");
    assert!(entries_dir.exists());
    assert!(entries_dir.is_dir());
}
