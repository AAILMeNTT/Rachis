use std::{
    fs,
    io::Error as IoError,
    path::{Path, PathBuf},
};

pub struct ContentService {
    flight_dir: PathBuf,
}

impl ContentService {
    /// Creates a new [`ContentService`] with the given Flight directory.
    ///
    /// # Fields
    ///
    /// - `flight_dir`: [`Path`] - The flight directory to use for file operations.
    pub fn new(flight_dir: &Path) -> Self {
        Self {
            flight_dir: flight_dir.to_path_buf(),
        }
    }

    pub fn new_file(
        &self,
        rel_path: impl AsRef<Path>,
        content: impl AsRef<str>,
    ) -> Result<(), IoError> {
        let rel_path: &Path = rel_path.as_ref();
        // Ensure parent directories exist
        fs::create_dir_all(self.flight_dir.join(rel_path.parent().unwrap()))?;
        // Create the file with the title and its contents
        self.write_file(rel_path, content)
    }

    /// Reads file contents from the given relative path.
    ///
    /// # Fields
    ///
    /// - `rel_path`: [`impl AsRef<Path>`](Path) - The relative path to the file to read.
    pub fn read_file(&self, rel_path: impl AsRef<Path>) -> Result<String, IoError> {
        fs::read_to_string(self.flight_dir.join(rel_path))
    }

    /// Writes file contents to the given relative path.
    ///
    /// # Fields
    ///
    /// - `rel_path`: [`impl AsRef<Path>`](Path) - The relative path to the file to write.
    /// - `content`: [`impl AsRef<str>`](str) - The contents to write to the file.
    pub fn write_file(
        &self,
        rel_path: impl AsRef<Path>,
        content: impl AsRef<str>,
    ) -> Result<(), IoError> {
        fs::write(self.flight_dir.join(rel_path), content.as_ref())
    }
}

pub enum FileType {
    Rachis,
    Markdown,
    Bbcode,
    Html,
    AsciiDoc,
    RichText,
}

impl FileType {
    /// Returns the file type for the given title, which may or may not have an extension.
    ///
    /// # Arguments
    ///
    /// - `title` - The title of the file, which may or may not have an extension.
    pub fn from_title(title: impl AsRef<str>) -> Option<Self> {
        let title: &str = title.as_ref();

        // Find the last "." and compare that with the list of valid extensions
        match title.rfind('.') {
            Some(i) => Self::from_ext(&title[i + 1..]),
            None => None,
        }
    }

    /// Returns the file type for the given extension.
    pub fn from_ext(ext: impl AsRef<str>) -> Option<Self> {
        match ext.as_ref().to_lowercase().as_str() {
            "rachis" => Some(FileType::Rachis),
            "md" => Some(FileType::Markdown),
            "bbcode" => Some(FileType::Bbcode),
            "html" => Some(FileType::Html),
            "adoc" | "asciidoc" => Some(FileType::AsciiDoc),
            "rtf" => Some(FileType::RichText),
            _ => None,
        }
    }

    /// Returns the file extension for this file type.
    pub fn as_ext(&self) -> &'static str {
        match self {
            FileType::Rachis => "rachis",
            FileType::Markdown => "md",
            FileType::Bbcode => "bbcode",
            FileType::Html => "html",
            FileType::AsciiDoc => "adoc",
            FileType::RichText => "rtf",
        }
    }
}
