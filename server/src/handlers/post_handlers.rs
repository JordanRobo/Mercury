use crate::models::*;
use diesel::prelude::*;

pub fn get_all_posts(conn: &mut SqliteConnection, include_author: bool) -> Result<Vec<PostWithAuthor>, DbError> {
    use crate::db::schema::{posts, authors};
    use diesel::prelude::*;

    let query = posts::table
        .left_join(authors::table)
        .select((posts::all_columns, authors::all_columns.nullable()));

    let results = query.load::<(Post, Option<Author>)>(conn)?;

    Ok(results
        .into_iter()
        .map(|(post, author)| PostWithAuthor {
            post,
            author: if include_author { author } else { None },
        })
        .collect())
}

pub fn get_post_by_id(conn: &mut SqliteConnection, post_id: String) -> Result<Option<Post>, DbError> {
    use crate::db::posts::dsl::*;

    let post = posts
        .filter(id.eq(post_id.to_string()))
        .first::<Post>(conn)
        .optional()?;

    Ok(post)
}

pub fn create_new_post(conn: &mut SqliteConnection, new_post: NewPost) -> Result<Post, DbError> {
    use crate::db::posts::dsl::*;

    let post = Post {
        id: xid::new().to_string(),
        title: new_post.title,
        slug: new_post.slug,
        content: new_post.content,
        feature_image: new_post.feature_image,
        excerpt: new_post.excerpt,
        published: new_post.published,
        author_id: new_post.author_id,
    };

    diesel::insert_into(posts)
        .values(&post)
        .execute(conn)?;

    Ok(post)
}

pub fn update_existing_post(conn: &mut SqliteConnection, post_id: String, post: NewPost) -> Result<Option<Post>, DbError> {
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