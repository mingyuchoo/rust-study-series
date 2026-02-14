use ratatui_diary::storage::Storage;
use tempfile::TempDir;
use chrono::NaiveDate;

#[test]
fn test_new_creates_entries_directory() {
    let temp = TempDir::new().unwrap();
    let storage = Storage::with_dir(temp.path()).unwrap();

    let entries_dir = temp.path().join("entries");
    assert!(entries_dir.exists());
    assert!(entries_dir.is_dir());
}

#[test]
fn test_save_creates_markdown_file() {
    let temp = TempDir::new().unwrap();
    let storage = Storage::with_dir(temp.path()).unwrap();

    let date = NaiveDate::from_ymd_opt(2026, 2, 14).unwrap();
    let content = "Test diary content";

    storage.save(date, content).unwrap();

    let file_path = temp.path().join("entries/2026-02-14.md");
    assert!(file_path.exists());

    let saved = std::fs::read_to_string(file_path).unwrap();
    assert_eq!(saved, content);
}

#[test]
fn test_load_existing_diary() {
    let temp = TempDir::new().unwrap();
    let storage = Storage::with_dir(temp.path()).unwrap();

    let date = NaiveDate::from_ymd_opt(2026, 2, 14).unwrap();
    let content = "Existing diary";
    storage.save(date, content).unwrap();

    let loaded = storage.load(date).unwrap();
    assert_eq!(loaded, content);
}

#[test]
fn test_load_nonexistent_diary() {
    let temp = TempDir::new().unwrap();
    let storage = Storage::with_dir(temp.path()).unwrap();

    let date = NaiveDate::from_ymd_opt(2026, 2, 14).unwrap();
    let result = storage.load(date);

    assert!(result.is_err());
}
