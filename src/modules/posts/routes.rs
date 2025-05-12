use crate::db::DbPool;
use crate::posts::{CreatePost, Post, UpdatePost};
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
                    (true, true) => handlers::get_posts_author_tags(&mut conn),
                    (true, false) => handlers::get_posts_author(&mut conn),
                    (false, true) => handlers::get_posts_tags(&mut conn),
                    (false, false) => handlers::get_all_posts(&mut conn),
                }
            }
            None => Post::fetch_all(&mut conn),
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
        Post::fetch_by_id(&mut conn, post_id)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(match post {
        Some(post) => HttpResponse::Ok().json(post),
        None => HttpResponse::NotFound().body("No Post found"),
    })
}

#[get("/posts/slug/{post_slug}")]
pub async fn get_post_slug(pool: Data<DbPool>, post_slug: Path<String>) -> Result<impl Responder> {
    let post_slug = post_slug.into_inner();

    let post = block(move || {
        let mut conn = pool.get()?;
        Post::fetch_by_slug(&mut conn, post_slug)
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
        Post::create(&mut conn, new_post)
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
        Post::update(&mut conn, post_id.into_inner(), body.into_inner())
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
        Post::delete(&mut conn, post_id)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(match post {
        Some(_) => HttpResponse::Ok().body("Post deleted"),
        None => HttpResponse::NotFound().body("Post not found"),
    })
}
