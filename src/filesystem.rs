use anyhow::Result;
use std::path::PathBuf;
use walkdir::WalkDir;

pub struct FileSystem {
    current_dir: PathBuf,
}

impl FileSystem {
    pub fn new() -> Self {
        Self {
            current_dir: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
        }
    }

    pub fn search_files(&self, query: &str) -> Result<Vec<PathBuf>> {
        let mut results = Vec::new();

        for entry in WalkDir::new(&self.current_dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry
                .file_name()
                .to_string_lossy()
                .to_lowercase()
                .contains(&query.to_lowercase())
            {
                results.push(entry.path().to_path_buf());
            }
        }

        Ok(results)
    }

    pub fn read_file(&self, path: &PathBuf) -> Result<String> {
        Ok(std::fs::read_to_string(path)?)
    }

    pub fn get_current_dir(&self) -> &PathBuf {
        &self.current_dir
    }

    pub fn set_current_dir(&mut self, path: PathBuf) -> Result<()> {
        if path.exists() && path.is_dir() {
            self.current_dir = path;
            Ok(())
        } else {
            Err(anyhow::anyhow!("Invalid directory path"))
        }
    }
}
