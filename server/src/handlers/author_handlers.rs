use crate::models::*;  
use actix_web::Result;
use diesel::prelude::*;

// Needs Update
pub fn get_all_authors(_conn: &mut SqliteConnection) -> Result<Vec<Author>, DbError> {
    Ok(Vec::new())
}

// Needs Update
pub fn get_author_by_id(_conn: &mut SqliteConnection, _author_id: String) -> Result<Option<Author>, DbError> {
    Ok(None)
}

// Needs Update
pub fn create_new_author(_conn: &mut SqliteConnection, author: Author) -> Result<Author, DbError> {
    Ok(author)
}  

// Needs Update
pub fn update_existing_author(_conn: &mut SqliteConnection, author_id: String, author: NewAuthor) -> Result<Option<Author>, DbError> {
    Ok(Some(Author {
        id: author_id,
        name: author.name,
        email: author.email,
        bio: author.bio,
        profile_picture: author.profile_picture,
    }))
}

// Needs Update
pub fn delete_author_by_id(_conn: &mut SqliteConnection, _author_id: String) -> Result<Option<usize>, DbError> {
    Ok(Some(1))
}