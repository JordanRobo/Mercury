use std::io::Error;

use actix_cors::Cors;
use actix_web::dev::{Server, Service};
use actix_web::{web, App, HttpResponse, HttpServer, Result};
use actix_web_httpauth::middleware::HttpAuthentication;
use auth::handlers::check_auth;

mod core;
mod domains;
pub mod setup;
pub mod utils;

pub use core::*;
pub use domains::*;

pub fn run() -> Result<Server, Error> {
    let db = db::establish_connection();
    let app_data = web::Data::new(db);

    let server = HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(check_auth);
        let db_clone = app_data.clone();

        App::new()
            .app_data(app_data.clone())
            .wrap(Cors::permissive()) // Permissive for development, change later
            .service(web::resource("/login").route(web::post().to(auth::handlers::login)))
            .service(
                web::scope("/check")
                    .wrap_fn(move |req, srv| {
                        let mut conn = db_clone
                            .get_ref()
                            .get()
                            .expect("Couldn't get db connection");
                        let is_admin_route_enabled = setup::check_admin(&mut conn);

                        if is_admin_route_enabled {
                            srv.call(req)
                        } else {
                            Box::pin(async move { Ok(req.into_response(HttpResponse::Forbidden().body("Admin setup route is not available"))) })
                        }
                    })
                    .route("", web::post().to(setup::create_admin))
                    .route("", web::get().to(HttpResponse::Ok)),
            )
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
