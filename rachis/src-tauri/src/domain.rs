use std::fmt;

use chrono::{DateTime, Timelike, Utc};
use uuid::Uuid;

/// A Flight is the top-level container — the folder that contains everything related to one
/// specific writing project.
///
/// # Fields
///
/// * `id` - The unique identifier for the Flight.
/// * `name` - The name given by the user to the Flight.
/// * `created_at` - The time the Flight was created.
/// * `updated_at` - The time the Flight was last updated.
///
/// TODO: Develop better documentation for Flight struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)] // Allows Flight structs to utilise Debug and Clone traits
pub struct Flight {
    /// Unique identifier for the Flight
    pub id: Uuid,
    /// A name given by the user to the Flight
    pub name: String,
    /// The time the Flight was created
    pub created_at: DateTime<Utc>,
    /// The time the Flight was last updated
    pub updated_at: DateTime<Utc>,
}

/// Implementation of the Flight struct
///
/// # Functions
///
/// * [Flight::new](Flight::new) - Creates a new Flight with the given name.
/// * [Flight::update_name](Flight::update_name) - Updates the Flight's name and modification timestamp
///
/// # Examples
///
/// TODO: Generate examples for Flight
impl Flight {
    /// Creates a new Flight with the given name.
    ///
    /// # Arguments
    ///
    /// * `name: String` - The name of the [Flight](Flight).
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
        }
    }

    /// Updates the Flight's name and modification timestamp
    /// Takes `&mut self` since the struct needs to be mutable
    ///
    /// # Arguments
    ///
    /// * `new_name: String` - The new name of the Flight
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

/// Defines the types of Rachises that may exist.
/// Rachis types come in primarily two groups: Generic Rachises and Entity Rachises.
/// Generic Rachis types are more meta and describe the story itself, while Entity Rachises
/// define some recognisable entity in the story.
///
/// # Types
///
/// * [RachisType.Act](RachisType.Act) - An Act Rachis. Conceptualised as the largest unit of the story, encapsulating multiple arcs.
/// * [RachisType.Arc](RachisType.Arc) - An Arc Rachis. Conceptualised as a collection of Scenes and comprise a character's journey through the story.
/// * [RachisType.Scene](RachisType.Scene) - A Scene Rachis. Conceptualised as a specific event or moment during the story.
/// * [RachisType.Default](RachisType.Default) - A default Rachis
/// * [RachisType.Character](RachisType.Character) - A Character Rachis. Conceptualised as a specific person, character, or otherwise entity in the story.
/// * [RachisType.Event](RachisType.Event) - An Event Rachis. Conceptualised as a specific happenstance in the story.
/// * [RachisType.Location](RachisType.Location) - A Location Rachis. Conceptualised as a specific place or area in the story.
/// * [RachisType.Item](RachisType.Item) - An Item Rachis. Conceptualised as a specific object or thing in the story.
/// * [RachisType.Note](RachisType.Note) - A Note Rachis. Conceptualised as a author-level commentary or note on the story.
#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum RachisType {
    // Generic Rachis types
    // These are more meta and describe the story itself
    /// An Act Rachis. Conceptualised as the largest unit of the story, encapsulating multiple arcs.
    Act,
    /// An Arc Rachis. Conceptualised as a collection of Scenes and comprise a character's journey through the story.
    Arc,
    /// A Scene Rachis. Conceptualised as a specific event or moment during the story.
    Scene,
    /// A default Rachis
    Default,

    // Entity Rachis types
    // These are Rachises that define some recognisable entity in the story
    /// A Character Rachis. Conceptualised as a specific person, character, or otherwise entity in the story.
    Character,
    /// An Event Rachis. Conceptualised as a specific happenstance in the story.
    Event,
    /// A Location Rachis. Conceptualised as a specific place or area in the story.
    Location,
    /// An Item Rachis. Conceptualised as a specific object or thing in the story.
    Item,
    /// A Note Rachis. Conceptualised as a author-level commentary or note on the story.
    Note,
}

