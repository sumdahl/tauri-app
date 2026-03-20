use std::env;

use crate::error::AppError;
use sqlx::{postgres::PgPoolOptions, PgPool};

const DEFAULT_BACKEND_URL: &str = env!("BACKEND_URL");

#[derive(Clone, Debug)]
pub struct AppState {
    pub db: Option<PgPool>,
    pub http: reqwest::Client,
    pub backend_url: String,
}

impl AppState {
    pub fn new(db: Option<PgPool>, http: reqwest::Client, backend_url: String) -> Self {
        Self {
            db,
            http,
            backend_url,
        }
    }

    /// Returns a reference to the PgPool.
    /// Only call this in desktop/server contexts where DB is guaranteed.
    pub fn db(&self) -> &PgPool {
        self.db
            .as_ref()
            .expect("Database not available on this platform")
    }

    pub async fn from_env() -> Result<Self, AppError> {
        let http = reqwest::Client::new();

        // Runtime .env override still works; compile-time default as fallback
        let backend_url =
            env::var("PUBLIC_BACKEND_URL").unwrap_or_else(|_| DEFAULT_BACKEND_URL.to_string());

        #[cfg(not(target_os = "android"))]
        let db = {
            let database_url = env::var("DATABASE_URL")
                .map_err(|_| AppError::Config("DATABASE_URL must be set.".to_string()))?;
            let pool = PgPoolOptions::new()
                .max_connections(10)
                .connect(&database_url)
                .await
                .map_err(AppError::Database)?;
            Some(pool)
        };

        #[cfg(target_os = "android")]
        let db = None;

        Ok(Self::new(db, http, backend_url))
    }
}
