use {
    crate::io::{
        context::{FlightContext, FlightError},
        finder::Finder,
    },
    chrono::{DateTime, Utc},
    serde::{Deserialize, Serialize},
    std::{
        fs,
        path::{Path, PathBuf},
    },
    ts_rs::TS,
    uuid::Uuid,
};

// ============================================================================
// Data Model (pure data + business logic — no I/O)
// ============================================================================

/// A lightweight registry that tracks all known Flights.
///
/// This is a **pure data model**: mutations modify data in memory only.
/// No file I/O happens here. Persistence is handled separately by
/// [load_from_disk] and [save_to_disk].
///
/// # Validation
///
/// Call [validate()](Registry::validate) to check structural integrity:
/// - All UUIDs are valid and unique
/// - All names are non-empty
/// - All paths are non-empty
/// - No duplicate paths
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Registry {
    /// Schema version for future migration support
    version: usize,
    /// The list of registered Flights
    flights: Vec<RegistryEntry>,
    /// A list of paths that the Registry can use to verify itself
    scan_paths: Vec<ScanPath>,
}

/// A single entry in the Flight registry.
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct RegistryEntry {
    /// Unique identifier for this Flight
    pub id: Uuid,
    /// Human-readable name for the Flight
    pub name: String,
    /// Path to the enveloping directory
    pub path: String,
    /// Whether this Flight is marked as a favourite
    pub is_favorite: bool,
    /// When this Flight was last opened by the user
    pub last_opened_at: DateTime<Utc>,
    /// When this Flight was first created
    pub created_at: DateTime<Utc>,
    /// Cached total word count across all Rachises in this Flight
    pub word_count: u32,
}

/// Partial update for a `RegistryEntry`. `None` fields are left unchanged.
#[derive(Debug, Clone, Default, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct RegistryEntryPatch {
    /// New display name (and thus new `.flight` filename).
    /// When set, the caller is responsible for renaming the `.flight` file on disk.
    pub name: Option<String>,
    /// New path to the enveloping directory.
    /// When set, the caller is responsible for updating scan paths.
    pub path: Option<String>,
    /// Set the favourite status explicitly (not toggled).
    /// `true` = favourited, `false` = unfavourited, `None` = unchanged.
    pub is_favorite: Option<bool>,
    /// Set `last_opened_at` to right now.
    pub bump_last_opened: bool,
    /// New cached word count.
    pub word_count: Option<u32>,
}

/// A path that the Registry can use to verify itself.
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ScanPath {
    /// The path to scan for `.flight` files
    pub path: PathBuf,
}

/// Describes the result of reconciling a single Flight entry.
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(tag = "type", content = "data")]
#[ts(export)]
pub enum ReconcileStatus {
    /// Flight is at its cached path; no action required.
    Found,
    /// Flight was found at a different path.
    Moved {
        /// The old path where the Flight was found.
        old_path: String,
        /// The new path where the Flight was found.
        new_path: String,
    },
    /// Flight could not be found in any scan paths.
    Mismatch {
        /// The cached path where the conflicting `.flight` was found.
        path: String,
        /// The UUID of the Flight currently at this location.
        mismatched_id: Option<Uuid>,
    },
    /// A `.flight` file was discovered on disk that is not registered.
    Discovered {
        /// The directory path where the `.flight` was found.
        path: String,
        /// The UUID of the discovered Flight.
        flight_id: Uuid,
    },
}

/// Report for one Flight entry after a reconciliation pass.
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ReconcileReport {
    /// The UUID of the Flight that was reconciled.
    pub id: Uuid,
    /// The human-readable name of the Flight.
    pub name: String,
    /// What happened during reconciliation.
    pub status: ReconcileStatus,
}

// #[allow(dead_code)]
// TODO: Make some Registry error enums for better error handling
impl Registry {
    /// The current schema version. Increment if the serialized format changes.
    const CURRENT_VERSION: usize = 1;

    /// Creates a new, empty Registry (in memory only).
    pub fn new() -> Self {
        Self {
            version: Self::CURRENT_VERSION,
            flights: Vec::new(),
            scan_paths: Vec::new(),
        }
    }

    // ———————— Read Operations ————————

    /// Returns a reference to all registered Flights.
    pub fn list(&self) -> &[RegistryEntry] {
        &self.flights
    }

    pub fn list_mut(&mut self) -> &mut [RegistryEntry] {
        &mut self.flights
    }

    /// Returns a reference to a specific Flight entry by ID.
    ///
    /// # Arguments
    ///
    /// - `id`: [`impl AsRef<Uuid>`](Uuid) - The ID of the Flight to retrieve.
    pub fn get(&self, id: impl AsRef<Uuid>) -> Option<&RegistryEntry> {
        self.list().iter().find(|entry| entry.id == *id.as_ref())
    }

    /// Returns a mutable reference to a specific Flight entry by ID.
    ///
    /// # Arguments
    ///
    /// - `id`: [`impl AsRef<Uuid>`](Uuid) - The ID of the Flight to retrieve.
    fn get_mut(&mut self, id: impl AsRef<Uuid>) -> Option<&mut RegistryEntry> {
        self.list_mut()
            .iter_mut()
            .find(|entry| entry.id == *id.as_ref())
    }

    /// Searches Flights by name (case-insensitive, partial match).
    ///
    /// # Arguments
    ///
    /// - `query`: [`impl AsRef<str>`](str) - The search query string.
    pub fn search_by_name(&self, query: impl AsRef<str>) -> Vec<&RegistryEntry> {
        let query: &str = query.as_ref();
        self.list()
            .iter()
            .filter(|e| e.name.to_lowercase().contains(&query.to_lowercase()))
            .collect()
    }

    /// Returns the Flight that was most recently opened, if any.
    pub fn most_recent(&self) -> Option<&RegistryEntry> {
        self.list()
            .iter()
            .max_by(|a, b| a.last_opened_at.cmp(&b.last_opened_at))
    }

