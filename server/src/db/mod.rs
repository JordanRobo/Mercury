pub mod schema;
pub mod connection;

pub use connection::{ establish_connection, DbPool };
pub use schema::posts;