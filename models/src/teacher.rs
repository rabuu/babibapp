use serde::{Deserialize, Serialize};

use babibapp_schema::schema::teachers;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable)]
pub struct Teacher {
    pub id: i32,
    pub name: String,
    pub prefix: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable)]
#[table_name = "teachers"]
pub struct NewTeacher {
    pub name: String,
    pub prefix: String,
}
