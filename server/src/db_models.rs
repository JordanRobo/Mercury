#![allow(unused)]
#![allow(clippy::all)]

use diesel::Queryable;
use serde::Serialize;

#[derive(Queryable, Debug, Serialize)]
pub struct User {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
}