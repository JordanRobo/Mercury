#![allow(unused)]
#![allow(clippy::all)]

use diesel::prelude::*;
use serde::{ Serialize, Deserialize };
use crate::db::schema::*;

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq, Serialize, Deserialize, Insertable)]
#[diesel(table_name = authors)]
pub struct Author {
    pub id: i32,
    pub name: Option<String>,
    pub email: Option<String>,
    pub bio: Option<String>,
    pub profile_picture: Option<String>,
}

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq, Serialize, Deserialize, Associations, Insertable)]
#[diesel(belongs_to(Post))]
#[diesel(belongs_to(Tag))]
#[diesel(table_name = post_tags)]
#[diesel(primary_key(post_id, tag_id))]
pub struct PostTag {
    pub post_id: i32,
    pub tag_id: i32,
}

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq, Serialize, Deserialize, Associations, Insertable, AsChangeset, Clone)]
#[belongs_to(Author)]
#[diesel(table_name = posts)]
pub struct Post {
    #[serde(default)]
    pub id: i32,
    pub title: Option<String>,
    pub slug: Option<String>,
    pub content: Option<String>,
    pub feature_image: Option<String>,
    pub excerpt: Option<String>,
    pub published: Option<bool>,
    pub author_id: Option<i32>,
}

#[derive(Insertable, Deserialize, AsChangeset)]
#[diesel(table_name = posts)]
pub struct NewPost {
    pub title: Option<String>,
    pub slug: Option<String>,
    pub content: Option<String>,
    pub feature_image: Option<String>,
    pub excerpt: Option<String>,
    pub published: Option<bool>,
    pub author_id: Option<i32>,
}

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq, Serialize, Deserialize, Insertable)]
#[diesel(table_name = tags)]
pub struct Tag {
    pub id: i32,
    pub name: Option<String>,
    pub slug: Option<String>,
}