use axum::{
    routing::{get, post},
    Json, Router, Extension,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// Import logic from the main library crate
use tauri_app_lib::{handlers, state::AppState};
use handlers::user::{SignupRequest, SignupResponse, SigninRequest, SigninResponse};
use sqlx::{postgres::PgPoolOptions};
use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::registry().with(tracing_subscriber::fmt::layer()).init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create database pool.");

    let app_state = Arc::new(AppState::new(pool));

    let app = Router::new()
        .route("/", get(root))
        .route("/signup", post(signup_route_handler))
        .route("/signin", post(signin_route_handler)) // Add the signin route
        .layer(CorsLayer::permissive())
        .layer(Extension(app_state));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Backend server listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello from the standalone Rust backend!"
}

async fn signup_route_handler(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<SignupRequest>,
) -> Result<Json<SignupResponse>, String> {
    handlers::user::signup_from_axum(payload, state.db.clone()).await.map(Json)
}

async fn signin_route_handler(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<SigninRequest>,
) -> Result<Json<SigninResponse>, String> {
    handlers::user::signin_from_axum(payload, state.db.clone()).await.map(Json)
}
