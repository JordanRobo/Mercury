use crate::db::schema::*;
use crate::posts::models::Post;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Identifiable, Associations, Debug, Serialize, Deserialize)]
#[diesel(belongs_to(Post))]
#[diesel(belongs_to(Tag))]
#[diesel(table_name = post_tags)]
#[diesel(primary_key(post_id, tag_id))]
pub struct PostTag {
    pub post_id: String,
    pub tag_id: String,
}

#[derive(Queryable, Debug, Serialize, Deserialize)]
#[diesel(table_name = tags)]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub slug: String,
}
