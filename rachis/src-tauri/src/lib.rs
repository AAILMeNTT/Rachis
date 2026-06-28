mod domain;
mod registry;
mod storage;
mod tag;
mod tree;

use domain::{Flight, Rachis, RachisType};
use registry::{Registry, RegistryEntry};
use std::path::PathBuf;
use std::sync::{Mutex, MutexGuard};
use storage::Database;
use tauri::{Manager, State};
use uuid::Uuid;

struct AppData {
    db: Mutex<storage::Database>,
    registry: Mutex<registry::Registry>,
    /// Directory where registry.json lives — needed for disk persistence
    registry_dir: PathBuf,
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

// ———————— Flight CRUD ————————

/// Returns the Flight from the database.
#[tauri::command(rename_all = "snake_case")]
fn get_flight(state: State<AppData>) -> Result<Option<Flight>, String> {
    let db: MutexGuard<'_, Database> = state.db.lock().unwrap();
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
    let db: MutexGuard<'_, Database> = state.db.lock().unwrap();

    db.update_flight(&flight)
        .map_err(|e: rusqlite::Error| e.to_string())
}

/// Deletes the Flight from the database.
#[tauri::command(rename_all = "snake_case")]
fn delete_flight(state: State<AppData>, id: Uuid) -> Result<(), String> {
    let db: MutexGuard<'_, Database> = state.db.lock().unwrap();
    db.delete_flight(&id)
        .map_err(|e: rusqlite::Error| e.to_string())
}

// ———————— Rachis CRUD ————————

/// Gets a Rachis from the database.
#[tauri::command(rename_all = "snake_case")]
fn get_rachis_by_id(state: State<AppData>, id: Uuid) -> Result<Option<Rachis>, String> {
    let db: MutexGuard<'_, Database> = state.db.lock().unwrap();
    db.get_rachis_by_id(&id)
        .map_err(|e: rusqlite::Error| e.to_string())
}

/// Gets a Rachis from the database by its title
#[tauri::command(rename_all = "snake_case")]
fn get_rachises_by_title(
    state: State<AppData>,
    title: Option<String>,
) -> Result<Vec<Rachis>, String> {
    let db: MutexGuard<'_, Database> = state.db.lock().unwrap();
    db.get_rachises_by_title(title).map_err(|e| e.to_string())
}

/// Lists some or all Rachises from the database.
#[tauri::command(rename_all = "snake_case")]
fn get_rachises_by_type(
    state: State<AppData>,
    r#type: Option<RachisType>,
) -> Result<Vec<Rachis>, String> {
    let db: MutexGuard<'_, Database> = state.db.lock().unwrap();
    db.get_rachises_by_type(r#type)
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

    let flight: Flight = db
        .get_flight()
        .map_err(|e: rusqlite::Error| e.to_string())?
        .ok_or("No flight exists. Create a Flight first.")?;

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

    let _update_rachis: Rachis = db
        .get_rachis_by_id(&rachis.id)
        .map_err(|e: rusqlite::Error| e.to_string())?
        .ok_or("Rachis not found")?;

    db.update_rachis(&_update_rachis.id, &rachis)
        .map_err(|e: rusqlite::Error| e.to_string())
}

/// Deletes a Rachis from the database.
#[tauri::command(rename_all = "snake_case")]
fn delete_rachis(state: State<AppData>, id: Uuid) -> Result<(), String> {
    let db: MutexGuard<'_, Database> = state.db.lock().unwrap();
    db.delete_rachis(id)
        .map_err(|e: rusqlite::Error| e.to_string())
}

// ———————— Registry Commands ————————

/// Lists all Flights in the registry.
#[tauri::command(rename_all = "snake_case")]
fn list_registry_flights(state: State<AppData>) -> Result<Vec<RegistryEntry>, String> {
    let registry: MutexGuard<'_, Registry> = state.registry.lock().unwrap();
    Ok(registry.list().to_vec())
}

/// Returns a single Flight from the registry by ID.
#[tauri::command(rename_all = "snake_case")]
fn get_registry_flight(state: State<AppData>, id: Uuid) -> Result<Option<RegistryEntry>, String> {
    let registry: MutexGuard<'_, Registry> = state.registry.lock().unwrap();
    Ok(registry.get(&id).cloned())
}

/// Adds a new Flight to the registry and persists to disk.
#[tauri::command(rename_all = "snake_case")]
fn add_registry_flight(
    state: State<AppData>,
    name: String,
    path: String,
) -> Result<RegistryEntry, String> {
    let mut registry: MutexGuard<'_, Registry> = state.registry.lock().unwrap();
    let entry: RegistryEntry = registry.add(name, path)?;
    registry::save_to_disk(&state.registry_dir, &registry)?;
    Ok(entry)
}

/// Removes a Flight from the registry by ID and persists to disk.
#[tauri::command(rename_all = "snake_case")]
fn remove_registry_flight(state: State<AppData>, id: Uuid) -> Result<bool, String> {
    let mut registry: MutexGuard<'_, Registry> = state.registry.lock().unwrap();
    let removed: bool = registry.remove(&id);
    if removed {
        registry::save_to_disk(&state.registry_dir, &registry)?;
    }
    Ok(removed)
}

/// Toggles the favourite status of a Flight and persists to disk.
#[tauri::command(rename_all = "snake_case")]
fn toggle_registry_flight_favorite(state: State<AppData>, id: Uuid) -> Result<bool, String> {
    let mut registry: MutexGuard<'_, Registry> = state.registry.lock().unwrap();
    let new_status: bool = registry.toggle_favorite(&id)?;
    registry::save_to_disk(&state.registry_dir, &registry)?;
    Ok(new_status)
}

/// Searches Flights in the registry by name (case-insensitive, partial match).
#[tauri::command(rename_all = "snake_case")]
fn search_registry_flights(
    state: State<AppData>,
    query: String,
) -> Result<Vec<RegistryEntry>, String> {
    let registry: MutexGuard<'_, Registry> = state.registry.lock().unwrap();
    Ok(registry.search(&query).into_iter().cloned().collect())
}

/// Returns the most recently opened Flight from the registry.
#[tauri::command(rename_all = "snake_case")]
fn get_most_recent_flight(state: State<AppData>) -> Result<Option<RegistryEntry>, String> {
    let registry: MutexGuard<'_, Registry> = state.registry.lock().unwrap();
    Ok(registry.most_recent().cloned())
}

/// Returns registry summary stats (total flights, total word count).
#[tauri::command(rename_all = "snake_case")]
fn get_registry_stats(state: State<AppData>) -> Result<(usize, usize), String> {
    let registry: MutexGuard<'_, Registry> = state.registry.lock().unwrap();
    Ok((registry.count(), registry.total_word_count()))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app: &mut tauri::App| {
            let app_dir: PathBuf = app.path().app_data_dir()?;
            std::fs::create_dir_all(&app_dir).expect("Failed to create app directory");

            // Open the database (single-file mode for right now, will change to read from registry later probably)
            let db_path: PathBuf = app_dir.join("rachis.db");
            let db: Database =
                Database::open(db_path.to_str().unwrap()).expect("Failed to open database");

            // Load (or create) the Flight registry
            let registry: Registry =
                registry::load_from_disk(&app_dir).expect("Failed to load registry");

            app.manage(AppData {
                db: Mutex::new(db),
                registry: Mutex::new(registry),
                registry_dir: app_dir,
            });
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            // Registry commands
            add_registry_flight,
            get_registry_flight,
            get_registry_stats,
            list_registry_flights,
            remove_registry_flight,
            search_registry_flights,
            toggle_registry_flight_favorite,
            // Flight commands
            create_flight,
            delete_flight,
            get_flight,
            get_most_recent_flight,
            update_flight,
            // Rachis commands
            create_rachis,
            delete_rachis,
            get_rachis_by_id,
            get_rachises_by_title,
            get_rachises_by_type,
            update_rachis,
            // Misc
            parse_tag,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
