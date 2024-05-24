use crate::db::Author;
use actix::Message;
use diesel::QueryResult;

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<Author>>")]
pub struct FetchAuthors;

#[derive(Message)]
#[rtype(result = "QueryResult<Author>")]
pub struct FetchAuthor {
    pub id: i32,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Author>")]
pub struct CreateAuthor {
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub email: Option<String>
}

#[derive(Message)]
#[rtype(result = "QueryResult<Author>")]
pub struct UpdateAuthor {
    pub id: i32,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub email: Option<String>
}

#[derive(Message)]
#[rtype(result = "QueryResult<usize>")]
pub struct DeleteAuthor {
    pub id: i32,
}