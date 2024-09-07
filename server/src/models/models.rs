use crate::db::schema::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = authors)]
pub struct Author {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub email: String,
    pub bio: Option<String>,
    pub profile_picture: Option<String>,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Queryable, Debug, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = enquiries)]
pub struct Enquiry {
    pub id: String,
    pub name: String,
    pub surname: Option<String>,
    pub email: String,
    pub job_title: Option<String>,
    pub company: Option<String>,
    pub enquiry_type: Option<String>,
    pub subject: Option<String>,
    pub message: String,
    pub status: Option<String>,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Queryable, Debug, Identifiable, Serialize, Deserialize)]
#[diesel(primary_key(file_path))]
#[diesel(table_name = media)]
pub struct Media {
    pub id: String,
    pub file_name: String,
    pub file_type: String,
    pub file_size: i32,
    pub file_path: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Queryable, Debug, Identifiable, Serialize, Deserialize)]
#[diesel(belongs_to(Post))]
#[diesel(belongs_to(Tag))]
#[diesel(table_name = post_tags)]
#[diesel(primary_key(post_id, tag_id))]
pub struct PostTag {
    pub post_id: Option<String>,
    pub tag_id: Option<String>,
}

#[derive(Queryable, Debug, Serialize, Deserialize)]
#[diesel(table_name = subscriptions)]
pub struct Subscription {
    pub id: Option<String>,
    pub email: String,
    pub status: Option<String>,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Queryable, Debug, Serialize, Deserialize)]
#[diesel(table_name = tags)]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub slug: String,
}

#[derive(Queryable, Debug, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub api_key: Option<String>,
    pub created_at: Option<NaiveDateTime>,
}
