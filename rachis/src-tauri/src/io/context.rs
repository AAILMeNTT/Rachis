/// Orchestrates content files and `.flight` metadata.
///
/// Each `FlightContext` is scoped to one Flight (project) directory. It owns:
/// - A [`ContentService`] for reading/writing content files on disk
/// - A [`Mutex<Connection>`](Connection) to the `.flight` metadata database
use rusqlite::{Connection, Error as RsqlError, Statement};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    fs::Metadata,
    io::Error as IoError,
    path::{Path, PathBuf},
    sync::{Mutex, MutexGuard},
};

use crate::entities::files::ProjectFiles;
use crate::io::content::ContentService;

/// Errors that can occur during FlightContext operations.
#[derive(Debug)]
pub enum FlightError {
    Db(RsqlError),
    Io(IoError),
    Custom(String),
}

impl Display for FlightError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            FlightError::Db(e) => write!(f, "Database error: {e}"),
            FlightError::Io(e) => write!(f, "File I/O error: {e}"),
            FlightError::Custom(msg) => write!(f, "Error: {msg}"),
        }
    }
}

impl Error for FlightError {}

impl From<RsqlError> for FlightError {
    fn from(e: RsqlError) -> Self {
        FlightError::Db(e)
    }
}

impl From<IoError> for FlightError {
    fn from(e: IoError) -> Self {
        FlightError::Io(e)
    }
}

/// The per-project coordinator between content files and project metadata.
///
/// # Fields
///
/// - `content`: [`ContentService`] - The service allowing I/O operations on content files
/// - `db`: [`Mutex<Connection>`] - A mutex-protected connection to the `.flight` metadata database
/// - `dir`: [`PathBuf`] - The project directory
pub struct FlightContext {
    /// The service allowing I/O operations on content files
    content_service: ContentService,
    /// A mutex-protected connection to the `.flight` metadata database
    db: Mutex<Connection>,
    /// The project directory
    dir: PathBuf,
}

impl FlightContext {
    /// Opens (or creates) a Flight in the given directory.
    ///
    /// This will:
    /// 1. Ensure the directory exists
    /// 2. Open (or create) the `.flight` SQLite database
    /// 3. Create the metadata tables if they don't exist
    /// 4. Create a `ContentService` scoped to this directory
    ///
    /// # Arguments
    ///
    /// - `dir` - The directory to open or create the Flight in
    pub fn open(project_dir: impl AsRef<Path>) -> Result<Self, FlightError> {
        let dir: PathBuf = project_dir.as_ref().to_path_buf();
        std::fs::create_dir_all(&dir)?;

        // Establish a connection to the `.flight` SQLite database
        let flight_db: Connection = Connection::open(dir.join(".flight"))?;

        // Enable WAL mode for concurrent reads/writes, along with optimised speed for WAL mode
        // More info about these here:
        // - https://sqlite.org/wal.html
        // - https://sqlite.org/pragma.html#pragma_journal_mode
        // - https://sqlite.org/pragma.html#pragma_synchronous
        flight_db.execute_batch(
            "
            PRAGMA journal_mode=WAL;
            PRAGMA synchronous=NORMAL;
            ",
        )?;

        // Create tables
        flight_db.execute_batch(
            "
            -- Stores cached entity information for faster lookups
            CREATE TABLE IF NOT EXISTS entity_cache (
                id TEXT PRIMARY KEY,
                file_path TEXT NOT NULL,
                tag_text TEXT NOT NULL,
                prefix TEXT,
                entity_name TEXT NOT NULL,
                display_text TEXT,
                lock_is_global INTEGER
            );

            -- Stores information about the files within the Flight project
            CREATE TABLE IF NOT EXISTS files (
                path TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                entity_type TEXT,
                word_count INTEGER DEFAULT 0,
                last_modified INTEGER NOT NULL,
                is_native BOOLEAN DEFAULT 0
            );

            -- Stores metadata about the Flight project
            CREATE TABLE IF NOT EXISTS flight_meta (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL
            );

            -- Stores the layout of the workspace, including open files and panes
            CREATE TABLE IF NOT EXISTS workspace_layouts (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL DEFAULT 'default',
                tree_json TEXT NOT NULL,
                saved_at INTEGER NOT NULL
            );
            ",
        )?;

        Ok(Self {
            content_service: ContentService::new(&dir),
            db: Mutex::new(flight_db),
            dir,
        })
    }

    /// Returns the project directory path.
    pub fn dir(&self) -> &Path {
        &self.dir
    }

