/// The `flight_meta` table stores the identity of a single writing project (Flight).
///
/// Each `.flight` file maps to exactly one row in this table.

use serde::{Deserialize, Serialize};

/// The `flight_meta` model. Maps to the `flight_meta` table in the `.flight` SQLite database.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FlightMetadata {
    /// UUID identifying this Flight (stored as TEXT in SQLite)
    pub id: String,
    /// Human-readable name given by the user
    pub name: String,
    /// Unix epoch seconds when this Flight was created
    pub created_at: i64,
    /// Unix epoch seconds when this Flight was last modified
    pub updated_at: i64,
}
