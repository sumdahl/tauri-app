use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    pub app_name: String,
    pub version: String,
    pub greeting_count: u64,
    pub initialized: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            app_name: "tauri-app".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            greeting_count: 0,
            initialized: false,
        }
    }
}

pub type AppStateMutex = Mutex<AppState>;

pub fn create_app_state() -> AppState {
    AppState::default()
}
