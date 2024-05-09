use actix_web::{
    get,
    web::Data,
    Responder, HttpResponse,
};
use actix::Addr;
use crate::{
    messages::FetchUser,
    AppState, DbActor};


#[get("/users")]
pub async fn fetch_users(state: Data<AppState>) -> impl Responder {
    // "GET /users".to_string()
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(FetchUser).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json("No users found"),
        _ => HttpResponse::InternalServerError().json("Unable to retrieve users"),
    }
}