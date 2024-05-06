use actix_web::middleware::Logger;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;

#[derive(Serialize)]
pub struct TestResponse {
    status: String,
    message: String,
}

#[get("/api/running")]
async fn test_api() -> impl Responder {
    const MESSAGE: &str = "🚀 Server is running";

    let response_json =&TestResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };
    HttpResponse::Ok().json(response_json)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    println!("🚀 Server started successfully");

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(test_api)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}