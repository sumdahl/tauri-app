use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, PasswordVerifier, SaltString},
    Argon2, PasswordHash,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{error::AppError, repository::user_repo, utils::jwt};

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
            .map_err(|error| AppError::PasswordHash(error.to_string()))
    })
    .await
    .map_err(|error| AppError::PasswordHash(error.to_string()))??;

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
    .map_err(|error| AppError::Auth(error.to_string()))??;

    let token = jwt::generate_token(&user.id).map_err(AppError::Jwt)?;

    Ok(SigninResponse {
        message: format!("Welcome back, {}!", user.fullname),
        user_id: user.id.to_string(),
        token,
    })
}
