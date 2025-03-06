use std::fs::OpenOptions;
use std::io::Error;
use std::io::Write;

use actix_cors::Cors;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer, Result};
use actix_web_httpauth::middleware::HttpAuthentication;
use auth::handlers::check_auth;

mod core;
mod modules;
pub mod utils;

pub use core::*;
pub use modules::*;

pub fn setup() -> std::io::Result<()> {
    let jwt_secret = utils::generate_secret(64);
    let site_id = utils::generate_secret(32);

    let default_salt = utils::generate_salt();
    let default_pass = utils::generate_secret(32);
    let default_hash = auth::handlers::create_password(&default_pass, &default_salt);

    let env_contents = format!("JWT_SECRET={}\nSITE_ID={}\nDEFAULT_PASSWORD={}\nDATABASE_URL=mercury.db", jwt_secret, site_id, default_hash);

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(".env")?;

    file.write_all(env_contents.as_bytes())?;

    println!("Setup complete. Environment file (.env) has been created with secure random values.");
    Ok(())
}

pub fn run() -> Result<Server, Error> {
    let db = db::establish_connection();
    let app_data = web::Data::new(db);

    let server = HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(check_auth);

        App::new()
            .app_data(app_data.clone())
            .wrap(Cors::permissive()) // Permissive for development, change later
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
