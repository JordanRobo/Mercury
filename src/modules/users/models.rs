use crate::{
    auth::AuthService,
    db::{schema::users, DbError},
    Utils,
};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Identifiable, Debug, Serialize, Deserialize, Insertable, Clone)]
#[diesel(table_name = users)]
#[diesel(primary_key(user_id))]
pub struct User {
    pub user_id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub pass_hash: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub last_login: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
    pub first_name: String,
    pub last_name: Option<String>,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUser {
    pub id: String,
}

impl User {
    pub fn new(user: CreateUser) -> Self {
        let user_id = xid::new().to_string();
        let now = Utils::get_current_timestamp();
        let salt = Utils::generate_salt();
        let pass_hash = AuthService::create_password(&user.password, &salt);


        Self { 
            user_id, 
            email: user.email, 
            first_name: user.first_name, 
            last_name: user.last_name, 
            pass_hash, 
            created_at: now, 
            updated_at: now, 
            last_login: now 
        }
    }

    pub fn create(conn: &mut SqliteConnection, user: CreateUser) -> Result<Self, DbError> {
        
        let new_user = Self::new(user);

        diesel::insert_into(users::table)
            .values(&new_user)
            .execute(conn)?;

        Ok(new_user)
    }

    pub fn update() -> Result<Self, DbError> {
        todo!("Create User update fn")
    }

    pub fn delete() -> Result<Self, DbError> {
        todo!("Create User delete fn")
    }

    pub fn fetch_all(conn: &mut SqliteConnection) -> Result<Vec<Self>, DbError> {
        let users = users::table.load::<Self>(conn)?;
        Ok(users)
    }
}
