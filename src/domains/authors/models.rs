use crate::db::schema::authors;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = authors)]
pub struct Author {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub email: String,
    pub bio: Option<String>,
    pub profile_picture: Option<String>,
    pub created_at: Option<NaiveDateTime>,
}
