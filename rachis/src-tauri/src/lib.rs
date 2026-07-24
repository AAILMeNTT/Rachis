mod domain;
mod entities;
mod io;
mod registry;
mod tag;
mod tree;

use {
    crate::{
        domain::rachis::{Rachis, RachisType},
        entities::{files::ProjectFile, flight_meta::FlightMetadata},
        io::{
            content::FileType,
            context::{FlightContext, FlightError},
        },
        registry::{ReconcileReport, Registry, RegistryEntry, RegistryEntryPatch},
    },
    std::{
        path::PathBuf,
        sync::{Mutex, MutexGuard},
    },
    tauri::{Manager, State},
    uuid::Uuid,
};

struct AppData {
    flight: Mutex<Option<FlightContext>>,
    registry: Mutex<Registry>,
    registry_dir: PathBuf,
}

/// Parses a tag and returns the parsed tag.
#[tauri::command(rename_all = "snake_case")]
fn parse_tag(input: &str) -> Option<tag::Tag> {
    println!("Parsing tag: {input:#?}");
    tag::Tag::parse(input)
}

// ———————— Flight CRUD ————————

#[tauri::command(rename_all = "snake_case")]
fn create_flight(
    state: State<AppData>,
    flight_path: String,
    flight_name: String,
) -> Result<FlightMetadata, String> {
    let ctx: FlightContext = FlightContext::open_conn(flight_path, &flight_name)
        .map_err(|e: FlightError| e.to_string())?;

    let metadata: FlightMetadata = ctx
        .init_flight_metadata(&Uuid::new_v4(), flight_name)
        .map_err(|e: FlightError| e.to_string())?;

    *state.flight.lock().unwrap() = Some(ctx);
    Ok(metadata)
}

#[tauri::command(rename_all = "snake_case")]
fn get_flight(
    state: State<AppData>,
    flight_path: String,
    flight_name: String,
    flight_id: Uuid,
) -> Result<Option<FlightMetadata>, String> {
    let ctx: FlightContext = FlightContext::open_conn(flight_path, flight_name)
        .map_err(|e: FlightError| e.to_string())?;

    let metadata: Option<FlightMetadata> = ctx
        .get_flight_metadata(&flight_id)
        .map_err(|e: FlightError| e.to_string())?;

    *state.flight.lock().unwrap() = Some(ctx);
    Ok(metadata)
}

// ———————— File CRUD ————————

/// Creates a new file in the open Flight.
///
/// Delegates to [`FlightContext::create_file`] for all business logic
/// (filename derivation, subdirectory mapping, file creation, metadata indexing).
///
/// # Arguments
///
/// - `title`: [`String`] - The display title of the Rachis. Used to derive the filename.
/// - `r#type`: [`Option<RachisType>`](RachisType) - The entity type. Affects the subdirectory.
/// - `content`: [`Option<String>`](String) - Optional initial content for the file.
///
/// # Returns
///
/// [`Rachis`] - A read-only payload describing the created file.
#[tauri::command(rename_all = "snake_case")]
fn create_file(
    state: State<AppData>,
    title: String,
    r#type: Option<RachisType>,
    content: Option<String>,
) -> Result<Rachis, String> {
    // Try to lock the flight mutex and get a reference to the FlightContext
    let flight_lock: MutexGuard<'_, Option<FlightContext>> =
        state.flight.lock().map_err(|e| e.to_string())?;
    let ctx: &FlightContext = flight_lock.as_ref().ok_or("No Flight is open")?;

    // If the flight context is available, create the file using it
    ctx.create_file(
        &title,
        FileType::from_title(&title).unwrap_or(FileType::Markdown),
        r#type.unwrap_or_default(),
        &content.unwrap_or_default(),
    )
    .map_err(|e: FlightError| e.to_string())
}

// #[tauri::command(rename_all = "snake_case")]
// fn get_rachis_by_id(state: State<AppData>, id: Uuid) -> Result<Option<Rachis>, String> {
//     // Try to lock the flight mutex and get a reference to the FlightContext
//     let flight_lock: MutexGuard<'_, Option<FlightContext>> =
//         state.flight.lock().map_err(|e| e.to_string())?;
//     let ctx: &FlightContext = flight_lock.as_ref().ok_or("No Flight is open")?;
// }

#[tauri::command(rename_all = "snake_case")]
fn save_file(state: State<AppData>, id: Uuid, content: String) -> Result<ProjectFile, String> {
    // Do the same damn shit all over again
    let flight_lock: MutexGuard<'_, Option<FlightContext>> =
        state.flight.lock().map_err(|e| e.to_string())?;
    let ctx: &FlightContext = flight_lock.as_ref().ok_or("No Flight is open")?;

    ctx.save_file(&id, &content)
        .map_err(|e: FlightError| e.to_string())
}

// ———————— Registry Commands ————————

