use diesel::SqliteConnection;

use crate::{authors::{Author, CreateAuthor}, db::DbError};

use super::{CreateUser, User};


pub struct UserService {}

impl UserService {
    pub fn new_admin(conn: &mut SqliteConnection, user: CreateUser) -> Result<User, DbError> {
        
        let new_user = User::create(conn, user)?;

        let author = CreateAuthor { 
            user_id: new_user.user_id.clone(), 
            user_name: new_user.first_name.clone() 
        };

        Author::create(conn, author)?;

        Ok(new_user)
    }
}