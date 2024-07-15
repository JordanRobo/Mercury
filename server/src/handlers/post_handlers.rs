use crate::models::*;
use diesel::prelude::*;
use crate::utils::{slug_gen, get_current_timestamp};

pub fn get_all_posts(conn: &mut SqliteConnection) -> Result<Vec<Post>, DbError> {
    use crate::db::posts::dsl::*;
    use diesel::prelude::*;

    Ok(posts.load(conn)?)
}

pub fn get_post_by_id(conn: &mut SqliteConnection, post_id: String) -> Result<Option<Post>, DbError> {
    use crate::db::posts::dsl::*;

    let post = posts
        .filter(id.eq(post_id.to_string()))
        .first::<Post>(conn)
        .optional()?;

    Ok(post)
}

pub fn create_new_post(conn: &mut SqliteConnection, new_post: CreatePost) -> Result<Post, DbError> {
    use crate::db::posts::dsl::*;

    let current_time = get_current_timestamp();

    let post = Post {
        id: xid::new().to_string(),
        title: new_post.title.clone(),
        slug: slug_gen(&new_post.title),
        excerpt: new_post.excerpt,
        content: new_post.content,
        author_id: new_post.author_id,
        feature_image: new_post.feature_image,
        status: new_post.status,
        published_at: None,
        created_at: Some(current_time),
        updated_at: Some(current_time),
    };

    diesel::insert_into(posts)
        .values(&post)
        .execute(conn)?;

    Ok(post)
}

pub fn update_existing_post(conn: &mut SqliteConnection, post_id: String, post: UpdatePost) -> Result<Option<Post>, DbError> {
    use crate::db::posts::dsl::*;

    diesel::update(posts.find(post_id.clone()))
        .set(&post)
        .execute(conn)?;

    let updated_post = posts.find(post_id)
        .first(conn)
        .optional()?;

    Ok(updated_post)
}

pub fn delete_post_by_id(conn: &mut SqliteConnection, post_id: String) -> Result<Option<usize>, DbError> {
    use crate::db::posts::dsl::*;

    let query = diesel::delete(posts.find(post_id))
        .execute(conn)?;

    if query > 0 {
        Ok(Some(query))
    } else {
        Ok(None)
    }
}