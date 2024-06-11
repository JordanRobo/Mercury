use actix_web::web;
use crate::services::*;

pub fn admin_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(get_posts)
            .service(get_post)
            .service(create_post)
            .service(update_post)
            .service(delete_post)
            .service(get_authors)
            .service(get_author)
            .service(create_author)
            .service(update_author)
            .service(delete_author)
    );
}