    // ———————— Write Operations ————————

    /// Adds a new Flight to the registry.
    ///
    /// Does NOT write to disk — use [save_to_disk] after calling this.
    ///
    /// # Arguments
    ///
    /// - `name`: [`impl AsRef<str>`](str) - The name of the Flight.
    /// - `path`: [`impl AsRef<str>`](str) - The path to the Flight's directory.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The name is empty
    /// - The path is empty
    /// - The id is already in use
    /// - A Flight already exists at the given path
    pub fn add_entry(
        &mut self,
        name: impl AsRef<str>,
        path: impl AsRef<str>,
        id: impl AsRef<Uuid>,
    ) -> Result<RegistryEntry, String> {
        let (name, path, id) = (name.as_ref().trim(), path.as_ref().trim(), id.as_ref());

        // Check for empty name/path
        if name.is_empty() {
            return Err("Flight name must not be empty.".into());
        }
        if path.is_empty() {
            return Err("Flight path must not be empty.".into());
        }

        // Check for duplicate id
        if self.list().iter().any(|e: &RegistryEntry| e.id == *id) {
            return Err(format!("A Flight already exists with id: {id:#?}"));
        }

        // Check for duplicate path
        if self.list().iter().any(|e: &RegistryEntry| e.path == path) {
            return Err(format!("A Flight already exists at path: {path:#?}"));
        }

        // We also need to add the path to the scan paths if it isn't already there
        if !self
            .scan_paths
            .iter()
            .any(|p: &ScanPath| p.path.as_os_str() == path)
        {
            self.scan_paths.push(ScanPath::new(path));
        }

        let now: DateTime<Utc> = Utc::now();

        let entry: RegistryEntry = RegistryEntry {
            id: *id,
            name: name.into(),
            path: path.into(),
            is_favorite: false,
            last_opened_at: now,
            created_at: now,
            word_count: 0,
        };

        self.flights.push(entry.clone());
        Ok(entry)
    }

    /// Removes a Flight from the registry by ID.
    ///
    /// Does NOT write to disk — use [save_to_disk] after calling this.
    ///
    /// # Arguments
    ///
    /// - `id`: [`impl AsRef<Uuid>`](Uuid) - The ID of the Flight to remove.
    ///
    /// # Returns
    ///
    /// `true` if a Flight was found and removed, `false` otherwise.
    pub fn remove_entry(&mut self, id: impl AsRef<Uuid>) -> bool {
        let id: &Uuid = id.as_ref();
        let initial_len: usize = self.list().len();
        self.flights.retain(|entry| entry.id != *id);
        self.list().len() < initial_len
    }

    /// Adds a scan path to the Registry.
    ///
    /// # Arguments
    ///
    /// - `path`: [`impl AsRef<Path>`](Path) - The path to add as a scan path.
    ///
    /// # Returns
    ///
    /// `Ok(())` if the path was added successfully, `Err(FlightError)` otherwise.
    pub fn add_scan_path(&mut self, path: impl AsRef<Path>) -> Result<(), FlightError> {
        match !path.as_ref().to_path_buf().is_dir() {
            true => Err(FlightError::Custom("Path is not a directory".into())),
            false => Ok(self.scan_paths.push(ScanPath::new(path))),
        }
    }

    /// Remove one or many scan paths from the Registry based on the given path.
    ///
    /// # Arguments
    ///
    /// - `path`: [`impl AsRef<Path>`](Path) - The path to remove from the scan paths.
    ///
    /// # Return
    ///
    /// `true` if the path was found and removed, `false` otherwise.
    pub fn remove_scan_path(&mut self, path: impl AsRef<Path>) -> bool {
        let initial_len: usize = self.scan_paths.len();
        self.scan_paths
            .retain(|sp: &ScanPath| sp.path != path.as_ref().to_path_buf());
        self.scan_paths.len() < initial_len
    }

    /// Searches the scan paths for a Flight with the given ID.
    ///
    /// See [ScanPath] for more information.
    ///
    /// # Arguments
    ///
    /// - `id`: [`impl AsRef<Uuid>`](Uuid) - The ID of the Flight to search for.
    ///
    /// # Return
    ///
    /// [`Ok(Some(RegistryEntry))`](RegistryEntry) if the Flight was found, `Ok(None)` if not found, or an error if an issue occurred.
    pub fn find_flight_in_scan_paths(
        &self,
        id: impl AsRef<Uuid>,
    ) -> Result<Option<&RegistryEntry>, FlightError> {
        Ok(match self.search_scan_paths(&id)? {
            Some(_) => self.list().iter().find(|e| e.id == *id.as_ref()),
            None => None,
        })
    }

    /// Searches all scan paths for a Flight with the given UUID.
    ///
    /// Each scan path is a **directory** that gets globbed for `**/*.flight` files.
    ///
    /// # Arguments
    ///
    /// - `id`: [`impl AsRef<Uuid>`](Uuid) — The UUID of the Flight to find.
    ///
    /// # Returns
    ///
    /// `Ok(Some(PathBuf))` — The path to the found `.flight` file.
    /// `Ok(None)` — Flight not found in any scan path.
    fn search_scan_paths(&self, id: impl AsRef<Uuid>) -> Result<Option<PathBuf>, FlightError> {
        for sp in &self.scan_paths {
            if let Some(found) = self.find_flight_in_dir(&id, &sp.path)? {
                return Ok(Some(found));
            }
        }
        Ok(None)
    }

