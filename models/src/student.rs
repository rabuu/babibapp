use serde::{Deserialize, Serialize};

use babibapp_schema::schema::students;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct Student {
    pub id: i32,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password_hash: String,
    pub admin: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[table_name = "students"]
pub struct NewStudent {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password_hash: String,
    pub admin: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterStudent {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
    pub admin: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginStudent {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LimitedViewStudent {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    // should admin be added?
}

#[derive(Debug, Serialize, Deserialize)]
pub enum StudentView {
    Limited(LimitedViewStudent),
    Full(Student),
}
