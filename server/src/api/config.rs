use actix_web::web;
use crate::api::{ create_author, fetch_author, fetch_authors, update_author, delete_author };
use crate::api::{ create_post, fetch_post, fetch_posts, update_post, delete_post };

pub fn author_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/authors")
            .service(fetch_author)
            .service(fetch_authors)
            .service(create_author)
            .service(update_author)
            .service(delete_author),
    );
}

pub fn post_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/posts")
            .service(fetch_post)
            .service(fetch_posts)
            .service(create_post)
            .service(update_post)
            .service(delete_post),
    );
}