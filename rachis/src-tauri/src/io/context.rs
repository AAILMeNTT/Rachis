use {
    crate::{
        domain::rachis::{Rachis, RachisType},
        entities::{files::ProjectFile, flight_meta::FlightMetadata},
        io::{
            content::{ContentService, FileType},
            finder::Finder,
        },
        tree::Tree,
    },
    chrono::Utc,
    rusqlite::{Connection, Error as RsqlError, Statement, params},
    std::{
        error::Error,
        fmt::{Display, Formatter, Result as FmtResult},
        fs::{self, Metadata},
        io::Error as IoError,
        path::{Path, PathBuf},
        sync::{Mutex, MutexGuard},
        time::UNIX_EPOCH,
    },
    uuid::Uuid,
    walkdir::Error as WalkError,
};

/// Errors that can occur during FlightContext operations.
#[derive(Debug)]
pub enum FlightError {
    Db(RsqlError),
    Io(IoError),
    Walk(WalkError),
    Custom(String),
    Json(serde_json::Error),
}

impl Display for FlightError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            FlightError::Db(e) => write!(f, "Database error: {e}"),
            FlightError::Io(e) => write!(f, "File I/O error: {e}"),
            FlightError::Walk(e) => write!(f, "Walk error: {e}"),
            FlightError::Json(e) => write!(f, "JSON error: {e}"),
            FlightError::Custom(msg) => write!(f, "Error: {msg}"),
        }
    }
}

impl Error for FlightError {}

impl From<serde_json::Error> for FlightError {
    fn from(e: serde_json::Error) -> Self {
        FlightError::Json(e)
    }
}

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

impl From<WalkError> for FlightError {
    fn from(e: WalkError) -> Self {
        FlightError::Walk(e)
    }
}

