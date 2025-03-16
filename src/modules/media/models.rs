use crate::db::schema::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Serialize, Deserialize)]
#[diesel(primary_key(media_id))]
#[diesel(table_name = media)]
pub struct Media {
    pub media_id: String,
    pub file_name: String,
    pub file_type: String,
    pub file_size: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
