use crate::authors::models::Author;
use crate::db::schema::posts;
use crate::tags::models::Tag;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub enum PostResponse {
    Posts(Vec<Post>),
    PostsAuthor(Vec<PostWithAuthor>),
    PostsTags(Vec<PostWithTags>),
    PostsAuthorTags(Vec<PostWithAuthorAndTags>),
}

#[derive(
    Queryable, Identifiable, Associations, Debug, Serialize, Deserialize, Default, Insertable,
)]
#[diesel(belongs_to(Author, foreign_key = author_id))]
#[diesel(primary_key(post_id))]
#[diesel(table_name = posts)]
pub struct Post {
    pub post_id: String,
    pub title: String,
    pub slug: String,
    pub excerpt: Option<String>,
    pub content: Option<String>,
    pub author_id: String,
    pub feature_image: Option<String>,
    pub status: Option<String>,
    pub published_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct PostWithAuthor {
    pub post: Post,
    pub author: Option<Author>,
}

#[derive(Debug, Serialize)]
pub struct PostWithTags {
    pub post: Post,
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Serialize)]
pub struct PostWithAuthorAndTags {
    pub post: Post,
    pub author: Option<Author>,
    pub tags: Option<Vec<Tag>>,
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
