use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, PasswordVerifier, SaltString},
    Argon2, PasswordHash,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{error::AppError, repository::user_repo, utils::jwt};

// ── Shared request / response types ─────────────────────────

#[derive(Debug, Deserialize, Serialize)]
pub struct SignupRequest {
    pub fullname: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignupResponse {
    pub message: String,
    pub user_id: String,
    pub token: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SigninRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SigninResponse {
    pub message: String,
    pub user_id: String,
    pub token: String,
}

// ── DB path (desktop + server) ───────────────────────────────

pub async fn signup(request: SignupRequest, db: &PgPool) -> Result<SignupResponse, AppError> {
    let fullname = request.fullname.trim().to_string();
    let email = request.email.trim().to_lowercase();
    let password = request.password;

    if fullname.len() < 2 {
        return Err(AppError::InvalidInput(
            "Full name must be at least 2 characters.".to_string(),
        ));
    }
    if email.is_empty() {
        return Err(AppError::InvalidInput("Email is required.".to_string()));
    }
    if password.len() < 8 {
        return Err(AppError::InvalidInput(
            "Password must be at least 8 characters.".to_string(),
        ));
    }

    let hashed_password = tokio::task::spawn_blocking(move || {
        let salt = SaltString::generate(&mut OsRng);
        Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
            .map_err(|e| AppError::PasswordHash(e.to_string()))
    })
    .await
    .map_err(|e| AppError::PasswordHash(e.to_string()))??;

    let user_id = Uuid::new_v4();
    user_repo::create_user(db, user_id, &fullname, &email, &hashed_password).await?;

    let token = jwt::generate_token(&user_id).map_err(AppError::Jwt)?;

    Ok(SignupResponse {
        message: format!("Welcome, {fullname}! Your account has been created."),
        user_id: user_id.to_string(),
        token,
    })
}

pub async fn signin(request: SigninRequest, db: &PgPool) -> Result<SigninResponse, AppError> {
    let email = request.email.trim().to_lowercase();
    let password = request.password;

    if email.is_empty() {
        return Err(AppError::InvalidInput("Email is required.".to_string()));
    }
    if password.is_empty() {
        return Err(AppError::InvalidInput("Password is required.".to_string()));
    }

    let user = user_repo::find_user_by_email(db, &email)
        .await?
        .ok_or_else(|| AppError::Auth("Invalid email or password.".to_string()))?;

    let hashed_password = user.hashed_password.clone();

    tokio::task::spawn_blocking(move || {
        let parsed_hash = PasswordHash::new(&hashed_password)
            .map_err(|_| AppError::Auth("Stored password hash is invalid.".to_string()))?;
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
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

// ── HTTP path (Android → Axum backend) ──────────────────────

#[derive(Debug, Deserialize)]
struct ApiError {
    message: String,
}

pub async fn signup_http(
    http: &reqwest::Client,
    backend_url: &str,
    request: SignupRequest,
) -> Result<SignupResponse, AppError> {
    let res = http
        .post(format!("{}/auth/signup", backend_url))
        .json(&request)
        .send()
        .await
        .map_err(|e| AppError::Http(e.to_string()))?;

    if !res.status().is_success() {
        let err: ApiError = res
            .json()
            .await
            .map_err(|e| AppError::Http(e.to_string()))?;
        return Err(AppError::Auth(err.message));
    }

    res.json::<SignupResponse>()
        .await
        .map_err(|e| AppError::Http(e.to_string()))
}

pub async fn signin_http(
    http: &reqwest::Client,
    backend_url: &str,
    request: SigninRequest,
) -> Result<SigninResponse, AppError> {
    let res = http
        .post(format!("{}/auth/signin", backend_url))
        .json(&request)
        .send()
        .await
        .map_err(|e| AppError::Http(e.to_string()))?;

    if !res.status().is_success() {
        let err: ApiError = res
            .json()
            .await
            .map_err(|e| AppError::Http(e.to_string()))?;
        return Err(AppError::Auth(err.message));
    }

    res.json::<SigninResponse>()
        .await
        .map_err(|e| AppError::Http(e.to_string()))
}
