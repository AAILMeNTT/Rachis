mod domain;
mod storage;
mod tag;

use domain::{Flight, Rachis};
use std::sync::{Mutex, MutexGuard};
use storage::Database;
use tauri::{Manager, State};

use crate::domain::RachisType;

struct AppData {
    db: Mutex<storage::Database>,
}

/// Parses a tag and returns the parsed tag.
#[tauri::command(rename_all = "snake_case")]
fn parse_tag(input: &str) -> Option<tag::Tag> {
    println!("Parsing tag: {}", input);
    tag::Tag::parse(input)
}

// ———————— Database Mgmt ————————
// 
// #[tauri::command(rename_all = "snake_case")]
// fn new_project(path: &str) -> Result<(), String> {}
// 
// #[tauri::command(rename_all = "snake_case")]
// fn open_project(path: &str) -> Result<(), String> {}
// 
// #[tauri::command(rename_all = "snake_case")]
// fn delete_project(path: &str) -> Result<(), String> {}

// ———————— Rachis CRUD ————————

/// Returns the Flight from the database.
#[tauri::command(rename_all = "snake_case")]
fn get_flight(state: State<AppData>) -> Result<Option<Flight>, String> {
    // Get a mutable reference to the database
    let db: MutexGuard<'_, Database> = state.db.lock().unwrap();

    // Get the requested Flight
    db.get_flight().map_err(|e: rusqlite::Error| e.to_string())
}

/// Inserts a Flight into the database.
#[tauri::command(rename_all = "snake_case")]
fn create_flight(state: State<AppData>, name: String) -> Result<Flight, String> {
    let db: MutexGuard<'_, Database> = state.db.lock().unwrap();
    let flight: Flight = Flight::new(name);

    db.create_flight(&flight)
        .map_err(|e: rusqlite::Error| e.to_string())?;

    Ok(flight)
}

/// Updates the Flight in the database.
#[tauri::command(rename_all = "snake_case")]
fn update_flight(state: State<AppData>, flight: Flight) -> Result<(), String> {
    // Get a mutable reference to the database
    let db: MutexGuard<'_, Database> = state.db.lock().unwrap();

    // Currently the only thing that users can update about a Flight is the name, so right now
    // this is a bit overkill. However, this also means that I theoretically never have to look
    // at this again
    db.update_flight(&flight)
        .map_err(|e: rusqlite::Error| e.to_string())
}

/// Deletes the Flight from the database.
#[tauri::command(rename_all = "snake_case")]
fn delete_flight(state: State<AppData>, id: uuid::Uuid) -> Result<(), String> {
    // Get a mutable reference to the database
    let db: MutexGuard<'_, Database> = state.db.lock().unwrap();

    db.delete_flight(&id)
        .map_err(|e: rusqlite::Error| e.to_string())
}

// ———————— Rachis CRUD ————————

/// Gets a Rachis from the database.
#[tauri::command(rename_all = "snake_case")]
fn get_rachis_by_id(state: State<AppData>, id: uuid::Uuid) -> Result<Option<Rachis>, String> {
    let db: MutexGuard<'_, Database> = state.db.lock().unwrap();
    db.get_rachis_by_id(&id)
        .map_err(|e: rusqlite::Error| e.to_string())
}

/// Gets a Rachis from the database by its title
#[tauri::command(rename_all = "snake_case")]
fn get_rachises_by_title(state: State<AppData>, title: String) -> Result<Vec<Rachis>, String> {
    let db: MutexGuard<'_, Database> = state.db.lock().unwrap();
    db.get_rachises_by_title(&title)
        .map_err(|e| e.to_string())
}

/// Lists some or all Rachises from the database.
#[tauri::command(rename_all = "snake_case")]
fn list_rachises(
    state: State<AppData>,
    r#type: Option<domain::RachisType>,
) -> Result<Vec<Rachis>, String> {
    let db: MutexGuard<'_, Database> = state.db.lock().unwrap();
    db.list_rachises(r#type)
        .map_err(|e: rusqlite::Error| e.to_string())
}

/// Inserts a Rachis into the database.
#[tauri::command(rename_all = "snake_case")]
fn create_rachis(
    state: State<AppData>,
    title: String,
    r#type: Option<RachisType>,
    content: Option<String>,
    path: Option<String>,
) -> Result<Rachis, String> {
    let db: MutexGuard<'_, Database> = state.db.lock().unwrap();

    // Get the flight from the project database
    let flight: Flight = db
        .get_flight()
        .map_err(|e: rusqlite::Error| e.to_string())?
        .ok_or("No flight exists. Create a Flight first.")?;

    // Build the full Rachis
    let rachis: Rachis = Rachis::new(
        flight.id,
        title,
        r#type.unwrap_or_default(),
        path.unwrap_or_default(),
        content.unwrap_or_default(),
    );

    db.create_rachis(&rachis)
        .map_err(|e: rusqlite::Error| e.to_string())?;
    
    Ok(rachis)
}

/// Updates a Rachis in the database.
#[tauri::command(rename_all = "snake_case")]
fn update_rachis(state: State<AppData>, rachis: Rachis) -> Result<(), String> {
    let db: MutexGuard<'_, Database> = state.db.lock().unwrap();

    // We need to make sure that the Rachis actually exists in the database
    let _update_rachis: Rachis = db
        .get_rachis_by_id(&rachis.id)
        .map_err(|e: rusqlite::Error| e.to_string())?
        .ok_or("Rachis not found")?;

    // And now we can update the Rachis
    db.update_rachis(&_update_rachis.id, &rachis)
        .map_err(|e: rusqlite::Error| e.to_string())
}

/// Deletes a Rachis from the database.
#[tauri::command(rename_all = "snake_case")]
fn delete_rachis(state: State<AppData>, id: uuid::Uuid) -> Result<(), String> {
    let db: MutexGuard<'_, Database> = state.db.lock().unwrap();
    db.delete_rachis(id)
        .map_err(|e: rusqlite::Error| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app: &mut tauri::App| {
            // Get the app directory and ensure it exists
            let app_dir: std::path::PathBuf = app.path().app_data_dir()?;
            std::fs::create_dir_all(&app_dir).expect("Failed to create app directory");

            // Append the database path and open
            let db_path: std::path::PathBuf = app_dir.join("rachis.db");
            let db: Database = Database::open(&db_path.to_str().unwrap()) // TODO: Use db_path when ready
                .expect("Failed to open database");
            app.manage(AppData { db: Mutex::new(db) });
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            delete_flight,
            delete_rachis,
            get_flight,
            get_rachis_by_id,
            get_rachises_by_title,
            create_flight,
            create_rachis,
            list_rachises,
            parse_tag,
            update_flight,
            update_rachis,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
