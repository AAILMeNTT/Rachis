use chrono::{DateTime, Utc};
use rusqlite::{self, CachedStatement, Connection, Error, Result, Row};
use uuid::Uuid;

use crate::domain::{Flight, Rachis, RachisType};

#[derive(Debug)]
pub struct Database {
    conn: Connection,
}

impl Database {
    /// Opens a new connection to the [Database] at the given path.
    ///
    /// # Arguments
    ///
    /// * `path: &str` - The path to the database file.
    ///
    /// # Returns
    ///
    /// * `Result<Database>` - The Database struct, or an error if the connection fails.
    ///
    /// # Examples
    ///
    /// TODO: Generate examples for Database::open()
    pub fn open(path: &str) -> Result<Self> {
        let conn: Connection = Connection::open(path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS flight (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL
            )",
            (),
        )?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS rachises (
                id TEXT PRIMARY KEY,
                flight_id TEXT NOT NULL,
                title TEXT NOT NULL,
                content TEXT NOT NULL,
                type TEXT NOT NULL,
                path TEXT NOT NULL,
                word_count INTEGER NOT NULL,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL
            )",
            (),
        )?;

        Ok(Database { conn })
    }

    // ———————— Flight CRUD ————————

    /// Returns the [Flight] for this [Database].
    ///
    /// # Returns
    ///
    /// * `Result<Flight>` - The Flight struct, or an error if the query fails.
    ///
    /// # Examples
    ///
    /// TODO: Generate examples for Database::get_flight()
    pub fn get_flight(&self) -> Result<Option<Flight>, Error> {
        let mut statement: CachedStatement<'_> = self
            .conn
            .prepare_cached("SELECT id, name, created_at, updated_at FROM flight LIMIT 1")?;

        let flight: Result<Flight, Error> = statement.query_row([], |row: &Row<'_>| {
            // Get the values of each column
            let id_str: String = row.get("id")?; // id
            let name: String = row.get("name")?; // name
            let created_at_timestamp: i64 = row.get("created_at")?; // created_at
            let updated_at_timestamp: i64 = row.get("updated_at")?; // updated_at

            // Convert to Uuid, returning a conversion error on failure
            let id: Uuid = Uuid::parse_str(&id_str)
                .map_err(|e: uuid::Error| Error::ToSqlConversionFailure(Box::new(e)))?;

            // Convert to DateTime<Utc>, returning an invalid parameter error on failure
            let created_at: DateTime<Utc> = DateTime::from_timestamp(created_at_timestamp, 0)
                .ok_or_else(|| {
                    Error::InvalidParameterName(String::from("Invalid created_at timestamp"))
                })?;
            let updated_at: DateTime<Utc> = DateTime::from_timestamp(updated_at_timestamp, 0)
                .ok_or_else(|| {
                    Error::InvalidParameterName(String::from("Invalid updated_at timestamp"))
                })?;

            // Return the Flight
            Ok(Flight {
                id,
                name,
                created_at,
                updated_at,
            })
        });

        match flight {
            Ok(flight) => Ok(Some(flight)),
            Err(Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e),
        }
    }

    /// Inserts a [Flight] into the database.
    ///
    /// # Arguments
    ///
    /// * `flight: &Flight` - The Flight to insert.
    ///
    /// # Examples
    ///
    /// TODO: Generate examples for Database::create_flight()
    pub fn create_flight(&self, flight: &Flight) -> Result<(), Error> {
        let mut statement: CachedStatement<'_> = self.conn.prepare_cached(
            "INSERT INTO flight (id, name, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)",
        )?;

        statement.execute(&[
            &flight.id.to_string(),
            &flight.name,
            &flight.created_at.timestamp().to_string(),
            &flight.updated_at.timestamp().to_string(),
        ])?;

        Ok(())
    }

    /// Updates the [Flight] in this [Database].
    ///
    /// # Arguments
    ///
    /// * `flight: &Flight` - The Flight to update.
    ///
    /// # Examples
    ///
    /// TODO: Generate examples for Database::update_flight()
    pub fn update_flight(&self, flight: &Flight) -> Result<(), Error> {
        // Using the current flight as a sort of template
        let curr_flight: Flight = self.get_flight()?.ok_or(Error::QueryReturnedNoRows)?;
        // The "WHERE" here is technically not needed since there should only ever be one Flight per Database, but... trust no one
        let mut statement: CachedStatement<'_> = self
            .conn
            .prepare_cached("UPDATE flight SET name = ?1, updated_at = ?2 WHERE id = ?3")?;

        // As of right now, the only thing the user can update about a Flight is its name
        // updated_at is automatically managed, and both the id and created_at should be immutable
        statement.execute(&[
            &flight.name,
            &Utc::now().timestamp().to_string(),
            &curr_flight.id.to_string(),
        ])?;

        Ok(())
    }

    /// Deletes the [Flight] with the given ID.
    ///
    /// # Arguments
    ///
    /// * `id: &Uuid` - The ID of the Flight to delete.
    ///
    /// # Examples
    ///
    /// TODO: Generate examples for Database::delete_flight()
    pub fn delete_flight(&self, id: &Uuid) -> Result<(), Error> {
        let mut statement: CachedStatement<'_> = self
            .conn
            .prepare_cached("DELETE FROM flight WHERE id = ?1")?;
        statement.execute(&[&id.to_string()])?;
        Ok(())
    }

    // ———————— Rachis CRUD ————————

    /// A helper function that queries a [Rachis] from a [Row].
    ///
    /// # Arguments
    ///
    /// * `id_str: String` - The ID of the [Rachis].
    /// * `flight_id_str: String` - The ID of the [Flight] that the [Rachis] belongs to.
    /// * `title: String` - The title of the [Rachis].
    /// * `content: String` - The content of the [Rachis].
    /// * `type_str: String` - The type of the [Rachis].
    /// * `path: String` - The path of the [Rachis].
    /// * `created_at_timestamp: i64` - The timestamp of when the [Rachis] was created.
    /// * `updated_at_timestamp: i64` - The timestamp of when the [Rachis] was last updated.
    /// * `word_count: i64` - The word count of the [Rachis].
    ///
    /// # Returns
    ///
    /// [Rachis](Rachis) - The [Rachis] that was queried.
    ///
    /// # Examples
    ///
    /// TODO: Generate examples for Database::_query_row_rachis()
    /// TODO: Come up with a better name for this (what was I thinking?)
    fn _query_row_rachis(
        id_str: String,
        flight_id_str: String,
        title: String,
        content: String,
        type_str: String,
        path: String,
        created_at_timestamp: i64,
        updated_at_timestamp: i64,
        word_count: i64,
    ) -> Result<Rachis, Error> {
        // Convert id and flight_id to Uuid, returning a conversion error on failure
        let id: Uuid = Uuid::parse_str(&id_str)
            .map_err(|e: uuid::Error| Error::ToSqlConversionFailure(Box::new(e)))?;
        let flight_id: Uuid = Uuid::parse_str(&flight_id_str)
            .map_err(|e: uuid::Error| Error::ToSqlConversionFailure(Box::new(e)))?;

        // Convert type_str to RachisType, returning an invalid parameter error if the conversion fails
        let r#type: RachisType = RachisType::from_str(&type_str)
            .map_err(|_| Error::InvalidParameterName(String::from("Invalid RachisType")))?;

        // Convert created_at and updated_at to DateTime<Utc>, returning an invalid parameter error if the conversion fails
        let created_at: DateTime<Utc> = DateTime::from_timestamp(created_at_timestamp, 0)
            .ok_or_else(|| {
                Error::InvalidParameterName(String::from("Invalid created_at timestamp"))
            })?;
        let updated_at: DateTime<Utc> = DateTime::from_timestamp(updated_at_timestamp, 0)
            .ok_or_else(|| {
                Error::InvalidParameterName(String::from("Invalid updated_at timestamp"))
            })?;

        Ok(Rachis {
            id,
            flight_id,
            title,
            content,
            r#type,
            path,
            created_at,
            updated_at,
            word_count: word_count as usize,
        })
    }

    /// Returns the [Rachis] with the given ID.
    ///
    /// # Arguments
    ///
    /// * `id: &Uuid` - The ID of the [Rachis] to retrieve.
    ///
    /// # Returns
    ///
    /// The [Rachis] with the given ID, or None if no such [Rachis] exists.
    ///
    /// # Examples
    ///
    /// TODO: Generate examples for Database::get_rachis_by_id()
    pub fn get_rachis_by_id(&self, id: &Uuid) -> Result<Option<Rachis>, Error> {
        let mut statement: CachedStatement<'_> = self
            .conn
            .prepare_cached("SELECT * FROM rachises WHERE id = ?1")?;

        let result: Result<Rachis, Error> =
            statement.query_row(&[&id.to_string()], |row: &Row<'_>| {
                Self::_query_row_rachis(
                    row.get("id")?,
                    row.get("flight_id")?,
                    row.get("title")?,
                    row.get("content")?,
                    row.get("type")?,
                    row.get("path")?,
                    row.get("created_at")?,
                    row.get("updated_at")?,
                    row.get("word_count")?,
                )
            });

        match result {
            Ok(rachis) => Ok(Some(rachis)),
            Err(Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e),
        }
    }

    /// Returns a vector of [Rachises](Rachis) that match a given title.
    ///
    /// # Arguments
    ///
    /// * `title` - The title to search for.
    ///
    /// # Returns
    ///
    /// A vector of [Rachises](Rachis) that match the given title.
    ///
    /// # Examples
    ///
    /// TODO: Generate examples for Database::get_rachises_by_title()
    pub fn get_rachises_by_title(&self, title: Option<String>) -> Result<Vec<Rachis>, Error> {
        let mut statement: CachedStatement<'_>;
        let result;
        let f = |row: &Row<'_>| -> Result<Rachis, Error> {
            Self::_query_row_rachis(
                row.get("id")?,
                row.get("flight_id")?,
                row.get("title")?,
                row.get("content")?,
                row.get("type")?,
                row.get("path")?,
                row.get("created_at")?,
                row.get("updated_at")?,
                row.get("word_count")?,
            )
        };

        // If the title is None, return all Rachises in the Flight
        if title.is_none() {
            statement = self
                .conn
                .prepare_cached("SELECT * FROM rachises ORDER BY created_at DESC")?;
            result = statement.query_map([], f)?;
        } else {
            statement = self.conn.prepare_cached(
                "SELECT * FROM rachises WHERE title = ?1 ORDER BY created_at DESC",
            )?;
            result = statement.query_map(&[&title.unwrap().to_string()], f)?;
        }

        let result: Vec<Rachis> = result.collect::<Result<Vec<Rachis>, Error>>()?;

        Ok(result)
    }

    /// Lists all Rachises in the database ordered by their creation date in descending order.
    ///
    /// # Returns
    ///
    /// * `Result<Vec<Rachis>, Error>` - Returns a vector of [Rachis] if successful, or an error if not.
    ///
    /// # Examples
    ///
    /// TODO: Generate examples for Database::get_rachises_by_type()
    pub fn get_rachises_by_type(&self, r#type: Option<RachisType>) -> Result<Vec<Rachis>, Error> {
        let mut statement: CachedStatement<'_> = self
            .conn
            .prepare_cached("SELECT * FROM rachises ORDER BY created_at DESC")?;

        let rachises: Vec<Rachis> = statement
            .query_map([], |row: &Row<'_>| {
                Self::_query_row_rachis(
                    row.get("id")?,
                    row.get("flight_id")?,
                    row.get("title")?,
                    row.get("content")?,
                    row.get("type")?,
                    row.get("path")?,
                    row.get("created_at")?,
                    row.get("updated_at")?,
                    row.get("word_count")?,
                )
            })?
            .collect::<Result<Vec<Rachis>, Error>>()?;

        let rachises: Vec<Rachis> = match r#type {
            Some(rachis_type) => rachises
                .into_iter()
                .filter(|rachis| rachis.r#type == rachis_type)
                .collect(),
            None => rachises,
        };

        Ok(rachises)
    }

    /// Inserts a new [Rachis] into the database.
    ///
    /// # Arguments
    ///
    /// * `rachis: &Rachis` - The [Rachis] to insert into the database.
    ///
    /// # Returns
    ///
    /// * `Result<(), Error>` - Returns Ok if successful, or an error if not.
    ///
    /// # Examples
    ///
    /// TODO: Generate examples for Database::create_rachis()
    pub fn create_rachis(&self, rachis: &Rachis) -> Result<(), Error> {
        let mut statement: CachedStatement<'_> = self
            .conn
            .prepare_cached("INSERT INTO rachises (id, flight_id, title, content, type, path, created_at, updated_at, word_count) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)")?;

        statement.execute(&[
            &rachis.id.to_string(),
            &rachis.flight_id.to_string(),
            &rachis.title,
            &rachis.content,
            &rachis.r#type.to_string(),
            &rachis.path,
            &rachis.created_at.timestamp().to_string(),
            &rachis.updated_at.timestamp().to_string(),
            &rachis.word_count.to_string(),
        ])?;

        Ok(())
    }

    /// Inserts multiple [Rachis] into the database.
    ///
    /// # Arguments
    ///
    /// * `rachises`: &[Rachis] - The [Rachis]es to insert.
    ///
    /// # Returns
    ///
    /// * [Result]<(), [Error]> - Returns Ok if successful, or an error if not.
    pub fn create_rachises(&self, rachises: &[Rachis]) -> Result<(), Error> {
        for rachis in rachises {
            self.create_rachis(rachis)?;
        }
        Ok(())
    }

    /// Updates an existing [Rachis] in the database.
    ///
    /// # Arguments
    ///
    /// * `id: &Uuid` - The ID of the [Rachis] to update.
    /// * `new_rachis: &Rachis` - The new [Rachis] to update the existing [Rachis] with.
    ///
    /// # Returns
    ///
    /// * `Result<(), Error>` - Returns Ok if successful, or an error if not.
    ///
    /// # Examples
    ///
    /// TODO: Generate examples for Database::update_rachis()
    pub fn update_rachis(&self, id: &Uuid, new_rachis: &Rachis) -> Result<(), Error> {
        self.get_rachis_by_id(id)?
            .ok_or(Error::QueryReturnedNoRows)?;

        let mut statement: CachedStatement<'_> = self.conn.prepare_cached(
            "UPDATE rachises SET title = ?1, content = ?2, type = ?3, path = ?4, updated_at = ?5, word_count = ?6 WHERE id = ?7"
        )?;

        println!("Title: {}", new_rachis.title);
        println!("Content: {}", new_rachis.content);
        println!("Type: {}", new_rachis.r#type);
        println!("Path: {}", new_rachis.path);
        println!("Word Count: {}", new_rachis.word_count);

        statement.execute(&[
            &new_rachis.title,
            &new_rachis.content,
            &new_rachis.r#type.to_string(),
            &new_rachis.path,
            &Utc::now().timestamp().to_string(),
            &new_rachis.word_count.to_string(),
            &id.to_string(),
        ])?;

        Ok(())
    }

    /// Deletes an existing [Rachis] from the database.
    ///
    /// # Arguments
    ///
    /// * `rachis_id: &Uuid` - The ID of the [Rachis] to delete.
    ///
    /// # Returns
    ///
    /// * `Result<(), Error>` - Returns Ok if successful, or an error if not.
    ///
    /// # Examples
    ///
    /// TODO: Generate examples for Database::delete_rachis()
    pub fn delete_rachis(&self, rachis_id: Uuid) -> Result<(), Error> {
        let mut statement: CachedStatement<'_> = self
            .conn
            .prepare_cached("DELETE FROM rachises WHERE id = ?1")?;

        statement.execute(&[&rachis_id.to_string()])?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests the creation of a new [Database] connection.
    #[test]
    fn test_new_connection() -> Result<(), Error> {
        let _ = Database::open(":memory:")?;
        Ok(())
    }

    /// Tests a [Flight] round trip
    #[test]
    fn test_flight_round_trip() -> Result<(), Error> {
        // Generate a new Database
        let db: Database = Database::open(":memory:")?;
        println!("Database: {:#?}", db);

        // Generate a new Flight
        let flight: Flight = Flight::new(String::from("My Writing Project"));
        println!("Flight: {:#?}", flight);

        // Insert the Flight into the Database
        db.create_flight(&flight)?;
        assert!(db.get_flight().is_ok());

        // Retrieve the Flight from the Database
        let loaded_flight: Flight = db.get_flight()?.ok_or(Error::QueryReturnedNoRows)?;
        println!("Retrieved Flight: {:#?}", loaded_flight);

        // Assert that the retrieved Flight is the same as the original
        assert_eq!(flight.id, loaded_flight.id);
        assert_eq!(flight.name, loaded_flight.name);
        assert_eq!(
            flight.created_at.timestamp(),
            loaded_flight.created_at.timestamp()
        );
        assert_eq!(
            flight.updated_at.timestamp(),
            loaded_flight.updated_at.timestamp()
        );

        Ok(())
    }

    /// Tests a [Flight] update
    #[test]
    fn test_flight_update() -> Result<(), Error> {
        // Generate a new Database
        let db: Database = Database::open(":memory:")?;
        println!("Database: {:#?}", db);

        // Generate a new Flight
        let flight: Flight = Flight::new(String::from("My Writing Project"));
        println!("Flight: {:#?}", flight);

        // Insert the Flight into the Database
        db.create_flight(&flight)?;
        assert!(db.get_flight().is_ok());

        // Retrieve the Flight from the Database
        let loaded_flight: Flight = db.get_flight()?.ok_or(Error::QueryReturnedNoRows)?;
        println!("Retrieved Flight: {:#?}", loaded_flight);

        // Assert that the retrieved Flight is the same as the original
        assert_eq!(flight.id, loaded_flight.id);
        assert_eq!(flight.name, loaded_flight.name);
        assert_eq!(
            flight.created_at.timestamp(),
            loaded_flight.created_at.timestamp()
        );
        assert_eq!(
            flight.updated_at.timestamp(),
            loaded_flight.updated_at.timestamp()
        );

        // Now that the Flight is in the Database, update it

        // Might be a good idea to add some sort of sleep here to make sure that the updated_at timestamp is different
        std::thread::sleep(std::time::Duration::from_secs(1));

        // Create a new Flight to base updates off of
        let new_flight: Flight = Flight::new(String::from("Better Title"));
        // Update the Flight data via the database
        db.update_flight(&new_flight)?;
        // Because this function only changes the data related to the Flight, I have to make sure that the actual Flight object itself reflects the database changes
        let updated_flight: Flight = db.get_flight()?.ok_or(Error::QueryReturnedNoRows)?;

        // Assert that the only fields that changed were the name and updated_at fields
        println!("Names: {} | {}", updated_flight.name, new_flight.name);
        assert_eq!(updated_flight.name, new_flight.name);
        println!("IDs: {} | {}", updated_flight.id, flight.id);
        assert_eq!(updated_flight.id, flight.id);
        println!(
            "Created At: {} | {}",
            updated_flight.created_at, flight.created_at
        );
        assert_eq!(updated_flight.created_at, flight.created_at);
        // The above assertion fails due to accuracy offsets:
        // assertion `left == right` failed
        //     left:  2026-05-14T18:30:51Z
        //     right: 2026-05-14T18:30:51.989195700Z
        // This means that I need to make sure that, when I save the "created_at" field, I don't include such high degrees of accuracy
        // DONE: Updated Flight constructor to use Utc::now().with_nanosecond(0).unwrap()
        // Also updated all other instances of "Utc::now()" to use Utc::now().with_nanosecond(0).unwrap()
        println!(
            "Updated At: {} | {}",
            updated_flight.updated_at, flight.updated_at
        );
        assert!(updated_flight.updated_at >= flight.updated_at);

        Ok(())
    }

    /// Tests a [Flight] deletion
    #[test]
    fn test_flight_deletion() -> Result<(), Error> {
        // Generate a new Database
        let db: Database = Database::open(":memory:")?;
        println!("Database: {:#?}", db);

        // Generate a new Flight
        let flight: Flight = Flight::new(String::from("My Writing Project"));
        println!("Flight: {:#?}", flight);

        // Insert the Flight into the Database
        db.create_flight(&flight)?;
        assert!(db.get_flight().is_ok());

        // Retrieve the Flight from the Database
        let loaded_flight: Flight = db.get_flight()?.ok_or(Error::QueryReturnedNoRows)?;
        println!("Retrieved Flight: {:#?}", loaded_flight);

        // Assert that the retrieved Flight is the same as the original
        assert_eq!(flight.id, loaded_flight.id);
        assert_eq!(flight.name, loaded_flight.name);
        assert_eq!(
            flight.created_at.timestamp(),
            loaded_flight.created_at.timestamp()
        );
        assert_eq!(
            flight.updated_at.timestamp(),
            loaded_flight.updated_at.timestamp()
        );

        // Delete the Flight from the Database
        db.delete_flight(&flight.id)?;
        assert!(db.get_flight()?.is_none());

        Ok(())
    }

    /// Tests a [Rachis] creation
    #[test]
    fn test_rachis_creation() -> Result<(), Error> {
        // Create a new Database
        let db = Database::open(":memory:")?;
        println!("Database: {:#?}", db);

        // Create a new Flight
        let flight: Flight = Flight::new(String::from("Hold My Rachis"));
        println!("Flight: {:#?}", flight);

        // ——————— Single Rachis Creation ———————

        // Create one new Rachis
        let rachis: Rachis = Rachis::new(
            flight.id,
            String::from("Hello, Rachis!"),
            RachisType::DEFAULT,
            String::from("Story"),
            String::new(),
        );
        println!("Rachis: {:#?}", rachis);

        // Insert the Flight into the Database
        db.create_flight(&flight)?;
        // Assert that the Flight is in the Database
        assert!(db.get_flight().is_ok());

        // Insert the Rachis into the Database
        db.create_rachis(&rachis)?;
        // Assert that the Rachis is in the Database
        assert!(db.get_rachis_by_id(&rachis.id).is_ok());

        // ——————— Multiple Rachis Creation ———————

        // Create many Rachises
        let rachises: Vec<Rachis> = vec![
            Rachis::new(
                flight.id,
                String::from("Under Guise of Stars"),
                RachisType::DEFAULT,
                String::from("Arc 1"),
                String::new(),
            ),
            Rachis::new(
                flight.id,
                String::from("Whom It May Concern"),
                RachisType::DEFAULT,
                String::from("Arc 1"),
                String::new(),
            ),
            Rachis::new(
                flight.id,
                String::from("Epistles for the Cosmos"),
                RachisType::DEFAULT,
                String::from("Arc 1"),
                String::new(),
            ),
        ];
        println!("Rachises: {:#?}", rachises);

        // Insert the Rachises into the Database
        for rachis in rachises {
            db.create_rachis(&rachis)?;
            // Assert that the Rachis is in the Database
            assert!(db.get_rachis_by_id(&rachis.id).is_ok());
        }

        Ok(())
    }

    /// Tests listing multiple [Rachises](Rachis)
    #[test]
    fn test_rachis_listing() -> Result<(), Error> {
        // Create a new Database
        let db = Database::open(":memory:")?;
        println!("Database: {:#?}", db);

        // Create a new Flight
        let flight: Flight = Flight::new(String::from("Hold My Rachis"));
        println!("Flight: {:#?}", flight);

        // and some Rachises
        let rachises: Vec<Rachis> = vec![
            Rachis::new(
                flight.id,
                String::from("Under Guise of Stars"),
                RachisType::DEFAULT,
                String::from("Arc 1"),
                String::new(),
            ),
            Rachis::new(
                flight.id,
                String::from("Whom It May Concern"),
                RachisType::DEFAULT,
                String::from("Arc 1"),
                String::new(),
            ),
            Rachis::new(
                flight.id,
                String::from("Epistles for the Cosmos"),
                RachisType::DEFAULT,
                String::from("Arc 1"),
                String::new(),
            ),
            Rachis::new(
                flight.id,
                String::from("Twilight Sparkle"),
                RachisType::CHARACTER,
                String::from("Arc 2"),
                String::new(),
            ),
            Rachis::new(
                flight.id,
                String::from("Pinkie Pie"),
                RachisType::CHARACTER,
                String::from("Arc 2"),
                String::new(),
            ),
            Rachis::new(
                flight.id,
                String::from("Canterlot Castle"),
                RachisType::LOCATION,
                String::from("Arc 2"),
                String::new(),
            ),
            Rachis::new(
                flight.id,
                String::from("Tirek's Return"),
                RachisType::EVENT,
                String::from("Arc 2"),
                String::new(),
            ),
        ];
        println!("Rachises: {:#?}", rachises);

        // Insert the Flight into the Database
        db.create_flight(&flight)?;
        // Assert that the Flight is in the Database
        assert!(db.get_flight().is_ok());

        // Insert the Rachises into the Database
        for rachis in rachises {
            db.create_rachis(&rachis)?;
            // Assert that the Rachis is in the Database
            assert!(db.get_rachis_by_id(&rachis.id).is_ok());
        }

        // List the Rachises in the Database
        let rachises: Vec<Rachis> = db.get_rachises_by_type(None)?;
        for rachis in &rachises {
            println!("Rachis: {:#?}", rachis);
        }
        assert_eq!(rachises.len(), 7);

        // List the Character Rachises in the Database
        let rachises: Vec<Rachis> = db.get_rachises_by_type(Some(RachisType::CHARACTER))?;
        for rachis in &rachises {
            println!("Rachis: {:#?}", rachis);
        }
        assert_eq!(rachises.len(), 2);

        Ok(())
    }

    #[test]
    fn test_rachis_update() -> Result<(), Error> {
        // Generate a new Database
        let db: Database = Database::open(":memory:")?;
        println!("Database: {:#?}", db);

        // Create a new Flight
        let flight: Flight = Flight::new(String::from("Hold My Rachis"));
        println!("Flight: {:#?}", flight);

        // Create a new Rachis
        let mut rachis: Rachis = Rachis::new(
            flight.id,
            String::from("Under Guise of Stars"),
            RachisType::DEFAULT,
            String::from("Arc 1"),
            String::new(),
        );
        println!("Rachis: {:#?}", rachis);

        // Insert the Flight into the Database
        db.create_flight(&flight)?;
        // Assert that the Flight is in the Database
        assert!(db.get_flight().is_ok());

        // Insert the Rachis into the Database
        db.create_rachis(&rachis)?;
        // Assert that the Rachis is in the Database
        assert!(db.get_rachis_by_id(&rachis.id).is_ok());

        // Add a second-long sleep
        std::thread::sleep(std::time::Duration::from_secs(1));

        // Create a new Rachis to base updates off of
        let new_rachis: Rachis = Rachis::new(
            flight.id,
            String::from("For What It's Worth"),
            RachisType::DEFAULT,
            String::from("Arc 9"),
            String::new(),
        );
        db.update_rachis(&rachis.id, &new_rachis)?;

        // Updated the database, now update the Rachis itself
        rachis = db
            .get_rachis_by_id(&rachis.id)?
            .ok_or(Error::QueryReturnedNoRows)?;

        println!("Titles: {} | {}", rachis.title, new_rachis.title);
        assert_eq!(rachis.title, new_rachis.title);
        println!("Types: {} | {}", rachis.r#type, new_rachis.r#type);
        assert_eq!(rachis.r#type, new_rachis.r#type);
        println!(
            "Contents: {} | {}",
            rachis.content.clone(),
            new_rachis.content.clone()
        );
        assert_eq!(rachis.content.clone(), new_rachis.content.clone());
        println!("Paths: {} | {}", rachis.path, new_rachis.path);
        assert_eq!(rachis.path, new_rachis.path);
        println!("Flights: {} | {}", rachis.flight_id, new_rachis.flight_id);
        assert_eq!(rachis.flight_id, new_rachis.flight_id);

        Ok(())
    }

    /// Tests that a Rachis can be deleted from the Database
    #[test]
    fn test_rachis_deletion() -> Result<(), Error> {
        // Generate a new Database
        let db: Database = Database::open(":memory:")?;
        println!("Database: {:#?}", db);

        // Create a new Flight
        let flight: Flight = Flight::new(String::from("Hold My Rachis"));
        println!("Flight: {:#?}", flight);

        // Insert the Flight into the Database
        db.create_flight(&flight)?;
        // Assert that the Flight is in the Database
        assert!(db.get_flight().is_ok());

        // Create a new Rachis to delete
        let rachis: Rachis = Rachis::new(
            flight.id,
            String::new(),
            RachisType::DEFAULT,
            String::new(),
            String::new(),
        );

        // Insert the Rachis into the Database
        db.create_rachis(&rachis)?;

        // Assert that the Rachis is in the Database
        assert!(db.get_rachis_by_id(&rachis.id).is_ok());

        // Delete the rachis
        db.delete_rachis(rachis.id)?;
        // Assert that the Rachis is no longer in the Database :(
        assert!(db.get_rachis_by_id(&rachis.id)?.is_none());

        Ok(())
    }

    /// Tests that some/all Rachises can be retrieved from the Database
    #[test]
    fn test_get_rachises() -> Result<(), Error> {
        // Generate a new Database
        let db: Database = Database::open(":memory:")?;

        // Generate a Flight and insert it into the Database
        let flight: Flight = Flight::new(String::from("Hold My Rachis"));
        db.create_flight(&flight)?;

        // Generate Rachises and insert them into the Database
        let rachises: Vec<Rachis> = vec![
            Rachis::new(
                flight.id,
                String::from("Under Guise of Stars"),
                RachisType::DEFAULT,
                String::new(),
                String::new(),
            ),
            Rachis::new(
                flight.id,
                String::from("Whom It May Concern"),
                RachisType::DEFAULT,
                String::new(),
                String::new(),
            ),
            Rachis::new(
                flight.id,
                String::from("Epistles for the Cosmos"),
                RachisType::DEFAULT,
                String::new(),
                String::new(),
            ),
        ];
        db.create_rachises(&rachises)?;

        // Get all Rachises from the Database
        let fetched: Vec<Rachis> = db.get_rachises_by_title(None)?;
        // Assert that the Rachises are in the list
        assert_eq!(rachises.len(), 3);
        assert_eq!(fetched[0].id, rachises[0].id);
        assert_eq!(fetched[1].id, rachises[1].id);
        assert_eq!(fetched[2].id, rachises[2].id);

        // Get Rachises named "Whom It May Concern" from the Database
        let fetched: Vec<Rachis> =
            db.get_rachises_by_title(Some(String::from("Whom It May Concern")))?;
        assert_eq!(fetched.len(), 1);
        assert_eq!(fetched[0].id, rachises[1].id);

        Ok(())
    }
}
