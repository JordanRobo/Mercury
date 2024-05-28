use actix_web::{
    get, post, delete, patch, web::{ Data, Json, Path }, HttpResponse, Responder
};
use serde::Deserialize;
use actix::Addr;
use crate::messages::{ FetchPosts, FetchPost, CreatePost, UpdatePost, DeletePost };
use crate::db::{ AppState, DbActor };

#[derive(Deserialize)]
pub struct PostBody {
    pub title: Option<String>,
    pub slug: Option<String>,
    pub content: Option<String>,
    pub feature_image: Option<String>,
    pub excerpt: Option<String>,
    pub published: Option<bool>,
    pub author_id: i32,
}

#[get("")]
pub async fn fetch_posts(state: Data<AppState>) -> impl Responder {
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(FetchPosts).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json("No posts found"),
        _ => HttpResponse::InternalServerError().json("Unable to retrieve posts"),
    }
}

#[get("/{id}")]
pub async fn fetch_post(state: Data<AppState>, id: Path<i32>) -> impl Responder {
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(FetchPost { 
        id: id.into_inner() 
    }).await 
    {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json("Post not found"),
        _ => HttpResponse::InternalServerError().json("Unable to retrieve post"),
    }
}

#[post("")]
pub async fn create_post(state: Data<AppState>, body: Json<PostBody>) -> impl Responder {
    let db: Addr<DbActor> = state.as_ref().db.clone();
    match db.send(CreatePost { 
        title: body.title.clone(),
        slug: body.slug.clone(),
        content: body.content.clone(),
        feature_image: body.feature_image.clone(),
        excerpt: body.excerpt.clone(),
        published: body.published,
        author_id: body.author_id,
    }).await 
    {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        _ => HttpResponse::InternalServerError().json("Failed to create post"),
    }
}

#[patch("/{id}")]
pub async fn update_post(state: Data<AppState>, body: Json<PostBody>, id: Path<i32>) -> impl Responder {
    let db: Addr<DbActor> = state.as_ref().db.clone();
    match db.send(UpdatePost { 
        id: id.into_inner(),
        title: body.title.clone(),
        slug: body.slug.clone(),
        content: body.content.clone(),
        feature_image: body.feature_image.clone(),
        excerpt: body.excerpt.clone(),
        published: body.published,
        author_id: body.author_id,
    }).await 
    {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        _ => HttpResponse::InternalServerError().json("Failed to update post"),
    }
}

#[delete("/{id}")]
pub async fn delete_post(state: Data<AppState>, id: Path<i32>) -> impl Responder {
    let db: Addr<DbActor> = state.as_ref().db.clone();
    match db.send(DeletePost { 
        id: id.into_inner() 
    }).await 
    {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        _ => HttpResponse::InternalServerError().json("Failed to delete post"),
    }
}