pub mod author_endpoints;
pub mod post_endpoints;
pub mod config;

pub use post_endpoints::{ create_post, fetch_post, fetch_posts, update_post, delete_post};
pub use author_endpoints::{ create_author, fetch_author, fetch_authors, update_author, delete_author};
pub use config::{ author_config, post_config };