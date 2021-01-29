use crate::schema::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Debug, Clone, Queryable)]
pub struct UserQuery {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Insertable)]
#[table_name="users"]
pub struct User {
    pub username: String,
    pub password: String, //crypted with generated salt by postgres
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}


#[derive(Debug, Clone, Queryable)]
pub struct DocumentQuery {
    pub id: i64,
    pub created: NaiveDateTime,
    pub last_updated: Option<NaiveDateTime>,
    pub filetype: String, // crate for filetype?
    pub version: i32,
    pub size: i64,
}

#[derive(Debug, Clone, Insertable)]
#[table_name="documents"]
pub struct Document {
    pub created: NaiveDateTime,
    pub last_updated: Option<NaiveDateTime>,
    pub filetype: String, // crate for filetype?
    pub version: i32,
    pub size: i64,
}
