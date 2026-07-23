use {
    serde::{Deserialize, Serialize},
    std::path::{Path, PathBuf},
    ts_rs::TS,
    uuid::Uuid,
};

/// Defines the structure for a Rachis object. This struct is only constructed as a payload from backend to provide a central point of information about a Rachis.
///
/// # Fields
///
/// - `id`: [`Uuid`] - The unique identifier for the Rachis.
/// - `flight_id`: [`Uuid`] - The ID of the Flight that this Rachis belongs to.
/// - `title`: [`String`] - The title of the Rachis.
/// - `r#type`: [`RachisType`] - The type of the Rachis.
/// - `path`: [`PathBuf`] - The path of the Rachis.
/// - `tags`: [`Vec<String>`](String) - The tags of the Rachis.
/// - `word_count`: [`u32`] - The word count of the Rachis.
///
/// # Functions
///
/// - [`new()`](Rachis::new) - Creates a new Rachis.
/// - [`id()`](Rachis::id) - Returns the ID of the Rachis.
/// - [`flight_id()`](Rachis::flight_id) - Returns the ID of the Flight that this Rachis belongs to.
/// - [`title()`](Rachis::title) - Returns the title of the Rachis.
/// - [`r#type()`](Rachis::r#type) - Returns the type of the Rachis.
/// - [`path()`](Rachis::path) - Returns the path of the Rachis.
/// - [`word_count()`](Rachis::word_count) - Returns the word count of the Rachis.
/// - [`tags()`](Rachis::tags) - Returns the tags of the Rachis.
#[derive(Debug, Clone, Serialize, PartialEq, Eq, TS)]
#[ts(export)]
pub struct Rachis {
    /// The unique identifier for the Rachis.
    pub id: Uuid,
    /// The ID of the Flight that this Rachis belongs to.
    pub flight_id: Uuid,
    /// The title of the Rachis.
    pub title: String,
    /// The type of the Rachis (see [RachisType]).
    pub r#type: RachisType,
    /// The path of the Rachis.
    pub path: PathBuf,
    /// The tags that are used in the Rachis.
    pub tags: Vec<String>, // TODO: Change to Vec<Tag> at some point
    /// The word count of the Rachis.
    pub word_count: u32,
}

impl Rachis {
    /// Creates a new Rachis.
    ///
    /// # Arguments
    ///
    /// - `id`: [`Uuid`] - The unique identifier for the Rachis.
    /// - `flight_id`: [`Uuid`] - The ID of the Flight that this Rachis belongs to.
    /// - `title`: [`String`] - The title of the Rachis.
    /// - `r#type`: [`RachisType`] - The type of the Rachis.
    /// - `path`: [`PathBuf`] - The path of the Rachis.
    /// - `tags`: [`Vec<String>`](String) - The tags that are used in the Rachis.
    /// - `word_count`: [`u32`] - The word count of the Rachis.
    pub fn new(
        id: Uuid,
        flight_id: Uuid,
        title: impl AsRef<str>,
        r#type: RachisType,
        path: impl AsRef<Path>,
        tags: Vec<impl AsRef<str>>,
        word_count: u32,
    ) -> Self {
        Self {
            id,
            flight_id,
            title: title.as_ref().to_string(),
            r#type,
            path: path.as_ref().to_path_buf(),
            tags: tags.into_iter().map(|t| t.as_ref().to_string()).collect(),
            word_count,
        }
    }
}

/// Defines the types of Rachises that may exist.
///
/// Rachis types come in primarily two groups: Generic Rachises and Entity Rachises.
/// Generic Rachis types are more meta and describe the story itself, while Entity Rachises
/// define some recognisable entity in the story.
///
/// # Types
///
/// ## Generic Rachis Types
///
/// - [`RachisType::ACT`](RachisType::ACT) - An Act Rachis. Conceptualised as the largest unit of the story, encapsulating multiple arcs.
/// - [`RachisType::ARC`](RachisType::ARC) - An Arc Rachis. Conceptualised as a collection of Scenes and comprise a character's journey through the story.
/// - [`RachisType::SCENE`](RachisType::SCENE) - A Scene Rachis. Conceptualised as a specific event or moment during the story.
/// - [`RachisType::DEFAULT`](RachisType::DEFAULT) - A default Rachis
///
/// ## Entity Rachis Types
///
/// - [`RachisType::CHARACTER`](RachisType::CHARACTER) - A Character Rachis. Conceptualised as a specific person, character, or otherwise entity in the story.
/// - [`RachisType::EVENT`](RachisType::EVENT) - An Event Rachis. Conceptualised as a specific happenstance in the story.
/// - [`RachisType::LOCATION`](RachisType::LOCATION) - A Location Rachis. Conceptualised as a specific place or area in the story.
/// - [`RachisType::ITEM`](RachisType::ITEM) - An Item Rachis. Conceptualised as a specific object or thing in the story.
/// - [`RachisType::NOTE`](RachisType::NOTE) - A Note Rachis. Conceptualised as a author-level commentary or note on the story.
///
/// # Functions
///
/// - [`as_str()`](RachisType::as_str) - Returns the string representation of the [RachisType].
/// - [`from_str()`](RachisType::from_str) - Returns a [RachisType] from a string representation.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, TS, Default)]
#[ts(export, repr(enum = name))]
pub enum RachisType {
    // Generic Rachis types
    // These are more meta and describe the story itself
    /// An Act [Rachis]. Conceptualised as the largest unit of the story, encapsulating multiple arcs.
    ACT,
    /// An Arc [Rachis]. Conceptualised as a collection of Scenes and comprise a character's journey through the story.
    ARC,
    /// A Scene [Rachis]. Conceptualised as a specific event or moment during the story.
    SCENE,
    /// A default [Rachis]
    #[default]
    DEFAULT,

    // Entity Rachis types
    // These are Rachises that define some recognisable entity in the story
    /// A Character [Rachis]. Conceptualised as a specific person, character, or otherwise entity in the story.
    CHARACTER,
    /// An Event [Rachis]. Conceptualised as a specific happenstance in the story.
    EVENT,
    /// A Location [Rachis]. Conceptualised as a specific place or area in the story.
    LOCATION,
    /// An Item [Rachis]. Conceptualised as a specific object or thing in the story.
    ITEM,
    /// A Note [Rachis]. Conceptualised as a author-level commentary or note on the story.
    NOTE,
}
