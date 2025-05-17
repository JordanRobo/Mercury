use crate::db::{schema::authors, DbError};
use crate::users::User;

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Identifiable, Debug, Serialize, Deserialize, Insertable, Clone)]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(table_name = authors)]
#[diesel(primary_key(author_id))]
pub struct Author {
    pub author_id: String,
    pub user_id: String,
    pub slug: String,
    pub bio: Option<String>,
    pub profile_picture: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct AuthorResponse {
    pub id: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub email: String,
    pub slug: String,
    pub bio: Option<String>,
    pub profile_picture: Option<String>,
}

impl AuthorResponse {
    pub fn fetch_by_id(conn: &mut SqliteConnection, input_id: &str) -> Result<Option<Self>, DbError> {
        use crate::db::schema::authors::dsl::*;
        use crate::db::schema::users::dsl::*;

        let author_resp = authors
            .inner_join(users)
            .filter(author_id.eq(input_id))
            .select((
                author_id,
                first_name,
                last_name,
                email,
                slug,
                bio,
                profile_picture,
            ))
            .first::<AuthorResponse>(conn)
            .optional()?;

        Ok(author_resp)
    }
}