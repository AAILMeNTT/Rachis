/// The `files` table indexes every content file in the project directory,
/// serving metadata only.
///
/// Each row represents one file (`.rachis`, `.md`, `.bbcode`, etc.) that
/// belongs to this Flight.
use {
    serde::{Deserialize, Serialize},
    ts_rs::TS,
};

/// The `files` model. Serves as an index of every content file in the project directory.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TS)]
pub struct ProjectFile {
    /// UUID identifying this file in the `.flight` database (stable across renames)
    pub id: String,
    /// Relative path from the project root, e.g. `"chapters/Chapter 2.rachis"`
    pub path: String,
    /// Display title of the file (derived from filename or user-set)
    pub title: String,
    /// Entity type, if this file represents a story entity.
    /// `None` (NULL) for non-entity files like chapter drafts.
    pub entity_type: Option<String>,
    /// Cached word count for the file
    pub word_count: u32,
    /// Unix epoch seconds of the last file modification time
    pub last_modified: i64,
    /// Whether this is a `.rachis` native file (true) or an imported format (false)
    pub is_native: bool,
}
