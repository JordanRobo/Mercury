use actix_web::{ delete, get, patch, post, web::{ Data, Path, Json }, HttpResponse};
use crate::handlers::{ get_all_authors, get_author_by_id, create_new_author, update_existing_author, delete_author_by_id };
use crate::db::DbPool;
use crate::models::*;

#[get("/authors")]
pub async fn get_authors(pool: Data<DbPool>) -> HttpResponse {
    let authors = get_all_authors(&pool).await;
    HttpResponse::Ok().json(authors)
}

#[get("/authors/{id}")]
pub async fn get_author(pool: Data<DbPool>, id: Path<String>) -> HttpResponse {
    let author = get_author_by_id(&pool, id.into_inner()).await;
    match author {
        Some(author) => HttpResponse::Ok().json(author),
        None => HttpResponse::NotFound().body("Author not found"),
    }
}

#[post("/authors")]
pub async fn create_author(pool: Data<DbPool>, body: Json<Author>) -> HttpResponse {
    let author = create_new_author(&pool, body.into_inner()).await;
    match author {
        Ok(author) => HttpResponse::Ok().json(author),
        Err(_) => HttpResponse::InternalServerError().body("Error trying to create author"),
    }
}

#[patch("/authors/{id}")]
pub async fn update_author(pool: Data<DbPool>, id: Path<String>, body: Json<NewAuthor>) -> HttpResponse {
    let author = update_existing_author(&pool, id.into_inner(), body.into_inner()).await;
    match author {
        Some(author) => HttpResponse::Ok().json(author),
        None => HttpResponse::NotFound().body("Author not found"),
    }
}

#[delete("/authors/{id}")]
pub async fn delete_author(pool: Data<DbPool>, id: Path<String>) -> HttpResponse {
    let author = delete_author_by_id(&pool, id.into_inner()).await;
    match author {
        Some(_) => HttpResponse::Ok().body("Author deleted"),
        None => HttpResponse::NotFound().body("Author not found"),
    }
}