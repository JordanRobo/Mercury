use crate::authors::models::Author;
use crate::db::{
    schema::{authors, post_tags, posts, tags},
    DbError,
};
use crate::posts::models::{CreatePost, Post, PostInclude, UpdatePost};
use crate::tags::models::{PostTag, Tag};
use crate::utils::{get_current_timestamp, slug_gen};
use diesel::prelude::*;

pub fn get_all_posts(conn: &mut SqliteConnection) -> Result<Vec<PostInclude>, DbError> {
    posts::table
        .load::<Post>(conn)
        .map(|posts| {
            posts
                .into_iter()
                .map(|post| PostInclude {
                    post,
                    author: None,
                    tags: None,
                })
                .collect()
        })
        .map_err(|e| -> DbError { Box::new(e) })
}

pub fn get_posts_author(conn: &mut SqliteConnection) -> Result<Vec<PostInclude>, DbError> {
    posts::table
        .left_join(authors::table)
        .load::<(Post, Option<Author>)>(conn)
        .map(|results| {
            results
                .into_iter()
                .map(|(post, author)| PostInclude {
                    post,
                    author,
                    tags: None,
                })
                .collect()
        })
        .map_err(|e| -> DbError { Box::new(e) })
}

pub fn get_posts_tags(conn: &mut SqliteConnection) -> Result<Vec<PostInclude>, DbError> {
    let posts_with_tags = posts::table
        .left_join(post_tags::table.inner_join(tags::table))
        .load::<(Post, Option<(PostTag, Tag)>)>(conn)?;

    let mut post_includes: Vec<PostInclude> = Vec::new();
    let mut current_post: Option<Post> = None;
    let mut current_tags: Vec<Tag> = Vec::new();

    for (post, tag_opt) in posts_with_tags {
        if current_post
            .as_ref()
            .map(|p| p.id != post.id)
            .unwrap_or(true)
        {
            if let Some(prev_post) = current_post.take() {
                post_includes.push(PostInclude {
                    post: prev_post,
                    author: None,
                    tags: Some(std::mem::take(&mut current_tags)),
                });
            }
            current_post = Some(post);
        }
        if let Some((_, tag)) = tag_opt {
            current_tags.push(tag);
        }
    }

    if let Some(last_post) = current_post.take() {
        post_includes.push(PostInclude {
            post: last_post,
            author: None,
            tags: Some(current_tags),
        });
    }

    Ok(post_includes)
}

pub fn get_posts_author_tags(conn: &mut SqliteConnection) -> Result<Vec<PostInclude>, DbError> {
    let posts_with_author_and_tags = posts::table
        .left_join(authors::table)
        .left_join(post_tags::table.inner_join(tags::table))
        .load::<(Post, Option<Author>, Option<(PostTag, Tag)>)>(conn)?;

    let mut post_includes: Vec<PostInclude> = Vec::new();
    let mut current_post: Option<Post> = None;
    let mut current_author: Option<Author> = None;
    let mut current_tags: Vec<Tag> = Vec::new();

    for (post, author, tag_opt) in posts_with_author_and_tags {
        if current_post
            .as_ref()
            .map(|p| p.id != post.id)
            .unwrap_or(true)
        {
            if let Some(prev_post) = current_post.take() {
                post_includes.push(PostInclude {
                    post: prev_post,
                    author: current_author.take(),
                    tags: Some(std::mem::take(&mut current_tags)),
                });
            }
            current_post = Some(post);
            current_author = author;
        }
        if let Some((_, tag)) = tag_opt {
            current_tags.push(tag);
        }
    }

    if let Some(last_post) = current_post.take() {
        post_includes.push(PostInclude {
            post: last_post,
            author: current_author,
            tags: Some(current_tags),
        });
    }

    Ok(post_includes)
}

pub fn get_post_by_id(
    conn: &mut SqliteConnection,
    post_id: String,
) -> Result<Option<Post>, DbError> {
    use crate::db::schema::posts::dsl::*;

    let post = posts
        .filter(id.eq(post_id.to_string()))
        .first::<Post>(conn)
        .optional()?;

    Ok(post)
}

pub fn create_new_post(conn: &mut SqliteConnection, new_post: CreatePost) -> Result<Post, DbError> {
    use crate::db::schema::posts::dsl::*;

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

    diesel::insert_into(posts).values(&post).execute(conn)?;

    Ok(post)
}

pub fn update_existing_post(
    conn: &mut SqliteConnection,
    post_id: String,
    post: UpdatePost,
) -> Result<Option<Post>, DbError> {
    use crate::db::schema::posts::dsl::*;

    diesel::update(posts.find(post_id.clone()))
        .set(&post)
        .execute(conn)?;

    let updated_post = posts.find(post_id).first(conn).optional()?;

    Ok(updated_post)
}

pub fn delete_post_by_id(
    conn: &mut SqliteConnection,
    post_id: String,
) -> Result<Option<usize>, DbError> {
    use crate::db::schema::posts::dsl::*;

    let query = diesel::delete(posts.find(post_id)).execute(conn)?;

    if query > 0 {
        Ok(Some(query))
    } else {
        Ok(None)
    }
}
