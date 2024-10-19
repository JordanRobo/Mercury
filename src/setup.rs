use actix_web::{
    error,
    web::{block, Data, Json},
    HttpResponse, Responder, Result,
};
use diesel::prelude::*;
use std::env;
use std::fs::OpenOptions;
use std::io::Write;

use crate::db::schema::users::dsl::*;
use crate::db::{DbError, DbPool};
use crate::users::models::{CreateUser, User};
use crate::utils::{generate_secret, get_current_timestamp, slug_gen};

pub fn run_setup() -> std::io::Result<()> {
    let jwt_secret = generate_secret(64);
    let site_id = generate_secret(32);

    let env_contents = format!("JWT_SECRET={}\nSITE_ID={}\nDATABASE_URL=mercury.db", jwt_secret, site_id);

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(".env")?;

    file.write_all(env_contents.as_bytes())?;

    println!("Setup complete. Environment file (.env) has been created with secure random values.");
    Ok(())
}

pub async fn create_admin(pool: Data<DbPool>, body: Json<CreateUser>) -> Result<impl Responder> {
    let admin_input = body.into_inner();

    let admin = block(move || {
        let mut conn = pool.get()?;
        new_admin(&mut conn, admin_input)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    env::remove_var("SETUP_TOKEN");

    Ok(HttpResponse::Created().json(admin))
}

fn new_admin(conn: &mut SqliteConnection, body: CreateUser) -> Result<User, DbError> {
    let current_time = get_current_timestamp();

    let admin = User {
        id: xid::new().to_string(),
        name: body.name.clone(),
        slug: slug_gen(&body.name),
        email: body.email,
        pass: body.pass,
        role: "Admin".to_string(),
        bio: None,
        profile_picture: None,
        created_at: Some(current_time),
    };

    diesel::insert_into(users)
        .values(&admin)
        .execute(conn)?;

    Ok(admin)
}

pub fn check_admin(conn: &mut SqliteConnection) -> bool {
    let count = users
        .count()
        .get_result(conn)
        .unwrap_or(0);
    count == 0
}
