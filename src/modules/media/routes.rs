use actix_multipart::Multipart;
use actix_web::{get, post, web::Data, HttpResponse, Responder, Result};

use crate::db::DbPool;

#[get("/media")]
pub async fn get_all_media() -> Result<impl Responder> {
    Ok(HttpResponse::Ok())
}

#[post("/media")]
pub async fn add_new_media(mut _payload: Multipart, _pool: Data<DbPool>) -> Result<impl Responder> {
    Ok(HttpResponse::Ok())
}
