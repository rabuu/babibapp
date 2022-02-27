use serde::{Deserialize, Serialize};

use babibapp_schema::schema::students;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct Student {
    pub id: i32,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password_hash: String,
    pub is_admin: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[table_name = "students"]
pub struct NewStudent {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password_hash: String,
    pub is_admin: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterStudent {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
    pub is_admin: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginStudent {
    pub email: String,
    pub password: String,
}
