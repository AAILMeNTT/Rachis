use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::{fmt, fs};
use ts_rs::TS;
use uuid::Uuid;

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
}

/// A single entry in the Flight registry.
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct RegistryEntry {
    /// Unique identifier for this Flight
    pub id: Uuid,
    /// Human-readable name for the Flight
    pub name: String,
    /// Absolute path to the `.rachis` database file
    pub path: String,
    /// Whether this Flight is marked as a favourite
    pub is_favorite: bool,
    /// When this Flight was last opened by the user
    pub last_opened_at: DateTime<Utc>,
    /// When this Flight was first created
    pub created_at: DateTime<Utc>,
    /// Cached total word count across all Rachises in this Flight
    pub word_count: usize,
}

impl Registry {
    /// The current schema version. Increment if the serialized format changes.
    const CURRENT_VERSION: usize = 1;

    /// Creates a new, empty Registry (in memory only).
    pub fn new() -> Self {
        Self {
            version: Self::CURRENT_VERSION,
            flights: Vec::new(),
        }
    }

    // ———————— Read Operations ————————

    /// Returns a reference to all registered Flights.
    pub fn list(&self) -> &[RegistryEntry] {
        &self.flights
    }

    /// Returns a reference to a specific Flight entry by ID.
    ///
    /// # Arguments
    ///
    /// - `id`: [`&Uuid`](Uuid) - The ID of the Flight to retrieve.
    pub fn get(&self, id: &Uuid) -> Option<&RegistryEntry> {
        self.flights.iter().find(|entry| entry.id == *id)
    }

    /// Returns a mutable reference to a specific Flight entry by ID.
    ///
    /// # Arguments
    ///
    /// - `id`: [`&Uuid`](Uuid) - The ID of the Flight to retrieve.
    fn get_mut(&mut self, id: &Uuid) -> Option<&mut RegistryEntry> {
        self.flights.iter_mut().find(|entry| entry.id == *id)
    }

    /// Searches Flights by name (case-insensitive, partial match).
    ///
    /// # Arguments
    ///
    /// - `query`: [`&str`](str) - The search query string.
    pub fn search(&self, query: &str) -> Vec<&RegistryEntry> {
        let query_lower: String = query.to_lowercase();
        self.flights
            .iter()
            .filter(|entry| entry.name.to_lowercase().contains(&query_lower))
            .collect()
    }

    /// Returns all favourited Flights.
    pub fn favorites(&self) -> Vec<&RegistryEntry> {
        self.flights
            .iter()
            .filter(|entry| entry.is_favorite)
            .collect()
    }

    /// Returns the Flight that was most recently opened, if any.
    pub fn most_recent(&self) -> Option<&RegistryEntry> {
        self.flights
            .iter()
            .max_by(|a, b| a.last_opened_at.cmp(&b.last_opened_at))
    }

    /// Returns the total number of registered Flights.
    pub fn count(&self) -> usize {
        self.flights.len()
    }

    /// Returns the sum of all word counts across all Flights.
    pub fn total_word_count(&self) -> usize {
        self.flights.iter().map(|e| e.word_count).sum()
    }

    // ———————— Write Operations ————————

