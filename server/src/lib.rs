use std::io::Error;

use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer, Result};
use env_logger::Env;
use serde::Serialize;
use services::*;

pub mod db;
pub mod handlers;
pub mod models;
pub mod services;
pub mod utils;

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

pub fn run() -> Result<Server, Error> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let db = db::establish_connection();
    let app_data = web::Data::new(db);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(app_data.clone())
            .route("/", web::get().to(index))
            .default_service(web::route().to(not_found))
            .configure(admin_config)
    })
    .bind(("127.0.0.1", 2323))?
    .run();

    Ok(server)
}
