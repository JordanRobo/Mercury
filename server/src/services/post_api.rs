use actix_web::{ error, delete, get, patch, post, web::{ Data, Json, Path, Query, block }, HttpResponse, Responder, Result };
use crate::handlers::{ create_new_post, delete_post_by_id, get_all_posts, get_post_by_id, update_existing_post };
use crate::db::DbPool;
use crate::models::*;

#[derive(serde::Deserialize)]
struct QueryParams {
    author: Option<bool>,
}

#[get("/posts")]
pub async fn get_posts(pool: Data<DbPool>, query: Query<QueryParams>) -> Result<impl Responder> {
    let include_author = query.author.unwrap_or(false);

    let posts = block(move || {
        let mut conn = pool.get()?;
        get_all_posts(&mut conn, include_author)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(posts))
}

#[get("/posts/{post_id}")]
pub async fn get_post(pool: Data<DbPool>, post_id: Path<String>) -> Result<impl Responder> {
    let post_id = post_id.into_inner();

    let post = block(move || {
        let mut conn = pool.get()?;
        get_post_by_id(&mut conn, post_id)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(match post {
        Some(post) => HttpResponse::Ok().json(post),
        None => HttpResponse::NotFound().body("No Post found")
    })

}

#[post("/posts")]
pub async fn create_post(pool: Data<DbPool>, body: Json<NewPost>) -> Result<impl Responder> {
    let new_post = body.into_inner();

    let post = block(move || {
        let mut conn = pool.get()?;
        create_new_post(&mut conn, new_post)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(post))
}

#[patch("/posts/{post_id}")]
pub async fn update_post(pool: Data<DbPool>, post_id: Path<String>, body: Json<NewPost>) -> Result<impl Responder> {

    let post = block(move || {
        let mut conn = pool.get()?;
        update_existing_post(&mut conn, post_id.into_inner(), body.into_inner())
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(match post {
        Some(post) => HttpResponse::Ok().json(post),
        None => HttpResponse::NotFound().body("Post not found"),
    })
}

#[delete("/posts/{post_id}")]
pub async fn delete_post(pool: Data<DbPool>, post_id: Path<String>) -> Result<impl Responder> {
    let post_id = post_id.into_inner();

    let post = block(move || {
        let mut conn = pool.get()?;
        delete_post_by_id(&mut conn, post_id)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(match post {
        Some(_) => HttpResponse::Ok().body("Post deleted"),
        None => HttpResponse::NotFound().body("Post not found"),
    })
    
}