use actix_web::{
    get, post, delete, patch, web::{ Data, Json, Path }, HttpResponse, Responder
};
use serde::Deserialize;
use actix::Addr;
use crate::messages::{ FetchAuthor, FetchAuthors, CreateAuthor, UpdateAuthor, DeleteAuthor };
use crate::db::{ AppState, DbActor };

#[derive(Deserialize)]
pub struct AuthorBody {
    pub name: Option<String>,
    pub email: Option<String>,
    pub bio: Option<String>,
    pub profile_picture: Option<String>,
}

#[get("")]
pub async fn fetch_authors(state: Data<AppState>) -> impl Responder {
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(FetchAuthors).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json("No authors found"),
        _ => HttpResponse::InternalServerError().json("Unable to retrieve authors"),
    }
}

#[get("/{id}")]
pub async fn fetch_author(state: Data<AppState>, id: Path<i32>) -> impl Responder {
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(FetchAuthor { 
        id: id.into_inner() 
    }).await 
    {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json("Author not found"),
        _ => HttpResponse::InternalServerError().json("Unable to retrieve author"),
    }
}


#[post("")]
pub async fn create_author(state: Data<AppState>, body: Json<AuthorBody>) -> impl Responder {
    let db: Addr<DbActor> = state.as_ref().db.clone();
    match db.send(CreateAuthor { 
        name: body.name.clone(),
        email: body.email.clone(),
        bio: body.bio.clone(),
        profile_picture: body.profile_picture.clone()
    }).await 
    {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        _ => HttpResponse::InternalServerError().json("Failed to create author"),
    }
}

#[patch("/{id}")]
pub async fn update_author(state: Data<AppState>, body: Json<AuthorBody>, id: Path<i32>) -> impl Responder {
    let db: Addr<DbActor> = state.as_ref().db.clone();
    match db.send(UpdateAuthor { 
        id: id.into_inner(),
        name: body.name.clone(),
        email: body.email.clone(),
        bio: body.bio.clone(), 
        profile_picture: body.profile_picture.clone()
    }).await 
    {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        _ => HttpResponse::InternalServerError().json("Failed to update author"),
    }
}

#[delete("/{id}")]
pub async fn delete_author(state: Data<AppState>, id: Path<i32>) -> impl Responder {
    let db: Addr<DbActor> = state.as_ref().db.clone();
    match db.send(DeleteAuthor { 
        id: id.into_inner() 
    }).await 
    {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        _ => HttpResponse::InternalServerError().json("Failed to delete author"),
    }
}