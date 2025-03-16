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
    pub fn create(conn: &mut SqliteConnection, input: CreateUser) -> Result<Self, DbError> {
        let id = xid::new().to_string();
        let now = Utils::get_current_timestamp();
        let salt = Utils::generate_salt();

        let new_user = User {
            user_id: id,
            email: input.email,
            first_name: input.first_name,
            last_name: Some(input.last_name.unwrap()),
            pass_hash: AuthService::create_password(&input.password, &salt),
            created_at: now,
            updated_at: now,
            last_login: now,
        };

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

    pub fn get_all(conn: &mut SqliteConnection) -> Result<Vec<Self>, DbError> {
        let users = users::table.load::<Self>(conn)?;
        Ok(users)
    }
}
