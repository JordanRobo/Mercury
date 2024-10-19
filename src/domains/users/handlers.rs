use crate::db::{schema::users, DbError};
use diesel::prelude::*;

use super::models::{CreateUser, User};

pub fn get_all_users(conn: &mut SqliteConnection) -> Result<Vec<User>, DbError> {
    let users = users::table.load::<User>(conn)?;
    Ok(users)
}

pub fn get_user_by_id(_conn: &mut SqliteConnection, _user_id: String) -> Result<User, DbError> {
    todo!("Get User by ID Database function")
}

pub fn create_new_user(_conn: &mut SqliteConnection, _body: CreateUser) -> Result<(), DbError> {
    todo!("Create new User Database function")
}
