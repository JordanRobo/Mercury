use crate::db::schema::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Serialize, Deserialize)]
#[diesel(primary_key(file_path))]
#[diesel(table_name = media)]
pub struct Media {
    pub id: String,
    pub file_name: String,
    pub file_type: String,
    pub file_size: i32,
    pub file_path: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
