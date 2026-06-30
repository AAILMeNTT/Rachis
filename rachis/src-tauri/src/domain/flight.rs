use chrono::{DateTime, Timelike, Utc};
use ts_rs::TS;
use uuid::Uuid;

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
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, TS)]
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

/// Implementation of the Flight struct
///
/// # Functions
///
/// - [new()](Flight::new) - Creates a new Flight with the given name.
/// - [update_name()](Flight::update_name) - Updates the Flight's name and modification timestamp
///
/// # Examples
///
/// TODO: Generate examples for Flight
impl Flight {
    /// Creates a new Flight with the given name.
    ///
    /// # Arguments
    ///
    /// - `name: String` - The name of the [Flight](Flight).
    ///
    /// # Returns
    ///
    /// The newly generated [Flight].
    ///
    /// # Examples
    ///
    /// TODO: Create example for Flight::new()
    pub fn new(name: String) -> Self {
        // let now: DateTime<Utc> = Utc::now();
        // The above saves subsecond information, whereas I only want to preserve up to the seconds place
        let now: DateTime<Utc> = Utc::now().with_nanosecond(0).unwrap();

        Self {
            id: Uuid::new_v4(),
            name,
            created_at: now,
            updated_at: now,
            is_favorite: false,
        }
    }

    /// Updates the Flight's name and modification timestamp.
    ///
    /// Takes `&mut self` since the struct needs to be mutable.
    ///
    /// # Arguments
    ///
    /// - `new_name`: [String] - The new name of the Flight
    ///
    /// # Examples
    ///
    /// TODO: Generate examples for Flight::update_name()
    pub fn update_name(&mut self, mut new_name: String) -> Result<(), String> {
        // Trim the new name
        new_name = new_name.trim().to_string();

        // Return error if the new name is empty
        if new_name.is_empty() {
            return Err(String::from("Flight name must not be empty"));
        }

        // Update the name and updated_at timestamp
        self.name = new_name;
        self.updated_at = Utc::now().with_nanosecond(0).unwrap();

        // Return success
        Ok(())
    }
}
