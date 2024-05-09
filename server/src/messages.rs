use crate::db_models::User;
use actix::Message;
use diesel::QueryResult;

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<User>>")]
pub struct FetchUser;

#[derive(Message)]
#[rtype(result = "QueryResult<User>")]
pub struct CreateUser {
    pub firstname: String,
    pub lastname: String,
    pub email: String
}

#[derive(Message)]
#[rtype(result = "QueryResult<User>")]
pub struct UpdateUser {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub email: String
}

#[derive(Message)]
#[rtype(result = "QueryResult<usize>")]
pub struct DeleteUser {
    pub id: i32,
}