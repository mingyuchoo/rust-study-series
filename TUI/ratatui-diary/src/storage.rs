use std::fs;
use std::path::{Path, PathBuf};
use std::io;
use std::collections::HashSet;
use std::ffi::OsString;
use chrono::NaiveDate;

pub struct Storage {
    entries_dir: PathBuf,
}

impl Storage {
    pub fn with_dir(base_dir: &Path) -> io::Result<Self> {
        let entries_dir = base_dir.join("entries");
        fs::create_dir_all(&entries_dir)?;
        Ok(Self { entries_dir })
    }

    pub fn new() -> io::Result<Self> {
        let base_dir = dirs::data_local_dir()
            .ok_or_else(|| io::Error::new(
                io::ErrorKind::NotFound,
                "Cannot find local data directory"
            ))?
            .join("ratatui-diary");
        Self::with_dir(&base_dir)
    }

    pub fn save(&self, date: NaiveDate, content: &str) -> io::Result<()> {
        let path = self.get_path(date);
        fs::write(path, content)
    }

    pub fn load(&self, date: NaiveDate) -> io::Result<String> {
        let path = self.get_path(date);
        fs::read_to_string(path)
    }

    pub fn delete(&self, date: NaiveDate) -> io::Result<()> {
        let path = self.get_path(date);
        fs::remove_file(path)
    }

    pub fn scan_entries(&self) -> io::Result<HashSet<NaiveDate>> {
        let mut entries = HashSet::new();

        for entry in fs::read_dir(&self.entries_dir)? {
            let entry = entry?;
            if let Some(date) = parse_filename(entry.file_name()) {
                entries.insert(date);
            }
        }

        Ok(entries)
    }

    fn get_path(&self, date: NaiveDate) -> PathBuf {
        self.entries_dir.join(format!("{}.md", date))
    }
}

fn parse_filename(filename: OsString) -> Option<NaiveDate> {
    let name = filename.to_str()?.strip_suffix(".md")?;
    NaiveDate::parse_from_str(name, "%Y-%m-%d").ok()
}
