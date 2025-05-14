use crate::db::{schema::*, DbError};
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

#[derive(Queryable, Debug, Serialize, Deserialize, Selectable, Identifiable)]
#[diesel(table_name = tags)]
#[diesel(primary_key(tag_id))]
pub struct Tag {
    pub tag_id: String,
    pub name: String,
    pub slug: String,
}

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct TagResponse {
    pub name: String,
    pub slug: String,
}

impl From<Tag> for TagResponse {
    fn from(value: Tag) -> Self {
        TagResponse { 
            name: value.name,
            slug: value.slug
        }
    }
}

impl TagResponse {
    pub fn get_post_tags(conn: &mut SqliteConnection, post: &Post) -> Result<Vec<Self>, DbError> {
        use crate::db::schema::tags::dsl::*;

        let tags_list: Vec<Tag> = PostTag::belonging_to(post)
            .inner_join(tags)
            .select(Tag::as_select())
            .load(conn)?;

        let response: Vec<Self> = tags_list.into_iter()
            .map(Self::from)
            .collect();

        Ok(response)
    }
}