    /// Scans the project directory for content files and indexes them in `.flight`.
    pub fn scan_directory(&self) -> Result<Vec<ProjectFiles>, FlightError> {
        let scanned: Vec<PathBuf> = self.content_service.scan_dir(".")?;

        #[cfg(test)]
        println!("{scanned:?}");

        let db: MutexGuard<'_, Connection> = self.db.lock().unwrap();

        for file_path in &scanned {
            // TODO: Consider a user option to include hidden files by default?
            // Skip all hidden files (including `.flight`)
            if file_path
                .file_name()
                .and_then(|s| s.to_str())
                .is_some_and(|s| s.starts_with('.'))
            {
                continue;
            }

            // Get the file name without the extension
            let title: String = file_path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("untitled")
                .into();
            // Get the file extension
            let ext: String = file_path
                .extension()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                // Might as well lowercase it GARRETT because someone probably does do that GARRETT
                .to_lowercase();

            let is_native: bool = ext == "rachis";

            // Get file modification time
            let metadata: Metadata = std::fs::metadata(file_path)?;
            let last_modified: i64 = metadata
                .modified()
                .ok()
                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|d| d.as_secs() as i64)
                .unwrap_or(0);

            db.execute(
                "INSERT OR IGNORE INTO files (path, title, entity_type, word_count, last_modified, is_native)
                 VALUES (?1, ?2, NULL, 0, ?3, ?4)",
                rusqlite::params![file_path.to_str(), title, last_modified, is_native],
            )?;
        }