/// Lists all Flights in the registry.
#[tauri::command(rename_all = "snake_case")]
fn list_registry_flights(state: State<AppData>) -> Result<Vec<RegistryEntry>, String> {
    let reg: MutexGuard<'_, Registry> = state.registry.lock().map_err(|e| e.to_string())?;
    Ok(reg.list().into())
}

/// Returns a single Flight from the registry by ID.
#[tauri::command(rename_all = "snake_case")]
fn get_registry_flight(state: State<AppData>, id: Uuid) -> Result<Option<RegistryEntry>, String> {
    let reg: MutexGuard<'_, Registry> = state.registry.lock().map_err(|e| e.to_string())?;
    Ok(reg.get(&id).cloned())
}

/// Adds a new Flight to the registry and persists to disk.
#[tauri::command(rename_all = "snake_case")]
fn add_registry_flight(
    state: State<AppData>,
    flight_name: String,
    flight_path: String,
    flight_id: Uuid,
) -> Result<RegistryEntry, String> {
    let mut reg: MutexGuard<'_, Registry> = state.registry.lock().map_err(|e| e.to_string())?;

    let entry: RegistryEntry = reg
        .add_entry(flight_name, flight_path, flight_id)
        .map_err(|e: String| e.to_string())?;
    registry::save_to_disk(&state.registry_dir, &reg).map_err(|e: FlightError| e.to_string())?;
    Ok(entry)
}

/// Removes a Flight from the registry by ID and persists to disk.
#[tauri::command(rename_all = "snake_case")]
fn remove_registry_flight(state: State<AppData>, id: Uuid) -> Result<bool, String> {
    let mut reg: MutexGuard<'_, Registry> = state.registry.lock().map_err(|e| e.to_string())?;

    match reg.remove_entry(id) {
        true => Ok(registry::save_to_disk(&state.registry_dir, &reg).map_err(|e| e.to_string())?),
        false => Err(String::from("Unable to remove flight: not found")),
    }
}

/// Toggles the favourite status of a Flight and persists to disk.
#[tauri::command(rename_all = "snake_case")]
fn update_registry_flight(
    state: State<AppData>,
    id: Uuid,
    patch: RegistryEntryPatch,
) -> Result<RegistryEntry, String> {
    let mut reg: MutexGuard<'_, Registry> = state.registry.lock().map_err(|e| e.to_string())?;

    let entry: RegistryEntry = reg
        .update(id, patch)
        .map_err(|e: FlightError| e.to_string())?;

    registry::save_to_disk(&state.registry_dir, &reg).map_err(|e: FlightError| e.to_string())?;

    Ok(entry)
}

/// Searches Flights in the registry by name (case-insensitive, partial match).
#[tauri::command(rename_all = "snake_case")]
fn search_registry_flights(
    state: State<AppData>,
    query: String,
) -> Result<Vec<RegistryEntry>, String> {
    let registry: MutexGuard<'_, Registry> = state.registry.lock().map_err(|e| e.to_string())?;
    Ok(registry
        .search_by_name(&query)
        .into_iter()
        .cloned()
        .collect())
}

/// Returns the most recently opened Flight from the registry.
#[tauri::command(rename_all = "snake_case")]
fn get_recent_registry_flight(state: State<AppData>) -> Result<Option<RegistryEntry>, String> {
    let registry: MutexGuard<'_, Registry> = state.registry.lock().map_err(|e| e.to_string())?;
    Ok(registry.most_recent().cloned())
}

/// Reconciles all registered Flights: checks cached paths, searches
/// `scan_paths` for moved Flights, and updates entries with new locations.
///
/// Should be called on startup and whenever the user requests a "Find Flights"
/// action. Persists changes to disk automatically.
#[tauri::command(rename_all = "snake_case")]
fn reconcile_registry_flights(state: State<AppData>) -> Result<Vec<ReconcileReport>, String> {
    let mut registry: MutexGuard<'_, Registry> =
        state.registry.lock().map_err(|e| e.to_string())?;

    let reports: Vec<ReconcileReport> = registry.reconcile_flights().map_err(|e| e.to_string())?;

    // Persist any path updates that were made
    registry::save_to_disk(&state.registry_dir, &registry).map_err(|e| e.to_string())?;

    Ok(reports)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app: &mut tauri::App| {
            let app_dir: PathBuf = app.path().app_data_dir()?;
            std::fs::create_dir_all(&app_dir).expect("Failed to create app directory");

            // Load (or create) the Flight registry
            let registry: Registry =
                registry::load_from_disk(&app_dir).expect("Failed to load registry");

            // TODO: Add some way to start app in last-opened Flight? perhaps a user setting
            app.manage(AppData {
                flight: Mutex::new(None),
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
            list_registry_flights,
            remove_registry_flight,
            search_registry_flights,
            update_registry_flight,
            get_recent_registry_flight,
            reconcile_registry_flights,
            // Rachis commands
            create_file,
            save_file,
            // FlightContext commands
            create_flight,
            get_flight,
            // Misc
            parse_tag,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
