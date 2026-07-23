/// The `workspace_layouts` table stores snapshots of the workspace widget tree.
///
/// Each row represents one saved layout for the workspace.
///
/// NOT YET UTILISED

use serde::{Deserialize, Serialize};

/// The `workspace_layouts` model — saved workspace tree snapshots.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct WorkspaceLayout {
    /// UUID identifying this layout (stored as TEXT)
    pub id: String,
    /// Human-readable name for this layout, e.g. `"default"`, `"outline-focus"`
    pub name: String,
    /// The full workspace tree serialized as a JSON string
    pub tree_json: String,
    /// Unix epoch seconds when this layout was saved
    pub saved_at: i64,
}
