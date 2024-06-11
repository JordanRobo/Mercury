pub mod post_handlers;
pub mod author_handlers;

pub use post_handlers::{ get_all_posts, get_post_by_id, create_new_post, update_existing_post, delete_post_by_id };
pub use author_handlers::{ get_all_authors, get_author_by_id, create_new_author, update_existing_author, delete_author_by_id };