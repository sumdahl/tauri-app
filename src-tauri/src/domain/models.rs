use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub fullname: String,
    pub email: String,
    pub hashed_password: String,
    pub created_at: Option<DateTime<Utc>>,
}
