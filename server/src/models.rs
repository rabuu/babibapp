use serde::{Deserialize, Serialize};

use crate::schema::students;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct Student {
    pub id: i32,
    pub last_name: String,
    pub first_name: String,
    pub is_admin: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, Queryable)]
#[table_name = "students"]
pub struct NewStudent {
    pub first_name: String,
    pub last_name: String,
}