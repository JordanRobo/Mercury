use crate::db::DbPool;
use crate::users::{
    handlers,
    models::{CreateUser, UpdateUser},
};
use actix_web::{
    delete, error, get, patch, post,
    web::{block, Data, Json, Path},
    HttpResponse, Responder, Result,
};

#[get("/users")]
pub async fn get_users(pool: Data<DbPool>) -> Result<impl Responder> {
    let users = block(move || {
        let mut conn = pool.get()?;
        handlers::get_all_users(&mut conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(users))
}

#[get("/users/{user_id}")]
pub async fn get_user(_pool: Data<DbPool>, _user_id: Path<String>) -> Result<impl Responder> {
    Ok(HttpResponse::NotImplemented().body("GET 'user/:id' still in development"))
}

#[get("/users/slug/{user_slug}")]
pub async fn get_user_slug(_pool: Data<DbPool>, _user_slug: Path<String>) -> Result<impl Responder> {
    Ok(HttpResponse::NotImplemented())
}

#[post("/users")]
pub async fn create_user(_pool: Data<DbPool>, _body: Json<CreateUser>) -> Result<impl Responder> {
    Ok(HttpResponse::NotImplemented())
}

#[patch("/users/{user_id}")]
pub async fn update_user(_pool: Data<DbPool>, _user_id: Path<String>, _body: Json<UpdateUser>) -> Result<impl Responder> {
    Ok(HttpResponse::NotImplemented())
}

#[delete("/users/{user_id}")]
pub async fn delete_user(_pool: Data<DbPool>, _user_id: Path<String>) -> Result<impl Responder> {
    Ok(HttpResponse::NotImplemented())
}