    fn find_flight_in_dir(
        &self,
        id: impl AsRef<Uuid>,
        path: impl AsRef<Path>,
    ) -> Result<Option<PathBuf>, FlightError> {
        let (id, path) = (id.as_ref(), path.as_ref().to_path_buf());

        // Early return if the path no longer exists
        if !path.exists() {
            return Ok(None);
        }

        let files: Vec<PathBuf> = Finder::new(&path).skip_hidden().find()?.files;
        for flight_path in files {
            // Extract the parent directory and filename stem to pass to read_flight_metadata
            if let Some(parent) = flight_path.parent() {
                let stem: &str = flight_path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("");
                if !stem.is_empty() {
                    if let Some(metadata) = FlightContext::read_flight_metadata(parent, stem)? {
                        if metadata.id == id.to_string() {
                            return Ok(Some(flight_path));
                        }
                    }
                }
            }
        }

        Ok(None)
    }

    /// Recursively scans a directory for `.flight` files whose UUIDs are NOT
    /// in the registry. Each one is reported as [`ReconcileStatus::Discovered`].
    ///
    /// # Arguments
    ///
    /// - `path`: [`impl AsRef<Path>`](Path) — The directory to scan.
    fn discover_flights_in_dir(
        &self,
        path: impl AsRef<Path>,
    ) -> Result<Vec<ReconcileReport>, FlightError> {
        let path: PathBuf = path.as_ref().to_path_buf();

        if !path.exists() {
            return Ok(Vec::new());
        }

        // Find all `.flight` files anywhere in the tree under non-hidden directories
        let flight_files: Vec<PathBuf> = Finder::new(&path)
            .skip_hidden_dirs()
            .extensions(["flight"])
            .find()?
            .files;

        let mut discovered: Vec<ReconcileReport> = Vec::new();

        for flight_path in &flight_files {
            if let Some(parent) = flight_path.parent() {
                let stem: &str = flight_path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("");
                if !stem.is_empty() {
                    if let Some(metadata) = FlightContext::read_flight_metadata(parent, stem)? {
                        let uuid: Uuid = Uuid::try_parse(&metadata.id)
                            .map_err(|e| FlightError::Custom(e.to_string()))?;
                        if !self.list().iter().any(|e: &RegistryEntry| e.id == uuid) {
                            discovered.push(ReconcileReport {
                                id: uuid,
                                name: metadata.name,
                                status: ReconcileStatus::Discovered {
                                    path: parent.to_string_lossy().into_owned(),
                                    flight_id: uuid,
                                },
                            });
                        }
                    }
                }
            }
        }

        Ok(discovered)
    }

    /// Checks every registered Flight's cached path to validate cache-disk pairing,
    /// then scans `scan_paths` for any `.flight` files not yet registered.
    ///
    /// ## Registered Flight reconciliation
    ///
    /// For Flights that are found at the same path on disk as suggested by the
    /// registry, no action is taken ([`ReconcileStatus::Found`]). For Flights
    /// that are found at a different path, the registry self-updates its cached
    /// path ([`ReconcileStatus::Moved`]). For cached paths that no longer hold
    /// the correct Flight (regardless of whether there is a Flight there or not),
    /// the entry is preserved in the registry for manual removal
    /// ([`ReconcileStatus::Mismatch`]).
    ///
    /// ## Flight discovery
    ///
    /// After reconciling registered entries, every directory in `scan_paths` is
    /// recursively scanned for `.flight` files whose UUIDs are not in the registry.
    /// Each discovered `.flight` is reported as [`ReconcileStatus::Discovered`]
    /// to allow callers to do... whatever with that.
    ///
    /// # Returns
    ///
    /// A list of [`ReconcileReports`](ReconcileReport) describing what happened
    /// to each Flight.
    ///
    /// # Examples
    ///
    /// TODO: Generate examples for how the hell to use this
    pub fn reconcile_flights(&mut self) -> Result<Vec<ReconcileReport>, FlightError> {
        let mut reports: Vec<ReconcileReport> = Vec::new();
        // Collect updates to apply after the checks
        let mut updates: Vec<(Uuid, PathBuf)> = Vec::new();

        for entry in self.list().iter() {
            let cached_path: &Path = Path::new(&entry.path);
            let name: &str = entry.name.as_str();

            let (mismatched_metadata, right_flight) = match cached_path.exists() {
                // If the path exists, check if the Flight at that path is expected
                true => match FlightContext::read_flight_metadata(cached_path, name)? {
                    // If the UUIDs match, the Flight is already (correctly) registered at the path
                    Some(metadata) if metadata.id == entry.id.to_string() => (None, true),
                    // If the .flight exists but has a different UUID, report it as mismatched
                    Some(metadata) => (Some(metadata), false),
                    // If the expected file doesn't exist, grab any .flight in the directory
                    None => {
                        let files = Finder::new(&cached_path)
                            .skip_hidden()
                            .extensions(["flight"])
                            .find()?
                            .files;

                        (
                            files.into_iter().find_map(|f| {
                                FlightContext::read_flight_metadata(
                                    f.parent()?,
                                    f.file_stem()?.to_str()?,
                                )
                                .ok()?
                            }),
                            false,
                        )
                    }
                },
                false => (None, false),
            };

            let status = match right_flight {
                true => ReconcileStatus::Found,
                false => match self.search_scan_paths(&entry.id)? {
                    Some(new_path) => {
                        updates.push((entry.id, new_path.clone()));
                        ReconcileStatus::Moved {
                            old_path: entry.path.clone(),
                            new_path: new_path.to_string_lossy().into_owned(),
                        }
                    }
                    None => {
                        // let mismatched_id: Option<Uuid> = Uuid::try_parse(&mismatched_metadata.unwrap().id).ok();
                        let mismatched_id: Option<Uuid> = mismatched_metadata
                            .map(|m| Uuid::try_parse(&m.id).ok())
                            .flatten();
                        ReconcileStatus::Mismatch {
                            path: entry.path.clone(),
                            mismatched_id,
                        }
                    }
                },
            };

            reports.push(ReconcileReport {
                id: entry.id,
                name: entry.name.clone(),
                status,
            });
        }

        // Apply path updates
        for (id, new_path) in updates {
            if let Some(entry) = self.get_mut(id) {
                entry.path = new_path.to_string_lossy().into_owned();
            }
        }

        // Discover unregistered .flight files in scan paths
        for scan_path in &self.scan_paths {
            reports.extend(self.discover_flights_in_dir(&scan_path.path)?);
        }

        Ok(reports)
    }

