use crate::{
    posts::{create_post, delete_post, get_post, get_post_slug, get_posts, update_post},
    users::{get_user, get_users},
};
use actix_web::web;

pub fn admin_api(cfg: &mut web::ServiceConfig) {
    cfg.configure(posts_admin_routes)
        .configure(users_admin_routes);
}

pub fn content_api(cfg: &mut web::ServiceConfig) {
    cfg.configure(posts_public_routes);
}

fn posts_public_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_posts)
        .service(get_post)
        .service(get_post_slug);
}

fn posts_admin_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_posts)
        .service(get_post)
        .service(get_post_slug)
        .service(create_post)
        .service(update_post)
        .service(delete_post);
}

fn users_admin_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_user).service(get_users);
}
