use actix_web::HttpRequest;
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::error::BabibappError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: i32,
    pub admin: bool,
    exp: i64,
}

impl Claims {
    pub fn new(id: i32, admin: bool, expiration_hours: i64) -> Self {
        Claims {
            id,
            admin,
            exp: (Utc::now() + Duration::hours(expiration_hours)).timestamp(),
        }
    }

    pub fn root(expiration_minutes: i64) -> Self {
        Claims {
            id: 0,
            admin: true,
            exp: (Utc::now() + Duration::minutes(expiration_minutes)).timestamp(),
        }
    }
}

pub fn create_jwt(claims: Claims, secret: String) -> Result<String, BabibappError> {
    let encoding_key = EncodingKey::from_secret(secret.as_bytes());
    jsonwebtoken::encode(&Header::default(), &claims, &encoding_key).map_err(|e| e.into())
}

pub fn decode_jwt(token: &str, secret: String) -> Result<Claims, BabibappError> {
    let decoding_key = DecodingKey::from_secret(secret.as_bytes());
    jsonwebtoken::decode::<Claims>(token, &decoding_key, &Validation::default())
        .map(|data| data.claims)
        .map_err(|e| e.into())
}

pub fn validate_token(token: &str, secret: String) -> Result<Claims, BabibappError> {
    let claims = decode_jwt(token, secret)?;
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
    pub fn from_jwt(jwt: &str) -> Self {
        TokenWrapper {
            token: jwt.to_string(),
        }
    }

    pub fn from_claims(claims: Claims, secret: String) -> Result<Self, BabibappError> {
        let jwt = create_jwt(claims, secret)?;
        Ok(Self::from_jwt(&jwt))
    }

    pub fn from_request(req: HttpRequest) -> Result<Self, BabibappError> {
        let auth_header = req
            .headers()
            .get(actix_web::http::header::AUTHORIZATION)
            .ok_or(anyhow::anyhow!("No authorization header"))?
            .to_str()?;

        let wrapped = Self::from_jwt(auth_header);
        Ok(wrapped)
    }

    pub fn validate(&self, secret: String) -> Result<Claims, BabibappError> {
        validate_token(&self.token, secret).map_err(|_| anyhow::anyhow!("Invalid token").into())
    }
}
