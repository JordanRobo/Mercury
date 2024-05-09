use actix::SyncArbiter;
use actix_web::{web::Data, App, HttpServer, HttpResponse, web};
use dotenv::dotenv;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection
};
use std::env;

mod services;
mod db_utils;
mod messages;
mod actors;
mod db_models;
mod schema;
mod insertables;

use db_utils::{get_pool, AppState, DbActor};
use services::fetch_users;

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Welcome to Mercury CMS API Server")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool: Pool<ConnectionManager<PgConnection>> = get_pool(&db_url);
    let db_addr = SyncArbiter::start(5, move || DbActor(pool.clone()));

    println!("🚀 Server started successfully");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState { db: db_addr.clone() }))
            .route("/", web::get().to(index))
            .service(
                web::scope("/api")
                .service(fetch_users)
            
            )
    })
    .bind(("127.0.0.1", 2323))?
    .run()
    .await
}