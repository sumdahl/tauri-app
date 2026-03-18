mod commands;
mod error;
mod state;

use state::create_app_state;
use std::sync::Mutex;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    log::info!("Starting application...");

    let app_state = Mutex::new(create_app_state());

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            commands::greet::greet,
            commands::app::get_app_info,
            commands::app::initialize_app,
            commands::app::reset_greeting_count,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
