use crate::db::DbError;
use crate::users::models::User;
use crate::utils::Utils;
use serde::{Deserialize, Serialize};

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use jsonwebtoken::errors::Error as JWTError;
use jsonwebtoken::{decode, DecodingKey, Validation};

#[derive(Deserialize, Clone)]
pub struct AuthReq {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthRes {
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: Sub,
    pub exp: usize,
    pub site: String,
}

#[derive(Serialize, Deserialize)]
pub struct Sub {
    pub id: String,
    pub name: String,
    pub email: String,
}

impl Claims {
    pub fn decode_token(token: &str) -> Result<Self, JWTError> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(Utils::get_env_var("JWT_SECRET").as_bytes()),
            &Validation::default(),
        )
        .map(|token_data| token_data.claims)
    }

    pub fn verify_expiry(&self) -> bool {
        let exp = DateTime::from_timestamp(self.exp as i64, 0);
        match exp {
            Some(exp_time) => Utc::now() <= exp_time,
            None => false,
        }
    }

    pub fn verify_site(&self) -> bool {
        self.site == Utils::get_env_var("SITE_ID")
    }
}

impl AuthReq {
    pub fn verify_password(self, encoded: &str) -> bool {
        let pwd = self.password.as_bytes();
        argon2::verify_encoded(encoded, pwd).unwrap_or(false)
    }

    pub fn match_credentials(self, conn: &mut SqliteConnection) -> Result<Option<User>, DbError> {
        use crate::db::schema::users::dsl::*;

        let user = users
            .filter(email.eq(&self.email))
            .first::<User>(conn)
            .optional()?;

        Ok(user)
    }
}
