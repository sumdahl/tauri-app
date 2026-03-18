use crate::error::AppError;
use crate::state::{AppState, AppStateMutex};
use tauri::State;

#[derive(serde::Serialize)]
pub struct AppInfo {
    pub name: String,
    pub version: String,
    pub greeting_count: u64,
    pub initialized: bool,
}

impl From<AppState> for AppInfo {
    fn from(state: AppState) -> Self {
        Self {
            name: state.app_name,
            version: state.version,
            greeting_count: state.greeting_count,
            initialized: state.initialized,
        }
    }
}

#[tauri::command]
pub fn get_app_info(state: State<AppStateMutex>) -> Result<AppInfo, AppError> {
    let app_state = state.lock().map_err(|e| AppError::State(e.to_string()))?;

    Ok(app_state.clone().into())
}

#[tauri::command]
pub fn initialize_app(state: State<AppStateMutex>) -> Result<AppInfo, AppError> {
    let mut app_state = state.lock().map_err(|e| AppError::State(e.to_string()))?;

    app_state.initialized = true;

    Ok(app_state.clone().into())
}

#[tauri::command]
pub fn reset_greeting_count(state: State<AppStateMutex>) -> Result<AppInfo, AppError> {
    let mut app_state = state.lock().map_err(|e| AppError::State(e.to_string()))?;

    app_state.greeting_count = 0;

    Ok(app_state.clone().into())
}
