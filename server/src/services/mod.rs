pub mod config;
pub mod post_api;
pub mod author_api;

pub use config::admin_config;
pub use post_api::{ get_posts, get_post, create_post, update_post, delete_post };
pub use author_api::{ get_authors, get_author, create_author, update_author, delete_author };