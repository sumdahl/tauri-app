use jsonwebtoken::{encode, Header, EncodingKey, TokenData};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};
use std::env;
use uuid::Uuid; // Import Uuid

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // Subject (user ID)
    pub exp: usize,  // Expiration time
    pub iat: usize,  // Issued at time
}

pub fn generate_token(user_id: &Uuid) -> Result<String, String> {
    let jwt_secret = env::var("JWT_SECRET").map_err(|_| "JWT_SECRET not set".to_string())?;
    let encoding_key = EncodingKey::from_secret(jwt_secret.as_bytes());

    let now = Utc::now();
    let expiration = now + Duration::hours(24); // Token valid for 24 hours

    let claims = Claims {
        sub: user_id.to_string(),
        iat: now.timestamp() as usize,
        exp: expiration.timestamp() as usize,
    };

    encode(&Header::default(), &claims, &encoding_key)
        .map_err(|e| format!("Failed to encode token: {}", e))
}

pub fn decode_token(token: &str) -> Result<TokenData<Claims>, String> {
    let jwt_secret = env::var("JWT_SECRET").map_err(|_| "JWT_SECRET not set".to_string())?;
    let decoding_key = jsonwebtoken::DecodingKey::from_secret(jwt_secret.as_bytes());
    let validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256);

    jsonwebtoken::decode::<Claims>(token, &decoding_key, &validation)
        .map_err(|e| format!("Failed to decode token: {}", e))
}