    /// Adds a new Flight to the registry.
    ///
    /// Does NOT write to disk — use [save_to_disk] after calling this.
    ///
    /// # Arguments
    ///
    /// - `name`: [`String`] - The name of the Flight.
    /// - `path`: [`String`] - The path to the Flight's directory.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The name is empty
    /// - The path is empty
    /// - A Flight already exists at the given path
    pub fn add(&mut self, name: String, path: String) -> Result<RegistryEntry, String> {
        let name: String = name.trim().into();
        let path: String = path.trim().into();

        if name.is_empty() {
            return Err("Flight name must not be empty.".into());
        }
        if path.is_empty() {
            return Err("Flight path must not be empty.".into());
        }

        // Check for duplicate path
        if self.flights.iter().any(|e| e.path == path) {
            return Err(format!("A Flight already exists at path: {}", path));
        }

        let now: DateTime<Utc> = Utc::now();

        let entry: RegistryEntry = RegistryEntry {
            id: Uuid::new_v4(),
            name,
            path,
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
    /// - `id`: [`&Uuid`](Uuid) - The ID of the Flight to remove.
    ///
    /// # Returns
    ///
    /// `true` if a Flight was found and removed, `false` otherwise.
    pub fn remove(&mut self, id: &Uuid) -> bool {
        let initial_len: usize = self.flights.len();
        self.flights.retain(|entry| entry.id != *id);
        self.flights.len() < initial_len
    }

    /// Updates an existing Flight's name and/or path.
    ///
    /// Does NOT write to disk — use [save_to_disk] after calling this.
    ///
    /// # Arguments
    ///
    /// - `id`: [`&Uuid`](Uuid) - The ID of the Flight to update.
    /// - `name`: [`String`] - The new name of the Flight.
    /// - `path`: [`String`] - The new path to the Flight's directory.
    ///
    /// # Errors
    ///
    /// Returns an error if the Flight isn't found.
    pub fn update(&mut self, id: &Uuid, name: String, path: String) -> Result<(), String> {
        let entry: &mut RegistryEntry = self
            .get_mut(id)
            .ok_or_else(|| format!("Flight not found: {}", id))?;

        let name: String = name.trim().into();
        let path: String = path.trim().into();

        if !name.is_empty() {
            entry.name = name;
        }
        if !path.is_empty() {
            entry.path = path;
        }

        Ok(())
    }

    /// Toggles the favourite status of a Flight.
    ///
    /// Does NOT write to disk — use [save_to_disk] after calling this.
    ///
    /// # Arguments
    ///
    /// - `id`: [`&Uuid`](Uuid) - The ID of the Flight to toggle.
    ///
    /// # Returns
    ///
    /// The new favourite status (`true` = now favourited).
    pub fn toggle_favorite(&mut self, id: &Uuid) -> Result<bool, String> {
        let entry: &mut RegistryEntry = self
            .get_mut(id)
            .ok_or_else(|| format!("Flight not found: {}", id))?;

        entry.is_favorite = !entry.is_favorite;
        Ok(entry.is_favorite)
    }

    /// Updates the `last_opened_at` timestamp for a Flight to now.
    ///
    /// Does NOT write to disk — use [save_to_disk] after calling this.
    ///
    /// # Arguments
    ///
    /// - `id`: [`&Uuid`](Uuid) - The ID of the Flight to update.
    ///
    /// # Returns
    ///
    /// `Ok(())` if the Flight was found and updated, an error otherwise.
    pub fn update_last_opened(&mut self, id: &Uuid) -> Result<(), String> {
        let entry: &mut RegistryEntry = self
            .get_mut(id)
            .ok_or_else(|| format!("Flight not found: {}", id))?;

        entry.last_opened_at = Utc::now();
        Ok(())
    }

    /// Updates the cached word count for a Flight.
    ///
    /// Does NOT write to disk — use [save_to_disk] after calling this.
    ///
    /// # Arguments
    ///
    /// - `id`: [`&Uuid`](Uuid) - The ID of the Flight to update.
    /// - `word_count`: [`usize`] - The new word count to set.
    ///
    /// # Returns
    ///
    /// `Ok(())` if the Flight was found and updated, an error otherwise.
    pub fn update_word_count(&mut self, id: &Uuid, word_count: usize) -> Result<(), String> {
        let entry: &mut RegistryEntry = self
            .get_mut(id)
            .ok_or_else(|| format!("Flight not found: {}", id))?;

        entry.word_count = word_count;
        Ok(())
    }

    // ———————— Validation ————————

    /// Validates the structural integrity of the registry.
    ///
    /// Checks:
    /// - Version is supported
    /// - All UUIDs are valid and non-nil
    /// - All names are non-empty
    /// - All paths are non-empty
    /// - No duplicate IDs
    /// - No duplicate paths
    pub fn validate(&self) -> Result<(), String> {
        // Check version
        if self.version == 0 || self.version > Self::CURRENT_VERSION {
            return Err(format!(
                "Unsupported registry version: {}. Expected version {}.",
                self.version,
                Self::CURRENT_VERSION
            ));
        }

        let mut seen_ids: Vec<Uuid> = Vec::with_capacity(self.flights.len());
        let mut seen_paths: Vec<&str> = Vec::with_capacity(self.flights.len());

        for entry in &self.flights {
            // UUID must be non-nil
            if entry.id.is_nil() {
                return Err("Found a Flight entry with a nil UUID.".into());
            }

            // No duplicate IDs
            if seen_ids.contains(&entry.id) {
                return Err(format!("Duplicate Flight ID found: {}", entry.id));
            }
            seen_ids.push(entry.id);

            // Name must be non-empty
            if entry.name.trim().is_empty() {
                return Err(format!("Flight {} has an empty name.", entry.id));
            }

            // Path must be non-empty
            if entry.path.trim().is_empty() {
                return Err(format!(
                    "Flight {} ({}) has an empty path.",
                    entry.name, entry.id
                ));
            }

            // No duplicate paths
            if seen_paths.contains(&entry.path.as_str()) {
                return Err(format!(
                    "Duplicate path found for Flight {} ({}): {}",
                    entry.name, entry.id, entry.path
                ));
            }
            seen_paths.push(&entry.path);
        }

        Ok(())
    }
}

impl Default for Registry {
    fn default() -> Self {
        Self {
            version: Self::CURRENT_VERSION,
            flights: Vec::new(),
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

impl fmt::Display for Registry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "— Entries:")?;
        for flight in &self.flights {
            writeln!(f, "{}", flight)?;
        }
        writeln!(f, "— Entry count: {}", self.count())?;
        writeln!(f, "— Total word count: {}", self.total_word_count())?;
        Ok(())
    }
}

impl fmt::Display for RegistryEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "——— Name: {}", self.name)?;
        writeln!(f, "——— ID: {}", self.id)?;
        writeln!(f, "——— Path: {}", self.path)?;
        writeln!(f, "——— Is Favorite: {}", self.is_favorite)?;
        writeln!(f, "——— Last Opened At: {}", self.last_opened_at)?;
        writeln!(f, "——— Created At: {}", self.created_at)?;
        writeln!(f, "——— Word Count: {}", self.word_count)?;
        Ok(())
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
/// - `dir`: [`&Path`](Path) - The directory containing the registry file.
///
/// # Errors
///
/// Returns an error if the file exists but cannot be parsed, or if
/// validation fails.
pub fn load_from_disk(dir: &Path) -> Result<Registry, String> {
    let path = dir.join(REGISTRY_FILENAME);

    // If the file doesn't exist yet, return a fresh registry
    if !path.exists() {
        return Ok(Registry::new());
    }

    // Read and parse the JSON file
    let content: String =
        fs::read_to_string(&path).map_err(|e| format!("Failed to read registry: {}", e))?;

    let registry: Registry = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse registry (corrupted?): {}", e))?;

    // Validate the loaded data
    registry
        .validate()
        .map_err(|e: String| format!("Registry validation failed: {}", e))?;

    Ok(registry)
}

/// Writes the registry to disk as pretty-printed JSON.
///
/// # Arguments
///
/// - `dir`: [`&Path`](Path) - The directory to write the registry file into.
/// - `registry`: [`&Registry`](Registry) - The registry to persist.
///
/// # Errors
///
/// Returns an error if validation fails or the file cannot be written.
pub fn save_to_disk(dir: &Path, registry: &Registry) -> Result<(), String> {
    // Validate before saving
    registry.validate()?;

    let path = dir.join(REGISTRY_FILENAME);

    let content: String = serde_json::to_string_pretty(registry)
        .map_err(|e| format!("Failed to serialize registry: {}", e))?;

    fs::write(&path, &content).map_err(|e| format!("Failed to write registry: {}", e))?;

    println!("Registry saved to: {:?}", path);
    Ok(())
}

// ============================================================================
// Tests (pure in-memory — no files created!)
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    /// Tests that a fresh registry starts empty.
    #[test]
    fn test_new_registry_is_empty() {
        let registry: Registry = Registry::new();
        println!("Registry:\n{}", registry);

        println!("Registry count: {:?}", registry.count());
        assert_eq!(registry.count(), 0);
        println!(
            "Registry total word count: {:?}",
            registry.total_word_count()
        );
        assert_eq!(registry.total_word_count(), 0);
        println!("Registry most recent: {:?}", registry.most_recent());
        assert!(registry.most_recent().is_none());
        println!("Registry favorites: {:?}", registry.favorites());
        assert!(registry.favorites().is_empty());
    }

