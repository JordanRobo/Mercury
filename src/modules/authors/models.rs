use crate::db::schema::authors;
use crate::users::models::User;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorRes {
    pub id: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub email: String,
    pub slug: String,
    pub bio: Option<String>,
    pub profile_picture: Option<String>,
}

impl From<(Author, User)> for AuthorRes {
    fn from(value: (Author, User)) -> Self {
        let (author, user) = value;
        AuthorRes {
            id: author.author_id,
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
            slug: author.slug,
            bio: author.bio,
            profile_picture: author.profile_picture,
        }
    }
}

impl Author {
    pub fn response(self, user: User) -> AuthorRes {
        (self, user).into()
    }
}