/// The per-project coordinator between content files and project metadata.
///
/// # Fields
///
/// - `content`: [`ContentService`] - The service allowing I/O operations on content files
/// - `db`: [`Mutex<Connection>`] - A mutex-protected connection to the `.flight` metadata database
/// - `dir`: [`PathBuf`] - The project directory
/// - `tree`: [`Option<Tree>`](Tree) - The workspace tree
pub struct FlightContext {
    /// The service allowing I/O operations on content files
    content_service: ContentService,
    /// A mutex-protected connection to the `.flight` metadata database
    db: Mutex<Connection>,
    /// The project directory
    dir: PathBuf,
    /// The workspace tree
    tree: Option<Tree>,
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
    /// - `flight_dir`: [`impl AsRef<Path>`](Path) - The directory to open or create the Flight in
    /// - `flight_name`: [`impl AsRef<str>`](str) - The name of the Flight to open or create
    pub fn open_conn(
        flight_dir: impl AsRef<Path>,
        flight_name: impl AsRef<str>,
    ) -> Result<Self, FlightError> {
        let (flight_dir, flight_name) = (flight_dir.as_ref().to_path_buf(), flight_name.as_ref());

        // Check if path exists already
        if !flight_dir.exists() {
            #[cfg(test)]
            println!("Path does not exist; creating: {flight_dir:#?}");

            fs::create_dir_all(&flight_dir)?;
        }

        #[cfg(test)]
        println!("Opening connection to {flight_dir:#?}");

        // Establish a connection to the `{name}.flight` SQLite database
        let flight_db: Connection =
            Connection::open(flight_dir.join(format!("{flight_name}.flight")))?;

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
                id TEXT PRIMARY KEY,
                path TEXT NOT NULL UNIQUE,
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
            content_service: ContentService::new(&flight_dir),
            db: Mutex::new(flight_db),
            dir: flight_dir,
            tree: None,
        })
    }

    /// Returns the project directory path.
    pub fn dir(&self) -> &Path {
        &self.dir
    }

    pub fn tree(&self) -> Option<&Tree> {
        self.tree.as_ref()
    }

    pub fn tree_mut(&mut self) -> Option<&mut Tree> {
        self.tree.as_mut()
    }

    /// Scans the project directory for content files and indexes them in `.flight`.
    pub fn scan_directory(&self) -> Result<Vec<ProjectFile>, FlightError> {
        let db: MutexGuard<'_, Connection> = self.db.lock().unwrap();

        // Skip all hidden entries and anything included
        let files: Vec<PathBuf> = Finder::new(self.dir())
            .skip_hidden()
            .exclude_pattern(r".*.flight(?:-(wal|shm))?$")
            .find()?
            .files;

        for file_path in files {
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
            let metadata: Metadata = fs::metadata(&file_path)?;
            let last_modified: i64 = metadata
                .modified()
                .ok()
                .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
                .map(|d| d.as_secs() as i64)
                .unwrap_or(0);

            // Convert the absolute path to a project-relative path for storage
            let rel_path: &Path = file_path.strip_prefix(self.dir()).unwrap_or(&file_path);

            // Check if this file path already exists in the DB to preserve its UUID
            let existing_id: Option<String> = db
                .query_row(
                    "SELECT id FROM files WHERE path = ?1",
                    params![rel_path.to_str()],
                    |row| row.get(0),
                )
                .ok();

            let file_id: String = match existing_id {
                Some(id) => id,
                None => Uuid::new_v4().to_string(),
            };

            db.execute(
                "INSERT INTO files (id, path, title, entity_type, word_count, last_modified, is_native)
                 VALUES (?1, ?2, ?3, NULL, 0, ?4, ?5)
                 ON CONFLICT(id) DO UPDATE SET
                    path = excluded.path,
                    title = excluded.title,
                    last_modified = excluded.last_modified,
                    is_native = excluded.is_native",
                params![file_id, rel_path.to_str(), title, last_modified, is_native],
            )?;
        }

        // Return the indexed files
        let mut stmt: Statement<'_> = db.prepare(
            "SELECT id, path, title, entity_type, word_count, last_modified, is_native FROM files ORDER BY path",
        )?;

        let results: Vec<ProjectFile> = stmt
            .query_map(params![], |row| {
                Ok(ProjectFile {
                    id: row.get("id")?,
                    path: row.get("path")?,
                    title: row.get("title")?,
                    entity_type: row.get("entity_type")?,
                    word_count: row.get("word_count")?,
                    last_modified: row.get("last_modified")?,
                    is_native: row.get("is_native")?,
                })
            })?
            .collect::<Result<Vec<ProjectFile>, _>>()?;

        Ok(results)
    }

    /// Returns metadata for a single file by its stable UUID.
    ///
    /// # Arguments
    ///
    /// - `file_id`: [`impl AsRef<Uuid>`](Uuid) - The UUID of the file to look up
    ///
    /// # Returns
    ///
    /// - [`Ok(ProjectFile)`](ProjectFile) - The file metadata
    /// - [`Err(FlightError::Db)`](FlightError) - If the file ID is not found
    pub fn get_file_metadata_by_id(
        &self,
        file_id: impl AsRef<Uuid>,
    ) -> Result<ProjectFile, FlightError> {
        let db: MutexGuard<'_, Connection> = self.db.lock().unwrap();
        let mut stmt = db.prepare_cached(
            "SELECT id, path, title, entity_type, word_count, last_modified, is_native FROM files WHERE id = ?1",
        )?;

        stmt.query_row(params![file_id.as_ref().to_string()], |row| {
            Ok(ProjectFile {
                id: row.get("id")?,
                path: row.get("path")?,
                title: row.get("title")?,
                entity_type: row.get("entity_type")?,
                word_count: row.get("word_count")?,
                last_modified: row.get("last_modified")?,
                is_native: row.get("is_native")?,
            })
        })
        .map_err(|e: RsqlError| FlightError::Db(e))
    }

    /// Updates `.flight` database after writing content.
    ///
    /// Takes a stable file UUID and the path on disk so that metadata remains
    /// correctly associated even if the file is externally renamed or moved.
    ///
    /// # Arguments
    ///
    /// - `file_id`: [`impl AsRef<Uuid>`](Uuid) - The stable UUID of the file
    /// - `path`: [`impl AsRef<Path>`](Path) - The current path of the file on disk, relative to the project root
    /// - `content`: [`impl AsRef<str>`](str) - The full text content of the file
    ///
    /// # Returns
    ///
    /// - [`Ok(ProjectFile)`](ProjectFile) - The updated file metadata
    /// - [`Err(FlightError)`](FlightError) - An error if the metadata could not be updated
    fn update_file_metadata(
        &self,
        file_id: impl AsRef<Uuid>,
        path: impl AsRef<Path>,
        content: impl AsRef<str>,
    ) -> Result<ProjectFile, FlightError> {
        let (file_id, path, content) = (file_id.as_ref(), path.as_ref(), content.as_ref());

        // Extract title from filename
        let title: String = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("untitled")
            .to_string();

        // Detect if native format
        let is_native: bool = path
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("rachis"));

        // u32 because there is no fucking way anyone has four billion words in a single file
        let word_count: u32 = content.split_whitespace().count() as u32;
        let now: i64 = Utc::now().timestamp();

        {
            // Insert new paths + update old ones
            // You might note that entity_type is NULL; this is (will be) handled by the entity cache (when i feel like getting around to it)
            // TODO: handle entity cache when i get around to it
            let db: MutexGuard<'_, Connection> = self.db.lock().unwrap();
            db.execute(
                "INSERT INTO files (id, path, title, entity_type, word_count, last_modified, is_native)
                VALUES (?1, ?2, ?3, NULL, ?4, ?5, ?6)
                ON CONFLICT(id) DO UPDATE SET
                    path = excluded.path,
                    title = excluded.title,
                    word_count = excluded.word_count,
                    last_modified = excluded.last_modified,
                    is_native = excluded.is_native",
                params![file_id.to_string(), path.to_str(), title, word_count, now, is_native],
            )?;

            db.execute("UPDATE flight_meta SET updated_at = ?1", params![now])?;
        }

        // Fetch and return the full metadata row
        self.get_file_metadata_by_id(file_id)
    }

    /// Initialises the metadata for a Flight, including its name and creation timestamp.
    ///
    /// # Arguments
    ///
    /// - `flight_id`: [`impl AsRef<Uuid>`](Uuid) - The ID of the Flight to initialise metadata for.
    /// - `flight_name`: [`impl AsRef<str>`](str) - The name of the Flight.
    ///
    /// # Returns
    ///
    /// - [`Ok(FlightMetadata)`](FlightMetadata) - The initialised Flight metadata.
    /// - [`Err(FlightError::Db)`](FlightError) - An error occurred during initialisation.
    pub fn init_flight_metadata(
        &self,
        flight_id: impl AsRef<Uuid>,
        flight_name: impl AsRef<str>,
    ) -> Result<FlightMetadata, FlightError> {
        let (flight_id, flight_name) = (flight_id.as_ref(), flight_name.as_ref());

        let now: i64 = Utc::now().timestamp();
        {
            let db: MutexGuard<'_, Connection> = self.db.lock().unwrap();
            db.execute(
                "INSERT INTO flight_meta (id, name, created_at, updated_at)
                VALUES (?1, ?2, ?3, ?4)
                ON CONFLICT(id) DO UPDATE SET
                    -- Only update these values
                    name = excluded.name,
                    updated_at = excluded.updated_at",
                params![flight_id.to_string(), flight_name, now, now],
            )?;
        }

        match self.get_flight_metadata(flight_id)? {
            Some(metadata) => Ok(metadata),
            None => Err(FlightError::Custom("Flight metadata not found".to_string())),
        }
    }

    /// Returns the metadata for a Flight, including its files.
    ///
    /// # Arguments
    ///
    /// - `flight_id`: [`impl AsRef<Uuid>`](Uuid) - The ID of the Flight to retrieve metadata for.
    ///
    /// # Returns
    ///
    /// - [`Ok(metadata)`](FlightMetadata) - The Flight metadata. May be `None` if no metadata is found.
    /// - [`Err(FlightError::Db(error))`](FlightError) - An error occurred while retrieving metadata.
    pub fn get_flight_metadata(
        &self,
        flight_id: impl AsRef<Uuid>,
    ) -> Result<Option<FlightMetadata>, FlightError> {
        let flight_id: &Uuid = flight_id.as_ref();

        let db: MutexGuard<'_, Connection> = self.db.lock().unwrap();
        let mut meta_stmt = db.prepare_cached("SELECT * FROM flight_meta WHERE id = ?1")?;
        let mut files_stmt = db.prepare_cached("SELECT * FROM files")?;

        let files_result: Vec<ProjectFile> = files_stmt
            .query_map(params![], |row| {
                Ok(ProjectFile {
                    id: row.get(0)?,
                    path: row.get(1)?,
                    title: row.get(2)?,
                    entity_type: row.get(3)?,
                    word_count: row.get(4)?,
                    last_modified: row.get(5)?,
                    is_native: row.get(6)?,
                })
            })?
            .collect::<Result<Vec<ProjectFile>, RsqlError>>()?;

        let meta_result: Result<Option<FlightMetadata>, RsqlError> =
            meta_stmt.query_row(params![flight_id.to_string()], |row| {
                Ok(Some(FlightMetadata {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    created_at: row.get(2)?,
                    updated_at: row.get(3)?,
                    files: files_result,
                }))
            });

        match meta_result {
            Ok(o) => Ok(o),
            Err(RsqlError::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(FlightError::Db(e)),
        }
    }

    /// Checks if a directory contains a `.flight` file and returns its UUID.
    ///
    /// Does not create a `.flight` file if one doesn't exist; used by the
    /// registry solely to reconcile moved Flights.
    ///
    /// # Arguments
    ///
    /// - `path`: [`impl AsRef<Path>`](Path) - The project directory to check for a `.flight` file.
    ///
    /// # Returns
    ///
    /// - `Ok(Some(Uuid))` — The directory has a `.flight` with this UUID
    /// - `Ok(None)` — No `.flight` file exists, or the file has no metadata yet
    /// - `Err(FlightError)` — The `.flight` file exists but couldn't be read
    /// Reads metadata from a `.flight` SQLite file in the given directory.
    ///
    /// # Arguments
    ///
    /// - `dir`: [`impl AsRef<Path>`](Path) — The **directory** containing the `.flight` file (not the file itself).
    /// - `name`: [`impl AsRef<str>`](str) — The Flight's display name; used to construct `{dir}/{name}.flight`.
    ///
    /// # Returns
    ///
    /// - `Ok(Some(FlightMetadata))` — If the `.flight` file exists and has metadata.
    /// - `Ok(None)` — If no `.flight` file exists at `{dir}/{name}.flight`.
    /// - `Err(FlightError)` — If the file exists but couldn't be read.
    pub fn read_flight_metadata(
        dir: impl AsRef<Path>,
        name: impl AsRef<str>,
    ) -> Result<Option<FlightMetadata>, FlightError> {
        let flight_path: PathBuf = dir.as_ref().join(format!("{}.flight", name.as_ref()));

        // If the file doesn't exist, there's nothing to check
        if !flight_path.exists() {
            return Ok(None);
        }

        let flight_db: Connection = Connection::open(flight_path)?;
        let mut stmt = flight_db
            .prepare_cached("SELECT id, name, created_at, updated_at FROM flight_meta LIMIT 1")?;

        match stmt.query_row(params![], |row| {
            Ok(FlightMetadata::new(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                Vec::new(),
            ))
        }) {
            Ok(meta) => Ok(Some(meta)),
            Err(RsqlError::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(FlightError::Db(e)),
        }
    }

    /// Creates a new Rachis file in the Flight directory.
    ///
    /// This method handles:
    /// 1. Deriving a filesystem-safe filename from the title
    /// 2. Mapping the [`RachisType`] to a subdirectory (entity types like CHARACTER
    ///    get their own folder; generic types live at root)
    /// 3. Writing the file to disk via [`ContentService`]
    /// 4. Indexing it in `.flight` metadata
    ///
    /// # Arguments
    ///
    /// - `title`: [`impl AsRef<str>`](str) - The display title of the Rachis. Used to derive the filename.
    /// - `file_type`: [`FileType`] - The type of the file to create.
    /// - `r#type`: [`RachisType`] - The entity type, which affects the subdirectory.
    /// - `content`: [`impl AsRef<str>`](str) - The initial content to write to the file.
    ///
    /// # Returns
    ///
    /// [`Ok(Rachis)`](Rachis) - A read-only payload describing the created file.
    /// [`Err(FlightError::Io)`](FlightError) - An error if the file could not be created.
    /// [`Err(FlightError::Io)`](FlightError) - An error if the file could not be created.
    pub fn create_file(
        &self,
        title: impl AsRef<str>,
        file_type: FileType,
        r#type: RachisType,
        content: impl AsRef<str>,
    ) -> Result<Rachis, FlightError> {
        let (title, content) = (title.as_ref(), content.as_ref());
        // Map entity types to their subdirectories and banish the others to the root
        let subdir: &str = match r#type {
            RachisType::CHARACTER => "Characters",
            RachisType::EVENT => "Events",
            RachisType::LOCATION => "Locations",
            RachisType::ITEM => "Items",
            RachisType::NOTE => "Notes",
            RachisType::ACT | RachisType::ARC | RachisType::SCENE | RachisType::DEFAULT => "",
        };

        // Derive filename
        let filename: String = sanitize(title) + file_type.as_ext();

        // Build the relative path
        let rel_path: PathBuf = PathBuf::from(subdir).join(filename);

        // Generate a stable UUID for this file before writing
        let file_id: Uuid = Uuid::new_v4();

        // Ensure parent directory exists (e.g. "Characters/" for entity types)
        if let Some(parent) = rel_path.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(self.dir().join(parent))?;
            }
        }

        // Write content to disk
        self.content_service.write_file(&rel_path, &content)?;

        // Index in `.flight` (uses the UUID we generated, not path-based)
        self.update_file_metadata(file_id, &rel_path, content)?;

        // TODO: Read flight_id from flight_meta once FlightContext exposes it
        let flight_id: Uuid = Uuid::nil();

        Ok(Rachis::new(
            file_id,
            flight_id,
            title,
            r#type,
            rel_path,
            // TODO: Extract c!...! tags from content
            Vec::<&str>::new(),
            // TODO: Compute word count from content
            0,
        ))
    }

    /// Saves content to an existing file identified by its stable UUID.
    ///
    /// Looks up the file's current path from the `.flight` index, writes the
    /// new content to disk, and updates metadata.
    ///
    /// # Arguments
    ///
    /// - `file_id`: [`impl AsRef<Uuid>`](Uuid) - The stable UUID of the file to save
    /// - `content`: [`impl AsRef<str>`](str) - The new full text content
    ///
    /// # Returns
    ///
    /// [`ProjectFile`] - The updated file metadata
    pub fn save_file(
        &self,
        file_id: impl AsRef<Uuid>,
        content: impl AsRef<str>,
    ) -> Result<ProjectFile, FlightError> {
        let file: ProjectFile = self.get_file_metadata_by_id(&file_id)?;
        let path: &Path = Path::new(&file.path);

        self.content_service.write_file(path, &content)?;
        self.update_file_metadata(file_id, path, content)
    }

    /// Queries the `workspace_layouts` table for a workspace of some name
    ///
    /// # Arguments
    ///
    /// - `id`: [`impl AsRef<Uuid>`](Uuid) - The stable UUID of the workspace layout
    ///
    /// # Returns
    ///
    /// [`Ok(Some(Tree))`](Tree) - The loaded workspace tree, if one exists
    /// `Ok(None)` - No workspace tree exists for the given ID
    /// [`Err(FlightError)`](FlightError) - An error occurred while loading the layout
    pub fn load_layout(&mut self, id: impl AsRef<Uuid>) -> Result<Option<Tree>, FlightError> {
        let tree_json: Option<String> = {
            let db = self.db.lock().unwrap();
            let mut stmt =
                db.prepare_cached("SELECT tree_json FROM workspace_layouts WHERE id = ?1")?;
            stmt.query_row(params![id.as_ref().to_string()], |row| row.get("tree_json"))
                .ok()
        };

        Ok(tree_json.map_or(None, |json| {
            let tree: Tree = serde_json::from_str(&json).ok()?;
            self.tree = Some(tree.clone());
            Some(tree)
        }))
    }

    pub fn save_layout(
        &mut self,
        id: impl AsRef<Uuid>,
        name: impl AsRef<str>,
    ) -> Result<(), FlightError> {
        let tree_json: String = serde_json::to_string(&self.tree)?;
        let db: MutexGuard<'_, Connection> = self.db.lock().unwrap();
        let mut stmt = db.prepare_cached(
            "INSERT INTO workspace_layouts (id, name, tree_json) VALUES (?1, ?2, ?3)
            ON CONFLICT (id) DO UPDATE SET name = ?2, tree_json = ?3",
        )?;
        stmt.execute(params![
            id.as_ref().to_string(),
            name.as_ref().to_string(),
            tree_json
        ])?;
        Ok(())
    }
}

