use crate::db::DbPool;
use crate::models::*;
use actix_web::Error;
use diesel::prelude::*;
use crate::db::posts::dsl::*;
use crate::db::authors::dsl::*;


pub async fn get_all_posts(pool: &DbPool, include_author: bool) -> Vec<PostWithAuthor> {
    let mut conn = pool.get().unwrap();

    if include_author {
        posts
            .left_join(authors)
            .load::<(Post, Option<Author>)>(&mut conn)
            .expect("Error loading posts")
            .into_iter()
            .map(|(post, author)| PostWithAuthor { post, author })
            .collect()
    } else {
        posts
            .load::<Post>(&mut conn)
            .expect("Error loading posts")
            .into_iter()
            .map(|post| PostWithAuthor { post, author: None })
            .collect()
    }
}


pub async fn get_post_by_id(pool: &DbPool, post_id: String) -> Option<Post> {
    let mut conn = pool.get().unwrap();
    let query = posts
        .find(post_id)
        .get_result::<Post>(&mut conn)
        .expect("Error loading post");
    Some(query)
}

pub async fn create_new_post(pool: &DbPool, post: Post) -> Result<Post, Error> {
    let mut conn = pool.get().unwrap();

    let post = Post {
        id: xid::new().to_string(),
        ..post
    };

    let query = diesel::insert_into(posts)
        .values(&post)
        .get_result::<Post>(&mut conn)
        .expect("Error saving new post");
    Ok(query)
}

pub async fn update_existing_post(pool: &DbPool, post_id: String, post: NewPost) -> Option<Post> {
    let mut conn = pool.get().unwrap();

    let updated_post = diesel::update(posts.find(post_id))
        .set(&post)
        .get_result::<Post>(&mut conn)
        .expect("Error updating post");

    Some(updated_post)
}

pub async fn delete_post_by_id(pool: &DbPool, post_id: String) -> Option<usize> {
    let mut conn = pool.get().unwrap();
    let query = diesel::delete(posts.find(post_id))
        .execute(&mut conn)
        .expect("Error deleting post");
    Some(query)
}