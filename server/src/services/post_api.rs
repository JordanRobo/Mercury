use actix_web::{ delete, get, patch, post, web::{ Data, Path, Json }, HttpResponse};
use crate::handlers::{ get_all_posts, get_post_by_id, create_new_post, update_existing_post, delete_post_by_id };
use crate::db::DbPool;
use crate::models::*;

#[get("/posts")]
pub async fn get_posts(pool: Data<DbPool>) -> HttpResponse {
    let posts = get_all_posts(&pool).await;
    HttpResponse::Ok().json(posts)
}

#[get("/posts/{id}")]
pub async fn get_post(pool: Data<DbPool>, id: Path<String>) -> HttpResponse {
    let post = get_post_by_id(&pool, id.into_inner()).await;
    match post {
        Some(post) => HttpResponse::Ok().json(post),
        None => HttpResponse::NotFound().body("Post not found"),
    }
}

#[post("/posts")]
pub async fn create_post(pool: Data<DbPool>, body: Json<Post>) -> HttpResponse {
    let post = create_new_post(&pool, body.into_inner()).await;
    match post {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(_) => HttpResponse::InternalServerError().body("Error trying to create post"),
    }
}

#[patch("/posts/{id}")]
pub async fn update_post(pool: Data<DbPool>, id: Path<String>, body: Json<NewPost>) -> HttpResponse {
    let post = update_existing_post(&pool, id.into_inner(), body.into_inner()).await;
    match post {
        Some(post) => HttpResponse::Ok().json(post),
        None => HttpResponse::NotFound().body("Post not found"),
    }
}

#[delete("/posts/{id}")]
pub async fn delete_post(pool: Data<DbPool>, id: Path<String>) -> HttpResponse {
    let post = delete_post_by_id(&pool, id.into_inner()).await;
    match post {
        Some(_) => HttpResponse::Ok().body("Post deleted"),
        None => HttpResponse::NotFound().body("Post not found"),
    }
}