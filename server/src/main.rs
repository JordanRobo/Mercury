mod db;
mod models;
mod handlers;
mod services;

use actix_web::{ App, HttpServer, HttpResponse, web, Result };
use serde::Serialize;
use services::post_config;

#[derive(Serialize)]
pub struct Response {
    pub message: String,
}

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Welcome to Mercury CMS API Server 🚀")
}

async fn not_found() -> Result<HttpResponse> {
    let response = Response {
        message: "Resource not found".to_string(),
    };
    Ok(HttpResponse::NotFound().json(response))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = db::establish_connection();
    let app_data = web::Data::new(db);

    HttpServer::new(move || 
        App::new()
            .app_data(app_data.clone())
            .route("/", web::get().to(index))
            .default_service(web::route().to(not_found))
            .configure(post_config)
    )
    .bind(("127.0.0.1", 2323))?
    .run()
    .await
}