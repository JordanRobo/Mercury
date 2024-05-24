#![allow(unused)]
#![allow(clippy::all)]

use diesel::Queryable;
use serde::Serialize;

#[derive(Queryable, Debug, Serialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub authorid: i32,
    pub published: Option<bool>,
}