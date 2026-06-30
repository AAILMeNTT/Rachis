use std::fs;
use std::io::Error as IoError;
use std::path::{Path, PathBuf};

pub struct ContentService {
    flight_dir: PathBuf,
}

impl ContentService {
    /// Creates a new [`ContentService`] with the given Flight directory.
    ///
    /// # Fields
    ///
    /// - `flight_dir`: [`Path`](Path) - The flight directory to use for file operations.
    pub fn new(flight_dir: &Path) -> Self {
        Self {
            flight_dir: flight_dir.to_path_buf(),
        }
    }

    /// Reads file contents from the given relative path.
    ///
    /// # Fields
    ///
    /// - `path`: [`&str`](str) - The relative path to the file to read.
    pub fn read(&self, rel_path: impl AsRef<Path>) -> Result<String, IoError> {
        fs::read_to_string(self.flight_dir.join(rel_path))
    }

    /// Writes file contents to the given relative path.
    ///
    /// # Fields
    ///
    /// - `path`: [`&str`](str) - The relative path to the file to write.
    /// - `contents`: [`&str`](str) - The contents to write to the file.
    pub fn write(&self, rel_path: impl AsRef<Path>, contents: &str) -> Result<(), IoError> {
        fs::write(self.flight_dir.join(rel_path), contents)
    }

    pub fn scan_dir(&self, rel_path: impl AsRef<Path>) -> Result<Vec<PathBuf>, IoError> {
        // Recursively scan the directory and return a list of file paths
        let path: &Path = rel_path.as_ref();
        let mut files: Vec<PathBuf> = Vec::new();
        for entry in fs::read_dir(self.flight_dir.join(path))? {
            let entry = entry?;
            let path: PathBuf = entry.path();
            match path.is_dir() {
                true => files.extend(self.scan_dir(path)?),
                false => files.push(path),
            }
        }
        Ok(files)
    }
}

enum FileTypes {
    Rachis,
    Markdown,
    Bbcode,
    Html,
    Asciidoc,
}

impl FileTypes {
    /// Returns the file type for the given extension.
    fn from_ext(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "rachis" => Some(FileTypes::Rachis),
            "md" => Some(FileTypes::Markdown),
            "bbcode" => Some(FileTypes::Bbcode),
            "html" => Some(FileTypes::Html),
            "adoc" | "asciidoc" => Some(FileTypes::Asciidoc),
            _ => None,
        }
    }

    /// Returns the file extension for this file type.
    fn as_ext(&self) -> &str {
        match self {
            FileTypes::Rachis => "rachis",
            FileTypes::Markdown => "md",
            FileTypes::Bbcode => "bbcode",
            FileTypes::Html => "html",
            FileTypes::Asciidoc => "adoc",
        }
    }
}
