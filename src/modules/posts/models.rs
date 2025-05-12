use crate::core::db::DbError;
use crate::db::schema::posts;
use crate::tags::Tag;
use crate::{authors::Author, Utils};

use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Queryable, Identifiable, Associations, Debug, Serialize, Deserialize, Default, Insertable,
)]
#[diesel(belongs_to(Author, foreign_key = author_id))]
#[diesel(primary_key(post_id))]
#[diesel(table_name = posts)]
pub struct Post {
    pub post_id: String,
    pub title: String,
    pub slug: String,
    pub excerpt: Option<String>,
    pub content: Option<String>,
    pub author_id: String,
    pub feature_image: Option<String>,
    pub status: Option<String>,
    pub published_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct PostResponse {
    #[serde(flatten)]
    pub post: Post,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<Author>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePost {
    pub title: String,
    pub excerpt: Option<String>,
    pub content: Option<String>,
    pub author_id: Option<String>,
    pub feature_image: Option<String>,
    pub status: Option<String>,
}

#[derive(Insertable, Deserialize, AsChangeset)]
#[diesel(table_name = posts)]
pub struct UpdatePost {
    pub title: String,
    pub slug: String,
    pub excerpt: Option<String>,
    pub content: Option<String>,
    pub author_id: Option<String>,
    pub feature_image: Option<String>,
    pub status: Option<String>,
    pub published_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Deserialize)]
struct PostQuery {
    inc: Option<String>,
}

impl Post {
    pub fn create(conn: &mut SqliteConnection, new_post: CreatePost) -> Result<Self, DbError> {
        use crate::db::schema::posts::dsl::*;

        let id = xid::new().to_string();
        let now = Utils::get_current_timestamp();

        let post = Self {
            post_id: id,
            title: new_post.title.clone(),
            slug: Utils::slug_gen(&new_post.title),
            excerpt: new_post.excerpt,
            content: new_post.content,
            author_id: new_post.author_id.unwrap(),
            feature_image: new_post.feature_image,
            status: new_post.status,
            published_at: None,
            created_at: now,
            updated_at: now,
        };

        diesel::insert_into(posts).values(&post).execute(conn)?;

        Ok(post)
    }

    pub fn update(
        conn: &mut SqliteConnection,
        input_id: String,
        post: UpdatePost,
    ) -> Result<Option<Self>, DbError> {
        use crate::db::schema::posts::dsl::*;

        diesel::update(posts.find(input_id.clone()))
            .set(&post)
            .execute(conn)?;

        let updated_post = posts.find(input_id).first(conn).optional()?;

        Ok(updated_post)
    }

    pub fn delete(conn: &mut SqliteConnection, input_id: String) -> Result<Option<usize>, DbError> {
        use crate::db::schema::posts::dsl::*;

        let query = diesel::delete(posts.find(input_id)).execute(conn)?;

        if query > 0 {
            Ok(Some(query))
        } else {
            Ok(None)
        }
    }
}

impl PostResponse {
    pub fn fetch_all(
        conn: &mut SqliteConnection,
        query_params: &PostQuery,
    ) -> Result<Self, DbError> {
        let posts = posts::table.load::<Post>(conn)?;

        let mut response = PostResponse {
        	post: None,
        	author: None,
         	tags: None
        }

        Ok(response)
    }

    pub fn fetch_by_id(
        conn: &mut SqliteConnection,
        input_id: String,
    ) -> Result<Option<Self>, DbError> {
        use crate::db::schema::posts::dsl::*;

        let post = posts
            .filter(post_id.eq(input_id.to_string()))
            .first::<Self>(conn)
            .optional()?;

        Ok(post)
    }

    pub fn fetch_by_slug(
        conn: &mut SqliteConnection,
        input_slug: String,
    ) -> Result<Option<Self>, DbError> {
        use crate::db::schema::posts::dsl::*;

        let post = posts
            .filter(slug.eq(input_slug.to_string()))
            .first::<Self>(conn)
            .optional()?;

        Ok(post)
    }
}