    /// Updates one or more fields on a `RegistryEntry` using a [`RegistryEntryPatch`].
    ///
    /// `None` fields in the patch are left unchanged — only the supplied values are applied.
    /// This replaces `toggle_favorite`, `update_last_opened`, `update_word_count`,
    /// and the original `update` with a single, unified method.
    ///
    /// Does NOT write to disk — use [`save_to_disk`] after calling this.
    /// Does NOT rename `.flight` files or sync scan paths — the caller is responsible
    /// for those side effects based on which fields were set in the patch.
    ///
    /// # Arguments
    ///
    /// - `id`: [`impl AsRef<Uuid>`](Uuid) - The ID of the Flight to update.
    /// - `patch`: [`RegistryEntryPatch`] - The fields to update.
    ///
    /// # Errors
    ///
    /// Returns an error if the Flight isn't found.
    pub fn update(
        &mut self,
        id: impl AsRef<Uuid>,
        patch: RegistryEntryPatch,
    ) -> Result<RegistryEntry, FlightError> {
        let id: &Uuid = id.as_ref();
        let entry: &mut RegistryEntry = self
            .get_mut(id)
            .ok_or_else(|| FlightError::Custom(format!("Flight not found: {id:#?}")))?;

        if let Some(name) = patch.name {
            let name: &str = name.trim();
            if !name.is_empty() {
                entry.name = name.into();
            }
        }
        if let Some(path) = patch.path {
            let path: &str = path.trim();
            if !path.is_empty() {
                entry.path = path.into();
            }
        }
        if let Some(is_favorite) = patch.is_favorite {
            entry.is_favorite = is_favorite;
        }
        if patch.bump_last_opened {
            entry.last_opened_at = Utc::now();
        }
        if let Some(word_count) = patch.word_count {
            entry.word_count = word_count;
        }

        Ok(entry.clone())
    }

    // ———————— Validation ————————

    /// Checks the structural integrity of the registry.
    ///
    /// Checks:
    /// - Version is supported
    /// - All UUIDs are valid and non-nil
    /// - All names are non-empty
    /// - All paths are non-empty
    /// - No duplicate IDs
    /// - No duplicate paths
    pub fn is_valid(&self) -> Result<(), String> {
        // Check version
        if self.version == 0 || self.version > Self::CURRENT_VERSION {
            return Err(format!(
                "Unsupported registry version: {:#?}. Expected version {:#?}.",
                self.version,
                Self::CURRENT_VERSION
            ));
        }

        let mut seen_ids: Vec<Uuid> = Vec::with_capacity(self.list().len());
        let mut seen_paths: Vec<&str> = Vec::with_capacity(self.list().len());

        for entry in &self.flights {
            // UUID must be non-nil
            if entry.id.is_nil() {
                return Err("Found a Flight entry with a nil UUID.".into());
            }

            // No duplicate IDs
            if seen_ids.contains(&entry.id) {
                return Err(format!("Duplicate Flight ID found: {:#?}", entry.id));
            }
            seen_ids.push(entry.id);

            // Name must be non-empty
            if entry.name.trim().is_empty() {
                return Err(format!("Flight {:#?} has an empty name.", entry.id));
            }

            // Path must be non-empty
            if entry.path.trim().is_empty() {
                return Err(format!(
                    "Flight {:#?} ({:#?}) has an empty path.",
                    entry.name, entry.id
                ));
            }

            // No duplicate paths
            if seen_paths.contains(&entry.path.as_str()) {
                return Err(format!(
                    "Duplicate path found for Flight {:#?} ({:#?}): {:#?}",
                    entry.name, entry.id, entry.path
                ));
            }
            seen_paths.push(&entry.path);
        }

        Ok(())
    }
}

impl RegistryEntry {
    pub fn to_flight_context(self) -> Result<FlightContext, FlightError> {
        Ok(FlightContext::open_conn(self.path, self.name)?)
    }
}

impl ScanPath {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }
}

impl Default for Registry {
    fn default() -> Self {
        Self {
            version: Self::CURRENT_VERSION,
            flights: Vec::new(),
            scan_paths: Vec::new(),
        }
    }
}

impl Default for RegistryEntry {
    fn default() -> Self {
        let now: DateTime<Utc> = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name: String::new(),
            path: String::new(),
            is_favorite: false,
            last_opened_at: now,
            created_at: now,
            word_count: 0,
        }
    }
}

// ============================================================================
// Persistence (separate from data model — handles file I/O)
// ============================================================================

/// The expected filename for the registry within the app data directory.
const REGISTRY_FILENAME: &str = "registry.json";

/// Loads a Registry from disk, or creates a new empty one if the file
/// doesn't exist.
///
/// # Arguments
///
/// - `dir`: [`impl AsRef<Path>`](Path) - The directory containing the registry file.
///
/// # Errors
///
/// Returns an error if the file exists but cannot be parsed, or if
/// validation fails.
pub fn load_from_disk(dir: impl AsRef<Path>) -> Result<Registry, FlightError> {
    let dir: &Path = dir.as_ref();
    let path: PathBuf = dir.join(REGISTRY_FILENAME);

    // If the file doesn't exist yet, return a fresh registry
    if !path.exists() {
        return Ok(Registry::new());
    }

    // Read and parse the JSON file
    let content: String = fs::read_to_string(&path).map_err(|e| FlightError::Io(e))?;

    let reg: Registry = serde_json::from_str(&content)
        .map_err(|e| FlightError::Custom(format!("OH GOD OH NO!!!!!! {}", e.to_string())))?;

    // Validate the loaded data
    reg.is_valid().map_err(|e: String| FlightError::Custom(e))?;

    Ok(reg)
}

