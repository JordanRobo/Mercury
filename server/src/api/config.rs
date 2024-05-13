use actix_web::web;
use crate::api::{ create_user, fetch_user, fetch_users, update_user, delete_user };

pub fn user_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(fetch_user)
            .service(fetch_users)
            .service(create_user)
            .service(update_user)
            .service(delete_user),
    );
}