impl RachisType {
    /// Returns the string representation of the [RachisType].
    ///
    /// # Returns
    ///
    /// * `&'static str` - The string representation of the [RachisType].
    ///
    /// # Examples
    ///
    /// TODO: Generate examples for RachisType::as_str()
    pub fn as_str(&self) -> &'static str {
        match self {
            RachisType::Act => "Act",
            RachisType::Arc => "Arc",
            RachisType::Scene => "Scene",
            RachisType::Default => "Default",
            RachisType::Character => "Character",
            RachisType::Event => "Event",
            RachisType::Location => "Location",
            RachisType::Item => "Item",
            RachisType::Note => "Note",
        }
    }

    /// Creates a [RachisType] from a string.
    ///
    /// # Arguments
    ///
    /// * `s: &str` - The string to create the [RachisType] from.
    ///
    /// # Returns
    ///
    /// * `Result<RachisType, String>` - The [RachisType] created from the string, or an error message if the string is invalid.
    ///
    /// # Examples
    ///
    /// TODO: Generate examples for RachisType::from_str()
    pub fn from_str(s: &str) -> Result<RachisType, String> {
        match s {
            "Act" | "act" => Ok(RachisType::Act),
            "Arc" | "arc" => Ok(RachisType::Arc),
            "Scene" | "scene" => Ok(RachisType::Scene),
            "Default" | "default" => Ok(RachisType::Default),
            "Character" | "character" => Ok(RachisType::Character),
            "Event" | "event" => Ok(RachisType::Event),
            "Location" | "location" => Ok(RachisType::Location),
            "Item" | "item" => Ok(RachisType::Item),
            "Note" | "note" => Ok(RachisType::Note),
            _ => Err(format!("Invalid RachisType: {}", s)),
        }
    }
}

impl fmt::Display for RachisType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                RachisType::Act => "Act",
                RachisType::Arc => "Arc",
                RachisType::Scene => "Scene",
                RachisType::Default => "Default",
                RachisType::Character => "Character",
                RachisType::Event => "Event",
                RachisType::Location => "Location",
                RachisType::Item => "Item",
                RachisType::Note => "Note",
            }
        )
    }
}

/// Defines the structure for a Rachis object.
///
/// # Fields
///
/// * `id: Uuid` - The unique identifier for the Rachis.
/// * `flight_id: Uuid` - The ID of the Flight that this Rachis belongs to.
/// * `title: String` - The title of the Rachis.
/// * `content: String` - The content of the Rachis.
/// * `r#type: RachisType` - The type of the Rachis.
/// * `path: String` - The path of the Rachis.
/// * `created_at: DateTime<Utc>` - The time the Rachis was created.
/// * `updated_at: DateTime<Utc>` - The time the Rachis was last updated.
/// * `word_count: usize` - The word count of the Rachis.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Rachis {
    /// The unique identifier for the Rachis.
    pub id: Uuid,
    /// The ID of the Flight that this Rachis belongs to.
    pub flight_id: Uuid,
    /// The title of the Rachis.
    pub title: String,
    /// The content of the Rachis.
    pub content: String,
    /// The type of the Rachis.
    /// See [RachisType](RachisType)
    pub r#type: RachisType,
    /// The path of the Rachis.
    pub path: String,
    /// The time the Rachis was created.
    pub created_at: DateTime<Utc>,
    /// The time the Rachis was last updated.
    pub updated_at: DateTime<Utc>,
    /// The word count of the Rachis.
    pub word_count: usize,
}

/// Methods for the Rachis struct.
///
/// # Functions
///
/// * [Rachis::new](Rachis::new) - Creates a new Rachis.
/// * [Rachis::update_name](Rachis::update_name) - Updates the name of the Rachis.
/// * [Rachis::update_content](Rachis::update_content) - Updates the content of the Rachis.
/// * [Rachis::update_word_count](Rachis::update_word_count) - Updates the word count of the Rachis.
impl Rachis {
    /// Creates a new Rachis.
    ///
    /// # Arguments
    ///
    /// * `flight_id: Uuid` - The ID of the Flight that this Rachis belongs to.
    /// * `title: String` - The title of the Rachis.
    /// * `r#type: [RachisType](RachisType)` - The type of the Rachis.
    /// * `path: String` - The path of the Rachis.
    ///
    /// # Returns
    ///
    /// [Rachis](Rachis) - The newly generated Rachis.
    ///
    /// # Examples
    ///
    /// TODO: Generate examples for Rachis::new()
    pub fn new(flight_id: Uuid, title: String, r#type: RachisType, path: String) -> Self {
        // Get the current timestamp
        let now: DateTime<Utc> = Utc::now().with_nanosecond(0).unwrap();

        // Return a new Rachis
        Self {
            id: Uuid::new_v4(),
            flight_id,
            title,
            content: String::from(""),
            r#type,
            path,
            created_at: now,
            updated_at: now,
            word_count: 0,
        }
    }

