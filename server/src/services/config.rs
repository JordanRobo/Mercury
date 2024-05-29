use actix_web::web;
use crate::services::*;

pub fn post_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(get_posts)
            .service(get_post)
    );
}