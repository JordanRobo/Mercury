pub mod author_messages;
pub mod post_messages;

pub use author_messages::{ FetchAuthors, FetchAuthor, CreateAuthor, UpdateAuthor, DeleteAuthor };
pub use post_messages::{ FetchPosts, FetchPost, CreatePost, UpdatePost, DeletePost };