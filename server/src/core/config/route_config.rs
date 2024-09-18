use crate::posts::routes::{create_post, delete_post, get_post, get_posts, update_post};
use actix_web::web;

pub fn admin_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(get_posts)
            .service(get_post)
            .service(create_post)
            .service(update_post)
            .service(delete_post),
    );
}
