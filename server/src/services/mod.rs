pub mod config;
pub mod post_api;

pub use config::post_config;
pub use post_api::{ get_posts, get_post, create_post, update_post, delete_post };