use schema::users;
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
    pub hash: String, //crypted with generated salt by postgres
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub created_at: NaiveDateTime,
}


