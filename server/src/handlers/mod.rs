pub mod post_handlers;

pub use post_handlers::{
    create_new_post, delete_post_by_id, get_all_posts, get_post_by_id, get_posts_author,
    get_posts_author_tags, get_posts_tags, update_existing_post,
};