    /// Tests adding a Flight to the registry (in memory only).
    #[test]
    fn test_add_flight() {
        let mut registry: Registry = Registry::new();

        let entry: RegistryEntry = registry
            .add("Ad Eternum".into(), "/home/user/ad_eternum.rachis".into())
            .expect("Failed to add flight");
        println!("Entry:\n{}", entry);

        println!("Entry name: {}", entry.name);
        assert_eq!(entry.name, "Ad Eternum");
        println!("Entry path: {}", entry.path);
        assert_eq!(entry.path, "/home/user/ad_eternum.rachis");
        println!("Entry is_favorite: {}", entry.is_favorite);
        assert!(!entry.is_favorite);
        println!("Registry count: {}", registry.count());
        assert_eq!(registry.count(), 1);
    }

    /// Tests that adding a Flight with an empty name fails.
    #[test]
    fn test_add_flight_empty_name() {
        let mut registry: Registry = Registry::new();

        let result: Result<RegistryEntry, String> =
            registry.add("   ".into(), "/path.rachis".into());
        println!("Result: {:?}", result);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("empty"));
    }

    /// Tests that adding a Flight with a duplicate path fails.
    #[test]
    fn test_add_flight_duplicate_path() {
        let mut registry: Registry = Registry::new();

        registry
            .add("Project A".into(), "/path.rachis".into())
            .expect("Failed to add first flight");

        let result: Result<RegistryEntry, String> =
            registry.add("Project B".into(), "/path.rachis".into());
        println!("Result: {:?}", result);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("already exists"));
    }

    /// Tests removing a Flight from the registry.
    #[test]
    fn test_remove_flight() {
        let mut registry: Registry = Registry::new();

        let entry: RegistryEntry = registry
            .add("Test".into(), "/test.rachis".into())
            .expect("Failed to add flight");
        println!("Entry:\n{}", entry);

        println!("Registry count: {:?}", registry.count());
        assert_eq!(registry.count(), 1);

        let removed: bool = registry.remove(&entry.id);
        println!("Removed: {:?}", removed);
        assert!(removed);
        println!("Registry count: {:?}", registry.count());
        assert_eq!(registry.count(), 0);

        // Removing a non-existent ID should return false
        let not_removed: bool = registry.remove(&Uuid::new_v4());
        assert!(!not_removed);
    }

    /// Tests toggling the favourite status.
    #[test]
    fn test_toggle_favorite() {
        let mut registry: Registry = Registry::new();

        let entry: RegistryEntry = registry
            .add("Test".into(), "/test.rachis".into())
            .expect("Failed to add flight");

        println!("Entry is_favorite: {:?}", entry.is_favorite);
        assert!(!entry.is_favorite);

        // Toggle on
        let new_status: bool = registry
            .toggle_favorite(&entry.id)
            .expect("Failed to toggle favorite");
        println!("Entry updated is_favorite: {:?}", new_status);
        assert!(new_status);

        println!("Favorites in registry: {:?}", registry.favorites());
        assert_eq!(registry.favorites().len(), 1);

        // Toggle off
        let new_status: bool = registry
            .toggle_favorite(&entry.id)
            .expect("Failed to toggle favorite");
        println!("Entry final is_favorite: {:?}", new_status);
        assert!(!new_status);
        println!("Favorites in registry: {:?}", registry.favorites());
        assert!(registry.favorites().is_empty());
    }

    /// Tests the most_recent() functionality.
    #[test]
    fn test_most_recent() {
        let mut registry: Registry = Registry::new();

        let first: RegistryEntry = registry
            .add("First".into(), "/first.rachis".into())
            .expect("Failed to add first");
        let second: RegistryEntry = registry
            .add("Second".into(), "/second.rachis".into())
            .expect("Failed to add second");

        // Second was the most recent addition, so it should be most recent
        let recent: &RegistryEntry = registry.most_recent().expect("No recent flight");
        println!("Recent flight: {:?}", recent);
        assert_eq!(recent.id, second.id);

        // Update last_opened for first
        std::thread::sleep(std::time::Duration::from_millis(10));
        registry
            .update_last_opened(&first.id)
            .expect("Failed to update last opened");

        // Now first should be most recent
        let recent: &RegistryEntry = registry.most_recent().expect("No recent flight");
        println!("Recent flight: {:?}", recent);
        assert_eq!(recent.id, first.id);
    }

    /// Tests searching by name (case-insensitive, partial match).
    #[test]
    fn test_search() {
        let mut registry: Registry = Registry::new();

        registry
            .add("Ad Eternum".into(), "/ad_eternum.rachis".into())
            .expect("Failed to add");
        registry
            .add("SE7ENFOLD".into(), "/se7enfold.rachis".into())
            .expect("Failed to add");
        registry
            .add("Sonder".into(), "/sonder.rachis".into())
            .expect("Failed to add");

        let results: Vec<&RegistryEntry> = registry.search("sonder");
        println!("Results for query 'sonder': {:?}", results);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Sonder");

        // Case-insensitive search
        let results: Vec<&RegistryEntry> = registry.search("SE7EN");
        println!("Results for query 'SE7EN': {:?}", results);
        assert_eq!(results.len(), 1);

        // Partial match
        let results: Vec<&RegistryEntry> = registry.search("n");
        println!("Results for query 'n': {:?}", results);
        assert_eq!(results.len(), 3);
    }

    /// Tests that validation rejects nil UUIDs.
    #[test]
    fn test_validation_nil_uuid() {
        let mut registry: Registry = Registry::new();

        registry.flights.push(RegistryEntry {
            id: Uuid::nil(),
            name: "NilUuid".into(),
            path: "/nil_uuid.rachis".into(),
            is_favorite: false,
            last_opened_at: Utc::now(),
            created_at: Utc::now(),
            word_count: 0,
        });
        println!("Registry after adding nil UUID: {:?}", registry);

        let result: Result<(), String> = registry.validate();
        println!("Validation result: {:?}", result);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("nil UUID"));
    }

    /// Tests that validation catches duplicate IDs.
    #[test]
    fn test_validation_duplicate_id() {
        let mut registry: Registry = Registry::new();

        let id: Uuid = Uuid::new_v4();
        registry.flights.push(RegistryEntry {
            id,
            name: "First".into(),
            path: "/first.rachis".into(),
            is_favorite: false,
            last_opened_at: Utc::now(),
            created_at: Utc::now(),
            word_count: 0,
        });
        println!("Registry after adding first entry:\n{}", registry);
        registry.flights.push(RegistryEntry {
            id,
            name: "Second".into(),
            path: "/second.rachis".into(),
            is_favorite: false,
            last_opened_at: Utc::now(),
            created_at: Utc::now(),
            word_count: 0,
        });
        println!("Registry after adding second entry:\n{}", registry);

        let result: Result<(), String> = registry.validate();
        println!("Validation result: {:?}", result);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Duplicate"));
    }

    // ———————— Persistence Tests (explicitly test I/O) ————————

    /// Tests that a non-existent file creates a fresh registry.
    #[test]
    fn test_load_missing_file_creates_empty() {
        let dir: PathBuf = std::env::temp_dir().join(format!("registry_test_{}", Uuid::new_v4()));
        println!("Test directory: {:?}", dir);
        std::fs::create_dir_all(&dir).expect("Failed to create test directory");
        // Clean the directory before loading to force an error
        defer_cleanup(&dir);

        let registry: Registry = load_from_disk(&dir).expect("Failed to load registry");
        println!("Registry after loading missing file:\n{}", registry);
        assert_eq!(registry.count(), 0);
    }

    /// Tests a full round-trip: create in memory, persist, load back.
    #[test]
    fn test_round_trip() {
        let dir: PathBuf = std::env::temp_dir().join(format!("registry_test_{}", Uuid::new_v4()));
        std::fs::create_dir_all(&dir).expect("Failed to create test directory");

        // Create and populate in memory
        let mut registry: Registry = Registry::new();
        registry
            .add("Alphabet Coup".into(), "/alphabet_coup.rachis".into())
            .expect("Failed to add");
        registry
            .add("Vivisection".into(), "/vivisection.rachis".into())
            .expect("Failed to add");
        println!("Registry after adding entries:\n{}", registry);

        let first_id: Uuid = registry.list()[0].id;
        registry
            .toggle_favorite(&first_id)
            .expect("Failed to toggle");
        println!("Registry after toggling favorite:\n{}", registry);

        // Persist
        let result = save_to_disk(&dir, &registry);
        assert!(
            result.is_ok(),
            "Failed to save registry: {}",
            result.err().unwrap()
        );

        // Load and verify
        let loaded: Registry = load_from_disk(&dir).expect("Failed to load");
        println!("Registry after loading from disk:\n{}", loaded);
        assert_eq!(loaded.count(), 2);

        let favorites: Vec<&RegistryEntry> = loaded.favorites();
        println!("Favorites:\n{:?}", favorites);
        assert_eq!(favorites.len(), 1);
        assert_eq!(favorites[0].name, "Alphabet Coup");

        // Clean up
        defer_cleanup(&dir);
    }

    #[test]
    fn test_default() {
        let registry: Registry = Registry::default();
        assert_eq!(registry.version, 1);
        assert!(registry.flights.is_empty());

        let entry: RegistryEntry = RegistryEntry::default();
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
    fn defer_cleanup(dir: &Path) {
        let _ = std::fs::remove_dir_all(dir);
    }
}
