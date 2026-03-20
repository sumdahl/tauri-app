use std::env;

#[path = "../domain/mod.rs"]
mod domain;
#[path = "../error.rs"]
mod error;
#[path = "../repository/mod.rs"]
mod repository;
#[path = "../services/mod.rs"]
mod services;
#[path = "../state.rs"]
mod state;
#[path = "../utils/mod.rs"]
mod utils;

use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use serde::Serialize;
use tower_http::cors::CorsLayer;

use crate::{
    error::AppError,
    services::auth::{self, SigninRequest, SigninResponse, SignupRequest, SignupResponse},
    state::AppState,
    utils::env::{init_tracing, load_env_files},
};

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    load_env_files();
    init_tracing();

    let bind_address = env::var("BACKEND_ADDRESS").unwrap_or_else(|_| "0.0.0.0:3000".to_string());

    let state = AppState::from_env().await?;

    let app = Router::new()
        .route("/health", get(health))
        .route("/auth/signup", post(signup))
        .route("/auth/signin", post(signin))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&bind_address).await?;
    tracing::info!(%bind_address, "Auth backend listening");
    axum::serve(listener, app).await.map_err(AppError::Io)
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

async fn signup(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>,
) -> Result<Json<SignupResponse>, AppError> {
    auth::signup(request, state.db()).await.map(Json) // ← state.db()
}

async fn signin(
    State(state): State<AppState>,
    Json(request): Json<SigninRequest>,
) -> Result<Json<SigninResponse>, AppError> {
    auth::signin(request, state.db()).await.map(Json) // ← state.db()
}