    /// Updates the name of this Rachis.
    ///
    /// # Arguments
    ///
    /// * `title: String` - The new title of the Rachis.
    ///
    /// # Returns
    ///
    /// `Result<(), String>` - Returns an error if the title is empty, otherwise returns Ok(()).
    ///
    /// # Examples
    ///
    /// TODO: Generate examples for Rachis::update_name()
    pub fn update_name(&mut self, mut title: String) -> Result<(), String> {
        // Trim the title
        title = title.trim().to_string();

        // Return an error if the title is empty
        if title.is_empty() {
            return Err(String::from("Rachis name must not be empty!"));
        }

        // Update the title
        self.title = title;
        // Update the updated_at timestamp
        self.updated_at = Utc::now().with_nanosecond(0).unwrap();

        // Return an OK result
        Ok(())
    }

    /// Updates the content of this Rachis.
    ///
    /// # Arguments
    ///
    /// * `new_content: String` - The new content of the Rachis.
    ///
    /// # Returns
    ///
    /// * `Result<(), String>` - Returns an error if the content is empty, otherwise returns Ok(()).
    ///
    /// # Examples
    ///
    /// TODO: Generate examples for Rachis::update_content()
    pub fn update_content(&mut self, new_content: String) -> Result<(), String> {
        // Set self content to the new content
        self.content = new_content;
        self.word_count = self.word_count();

        // Return an OK result
        Ok(())
    }

    /// Gets the word count of this Rachis.
    ///
    /// # Returns
    ///
    /// * `usize` - The word count of this Rachis.
    ///
    /// # Examples
    ///
    /// TODO: Generate examples for Rachis::word_count()
    pub fn word_count(&mut self) -> usize {
        // Update word count of this Rachis
        self.content.trim().split_whitespace().count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests a [Flight's](Flight) creation.
    #[test]
    fn test_flight_creation() {
        let flight: Flight = Flight::new(String::from("My Writing Project"));

        // Verify that all fields are set
        assert_eq!(flight.name, "My Writing Project");
        assert!(flight.created_at <= flight.updated_at);

        // Test Flight updating
        let mut flight: Flight = flight; // Make mutable copy
        assert!(flight.update_name(String::from("Better Title")).is_ok()); // Change title
        assert_eq!(flight.name, "Better Title"); // New title should be "Better Title"
        assert!(flight.created_at < flight.updated_at); // Updated timestamp should be greater
    }

    /// Tests a [Flight's](Flight) round-trip through JSON.
    #[test]
    fn test_flight_json() {
        // Create a Flight, serialise it to JSON, then deserialise it
        let flight: Flight = Flight::new(String::from("My Writing Project"));
        let json: String = serde_json::to_string(&flight).unwrap();
        let deserialised: Flight = serde_json::from_str(&json).unwrap();
        // Assert that the deserialised Flight is the same as the original
        assert_eq!(flight.id, deserialised.id);
        assert_eq!(flight.name, deserialised.name);
        assert_eq!(
            flight.created_at.timestamp(),
            deserialised.created_at.timestamp()
        );
    }

    #[test]
    fn test_rachis_creation() {
        let flight: Flight = Flight::new(String::from("Hold My Rachis"));
        let rachis: Rachis = Rachis::new(
            flight.id,
            String::from("Hello, Rachis!"),
            RachisType::Default,
            String::from("Story"),
        );

        // Assert Rachis flight_id == Flight id
        assert_eq!(rachis.flight_id, flight.id);
        // Assert Rachis title == "Hello, Rachis!"
        assert_eq!(rachis.title, "Hello, Rachis!");
        // Assert Rachis path == "Story"
        assert_eq!(rachis.path, "Story");
        // Assert Rachis type == Default
        assert_eq!(rachis.r#type, RachisType::Default);
        // Assert Rachis content == ""
        assert_eq!(rachis.content, "");
        // Assert Rachis word count == 0
        assert_eq!(rachis.word_count, 0)
    }

    #[test]
    fn test_rachis_updates() {
        let flight: Flight = Flight::new(String::from("Hold My Rachis"));
        let mut rachis: Rachis = Rachis::new(
            flight.id,
            String::from("Hello, Rachis!"),
            RachisType::Default,
            String::from("Story"),
        );

        // Assert name update is OK
        assert!(rachis
            .update_name(String::from("My Little Pony: Ad Eternum"))
            .is_ok());
        // Assert content update is OK
        assert!(rachis
            .update_content(String::from(
                "Twilight woke up for the third time in only two weeks."
            ))
            .is_ok());
    }
}
