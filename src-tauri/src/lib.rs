pub mod commands;
pub mod domain;
pub mod error;
pub mod repository;
pub mod services;
pub mod state;
pub mod utils;

use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use state::AppState;
use std::env;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("tauri_app=debug".parse().unwrap()),
        )
        .init();

    tracing::info!("Starting application...");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to the database. Is it running?");

    tracing::info!("Database connected.");

    let app_state = AppState::new(pool);

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            commands::auth::signup,
            commands::auth::signin,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