        // Return the indexed files
        let mut stmt: Statement<'_> = db.prepare(
            "SELECT path, title, entity_type, word_count, last_modified, is_native FROM files ORDER BY path",
        )?;

        let results: Vec<ProjectFiles> = stmt
            .query_map([], |row| {
                Ok(ProjectFiles {
                    path: row.get(0)?,
                    title: row.get(1)?,
                    entity_type: row.get(2)?,
                    word_count: row.get(3)?,
                    last_modified: row.get(4)?,
                    is_native: row.get(5)?,
                })
            })?
            .collect::<Result<_, _>>()?;

        Ok(results)
    }

    /// Reads a content file from disk, relative to the project root.
    ///
    /// # Arguments
    ///
    /// - `rel_path` - The path of the file to read, relative to the project root
    ///
    /// # Returns
    ///
    /// - `Ok(String)` - The contents of the file
    /// - `Err` - An error if the file could not be read
    pub fn read_file(&self, rel_path: impl AsRef<Path>) -> Result<String, FlightError> {
        Ok(self.content_service.read(rel_path)?)
    }

    /// Writes content to a file on disk and updates `.flight` metadata.
    ///
    /// # Arguments
    ///
    /// - `rel_path` - The path of the file to write, relative to the project root
    /// - `content`: [`&str`](str) - The content to write to the file
    ///
    /// # Returns
    ///
    /// - `Ok(())` - The file was written successfully
    /// - `Err` - An error if the file could not be written
    pub fn write_file(&self, rel_path: impl AsRef<Path>, content: &str) -> Result<(), FlightError> {
        Ok({
            self.content_service.write(rel_path.as_ref(), content)?;
            self.update_file_metadata(rel_path, content)?;
        })
    }

    pub fn get_file_metadata(
        &self,
        rel_path: impl AsRef<Path>,
    ) -> Result<Option<ProjectFiles>, FlightError> {
        let rel_path: &Path = rel_path.as_ref();
        let db: MutexGuard<'_, Connection> = self.db.lock().unwrap();

        let mut stmt = db.prepare_cached("SELECT * FROM files WHERE path = ?1")?;

        let result: Result<Option<ProjectFiles>, RsqlError> =
            stmt.query_row([&rel_path.to_str()], |row| {
                Ok(Some(ProjectFiles {
                    path: row.get("path")?,
                    title: row.get("title")?,
                    entity_type: row.get("entity_type")?,
                    word_count: row.get("word_count")?,
                    last_modified: row.get("last_modified")?,
                    is_native: row.get("is_native")?,
                }))
            });

        match result {
            Ok(o) => Ok(o),
            Err(RsqlError::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(FlightError::Db(e)),
        }
    }

    /// Updates `.flight` database after writing content
    ///
    /// # Arguments
    ///
    /// - `path` - The path of the file to update
    /// - `content`: [`&str`](str) - The content of the file
    fn update_file_metadata(
        &self,
        path: impl AsRef<Path>,
        content: &str,
    ) -> Result<(), FlightError> {
        let path: &Path = path.as_ref();
        let db: MutexGuard<'_, Connection> = self.db.lock().unwrap();

        // Extract title from filename
        let title: String = path
            .file_stem()
            // If there's a file stem, convert it to a &str
            .and_then(|s| s.to_str())
            // If there's no file stem, use "untitled"
            .unwrap_or("untitled")
            .to_string();

        // Detect if native format
        let is_native: bool = path
            .extension()
            // If there's an extension, check if it's "rachis" (case-insensitive)
            .is_some_and(|ext| ext.eq_ignore_ascii_case("rachis"));

        // u32 because there is no fucking way anyone has four billion words in a single file
        let word_count: u32 = content.split_whitespace().count() as u32;
        // Get the current timestamp in seconds since Unix epoch
        let now: i64 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

        // Insert new paths + update old ones
        // You might note that entity_type is NULL; this is (will be) handled by the entity cache when i feel like getting around to it
        db.execute(
            "INSERT INTO files (path, title, entity_type, word_count, last_modified, is_native)
             VALUES (?1, ?2, NULL, ?3, ?4, ?5)
             ON CONFLICT(path) DO UPDATE SET
                title = excluded.title,
                word_count = excluded.word_count,
                last_modified = excluded.last_modified,
                is_native = excluded.is_native",
            rusqlite::params![path.to_str(), title, word_count, now, is_native],
        )?;

        // Also update the Flight's updated_at
        db.execute(
            "UPDATE flight_meta SET updated_at = ?1",
            rusqlite::params![now],
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{env, fs, process};

    fn make_test_dir() -> Result<PathBuf, IoError> {
        // because i can't run more than one test at a time if they use this function
        use std::sync::atomic::{AtomicU32, Ordering};

        // New unique directory for each test run according to the test and process ID
        static COUNTER: AtomicU32 = AtomicU32::new(0);
        let id: u32 = COUNTER.fetch_add(1, Ordering::Relaxed);

        // Format test directory name
        let dir: PathBuf = env::temp_dir().join(format!("rachis_test{}_{}", id, process::id()));
        println!("Temp dir: {:?}", dir);

        // Clear it if it (somehow) exists already
        // If you have a directory named this... sorry I guess
        match fs::remove_dir_all(&dir) {
            Ok(_) => (),
            Err(_) => println!("Directory doesn't exist"),
        }
        // And recreate it
        match fs::create_dir_all(&dir) {
            Ok(_) => println!("Created directory"),
            Err(e) => {
                println!("Unable to create dir: {:?}", e);
                return Err(IoError::from(e));
            }
        }
        Ok(dir)
    }

    fn add_file(
        dir: impl AsRef<Path>,
        name: &str,
        ext: &str,
        contents: &str,
    ) -> Result<(), IoError> {
        Ok(fs::write(
            dir.as_ref().join(format!("{}.{}", name, ext)),
            contents,
        )?)
    }

    fn add_dir(dir: impl AsRef<Path>) -> Result<PathBuf, IoError> {
        let path = dir.as_ref();
        fs::create_dir_all(path)?;
        Ok(path.to_path_buf())
    }

    #[test]
    fn test_open_creates_tables() -> Result<(), FlightError> {
        let dir: PathBuf = make_test_dir()?;

        let ctx: FlightContext = FlightContext::open(&dir).expect("Failed to open FlightContext");
        let db: MutexGuard<'_, Connection> = ctx.db.lock().unwrap();

        // Verify all tables exist
        let mut stmt: Statement<'_> =
            db.prepare("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name")?;
        let query = stmt.query_map([], |row| row.get(0))?;
        let tables: Vec<String> = query.collect::<Result<Vec<_>, _>>()?;
        println!("Tables: {:#?}", tables);
        assert!(tables.contains(&"entity_cache".to_string()));
        assert!(tables.contains(&"files".to_string()));
        assert!(tables.contains(&"flight_meta".to_string()));
        assert!(tables.contains(&"workspace_layouts".to_string()));

        fs::remove_dir_all(&dir)?;
        Ok(())
    }

    #[test]
    fn test_scan_directory() -> Result<(), IoError> {
        let flight_dir: PathBuf = make_test_dir()?;

        // Create some test files
        add_file(&flight_dir, "Hello", "rachis", "World")?;
        add_file(&flight_dir, "notes", "md", "# Markdown Notes")?;
        let characters: PathBuf = add_dir(&flight_dir.join("characters"))?;
        add_file(
            &characters,
            "Twilight Sparkle",
            "rachis",
            "dear princess celestia i don't know how to read",
        )?;

        let ctx: FlightContext =
            FlightContext::open(&flight_dir).expect("Failed to open FlightContext");
        let files: Vec<ProjectFiles> = ctx.scan_directory().expect("Failed to scan directory");

        // Should find 3 files (skipping .flight)
        println!("Found {:#?} files: {:#?}", files.len(), files);
        assert_eq!(files.len(), 3);

        // Check that is_native is correct
        let (rachis_files, nonrachis_files): (Vec<&ProjectFiles>, Vec<&ProjectFiles>) =
            files.iter().partition(|f| f.is_native);
        println!(
            "Found {:#?} native files: {:#?}",
            rachis_files.len(),
            rachis_files
        );
        println!(
            "Found {:#?} non-native files: {:#?}",
            nonrachis_files.len(),
            nonrachis_files
        );
        assert_eq!(rachis_files.len(), 2);
        assert_eq!(nonrachis_files.len(), 1);

        fs::remove_dir_all(&flight_dir)?;
        Ok(())
    }

    #[test]
    fn test_write_file_creates_metadata() -> Result<(), FlightError> {
        // Create a directory and add some random files to it
        let dir: PathBuf = make_test_dir()?;
        add_file(&dir, "Hello", "rachis", "")?;
        // Print all files in directory
        // fs::read_dir(&dir)?.for_each(|e| println!("Found: {e:?}"));

        // (Create and) Establish a connection to the Flight database
        let ctx: FlightContext = FlightContext::open(&dir).expect("Failed to open FlightContext");
        // Add contents to the file and save metadata
        ctx.write_file("Hello.rachis", "World")?;

        // Get the metadata (hopefully it updated)
        let metadata: Option<ProjectFiles> = ctx
            .get_file_metadata("Hello.rachis")
            .expect("Failed to get file metadata");
        println!("Found metadata: {metadata:#?}");
        assert!(metadata.is_some());

        let m: &ProjectFiles = &metadata.unwrap();
        assert_eq!(m.path, "Hello.rachis");
        assert_eq!(m.title, "Hello");
        assert_eq!(m.entity_type, None);
        assert_eq!(m.word_count, 1);
        assert_eq!(m.is_native, true);
        Ok(())
    }

    #[test]
    fn test_write_file_upserts_instead_of_duplicating() -> Result<(), FlightError> {
        let dir: PathBuf = make_test_dir()?;
        add_file(&dir, "Whom It May Concern", "rachis", "")?;

        // (Create and) Establish a connection to the Flight database
        let ctx: FlightContext = FlightContext::open(&dir).expect("Failed to open FlightContext");
        // Add contents to the file and save metadata
        ctx.write_file(
            "Whom It May Concern.rachis",
            "TODO: Write the whole damn chapter",
        )?;
        // Get the metadata
        let metadata1: Option<ProjectFiles> = ctx
            .get_file_metadata("Whom It May Concern.rachis")
            .expect("Failed to get file metadata");
        println!("Found metadata: {metadata1:#?}");
        assert!(metadata1.is_some());

        let m: &ProjectFiles = &metadata1.unwrap();
        assert_eq!(m.path, "Whom It May Concern.rachis");
        assert_eq!(m.title, "Whom It May Concern");
        assert_eq!(m.entity_type, None);
        assert_eq!(m.word_count, 6); // SIX
        assert_eq!(m.is_native, true);

        // Wanna see me do it again?
        ctx.write_file(
            "Whom It May Concern.rachis",
            "For one, I think you're onto something.",
        )?;

        // Get the metadata again
        let metadata2: Option<ProjectFiles> = ctx
            .get_file_metadata("Whom It May Concern.rachis")
            .expect("Failed to get file metadata");
        println!("Found metadata: {metadata2:#?}");
        assert!(metadata2.is_some());

        let m: &ProjectFiles = &metadata2.unwrap();
        assert_eq!(m.word_count, 7); // SEVEEEEEEEEEEEEEEEEEN

        // And now verify row count
        let db: MutexGuard<'_, Connection> = ctx.db.lock().unwrap();
        let count: i32 = db.query_row(
            "SELECT COUNT(*) FROM files WHERE path = ?1",
            rusqlite::params!["Whom It May Concern.rachis"],
            |row| row.get(0),
        )?;
        assert_eq!(count, 1, "UPSERT should not create duplicate rows");

        Ok(())
    }

    #[test]
    fn test_write_file_updates_flight_timestamp() {}

    #[test]
    fn test_read_file() {}

    #[test]
    fn test_scan_directory_skips_hidden() {}

    #[test]
    fn test_scan_directory_subdirs() {}

    #[test]
    fn test_open_existing_flight() {}

    #[test]
    fn test_open_nonexistent_dir() {}
}
