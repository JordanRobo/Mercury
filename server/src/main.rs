use actix_web::middleware::Logger;
use actix_web::{get, App, HttpServer, Responder};



#[get("/")]
async fn test_api() -> impl Responder {
    let ascii_art = r#"
     __  __                                   _____ __  __  _____ 
    |  \/  |                                 / ____|  \/  |/ ____|
    | \  / | ___ _ __ ___ _   _ _ __ _   _  | |    | \  / | (___  
    | |\/| |/ _ \ '__/ __| | | | '__| | | | | |    | |\/| |\___ \ 
    | |  | |  __/ | | (__| |_| | |  | |_| | | |____| |  | |____) |
    |_|  |_|\___|_|  \___|\__,_|_|   \__, |  \_____|_|  |_|_____/ 
                                      __/ |                       
                                     |___/                        
"#;
    ascii_art.to_owned()
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