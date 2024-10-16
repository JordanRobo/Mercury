use std::io::Error;

use actix_web::dev::Server;
use actix_web::{web, App, HttpServer, Result};
use actix_web_httpauth::middleware::HttpAuthentication;
use auth::handlers::check_auth;

mod core;
mod domains;
pub mod utils;

pub use core::*;
pub use domains::*;

pub fn run() -> Result<Server, Error> {
    let db = db::establish_connection();
    let app_data = web::Data::new(db);

    let server = HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(check_auth);

        App::new()
            .app_data(app_data.clone())
            .service(web::resource("/login").route(web::post().to(auth::handlers::login)))
            .service(
                web::scope("/admin")
                    .wrap(auth)
                    .configure(config::admin_api),
            )
            .service(web::scope("/content").configure(config::content_api))
    })
    .bind(("127.0.0.1", 2030))?
    .run();

    Ok(server)
}
