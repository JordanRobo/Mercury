use crate::core::db::DbError;
use crate::db::schema::posts;
use crate::{
    authors::{Author, AuthorResponse}, 
    Utils, 
    tags::TagResponse
};

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
    pub author: Option<AuthorResponse>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TagResponse>>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePost {
    pub title: String,
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
pub struct PostQuery {
    inc: Option<String>,
}

impl PostQuery {
    // Helper method to check if a specific include is requested
    pub fn includes(&self, include: &str) -> bool {
        self.inc
            .as_ref()
            .map(|inc| inc.split(',').any(|i| i.trim() == include))
            .unwrap_or(false)
    }
}

impl Post {
    pub fn new(post: CreatePost) -> Self {
        let post_id = xid::new().to_string();
        let now = Utils::get_current_timestamp();
        let slug = Utils::slug_gen(&post.title);

        let mut post_excerpt = String::from(post.content.clone().unwrap());
        post_excerpt.truncate(150);
        post_excerpt.push_str("...");
        
        Self { 
            post_id, 
            title: post.title, 
            slug, 
            excerpt: Some(post_excerpt), 
            content: post.content, 
            author_id: post.author_id.unwrap(), 
            feature_image: post.feature_image, 
            status: post.status, 
            published_at: None, 
            created_at: now, 
            updated_at: now
        }
    }

    pub fn create(conn: &mut SqliteConnection, new_post: CreatePost) -> Result<Self, DbError> {
        use crate::db::schema::posts::dsl::*;

        let post = Post::new(new_post);

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
    pub fn fetch_all(conn: &mut SqliteConnection, query_params: &PostQuery) -> Result<Vec<Self>, DbError> {
        use crate::db::schema::posts::dsl::*;
        
        let all_posts = posts.load::<Post>(conn)?;
        
        let include_author = query_params.includes("author");
        let include_tags = query_params.includes("tags");
        
        let mut post_responses = Vec::new();
        
        for post in all_posts {
            let mut response = PostResponse {
                post,
                author: None,
                tags: None,
            };
            
            if include_author {
                let author_result = AuthorResponse::fetch_by_id(conn, &response.post.author_id)?;
                response.author = Some(author_result);
            }
            
            if include_tags {
                let tags_result = TagResponse::get_post_tags(conn, &response.post)?;
                response.tags = Some(tags_result);
            }
            
            post_responses.push(response);
        }
        
        Ok(post_responses)
    }

    pub fn fetch_by_id(conn: &mut SqliteConnection, input_id: &str, query_params: &PostQuery) -> Result<Option<Self>, DbError> {
        todo!("Create PostResponse::fetch_by_id()")
    }

    pub fn fetch_by_slug(conn: &mut SqliteConnection, input_slug: &str, query_params: &PostQuery) -> Result<Option<Self>, DbError> {
        todo!("Create PostResponse::fetch_by_slug()")
    }
}
