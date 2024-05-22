use serde::{Deserialize, Serialize};
use uuid::Uuid;
use diesel::prelude::*;
use crate::schema::students;

#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize, Debug, Clone)]
#[table_name = "students"]
pub struct Student {
    pub id: Uuid,
    pub name: String,
    pub age: i32,
    pub department: String,
}

#[derive(Deserialize)]
pub struct CreateStudent {
    pub name: String,
    pub age: i32,
    pub department: String,
}

#[derive(Deserialize, AsChangeset)]
#[table_name = "students"]
pub struct UpdateStudent {
    pub name: Option<String>,
    pub age: Option<i32>,
    pub department: Option<String>,
}
