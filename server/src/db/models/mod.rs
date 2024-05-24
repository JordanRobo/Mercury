pub mod authors;
pub mod posts;
pub mod insertables;

pub use authors::Author;
pub use posts::Post;
pub use insertables::{ NewAuthor, NewPost };