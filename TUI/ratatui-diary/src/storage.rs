use std::fs;
use std::path::{Path, PathBuf};
use std::io;
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

    fn get_path(&self, date: NaiveDate) -> PathBuf {
        self.entries_dir.join(format!("{}.md", date))
    }
}
