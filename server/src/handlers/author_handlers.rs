use crate::db::DbPool;
use crate::models::*;  
use actix_web::Error;
use diesel::prelude::*;
use crate::db::authors::dsl::*;

pub async fn get_all_authors(pool: &DbPool) -> Vec<Author> {
    let mut conn = pool.get().unwrap();
    authors.load::<Author>(&mut conn).expect("Error loading authors")
}

pub async fn get_author_by_id(pool: &DbPool, author_id: String) -> Option<Author> {
    let mut conn = pool.get().unwrap();
    let query = authors
        .find(author_id)
        .get_result::<Author>(&mut conn)
        .expect("Error loading author");
    Some(query)
}

pub async fn create_new_author(pool: &DbPool, author: Author) -> Result<Author, Error> {
    let mut conn = pool.get().unwrap();

    let author = Author {
        id: xid::new().to_string(),
        ..author
    };

    let query = diesel::insert_into(authors)
        .values(&author)
        .get_result::<Author>(&mut conn)
        .expect("Error saving new author");
    Ok(query)
}

pub async fn update_existing_author(pool: &DbPool, author_id: String, author: NewAuthor) -> Option<Author> {
    let mut conn = pool.get().unwrap();

    let updated_author = diesel::update(authors.find(author_id))
        .set(&author)
        .get_result::<Author>(&mut conn)
        .expect("Error updating author");

    Some(updated_author)
}

pub async fn delete_author_by_id(pool: &DbPool, author_id: String) -> Option<usize> {
    let mut conn = pool.get().unwrap();
    let query = diesel::delete(authors.find(author_id))
        .execute(&mut conn)
        .expect("Error deleting post");
    Some(query)
}