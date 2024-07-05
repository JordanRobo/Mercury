use actix_web::{error, delete, get, patch, post, web::{Data, Json, Path, block}, HttpResponse, Result, Responder};
use crate::handlers::{get_all_authors, get_author_by_id, create_new_author, update_existing_author, delete_author_by_id};
use crate::db::DbPool;
use crate::models::*;

#[get("/authors")]
pub async fn get_authors(pool: Data<DbPool>) -> Result<impl Responder> {
    let authors = block(move || {
        let mut conn = pool.get()?;
        get_all_authors(&mut conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(authors))
}

#[get("/authors/{author_id}")]
pub async fn get_author(pool: Data<DbPool>, id: Path<String>) -> Result<impl Responder> {
    let author_id = id.into_inner();
    let author = block(move || {
        let mut conn = pool.get()?;
        get_author_by_id(&mut conn, author_id)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    match author {
        Some(author) => Ok(HttpResponse::Ok().json(author)),
        None => Ok(HttpResponse::NotFound().finish()),
    }
}

#[post("/authors")]
pub async fn create_author(pool: Data<DbPool>, body: Json<Author>) -> Result<impl Responder> {
    let author = body.into_inner();
    let created_author = block(move || {
        let mut conn = pool.get()?;
        create_new_author(&mut conn, author)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(created_author))
}

#[patch("/authors/{author_id}")]
pub async fn update_author(pool: Data<DbPool>, id: Path<String>, body: Json<NewAuthor>) -> Result<impl Responder> {
    let author_id = id.into_inner();
    let author = body.into_inner();
    let updated_author = block(move || {
        let mut conn = pool.get()?;
        update_existing_author(&mut conn, author_id, author)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    match updated_author {
        Some(author) => Ok(HttpResponse::Ok().json(author)),
        None => Ok(HttpResponse::NotFound().finish()),
    }
}

#[delete("/authors/{author_id}")]
pub async fn delete_author(pool: Data<DbPool>, id: Path<String>) -> Result<impl Responder> {
    let author_id = id.into_inner();
    let result = block(move || {
        let mut conn = pool.get()?;
        delete_author_by_id(&mut conn, author_id)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    match result {
        Some(_) => Ok(HttpResponse::NoContent().finish()),
        None => Ok(HttpResponse::NotFound().finish()),
    }
}