use {
    chrono::{DateTime, Utc},
    serde::{Deserialize, Serialize},
    ts_rs::TS,
    uuid::Uuid,
};

/// A Flight is the top-level container — the folder that contains everything related to one
/// specific writing project.
///
/// # Fields
///
/// - `id`: [Uuid] - The unique identifier for the Flight.
/// - `name`: [String] - The name given by the user to the Flight.
/// - `created_at`: [DateTime<Utc>] - The time the Flight was created.
/// - `updated_at`: [DateTime<Utc>] - The time the Flight was last updated.
/// - `is_favorite`: [bool] - Whether this Flight is favourited by the user.
///
/// TODO: Develop better documentation for Flight struct
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Flight {
    /// Unique identifier for the Flight
    pub id: Uuid,
    /// A name given by the user to the Flight
    pub name: String,
    /// The time the Flight was created
    pub created_at: DateTime<Utc>,
    /// The time the Flight was last updated
    pub updated_at: DateTime<Utc>,
    /// Whether this Flight is favourited by the user
    pub is_favorite: bool,
}