/// Writes the registry to disk as pretty-printed JSON.
///
/// # Arguments
///
/// - `dir`: [`impl AsRef<Path>`](Path) - The directory to write the registry file into.
/// - `registry`: [`&Registry`](Registry) - The registry to persist.
///
/// # Errors
///
/// Returns an error if validation fails or the file cannot be written.
pub fn save_to_disk(dir: impl AsRef<Path>, registry: &Registry) -> Result<bool, FlightError> {
    // Validate before saving
    registry.is_valid().map_err(|e| FlightError::Custom(e))?;

    let path: PathBuf = dir.as_ref().join(REGISTRY_FILENAME);
    let content: String = serde_json::to_string_pretty(registry)
        .map_err(|e| FlightError::Custom(format!("Failed to serialize registry: {e:#?}")))?;

    fs::write(&path, &content).map_err(|e| FlightError::Io(e))?;

    println!("Registry saved to: {path:#?}");
    // im just now realising that returning a bool in a result is a little ridiculous
    Ok(true)
}

// ============================================================================
// Tests (pure in-memory — no files created!)
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::{env, fs};

    /// Tests that a fresh registry starts empty.
    #[test]
    fn test_new_registry_is_empty() {
        let reg: Registry = Registry::new();
        println!("Registry: {reg:#?}");

        println!("Registry count: {:?}", reg.list().len());
        assert_eq!(reg.list().len(), 0);
        println!(
            "Registry total word count: {:?}",
            reg.list().iter().map(|e| e.word_count).sum::<u32>()
        );
        assert_eq!(reg.list().iter().map(|e| e.word_count).sum::<u32>(), 0);
        println!("Registry most recent: {:?}", reg.most_recent());
        assert!(reg.most_recent().is_none());
    }

    /// Tests adding a Flight to the registry (in memory only).
    #[test]
    fn test_add_flight() {
        let mut reg: Registry = Registry::new();

        let entry: RegistryEntry = reg
            .add_entry("Ad Eternum", "/home/user/ad_eternum", Uuid::new_v4())
            .expect("Failed to add flight");
        println!("Entry: {entry:#?}");

        println!("Entry name: {}", entry.name);
        assert_eq!(entry.name, "Ad Eternum");
        println!("Entry path: {}", entry.path);
        assert_eq!(entry.path, "/home/user/ad_eternum");
        println!("Entry is_favorite: {}", entry.is_favorite);
        assert!(!entry.is_favorite);
        println!("Registry count: {}", reg.list().len());
        assert_eq!(reg.list().len(), 1);
    }

    /// Tests that adding a Flight with an empty name fails.
    #[test]
    fn test_add_flight_empty_name() {
        let mut reg: Registry = Registry::new();

        let result: Result<RegistryEntry, String> = reg.add_entry("   ", "/path", Uuid::new_v4());
        println!("Result: {result:#?}");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("empty"));
    }

    /// Tests that adding a Flight with a duplicate path fails.
    #[test]
    fn test_add_flight_duplicate_path() {
        let mut reg: Registry = Registry::new();

        reg.add_entry("Project A", "/path", Uuid::new_v4())
            .expect("Failed to add first flight");

        let result: Result<RegistryEntry, String> =
            reg.add_entry("Project B", "/path", Uuid::new_v4());
        println!("Result: {result:#?}");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("already exists"));
    }

    /// Tests removing a Flight from the registry.
    #[test]
    fn test_remove_flight() {
        let mut reg: Registry = Registry::new();

        let entry: RegistryEntry = reg
            .add_entry("Test", "/test", Uuid::new_v4())
            .expect("Failed to add flight");
        println!("Entry:\n{entry:#?}");

        println!("Registry count: {:?}", reg.list().len());
        assert_eq!(reg.list().len(), 1);

        let removed: bool = reg.remove_entry(entry.id);
        println!("Removed: {removed:#?}");
        assert!(removed);
        println!("Registry count: {:?}", reg.list().len());
        assert_eq!(reg.list().len(), 0);

        // Removing a non-existent ID should return false
        let not_removed: bool = reg.remove_entry(Uuid::new_v4());
        assert!(!not_removed);
    }

    /// Tests toggling the favourite status.
    #[test]
    fn test_toggle_favorite() {
        let mut reg: Registry = Registry::new();

        let id: Uuid = Uuid::new_v4();
        reg.add_entry("Test", "/test", id)
            .expect("Failed to add flight");

        // Re-fetch to check initial state
        let entry: &RegistryEntry = reg.get(&id).expect("Entry should exist");
        println!("Entry is_favorite: {:?}", entry.is_favorite);
        assert!(!entry.is_favorite);

        reg.update(
            id,
            RegistryEntryPatch {
                is_favorite: Some(true),
                ..Default::default()
            },
        )
        .expect("Failed to toggle favorite");

        let entry: &RegistryEntry = reg.get(id).expect("Entry should exist");
        println!("Entry is_favorite: {:?}", entry.is_favorite);
        assert!(entry.is_favorite);

        let favourites: Vec<&RegistryEntry> =
            reg.flights.iter().filter(|e| e.is_favorite).collect();

        println!("Favorites in registry: {favourites:#?}");
        assert_eq!(favourites.len(), 1);

        // Toggle off via patch
        reg.update(
            id,
            RegistryEntryPatch {
                is_favorite: Some(false),
                ..Default::default()
            },
        )
        .expect("Failed to toggle favorite");

        let entry: &RegistryEntry = reg.get(id).expect("Entry should exist");
        assert!(!entry.is_favorite);
        let favourites: Vec<&RegistryEntry> =
            reg.flights.iter().filter(|e| e.is_favorite).collect();

        println!("Favorites in registry: {favourites:#?}");
        assert!(favourites.is_empty());
    }

    /// Tests the most_recent() functionality.
    #[test]
    fn test_most_recent() {
        let mut reg: Registry = Registry::new();

        let first: RegistryEntry = reg
            .add_entry("First", "/first", Uuid::new_v4())
            .expect("Failed to add first");
        let second: RegistryEntry = reg
            .add_entry("Second", "/second", Uuid::new_v4())
            .expect("Failed to add second");

        // Second was the most recent addition, so it should be most recent
        let recent: &RegistryEntry = reg.most_recent().expect("No recent flight");
        println!("Recent flight: {recent:#?}");
        assert_eq!(recent.id, second.id);

        // Update last_opened for first
        std::thread::sleep(std::time::Duration::from_millis(10));
        reg.update(
            first.id,
            RegistryEntryPatch {
                bump_last_opened: true,
                ..Default::default()
            },
        )
        .expect("Failed to update last opened");

        // Now first should be most recent
        let recent: &RegistryEntry = reg.most_recent().expect("No recent flight");
        println!("Recent flight: {recent:#?}");
        assert_eq!(recent.id, first.id);
    }

    /// Tests searching by name (case-insensitive, partial match).
    #[test]
    fn test_search() {
        let mut reg: Registry = Registry::new();

        reg.add_entry("Ad Eternum", "/ad_eternum", Uuid::new_v4())
            .expect("Failed to add");
        reg.add_entry("SE7ENFOLD", "/se7enfold", Uuid::new_v4())
            .expect("Failed to add");
        reg.add_entry("Sonder", "/sonder", Uuid::new_v4())
            .expect("Failed to add");

        let results: Vec<&RegistryEntry> = reg.search_by_name("sonder");
        println!("Results for query 'sonder': {results:#?}");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Sonder");

        // Case-insensitive search
        let results: Vec<&RegistryEntry> = reg.search_by_name("SE7EN");
        println!("Results for query 'SE7EN': {results:#?}");
        assert_eq!(results.len(), 1);

        // Partial match
        let results: Vec<&RegistryEntry> = reg.search_by_name("n");
        println!("Results for query 'n': {results:#?}");
        assert_eq!(results.len(), 3);
    }

    /// Tests that validation rejects nil UUIDs.
    #[test]
    fn test_validation_nil_uuid() {
        let mut reg: Registry = Registry::new();

        reg.flights.push(RegistryEntry {
            id: Uuid::nil(),
            name: "NilUuid".into(),
            path: "/nil_uuid".into(),
            is_favorite: false,
            last_opened_at: Utc::now(),
            created_at: Utc::now(),
            word_count: 0,
        });
        println!("Registry after adding nil UUID: {reg:#?}");

        let result: Result<(), String> = reg.is_valid();
        println!("Validation result: {result:#?}");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("nil UUID"));
    }

    /// Tests that validation catches duplicate IDs.
    #[test]
    fn test_validation_duplicate_id() {
        let mut reg: Registry = Registry::new();

        let id: Uuid = Uuid::new_v4();
        let now = Utc::now();
        reg.flights.push(RegistryEntry {
            id,
            name: "First".into(),
            path: "/first".into(),
            is_favorite: false,
            last_opened_at: now,
            created_at: now,
            word_count: 0,
        });
        println!("Registry after adding first entry: {reg:#?}");
        reg.flights.push(RegistryEntry {
            id,
            name: "Second".into(),
            path: "/second".into(),
            is_favorite: false,
            last_opened_at: now,
            created_at: now,
            word_count: 0,
        });
        println!("Registry after adding second entry: {reg:#?}");

        let result: Result<(), String> = reg.is_valid();
        println!("Validation result: {result:#?}");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Duplicate"));
    }

    // ———————— Persistence Tests (explicitly test I/O) ————————

    /// Tests that a non-existent file creates a fresh registry.
    #[test]
    fn test_load_missing_file_creates_empty() {
        let dir: PathBuf = env::temp_dir().join(format!("registry_test_{}", Uuid::new_v4()));
        println!("Test directory: {dir:#?}");
        fs::create_dir_all(&dir).expect("Failed to create test directory");
        // Clean the directory before loading to force an error
        _defer_cleanup(&dir);

        let reg: Registry = load_from_disk(&dir).expect("Failed to load registry");
        println!("Registry after loading missing file: {reg:#?}",);
        assert_eq!(reg.list().len(), 0);
    }

    /// Tests a full round-trip: create in memory, persist, load back.
    #[test]
    fn test_round_trip() {
        let dir: PathBuf = env::temp_dir().join(format!("registry_test_{}", Uuid::new_v4()));
        fs::create_dir_all(&dir).expect("Failed to create test directory");

        // Create and populate in memory
        let mut reg: Registry = Registry::new();
        reg.add_entry("Alphabet Coup", "/alphabet_coup", Uuid::new_v4())
            .expect("Failed to add");
        reg.add_entry("Vivisection", "/vivisection", Uuid::new_v4())
            .expect("Failed to add");
        println!("Registry after adding entries: {reg:#?}");

        let first_id: Uuid = reg.list()[0].id;
        reg.update(
            first_id,
            RegistryEntryPatch {
                is_favorite: Some(true),
                ..Default::default()
            },
        )
        .expect("Failed to toggle");
        println!("Registry after toggling favorite: {reg:#?}");

        // Persist
        let result = save_to_disk(&dir, &reg);
        assert!(
            result.is_ok(),
            "Failed to save registry: {}",
            result.err().unwrap()
        );

        // Load and verify
        let loaded: Registry = load_from_disk(&dir).expect("Failed to load");
        println!("Registry after loading from disk: {loaded:#?}");
        assert_eq!(loaded.list().len(), 2);

        let favorites: Vec<&RegistryEntry> =
            loaded.flights.iter().filter(|e| e.is_favorite).collect();
        println!("Favorites: {favorites:#?}");
        assert_eq!(favorites.len(), 1);
        assert_eq!(favorites[0].name, "Alphabet Coup");

        // Clean up
        _defer_cleanup(&dir);
    }

    #[test]
    fn test_default() {
        let reg: Registry = Default::default();
        assert_eq!(reg.version, 1);
        assert!(reg.flights.is_empty());

        let entry: RegistryEntry = Default::default();
        assert!(!entry.id.is_nil());
        assert_eq!(entry.name, "");
        assert_eq!(entry.path, "");
        assert!(!entry.is_favorite);
        assert_ne!(entry.last_opened_at, DateTime::<Utc>::default());
        assert_ne!(entry.created_at, DateTime::<Utc>::default());
        assert_eq!(entry.word_count, 0);
    }

    /// Schedules a directory for cleanup after a test completes.
    /// Called explicitly rather than relying on Drop to avoid surprises.
    fn _defer_cleanup(dir: impl AsRef<Path>) {
        let _ = fs::remove_dir_all(dir);
    }

    /// Creates a temp root for reconciliation tests
    fn _reconcile_test_root() -> PathBuf {
        let dir: PathBuf = env::temp_dir().join(format!("rachis_reconcile_{}", Uuid::new_v4()));
        fs::create_dir_all(&dir).expect("Failed to create reconcile test root");
        dir
    }

    /// Creates a `.flight` at the given directory path, initialised with the UUID and name
    fn _setup_flight_at(
        dir: impl AsRef<Path>,
        uuid: impl AsRef<Uuid>,
        name: impl AsRef<str>,
    ) -> Result<FlightContext, FlightError> {
        let (dir, uuid, name) = (dir.as_ref(), uuid.as_ref(), name.as_ref());

        let ctx: FlightContext = FlightContext::open_conn(dir, &name)?;
        ctx.init_flight_metadata(uuid, name)?;
        Ok(ctx)
    }

    /// Adds a registry entry with a specific UUID
    fn _add_entry_with_id(
        registry: &mut Registry,
        id: impl AsRef<Uuid>,
        name: impl AsRef<str>,
        path: impl AsRef<str>,
    ) {
        let (id, name, path) = (id.as_ref(), name.as_ref(), path.as_ref());

        let now: DateTime<Utc> = Utc::now();
        registry.flights.push(RegistryEntry {
            id: *id,
            name: name.into(),
            path: path.into(),
            is_favorite: false,
            last_opened_at: now,
            created_at: now,
            word_count: 0,
        });
    }

    #[test]
    fn test_reconcile_flight_at_cached_path() -> Result<(), FlightError> {
        let root: PathBuf = _reconcile_test_root();
        let id: Uuid = Uuid::new_v4();
        let flight_dir: PathBuf = root.join("my_novel");
        let ctx: FlightContext = _setup_flight_at(&flight_dir, &id, "My Novel")?;
        drop(ctx);

        let mut reg: Registry = Registry::new();
        _add_entry_with_id(&mut reg, id, "My Novel", flight_dir.to_str().unwrap());
        reg.add_scan_path(&root)?;
        println!("Registry: {reg:#?}");

        let reports: Vec<ReconcileReport> = reg.reconcile_flights()?;
        println!("Reconcile reports: {reports:#?}");

        assert_eq!(reports.len(), 1);
        assert_eq!(reports[0].id, id);
        assert!(matches!(reports[0].status, ReconcileStatus::Found));

        _defer_cleanup(root);
        Ok(())
    }

    #[test]
    fn test_reconcile_flight_moved() -> Result<(), FlightError> {
        let root: PathBuf = _reconcile_test_root();
        let id: Uuid = Uuid::new_v4();

        // Create the original flight directory
        let original_dir: PathBuf = root.join("original");
        let ctx: FlightContext = _setup_flight_at(&original_dir, &id, "My Novel")?;
        drop(ctx);

        // Create a new flight with same UUID in a scan-able subdirectory
        let new_dir: PathBuf = root.join("moved_here");
        let ctx2: FlightContext = _setup_flight_at(&new_dir, &id, "My Novel")?;
        drop(ctx2);

        // Remove the original to simulate a move
        fs::remove_dir_all(&original_dir)?;

        let mut reg: Registry = Registry::new();
        _add_entry_with_id(&mut reg, id, "My Novel", original_dir.to_str().unwrap());
        reg.add_scan_path(&root)?;
        println!("Registry: {reg:#?}");

        let reports: Vec<ReconcileReport> = reg.reconcile_flights()?;
        println!("Reconcile reports: {reports:#?}");

        assert_eq!(reports.len(), 1);
        assert!(
            matches!(&reports[0].status, ReconcileStatus::Moved { .. }),
            "Expected Moved status, got {:?}",
            reports[0].status
        );

        // Registry entry path should be updated
        let entry: &RegistryEntry = reg.get(&id).expect("Entry should exist");
        assert!(
            entry.path.contains("moved_here"),
            "Entry path should be updated to moved_here, is: {}",
            entry.path
        );

        _defer_cleanup(&root);
        Ok(())
    }

    #[test]
    fn test_reconcile_flight_orphaned() -> Result<(), FlightError> {
        let root: PathBuf = _reconcile_test_root();
        let id: Uuid = Uuid::new_v4();
        let flight_dir: PathBuf = root.join("i deleted it on purpose");
        let ctx: FlightContext = _setup_flight_at(&flight_dir, &id, "i deleted it on purpose")?;
        drop(ctx);

        // Delete the flight directory so it's guaranteed to mismatch
        fs::remove_dir_all(&flight_dir)?;

        let mut reg: Registry = Registry::new();
        _add_entry_with_id(
            &mut reg,
            id,
            "i deleted it on purpose",
            flight_dir.to_string_lossy(),
        );
        println!("Registry: {reg:#?}");
        // No scan_paths — nothing to search

        let reports: Vec<ReconcileReport> = reg.reconcile_flights()?;
        println!("Reconcile reports: {reports:#?}");

        assert_eq!(reports.len(), 1);
        assert!(
            matches!(
                &reports[0].status,
                ReconcileStatus::Mismatch {
                    mismatched_id: None,
                    ..
                }
            ),
            "Expected orphaned Mismatch, got {:?}",
            reports[0].status
        );

        _defer_cleanup(root);
        Ok(())
    }

    #[test]
    fn test_reconcile_flight_mismatch() -> Result<(), FlightError> {
        let root: PathBuf = _reconcile_test_root();
        let (original_id, other_id) = (Uuid::new_v4(), Uuid::new_v4());

        let flight_dir: PathBuf = root.join("mismatch_flight");
        let ctx: FlightContext = _setup_flight_at(&flight_dir, &original_id, "Original")?;
        drop(ctx);

        // Wipe the old .flight files and create a fresh one with a different UUID
        for entry in fs::read_dir(&flight_dir)? {
            let entry_path: PathBuf = entry?.path();
            if entry_path
                .file_name()
                .and_then(|n| n.to_str())
                .map_or(false, |n| n.contains(".flight"))
            {
                fs::remove_file(&entry_path)?;
            }
        }
        let superimposed: FlightContext = FlightContext::open_conn(&flight_dir, "Superimposed")?;
        superimposed.init_flight_metadata(&other_id, "Superimposed")?;
        drop(superimposed);

        let mut reg: Registry = Registry::new();
        _add_entry_with_id(
            &mut reg,
            original_id,
            "Original",
            flight_dir.to_str().unwrap(),
        );
        println!("Registry: {reg:#?}");

        let reports: Vec<ReconcileReport> = reg.reconcile_flights()?;
        println!("Reports: {reports:#?}");

        assert_eq!(reports.len(), 1);
        assert_eq!(reports[0].id, original_id);
        assert!(
            matches!(
                &reports[0].status,
                ReconcileStatus::Mismatch {
                    mismatched_id: Some(found_id), ..
                } if *found_id == other_id
            ),
            "Expected Mismatch with UUID {other_id}, got {:?}",
            reports[0].status
        );

        _defer_cleanup(&root);
        Ok(())
    }

    #[test]
    fn test_reconcile_flight_no_scan_paths() -> Result<(), FlightError> {
        let root: PathBuf = _reconcile_test_root();
        let id: Uuid = Uuid::new_v4();
        let flight_dir: PathBuf = root.join("solamente");
        let ctx: FlightContext = _setup_flight_at(&flight_dir, &id, "Solamente")?;
        drop(ctx);

        fs::remove_dir_all(&flight_dir)?;

        let mut reg: Registry = Registry::new();
        _add_entry_with_id(&mut reg, id, "Solamente", flight_dir.to_string_lossy());
        println!("Registry: {reg:#?}");

        let reports: Vec<ReconcileReport> = reg.reconcile_flights()?;
        println!("Reports: {reports:#?}");

        assert!(matches!(
            reports[0].status,
            ReconcileStatus::Mismatch { .. }
        ));
        assert_eq!(reports.len(), 1);

        _defer_cleanup(&root);
        Ok(())
    }

    #[test]
    fn test_reconcile_multiple_flights() -> Result<(), FlightError> {
        let root: PathBuf = _reconcile_test_root();
        let (a_id, b_id, c_id) = (Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4());

        // Flight A: at cached path (Found)
        let a_dir: PathBuf = root.join("aurora");
        let ctx_a: FlightContext = _setup_flight_at(&a_dir, &a_id, "Aurora")?;
        drop(ctx_a);

        // Flight B: moved from beta_orig to beta_new
        let b_orig: PathBuf = root.join("bor");
        let ctx_b: FlightContext = _setup_flight_at(&b_orig, &b_id, "Bor")?;
        drop(ctx_b);
        let b_new: PathBuf = root.join("bori");
        let ctx_b2: FlightContext = _setup_flight_at(&b_new, &b_id, "Bori")?;
        drop(ctx_b2);
        fs::remove_dir_all(&b_orig)?;

        // Flight C: orphaned (deleted, no scan_paths for it)
        let c_dir: PathBuf = root.join("alice");
        let ctx_c: FlightContext = _setup_flight_at(&c_dir, &c_id, "Alice")?;
        drop(ctx_c);
        fs::remove_dir_all(&c_dir)?;

        let mut reg: Registry = Registry::new();
        _add_entry_with_id(&mut reg, a_id, "Aurora", a_dir.to_str().unwrap());
        _add_entry_with_id(&mut reg, b_id, "Bori", b_orig.to_str().unwrap());
        _add_entry_with_id(&mut reg, c_id, "Alice", c_dir.to_str().unwrap());
        reg.add_scan_path(&root)?;

        let reports: Vec<ReconcileReport> = reg.reconcile_flights()?;

        assert_eq!(reports.len(), 3);
        assert!(
            matches!(reports[0].status, ReconcileStatus::Found),
            "Aurora should be Found, got {:?}",
            reports[0].status
        );
        assert!(
            matches!(reports[1].status, ReconcileStatus::Moved { .. }),
            "Bori should be Moved, got {:?}",
            reports[1].status
        );
        assert!(
            matches!(
                reports[2].status,
                ReconcileStatus::Mismatch {
                    mismatched_id: None,
                    ..
                }
            ),
            "Alice should be orphaned Mismatch, got {:?}",
            reports[2].status
        );

        // Beta's path should be updated
        let beta: &RegistryEntry = reg.get(&b_id).unwrap();
        assert!(
            beta.path.contains("bori"),
            "Bori path should be updated to bori, is: {}",
            beta.path
        );

        _defer_cleanup(&root);
        Ok(())
    }
}
