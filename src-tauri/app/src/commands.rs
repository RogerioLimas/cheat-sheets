//! IPC bridge: thin `#[tauri::command]` wrappers (steering rule 1). They
//! validate input, delegate to core, and map errors to the serializable
//! `WireError`. No business logic here.

use std::sync::Mutex;

use tauri::State;

use cheatkeys_core::catalog::Catalog;
use cheatkeys_core::error::WireError;
use cheatkeys_core::model::{CheatSheet, CheatSheetSummary};

/// App-wide state: the loaded catalog behind a Mutex (steering rule 6).
pub struct AppState {
    pub catalog: Mutex<Catalog>,
}

/// Treat the `query` arriving over IPC as untrusted input (security rule 3):
/// cap its length so a pathological string can't drive unbounded work.
const MAX_QUERY_LEN: usize = 128;

#[tauri::command]
pub fn list_cheat_sheets(
    query: String,
    state: State<'_, AppState>,
) -> Result<Vec<CheatSheetSummary>, WireError> {
    let query: String = query.chars().take(MAX_QUERY_LEN).collect();
    let catalog = state
        .catalog
        .lock()
        .map_err(|_| WireError {
            code: "internal".into(),
            message: "catalog unavailable".into(),
        })?;
    Ok(catalog.search(&query))
}

#[tauri::command]
pub fn get_cheat_sheet(
    id: String,
    state: State<'_, AppState>,
) -> Result<CheatSheet, WireError> {
    // Validate id at the trust boundary: ids are simple slugs, never paths.
    if id.is_empty() || !id.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_') {
        return Err(WireError {
            code: "invalid_id".into(),
            message: "invalid cheat sheet id".into(),
        });
    }
    let catalog = state
        .catalog
        .lock()
        .map_err(|_| WireError {
            code: "internal".into(),
            message: "catalog unavailable".into(),
        })?;
    catalog.get(&id).map_err(WireError::from)
}
