use crate::db::{schema::posts, DbError};
use crate::posts::models::{
    CreatePost, Post, PostResponse, PostWithAuthor, PostWithAuthorAndTags, PostWithTags, UpdatePost,
};
use crate::tags::models::Tag;
use crate::users::models::Author;
use crate::utils::{get_current_timestamp, slug_gen};
use diesel::prelude::*;

// Mock data
fn mock_post() -> Post {
    Post {
        id: "1".to_string(),
        title: "First Post".to_string(),
        slug: "first-post".to_string(),
        excerpt: Some("This is an excerpt".to_string()),
        content: Some("This is the content of the first post".to_string()),
        author_id: Some("1".to_string()),
        feature_image: None,
        status: Some("published".to_string()),
        published_at: None,
        created_at: None,
        updated_at: None,
    }
}

fn mock_author() -> Author {
    Author {
        id: "1".to_string(),
        name: "John Doe".to_string(),
        slug: "john-doe".to_string(),
        email: "john@example.com".to_string(),
        bio: "A sample bio".to_string(),
        profile_picture: "profile.jpg".to_string(),
    }
}

fn mock_tags() -> Vec<Tag> {
    vec![
        Tag {
            id: "1".to_string(),
            name: "tag 1".to_string(),
            slug: "tag-1".to_string(),
        },
        Tag {
            id: "2".to_string(),
            name: "tag 2".to_string(),
            slug: "tag-2".to_string(),
        },
    ]
}

pub fn get_all_posts(conn: &mut SqliteConnection) -> Result<PostResponse, DbError> {
    let posts = posts::table.load::<Post>(conn)?;
    Ok(PostResponse::Posts(posts))
}

pub fn get_posts_author(_conn: &mut SqliteConnection) -> Result<PostResponse, DbError> {
    let post_with_author = PostWithAuthor {
        post: mock_post(),
        author: Some(mock_author()),
    };
    Ok(PostResponse::PostsAuthor(vec![post_with_author]))
}

pub fn get_posts_tags(_conn: &mut SqliteConnection) -> Result<PostResponse, DbError> {
    let post_with_tags = PostWithTags {
        post: mock_post(),
        tags: Some(mock_tags()),
    };
    Ok(PostResponse::PostsTags(vec![post_with_tags]))
}

pub fn get_posts_author_tags(_conn: &mut SqliteConnection) -> Result<PostResponse, DbError> {
    let post_with_author_and_tags = PostWithAuthorAndTags {
        post: mock_post(),
        author: Some(mock_author()),
        tags: Some(mock_tags()),
    };
    Ok(PostResponse::PostsAuthorTags(vec![
        post_with_author_and_tags,
    ]))
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

pub fn get_post_by_slug(
    conn: &mut SqliteConnection,
    post_slug: String,
) -> Result<Option<Post>, DbError> {
    use crate::db::schema::posts::dsl::*;

    let post = posts
        .filter(slug.eq(post_slug.to_string()))
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
