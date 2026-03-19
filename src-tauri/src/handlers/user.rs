use serde::{Deserialize, Serialize};
use tauri::State;
use crate::state::AppState;
use uuid::Uuid;
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHasher, SaltString, PasswordVerifier
    },
    Argon2, PasswordHash
};
use crate::utils::jwt;
use sqlx::{PgPool, self}; // Import PgPool

// --- Signup Structs ---
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

// --- Signin Structs ---
#[derive(Debug, Deserialize)]
pub struct SigninRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct SigninResponse {
    pub message: String,
    pub token: String,
}


// --- Signup Logic ---
async fn signup_core(request: SignupRequest, db_pool: PgPool) -> Result<SignupResponse, String> {
    let password_to_hash = request.password.clone();
    let hashed_password = tauri::async_runtime::spawn_blocking(move || -> Result<String, String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2.hash_password(password_to_hash.as_bytes(), &salt)
            .map_err(|e| format!("Password hashing failed: {}", e))?;
        Ok(password_hash.to_string())
    }).await.map_err(|e| format!("Threading error: {}", e))??;

    let user_id = Uuid::new_v4();

    sqlx::query!(
        "INSERT INTO users (id, fullname, email, hashed_password) VALUES ($1, $2, $3, $4)",
        user_id,
        request.fullname,
        request.email,
        hashed_password
    )
    .execute(&db_pool)
    .await
    .map_err(|e| {
        if let Some(db_err) = e.as_database_error() {
            if db_err.is_unique_violation() {
                return "A user with this email already exists.".to_string();
            }
        }
        format!("Failed to create user: {}", e)
    })?;

    let token = jwt::generate_token(&user_id)?;
    Ok(SignupResponse {
        message: format!("User {} created successfully!", request.fullname),
        user_id: user_id.to_string(),
        token,
    })
}

// --- Signin Logic ---
async fn signin_core(request: SigninRequest, db_pool: PgPool) -> Result<SigninResponse, String> {
    let user = sqlx::query!(
        "SELECT id, fullname, hashed_password FROM users WHERE email = $1",
        request.email
    )
    .fetch_optional(&db_pool)
    .await
    .map_err(|e| format!("Database query failed: {}", e))?
    .ok_or_else(|| "Invalid email or password.".to_string())?;

    let hashed_password_string = user.hashed_password.clone();
    let request_password_string = request.password.clone();

    tauri::async_runtime::spawn_blocking(move || {
        let parsed_hash = PasswordHash::new(&hashed_password_string)
            .map_err(|_| "Stored password hash is invalid".to_string())?;
        Argon2::default().verify_password(request_password_string.as_bytes(), &parsed_hash)
            .map_err(|_| "Invalid email or password.".to_string())
    }).await.map_err(|e| format!("Threading error: {}", e))??;

    let token = jwt::generate_token(&user.id)?;

    Ok(SigninResponse {
        message: format!("Welcome back, {}!", user.fullname),
        token,
    })
}


// --- Public Wrappers for Tauri and Axum ---

#[tauri::command]
pub async fn signup(request: SignupRequest, state: State<'_, AppState>) -> Result<SignupResponse, String> {
    signup_core(request, state.db.clone()).await
}

pub async fn signup_from_axum(request: SignupRequest, db_pool: PgPool) -> Result<SignupResponse, String> {
    signup_core(request, db_pool).await
}

#[tauri::command]
pub async fn signin(request: SigninRequest, state: State<'_, AppState>) -> Result<SigninResponse, String> {
    signin_core(request, state.db.clone()).await
}

pub async fn signin_from_axum(request: SigninRequest, db_pool: PgPool) -> Result<SigninResponse, String> {
    signin_core(request, db_pool).await
}
