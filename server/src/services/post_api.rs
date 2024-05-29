use actix_web::{ web::{ Data, Path }, get, HttpResponse};
use crate::handlers::{ get_all_posts, get_post_by_id };
use crate::db::DbPool;

#[get("/posts")]
pub async fn get_posts(pool: Data<DbPool>) -> HttpResponse {
    let posts = get_all_posts(&pool).await;
    HttpResponse::Ok().json(posts)
}

#[get("/posts/{id}")]
pub async fn get_post(pool: Data<DbPool>, id: Path<i32>) -> HttpResponse {
    let post = get_post_by_id(&pool, id.into_inner()).await;
    match post {
        Some(post) => HttpResponse::Ok().json(post),
        None => HttpResponse::NotFound().body("Post not found"),
    }
}