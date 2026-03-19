pub fn load_env_files() {
    let _ = dotenvy::from_filename("src-tauri/.env");
    let _ = dotenvy::from_filename(".env");
}

pub fn init_tracing() {
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("tauri_app=debug"));

    let _ = tracing_subscriber::fmt().with_env_filter(filter).try_init();
}
