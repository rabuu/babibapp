use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::error::BabibappError;

// TODO: Move to settings
const JWT_EXPIRATION_HOURS: i64 = 24;
const SECRET: &str = "SECRET";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    email: String,
    exp: i64,
}

impl Claims {
    pub fn new(email: String) -> Self {
        Claims {
            email,
            exp: (Utc::now() + Duration::hours(JWT_EXPIRATION_HOURS)).timestamp(),
        }
    }
}

pub fn create_jwt(claims: Claims) -> Result<String, BabibappError> {
    let encoding_key = EncodingKey::from_secret(SECRET.as_bytes());
    jsonwebtoken::encode(&Header::default(), &claims, &encoding_key).map_err(|e| e.into())
}

pub fn decode_jwt(token: &str) -> Result<Claims, BabibappError> {
    let decoding_key = DecodingKey::from_secret(SECRET.as_bytes());
    jsonwebtoken::decode::<Claims>(token, &decoding_key, &Validation::default())
        .map(|data| data.claims)
        .map_err(|e| e.into())
}

pub fn validate_token(token: &str) -> Result<Claims, BabibappError> {
    let claims = decode_jwt(token)?;
    if claims.exp < Utc::now().timestamp() {
        return Err(BabibappError::from_msg("Token expired!"));
    }
    Ok(claims)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenWrapper {
    pub token: String,
}

impl TokenWrapper {
    pub fn validate(&self) -> Result<Claims, BabibappError> {
        validate_token(&self.token)
    }
}
