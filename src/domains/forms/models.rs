use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Form {
    pub id: String,
    pub created_at: Option<NaiveDateTime>,
    pub unread: i32,
    pub form_data: String,
}
