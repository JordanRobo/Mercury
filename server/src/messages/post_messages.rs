use crate::db::Post;
use actix::Message;
use diesel::QueryResult;

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<Post>>")]
pub struct FetchPosts;

#[derive(Message)]
#[rtype(result = "QueryResult<Post>")]
pub struct FetchPost {
    pub id: i32,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Post>")]
pub struct CreatePost {
    pub title: Option<String>,
    pub content: Option<String>,
    pub authorid: Option<i32>
}

#[derive(Message)]
#[rtype(result = "QueryResult<Post>")]
pub struct UpdatePost {
    pub id: i32,
    pub title: Option<String>,
    pub content: Option<String>,
    pub authorid: Option<i32>,
    pub published: Option<bool>
}

#[derive(Message)]
#[rtype(result = "QueryResult<usize>")]
pub struct DeletePost {
    pub id: i32,
}