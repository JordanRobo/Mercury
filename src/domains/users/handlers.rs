use crate::auth::handlers::hash_password;
use crate::db::DbError;
use crate::utils::{generate_salt, get_current_timestamp, slug_gen};
use actix_web::HttpResponse;
use diesel::prelude::*;

use super::models::{CreateUser, User};

pub fn get_all_users(conn: &mut SqliteConnection) -> Result<Vec<User>, DbError> {
    use crate::db::schema::users::dsl::*;

    let response = users.load::<User>(conn)?;
    Ok(response)
}

pub fn get_user_by_id(_conn: &mut SqliteConnection, _user_id: String) -> Result<User, DbError> {
    todo!("Get User by ID Database function")
}

pub fn create_new_user(_conn: &mut SqliteConnection, _body: CreateUser) -> Result<(), DbError> {
    todo!("Create new User Database function")
}

pub fn create_admin(conn: &mut SqliteConnection, body: CreateUser) -> Result<HttpResponse, DbError> {
    use crate::db::schema::users::dsl::*;

    let count = users
        .count()
        .get_result(conn)
        .unwrap_or(0);

    if count == 0 {
        let current_time = get_current_timestamp();
        let salt = generate_salt();

        let admin = User {
            id: xid::new().to_string(),
            name: body.name.clone(),
            slug: slug_gen(&body.name),
            email: body.email,
            pass_hash: hash_password(&body.password, &salt),
            pass_salt: salt.to_vec(),
            role: "Admin".to_string(),
            bio: None,
            profile_picture: None,
            created_at: Some(current_time),
        };

        diesel::insert_into(users)
            .values(&admin)
            .execute(conn)?;

        Ok(HttpResponse::Created().body("Admin account successfuly created"))
    } else {
        Ok(HttpResponse::Forbidden().body("Admin Account already created"))
    }
}
