use crate::state::AppState;

pub mod commands;
pub mod domain;
pub mod error;
pub mod repository;
pub mod services;
pub mod state;
pub mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    utils::env::load_env_files();
    utils::env::init_tracing();

    let state = AppState::from_env().await.unwrap();

    tracing::info!("Starting application...");

    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            commands::auth::signup,
            commands::auth::signin
        ])
        .run(tauri::generate_context!())
        .unwrap_or_else(|error| tracing::error!(?error, "error while running tauri application"));
}
