pub mod error;
pub mod handlers;
pub mod state;
pub mod utils;

use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use state::AppState;
use std::env;

#[tauri::command]
fn hello_world() -> String {
    "Hello from Tauri!".into()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
// 1. Add `async` here
pub async fn run() {
    // Load environment variables from .env file
    dotenv().ok();

    // Initialize logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    log::info!("Starting application...");

    // Setup the database connection pool
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        // 2. Change to `connect` and add `.await` to fail-fast if DB is down
        .connect(&database_url)
        .await
        .expect("Failed to connect to the database. Is Docker running?");

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
