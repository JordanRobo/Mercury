use crate::db::schema::{ authors, posts };
use diesel::Insertable;
use serde::Serialize;

#[derive(Insertable, Serialize, Clone)]
#[diesel(table_name=authors)]
pub struct NewAuthor {
    pub firstname: String,
    pub lastname: String,
    pub email: String,
}

#[derive(Insertable, Serialize, Clone)]
#[diesel(table_name=posts)]
pub struct NewPost {
    pub title: String,
    pub content: String,
    pub authorid: i32,
}