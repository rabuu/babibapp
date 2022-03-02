use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenWrapper {
    pub token: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmailWrapper {
    pub email: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PasswordWrapper {
    pub password: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NameWrapper {
    pub first_name: String,
    pub last_name: String,
}
