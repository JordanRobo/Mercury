use diesel::prelude::*;
use serde::{ Serialize, Deserialize };
use crate::db::schema::*;
use chrono::NaiveDateTime;

#[derive(Queryable, Debug, Serialize, Deserialize, Insertable)]
#[diesel(belongs_to(Author))]
#[diesel(table_name = posts)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub slug: String,
    pub excerpt: Option<String>,
    pub content: Option<String>,
    pub author_id: Option<String>,
    pub feature_image: Option<String>,
    pub status: Option<String>,
    pub published_at: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePost {
    pub title: String,
    pub excerpt: Option<String>,
    pub content: Option<String>,
    pub author_id: Option<String>,
    pub feature_image: Option<String>,
    pub status: Option<String>,
}

#[derive(Insertable, Deserialize, AsChangeset)]
#[diesel(table_name = posts)]
pub struct UpdatePost {
    pub title: String,
    pub slug: String,
    pub excerpt: Option<String>,
    pub content: Option<String>,
    pub author_id: Option<String>,
    pub feature_image: Option<String>,
    pub status: Option<String>,
    pub published_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}