/// Ensures a filename does not contain characters forbidden by common
/// operating systems.
///
/// Only removes characters that are invalid on Windows, macOS, and Linux (`\ /
/// : * ? " < > |` and NUL, whatever the hell that is)
///
/// # Arguments
///
/// - `title`: [`impl AsRef<str>`](str) - The filename to sanitise.
fn sanitize(title: impl AsRef<str>) -> String {
    title
        .as_ref()
        .chars()
        // Filter out evil nefarious characters that will kill you
        // .filter(|c: &char| {
        //     !matches!(
        //         c,
        //         '\\' | '/' | ':' | '*' | '?' | '"' | '<' | '>' | '|' | '\0'
        //     )
        // })
        // Instead of filtering out evil nefarious characters that will kill you, replace them with exceptionally kind spaces that will give out candy
        .filter_map(|c| {
            Some(match c {
                '\\' | '/' | ':' | '*' | '?' | '"' | '<' | '>' | '|' | '\0' => ' ',
                _ => c,
            })
        })
        .collect::<String>()
        // Split by whitespace because now there could still be evil nefarious stacked whitespaces that will kill you
        .split_whitespace()
        .collect::<Vec<&str>>()
        // Join by whitespace because there are now no more evil nefarious characters in this string that will kill you
        .join(" ")

    // TODO: perhaps add a user setting to enforce/not enforce this in case they know they aren't gonna be switching OSs at any point in time... but if i do add in collaboration this will have to be forced on just for safety me thinkies
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{env, fs, process, thread, time::Duration};

    fn make_test_dir() -> Result<PathBuf, IoError> {
        // because i can't run more than one test at a time if they use this function
        use std::sync::atomic::{AtomicU32, Ordering};

        // New unique directory for each test run according to the test and process ID
        static COUNTER: AtomicU32 = AtomicU32::new(0);
        let id: u32 = COUNTER.fetch_add(1, Ordering::Relaxed);

        // Format test directory name
        let dir: PathBuf = env::temp_dir().join(format!("rachis_test{id}_{}", process::id()));
        println!("Temp dir: {dir:#?}");

        // Clear it if it (somehow) exists already
        // If you have a directory named this... sorry I guess
        if dir.exists() {
            fs::remove_dir_all(&dir)?;
        }
        // And recreate it
        fs::create_dir_all(&dir)?;

        Ok(dir)
    }

    fn add_file(
        dir: impl AsRef<Path>,
        name: impl AsRef<str>,
        ext: impl AsRef<str>,
        contents: impl AsRef<str>,
    ) -> Result<(), IoError> {
        let (name, ext, contents) = (name.as_ref(), ext.as_ref(), contents.as_ref());

        Ok(fs::write(
            dir.as_ref().join(format!("{name}.{ext}")),
            contents,
        )?)
    }

    fn add_dir(dir: impl AsRef<Path>) -> Result<PathBuf, IoError> {
        fs::create_dir_all(&dir)?;
        Ok(dir.as_ref().to_path_buf())
    }

    #[test]
    fn test_open_creates_tables() -> Result<(), FlightError> {
        let dir: PathBuf = make_test_dir()?;

        let ctx: FlightContext = FlightContext::open_conn(&dir, "TestOpenCreatesTables")
            .expect("Failed to open FlightContext");
        let db: MutexGuard<'_, Connection> = ctx.db.lock().unwrap();

        // Verify all tables exist
        let mut stmt: Statement<'_> =
            db.prepare("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name")?;
        let query = stmt.query_map(params![], |row| row.get(0))?;
        let tables: Vec<String> = query.collect::<Result<Vec<String>, _>>()?;
        println!("Tables: {tables:#?}");
        assert!(tables.contains(&"entity_cache".to_string()));
        assert!(tables.contains(&"files".to_string()));
        assert!(tables.contains(&"flight_meta".to_string()));
        assert!(tables.contains(&"workspace_layouts".to_string()));

        fs::remove_dir_all(dir)?;
        Ok(())
    }

    #[test]
    fn test_scan_directory() -> Result<(), IoError> {
        let dir: PathBuf = make_test_dir()?;

        // Create some test files
        add_file(&dir, "Hello", "rachis", "World")?;
        add_file(&dir, "notes", "md", "# Markdown Notes")?;
        let characters: PathBuf = add_dir(&dir.join("characters"))?;
        add_file(
            &characters,
            "Twilight Sparkle",
            "rachis",
            "dear princess celestia i don't know how to read",
        )?;

        let ctx: FlightContext = FlightContext::open_conn(&dir, "TestScanDirectory")
            .expect("Failed to open FlightContext");
        let files: Vec<ProjectFile> = ctx.scan_directory().expect("Failed to scan directory");

        // Should find 3 files (skipping .flight)
        println!("Found {:#?} files: {:#?}", files.len(), files);
        assert_eq!(files.len(), 3);

        // Check that is_native is correct
        let (rachis_files, nonrachis_files): (Vec<&ProjectFile>, Vec<&ProjectFile>) =
            files.iter().partition(|f| f.is_native);
        println!(
            "Found {:#?} native files: {:#?}",
            rachis_files.len(),
            rachis_files
        );
        println!(
            "Found {:#?} non-native files: {nonrachis_files:#?}",
            nonrachis_files.len()
        );
        assert_eq!(rachis_files.len(), 2);
        assert_eq!(nonrachis_files.len(), 1);

        fs::remove_dir_all(dir)?;
        Ok(())
    }

    #[test]
    fn test_scan_directory_skips_hidden() -> Result<(), FlightError> {
        let visible: Vec<(&str, &str, &str)> = vec![
            ("i am a chapter", "rachis", "content"),
            (
                "Alphabet Soup",
                "rachis",
                "I forgot what this story was about",
            ),
            (
                "Alphabet Coup",
                "rachis",
                "I remember this one though it's cool",
            ),
        ];
        let hidden: Vec<(&str, &str, &str)> = vec![
            (".secret_cia_file", "rachis", "shh don't tell anyone"),
            (
                ".obsidian",
                "md",
                "yes I know .obsidian is a folder don't tell me",
            ),
        ];

        let dir: PathBuf = make_test_dir()?;
        for (name, ext, contents) in &visible {
            add_file(&dir, name, ext, contents)?;
        }
        for (name, ext, contents) in hidden {
            add_file(&dir, name, ext, contents)?;
        }

        let ctx: FlightContext = FlightContext::open_conn(&dir, "TestScanDirectorySkipsHidden")
            .expect("Failed to open FlightContext");
        let files: Vec<ProjectFile> = ctx.scan_directory().expect("Failed to scan");

        assert_eq!(files.len(), visible.len());
        for (name, _, _) in &visible {
            let filename: String = format!("{name}.rachis");
            println!("{filename:#?}");
            assert!(
                files.iter().any(|pf| pf.path.ends_with(&filename)),
                "Should find visible file: {filename}"
            );
        }

        fs::remove_dir_all(dir)?;
        Ok(())
    }

    #[test]
    fn test_scan_directory_subdirs() -> Result<(), FlightError> {
        let dir: PathBuf = make_test_dir()?;
        add_file(&dir, "root_file", "rachis", "root")?;
        let bub: PathBuf = add_dir(dir.join("bub"))?;
        add_file(&bub, "nested", "md", "# Nested")?;
        let deep: PathBuf = add_dir(bub.join("deep"))?;
        add_file(&deep, "deeply", "rachis", "buried")?;

        let ctx: FlightContext = FlightContext::open_conn(&dir, "TestScanDirectorySubdirs")
            .expect("Failed to open FlightContext");
        let files: Vec<ProjectFile> = ctx.scan_directory().expect("Failed to scan");

        assert_eq!(files.len(), 3, "Should find files in nested dirs");

        assert!(files.iter().any(|f| f.path.ends_with("root_file.rachis")));
        assert!(files.iter().any(|f| f.path.ends_with("bub/nested.md")));
        assert!(
            files
                .iter()
                .any(|f| f.path.ends_with("bub/deep/deeply.rachis"))
        );

        fs::remove_dir_all(dir)?;
        Ok(())
    }

    #[test]
    fn test_open_existing_flight() -> Result<(), FlightError> {
        let dir: PathBuf = make_test_dir()?;
        add_file(&dir, "persist", "rachis", "data")?;

        // Scan and index on first open
        let ctx: FlightContext =
            FlightContext::open_conn(&dir, "TestOpenExistingFlight").expect("Failed to open");
        // ctx.scan_directory()?;
        let files_first: Vec<ProjectFile> = ctx.scan_directory().expect("Failed to scan");
        assert_eq!(files_first.len(), 1, "Should have 1 file");
        drop(ctx);

        // Reopen the same .flight on all subsequent opens
        let ctx2: FlightContext =
            FlightContext::open_conn(&dir, "TestOpenExistingFlight").expect("Failed to reopen");
        let files_second: Vec<ProjectFile> = ctx2.scan_directory().expect("Failed to scan");
        assert_eq!(files_second.len(), 1, "Should still have 1 file");
        assert!(files_second[0].path.ends_with("persist.rachis"));

        // UUID should persist across reopens
        assert_eq!(
            files_first[0].id, files_second[0].id,
            "File UUID should be stable across reopens"
        );

        fs::remove_dir_all(dir)?;
        Ok(())
    }

    #[test]
    fn test_open_nonexistent_dir() -> Result<(), FlightError> {
        let dir: PathBuf = make_test_dir()?;
        // Do a little move I like to call "causing problems on purpose"
        fs::remove_dir_all(&dir)?;

        // except opening a connection will actually create it so I can't even cause problems right
        FlightContext::open_conn(&dir, "TestOpenNonexistentDir")
            .expect("Should create and open nonexistent dir");

        assert!(dir.exists(), "Directory should be created");
        assert!(
            dir.join("TestOpenNonexistentDir.flight").exists(),
            "TestOpenNonexistentDir.flight file should exist"
        );

        fs::remove_dir_all(dir)?;
        Ok(())
    }

    #[test]
    fn test_init_flight_metadata() -> Result<(), FlightError> {
        let dir: PathBuf = make_test_dir()?;

        let ctx: FlightContext =
            FlightContext::open_conn(&dir, "TestInitFlightMetadata").expect("Failed to open");
        let flight_id: Uuid = Uuid::new_v4();
        let metadata: FlightMetadata = ctx.init_flight_metadata(&flight_id, "SE7ENFOLD")?;

        assert_eq!(metadata.name, "SE7ENFOLD");
        assert_eq!(metadata.id, flight_id.to_string());
        assert!(metadata.files.is_empty(), "New Flight should have no files");
        assert_eq!(metadata.created_at, metadata.updated_at);

        // Re-init should update the name but keep created_at
        thread::sleep(Duration::from_secs(1));
        let reinit: FlightMetadata = ctx.init_flight_metadata(&flight_id, "5IVEOFHEART5")?;
        assert_eq!(reinit.name, "5IVEOFHEART5");
        assert_eq!(
            reinit.created_at, metadata.created_at,
            "created_at should stay the same"
        );
        assert!(reinit.updated_at >= reinit.created_at);

        fs::remove_dir_all(&dir)?;
        Ok(())
    }

    #[test]
    fn test_create_file_entity_subdir() -> Result<(), FlightError> {
        let dir: PathBuf = make_test_dir()?;
        let ctx: FlightContext =
            FlightContext::open_conn(&dir, "TestCreateFileEntitySubdir").expect("Failed to open");

        // CHARACTER types should go in Characters/ subdirectory
        let rachis: Rachis = ctx.create_file(
            "Twilight Sparkle",
            FileType::Rachis,
            RachisType::CHARACTER,
            "lavender unicorn syndrome",
        )?;

        let path_str: &str = rachis.path.to_str().unwrap();
        println!("Path of rachis: {path_str:#?}");
        assert!(path_str.starts_with("Characters/"));
        assert!(dir.join(rachis.path).exists());

        fs::remove_dir_all(dir)?;
        Ok(())
    }

    #[test]
    fn test_save_file_unknown_id() -> Result<(), FlightError> {
        let dir: PathBuf = make_test_dir()?;
        let ctx: FlightContext =
            FlightContext::open_conn(&dir, "TestSaveFileUnknownId").expect("Failed to open");

        let result: Result<ProjectFile, FlightError> =
            ctx.save_file(&Uuid::nil(), "orphan content");

        assert!(
            result.is_err(),
            "Saving with a non-existent ID should error"
        );

        fs::remove_dir_all(&dir)?;
        Ok(())
    }

    #[test]
    fn test_create_file_then_scan() -> Result<(), FlightError> {
        let dir: PathBuf = make_test_dir()?;
        let ctx: FlightContext =
            FlightContext::open_conn(&dir, "TestCreateFileThenScan").expect("Failed to open");

        ctx.create_file(
            "Sonder",
            FileType::Rachis,
            RachisType::DEFAULT,
            "I woke up at 4:30 in the morning today.",
        )?;
        ctx.create_file("Vivisection", FileType::Markdown, RachisType::NOTE, "It was then that he replaced her wings with propellers not dissimilar from those one might see on a multicoloured helicopter hat, and up she went.")?;

        let files: Vec<ProjectFile> = ctx.scan_directory()?;
        assert_eq!(files.len(), 2, "scan_directory should find 2 files");
        // Order may not be preserved so just see if any of them match the names
        // Definitely do NOT check just by name in live since theoretically every single file could just be named "untitled" at a different directory level
        assert!(files.iter().any(|f| f.title == "Sonder"));
        assert!(files.iter().any(|f| f.title == "Vivisection"));

        fs::remove_dir_all(dir)?;
        Ok(())
    }

    #[test]
    fn test_sanitisation_filter() {
        let titles: Vec<(&str, &str)> = vec![
            // Most characters should pass through because they aren't stupid
            ("Light as a Feather,", "Light as a Feather,"),
            ("But, To Me Personally,", "But, To Me Personally,"),
            ("Heavy as the Sky.", "Heavy as the Sky."),
            ("See Where I Originate", "See Where I Originate"),
            (
                "And My Greatest Achievements.",
                "And My Greatest Achievements.",
            ),
            // except these ones
            (r#"Dir\Cool Title?:*"<YIPPEE>|"#, "Dir Cool Title YIPPEE"),
            ("Project/Idiot:Guy", "Project Idiot Guy"),
            (
                "What<ever> | You*Want?__really",
                "What ever You Want __really",
            ),
            (r#"giant"bugs"#, "giant bugs"),
        ];

        for (title, expected) in titles {
            let result: String = sanitize(title);
            println!("Original title: {title:#?}\nSanitised title: {result:#?}");
            assert_eq!(result, expected);
        }
    }
}
