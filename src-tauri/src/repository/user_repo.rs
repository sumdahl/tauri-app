use sqlx::PgPool;
use uuid::Uuid;

use crate::{domain::models::User, error::AppError};

pub async fn create_user(
    db: &PgPool,
    user_id: Uuid,
    fullname: &str,
    email: &str,
    hashed_password: &str,
) -> Result<(), AppError> {
    sqlx::query(
        "INSERT INTO users (id, fullname, email, hashed_password) VALUES ($1, $2, $3, $4)",
    )
    .bind(user_id)
    .bind(fullname)
    .bind(email)
    .bind(hashed_password)
    .execute(db)
    .await
    .map_err(|error| {
        if let Some(db_error) = error.as_database_error() {
            if db_error.is_unique_violation() {
                return AppError::InvalidInput("A user with this email already exists.".to_string());
            }
        }

        AppError::Database(error)
    })?;

    Ok(())
}

pub async fn find_user_by_email(db: &PgPool, email: &str) -> Result<Option<User>, AppError> {
    sqlx::query_as::<_, User>(
        "SELECT id, fullname, email, hashed_password, created_at FROM users WHERE email = $1",
    )
    .bind(email)
    .fetch_optional(db)
    .await
    .map_err(AppError::Database)
}
