use crate::db::DbPool;
use crate::models::Post;
use diesel::prelude::*;
use crate::db::posts::dsl::*;

pub async fn get_all_posts(pool: &DbPool) -> Vec<Post> {
    let mut conn = pool.get().unwrap();
    posts.load::<Post>(&mut conn).expect("Error loading posts")
}

pub async fn get_post_by_id(pool: &DbPool, post_id: i32) -> Option<Post> {
    let mut conn = pool.get().unwrap();
    let post = posts
        .find(post_id)
        .get_result::<Post>(&mut conn)
        .expect("Error loading post");
    Some(post)
}