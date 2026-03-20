use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Authentication failed: {0}")]
    Auth(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("JWT error: {0}")]
    Jwt(String),

    #[error("Password hashing error: {0}")]
    PasswordHash(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("HTTP error: {0}")]
    Http(String),
}

// Required for Tauri commands
impl serde::Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

impl AppError {
    fn status_code(&self) -> axum::http::StatusCode {
        match self {
            Self::Auth(_) => axum::http::StatusCode::UNAUTHORIZED,
            Self::InvalidInput(_) | Self::Config(_) | Self::Serialization(_) => {
                axum::http::StatusCode::BAD_REQUEST
            }
            Self::NotFound(_) => axum::http::StatusCode::NOT_FOUND,
            Self::Database(_)
            | Self::Jwt(_)
            | Self::PasswordHash(_)
            | Self::Io(_)
            | Self::Http(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl axum::response::IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let payload = axum::Json(serde_json::json!({ "message": self.to_string() }));
        (self.status_code(), payload).into_response()
    }
}
