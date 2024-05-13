use actix_web::{
    get, post, delete, patch, web::{ Data, Json, Path }, HttpResponse, Responder
};
use serde::Deserialize;
use actix::Addr;
use crate::messages::{ FetchUsers, FetchUser, CreateUser, UpdateUser, DeleteUser };
use crate::db::{ AppState, DbActor };

#[derive(Deserialize)]
pub struct UserBody {
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub email: Option<String>
}

#[get("")]
pub async fn fetch_users(state: Data<AppState>) -> impl Responder {
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(FetchUsers).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json("No users found"),
        _ => HttpResponse::InternalServerError().json("Unable to retrieve users"),
    }
}

#[get("/{id}")]
pub async fn fetch_user(state: Data<AppState>, id: Path<i32>) -> impl Responder {
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(FetchUser { 
        id: id.into_inner() 
    }).await 
    {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json("User not found"),
        _ => HttpResponse::InternalServerError().json("Unable to retrieve user"),
    }
}


#[post("")]
pub async fn create_user(state: Data<AppState>, body: Json<UserBody>) -> impl Responder {
    let db: Addr<DbActor> = state.as_ref().db.clone();
    match db.send(CreateUser { 
        firstname: body.firstname.clone(),
        lastname: body.lastname.clone(),
        email: body.email.clone()
    }).await 
    {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        _ => HttpResponse::InternalServerError().json("Failed to create user"),
    }
}

#[patch("/{id}")]
pub async fn update_user(state: Data<AppState>, body: Json<UserBody>, id: Path<i32>) -> impl Responder {
    let db: Addr<DbActor> = state.as_ref().db.clone();
    match db.send(UpdateUser { 
        id: id.into_inner(),
        firstname: body.firstname.clone(),
        lastname: body.lastname.clone(),
        email: body.email.clone() 
    }).await 
    {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        _ => HttpResponse::InternalServerError().json("Failed to update user"),
    }
}

#[delete("/{id}")]
pub async fn delete_user(state: Data<AppState>, id: Path<i32>) -> impl Responder {
    let db: Addr<DbActor> = state.as_ref().db.clone();
    match db.send(DeleteUser { 
        id: id.into_inner() 
    }).await 
    {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        _ => HttpResponse::InternalServerError().json("Failed to delete user"),
    }
}