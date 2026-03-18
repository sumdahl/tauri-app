use crate::error::AppError;
use crate::state::AppStateMutex;
use tauri::State;

#[derive(serde::Serialize)]
pub struct GreetResponse {
    pub message: String,
    pub greeting_count: u64,
}

#[tauri::command]
pub fn greet(name: &str, state: State<AppStateMutex>) -> Result<GreetResponse, AppError> {
    if name.trim().is_empty() {
        return Err(AppError::InvalidInput("Name cannot be empty".to_string()));
    }

    let mut app_state = state.lock().map_err(|e| AppError::State(e.to_string()))?;

    app_state.greeting_count += 1;
    let count = app_state.greeting_count;

    Ok(GreetResponse {
        message: format!("Hello, {}! You've been greeted from Rust!", name),
        greeting_count: count,
    })
}
