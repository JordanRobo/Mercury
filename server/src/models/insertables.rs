use crate::db::schema::{ authors, posts, tags };
use diesel::Insertable;
use serde::Serialize;

#[derive(Insertable, Serialize, Clone)]
#[diesel(table_name=authors)]
pub struct NewAuthor {
    pub name: String,
    pub email: String,
    pub bio: Option<String>,
    pub profile_picture: Option<String>
}

#[derive(Insertable, Serialize, Clone)]
#[diesel(table_name=posts)]
pub struct NewPost {
    pub title: String,
    pub slug: String,
    pub content: String,
    pub feature_image: Option<String>,
    pub excerpt: Option<String>,
    pub published: Option<bool>,
    pub author_id: Option<i32>
}

#[derive(Insertable, Serialize, Clone)]
#[diesel(table_name=tags)]
pub struct NewTag {
    pub title: String,
    pub slug: String
}