use crate::schema::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

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