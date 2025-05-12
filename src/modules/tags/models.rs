use crate::db::schema::*;
use crate::posts::Post;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Identifiable, Associations, Debug, Serialize, Deserialize)]
#[diesel(belongs_to(Post))]
#[diesel(belongs_to(Tag))]
#[diesel(table_name = post_tags)]
pub struct PostTag {
    pub id: String,
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
