use crate::db::DbPool;
use crate::handlers::{
    create_new_post, delete_post_by_id, get_all_posts, get_post_by_id, get_posts_author,
    get_posts_author_tags, get_posts_tags, update_existing_post,
};
use crate::models::*;
use actix_web::{
    delete, error, get, patch, post,
    web::{block, Data, Json, Path, Query},
    HttpResponse, Responder, Result,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct PostQuery {
    inc: Option<String>,
}

#[get("/posts")]
pub async fn get_posts(pool: Data<DbPool>, query: Query<PostQuery>) -> Result<impl Responder> {
    let posts = block(move || {
        let mut conn = pool.get()?;
        match &query.inc {
            Some(inc) => {
                let includes: Vec<&str> = inc.split(',').collect();
                match (includes.contains(&"author"), includes.contains(&"tags")) {
                    (true, true) => get_posts_author_tags(&mut conn),
                    (true, false) => get_posts_author(&mut conn),
                    (false, true) => get_posts_tags(&mut conn),
                    (false, false) => get_all_posts(&mut conn),
                }
            }
            None => get_all_posts(&mut conn),
        }
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
        None => HttpResponse::NotFound().body("No Post found"),
    })
}

#[post("/posts")]
pub async fn create_post(pool: Data<DbPool>, body: Json<CreatePost>) -> Result<impl Responder> {
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
pub async fn update_post(
    pool: Data<DbPool>,
    post_id: Path<String>,
    body: Json<UpdatePost>,
) -> Result<impl Responder> {
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
