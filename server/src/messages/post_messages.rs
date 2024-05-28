use crate::db::*;
use actix::Message;
use diesel::QueryResult;

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<(Post, Author, Vec<Tag>)>>")]
pub struct FetchPosts;

#[derive(Message)]
#[rtype(result = "QueryResult<(Post, Author, Vec<Tag>)>")]
pub struct FetchPost {
    pub id: i32,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Post>")]
pub struct CreatePost {
    pub title: Option<String>,
    pub slug: Option<String>,
    pub content: Option<String>,
    pub feature_image: Option<String>,
    pub excerpt: Option<String>,
    pub published: Option<bool>,
    pub author_id: i32
}

#[derive(Message)]
#[rtype(result = "QueryResult<Post>")]
pub struct UpdatePost {
    pub id: i32,
    pub title: Option<String>,
    pub slug: Option<String>,
    pub content: Option<String>,
    pub feature_image: Option<String>,
    pub excerpt: Option<String>,
    pub published: Option<bool>,
    pub author_id: i32
}

#[derive(Message)]
#[rtype(result = "QueryResult<usize>")]
pub struct DeletePost {
    pub id: i32,
}

#[derive(Message)]
#[rtype(result = "QueryResult<usize>")]
pub struct DeletePosts {
    pub ids: Vec<i32>,
}