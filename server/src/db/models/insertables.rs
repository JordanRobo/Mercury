use crate::db::schema::users;
use diesel::Insertable;
use serde::Serialize;

#[derive(Insertable, Serialize, Clone)]
#[diesel(table_name=users)]
pub struct NewUser {
    pub firstname: String,
    pub lastname: String,
    pub email: String,
}