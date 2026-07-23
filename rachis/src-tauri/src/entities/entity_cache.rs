/// The `entity_cache` table stores parsed tag data for fast lookups.
///
/// Tags are parsed on-read from content files. This caches results so entity
/// lookups (for tag auto-completion, landing page stats, etc.) are fast, and
/// should be rebuilt when content is saved.

use serde::{Deserialize, Serialize};

/// The `entity_cache` model, storing cached parsed tag data.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct EntityCache {
    /// UUID identifying this cached tag entry
    pub id: String,
    /// Relative path of the file this tag was parsed from
    pub file_path: String,
    /// The raw tag text as it appears in the source file
    pub tag_text: String,
    /// Tag prefix character: `"c"`, `"e"`, `"l"`, `"i"`, `"n"`, or `None` for custom tags
    pub prefix: Option<String>,
    /// The resolved entity name from the tag
    pub entity_name: String,
    /// Optional display text from the tag
    pub display_text: Option<String>,
    /// Defines the lock status of this entity's name
    ///
    /// - `None` - Any entity may share the same name as this
    /// - `Some(false)` - No entity of the same type/all entities of a different type may share the same name as this
    /// - `Some(true)` - No entity may share the same name as this
    pub lock_is_global: Option<bool>,
}
