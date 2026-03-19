use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, PasswordVerifier, SaltString},
    Argon2, PasswordHash,
};
use serde::{Deserialize, Serialize};
use tauri::State;
use uuid::Uuid;

use crate::{error::AppError, state::AppState, utils::jwt};

// ── Request / Response types ─────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct SignupRequest {
    pub fullname: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct SignupResponse {
    pub message: String,
    pub user_id: String,
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct SigninRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct SigninResponse {
    pub message: String,
    pub user_id: String,
    pub token: String,
}

// ── Core logic ───────────────────────────────────────────────

async fn signup_core(request: SignupRequest, db: sqlx::PgPool) -> Result<SignupResponse, AppError> {
    let password = request.password.clone();

    let hashed_password = tauri::async_runtime::spawn_blocking(move || {
        let salt = SaltString::generate(&mut OsRng);
        Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map(|h| h.to_string())
            .map_err(|e| AppError::PasswordHash(e.to_string()))
    })
    .await
    .map_err(|e| AppError::PasswordHash(e.to_string()))??;

    let user_id = Uuid::new_v4();

    sqlx::query!(
        "INSERT INTO users (id, fullname, email, hashed_password) VALUES ($1, $2, $3, $4)",
        user_id,
        request.fullname,
        request.email,
        hashed_password
    )
    .execute(&db)
    .await
    .map_err(|e| {
        if let Some(db_err) = e.as_database_error() {
            if db_err.is_unique_violation() {
                return AppError::InvalidInput(
                    "A user with this email already exists.".to_string(),
                );
            }
        }
        AppError::Database(e)
    })?;

    let token = jwt::generate_token(&user_id).map_err(AppError::Jwt)?;

    Ok(SignupResponse {
        message: format!(
            "Welcome, {}! Your account has been created.",
            request.fullname
        ),
        user_id: user_id.to_string(),
        token,
    })
}

async fn signin_core(request: SigninRequest, db: sqlx::PgPool) -> Result<SigninResponse, AppError> {
    let user = sqlx::query!(
        "SELECT id, fullname, hashed_password FROM users WHERE email = $1",
        request.email
    )
    .fetch_optional(&db)
    .await
    .map_err(AppError::Database)?
    .ok_or_else(|| AppError::Auth("Invalid email or password.".to_string()))?;

    let hashed = user.hashed_password.clone();
    let password = request.password.clone();

    tauri::async_runtime::spawn_blocking(move || {
        let parsed = PasswordHash::new(&hashed)
            .map_err(|_| AppError::Auth("Stored password hash is invalid.".to_string()))?;
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed)
            .map_err(|_| AppError::Auth("Invalid email or password.".to_string()))
    })
    .await
    .map_err(|e| AppError::Auth(e.to_string()))??;

    let token = jwt::generate_token(&user.id).map_err(AppError::Jwt)?;

    Ok(SigninResponse {
        message: format!("Welcome back, {}!", user.fullname),
        user_id: user.id.to_string(),
        token,
    })
}

// ── Tauri commands ───────────────────────────────────────────

#[tauri::command]
pub async fn signup(
    fullname: String,
    email: String,
    password: String,
    state: State<'_, AppState>,
) -> Result<SignupResponse, AppError> {
    signup_core(
        SignupRequest {
            fullname,
            email,
            password,
        },
        state.db.clone(),
    )
    .await
}

#[tauri::command]
pub async fn signin(
    email: String,
    password: String,
    state: State<'_, AppState>,
) -> Result<SigninResponse, AppError> {
    signin_core(SigninRequest { email, password }, state.db.clone()).await
}
