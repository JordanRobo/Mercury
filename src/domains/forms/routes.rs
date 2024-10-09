use actix_web::{
    post,
    web::{Data, Path},
    HttpResponse, Responder, Result,
};

use crate::db::DbPool;

#[post("/forms/{form_name}")]
async fn create_form(pool: Data<DbPool>, form_name: Path<String>) -> Result<impl Responder> {
    Ok(HttpResponse::ImATeapot())
}
