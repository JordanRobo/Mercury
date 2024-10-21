use crate::db::schema::users;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Identifiable, Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub email: String,
    pub pass_hash: String,
    pub pass_salt: Vec<u8>,
    pub role: String,
    pub bio: Option<String>,
    pub profile_picture: Option<String>,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Author {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub email: String,
    pub bio: String,
    pub profile_picture: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUser {
    pub id: String,
}
