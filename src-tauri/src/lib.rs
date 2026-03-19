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

    tracing::info!("Starting application...");

    tauri::Builder::default()
        .run(tauri::generate_context!())
        .unwrap_or_else(|error| tracing::error!(?error, "error while running tauri application"));
}
