use crate::db::User;
use actix::Message;
use diesel::QueryResult;

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<User>>")]
pub struct FetchUsers;

#[derive(Message)]
#[rtype(result = "QueryResult<User>")]
pub struct FetchUser {
    pub id: i32,
}

#[derive(Message)]
#[rtype(result = "QueryResult<User>")]
pub struct CreateUser {
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub email: Option<String>
}

#[derive(Message)]
#[rtype(result = "QueryResult<User>")]
pub struct UpdateUser {
    pub id: i32,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub email: Option<String>
}

#[derive(Message)]
#[rtype(result = "QueryResult<usize>")]
pub struct DeleteUser {
    pub id: i32,
}