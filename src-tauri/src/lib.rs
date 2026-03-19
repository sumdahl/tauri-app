pub mod error;
pub mod state;
pub mod handlers;
pub mod utils;

use state::AppState;
use std::env;
use sqlx::postgres::PgPoolOptions;
use dotenvy::dotenv;

#[tauri::command]
fn hello_world() -> String {
    "Hello from Tauri!".into()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Load environment variables from .env file
    dotenv().ok();

    // Initialize logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    log::info!("Starting application...");

    // Setup the database connection pool
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_lazy(&database_url)
        .expect("Failed to create database pool.");

    let app_state = AppState::new(pool);

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            hello_world,
            handlers::user::signup,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
