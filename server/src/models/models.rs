use diesel::prelude::*;
use serde::{ Serialize, Deserialize };
use crate::db::schema::*;

pub type DbError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq, Serialize, Deserialize, Insertable, AsChangeset, Clone)]
#[diesel(table_name = authors)]
pub struct Author {
    #[serde(default)]
    pub id: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub bio: Option<String>,
    pub profile_picture: Option<String>,
}

#[derive(Insertable, Deserialize, AsChangeset)]
#[diesel(table_name = authors)]
pub struct NewAuthor {
    pub name: Option<String>,
    pub email: Option<String>,
    pub bio: Option<String>,
    pub profile_picture: Option<String>,
}

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq, Serialize, Deserialize, Associations, Insertable)]
#[diesel(belongs_to(Post))]
#[diesel(belongs_to(Tag))]
#[diesel(table_name = post_tags)]
#[diesel(primary_key(post_id, tag_id))]
pub struct PostTag {
    pub post_id: String,
    pub tag_id: String,
}

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq, Serialize, Deserialize, Associations, Insertable, AsChangeset, Clone)]
#[diesel(belongs_to(Author))]
#[diesel(table_name = posts)]
pub struct Post {
    pub id: String,
    pub title: Option<String>,
    pub slug: Option<String>,
    pub content: Option<String>,
    pub feature_image: Option<String>,
    pub excerpt: Option<String>,
    pub published: Option<i32>,
    pub author_id: Option<String>,
}

#[derive(Insertable, Deserialize, AsChangeset)]
#[diesel(table_name = posts)]
pub struct NewPost {
    pub title: Option<String>,
    pub slug: Option<String>,
    pub content: Option<String>,
    pub feature_image: Option<String>,
    pub excerpt: Option<String>,
    pub published: Option<i32>,
    pub author_id: Option<String>,
}

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq, Serialize, Deserialize, Insertable)]
#[diesel(table_name = tags)]
pub struct Tag {
    pub id: String,
    pub name: Option<String>,
    pub slug: Option<String>,
}

#[derive(Serialize)]
pub struct PostWithAuthor {
    #[serde(flatten)]
    pub post: Post,
    pub author: Option<Author>,
}