/// The `flight_meta` table stores the identity of a single writing project (Flight).
///
/// Each `.flight` file maps to exactly one row in this table.
use {
    crate::entities::files::ProjectFile,
    serde::{Deserialize, Serialize},
    ts_rs::TS,
};

/// The `flight_meta` model. Maps to the `flight_meta` table in the `.flight`
/// SQLite database.
///
/// # Fields
///
/// - `id`: [`String`] - UUID identifying this Flight (stored as TEXT in SQLite)
/// - `name`: [`String`] - Human-readable name given by the user
/// - `created_at`: [`i64`] - Unix epoch seconds when this Flight was created
/// - `updated_at`: [`i64`] - Unix epoch seconds when this Flight was last modified
/// - `files`: [`Vec<ProjectFile>`](ProjectFile) - List of files associated with this Flight
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct FlightMetadata {
    /// UUID identifying this Flight (stored as TEXT in SQLite)
    pub id: String,
    /// Human-readable name given by the user
    pub name: String,
    /// Unix epoch seconds when this Flight was created
    pub created_at: i64,
    /// Unix epoch seconds when this Flight was last modified
    pub updated_at: i64,
    /// List of files associated with this Flight
    pub files: Vec<ProjectFile>,
}

impl FlightMetadata {
    pub fn new(
        id: String,
        name: String,
        created_at: i64,
        updated_at: i64,
        files: Vec<ProjectFile>,
    ) -> Self {
        Self {
            id,
            name,
            created_at,
            updated_at,
            files,
        }
    }
}
