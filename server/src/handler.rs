
pub fn users(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/users")
            .route()
    )
}

[#get("/")]