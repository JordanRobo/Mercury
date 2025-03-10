use super::models::Claims;
use crate::db::DbError;
use crate::users::models::User;
use crate::utils::get_env_var;

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use jsonwebtoken::{decode, DecodingKey, Validation};

pub(super) fn decode_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(get_env_var("JWT_SECRET").as_bytes()),
        &Validation::default(),
    )
    .map(|token_data| token_data.claims)
}

pub(super) fn check_token_expiry(claims: &Claims) -> bool {
    let exp = DateTime::from_timestamp(claims.exp as i64, 0);
    match exp {
        Some(exp_time) => Utc::now() <= exp_time,
        None => false,
    }
}

pub(super) fn check_token_site(claims: &Claims) -> bool {
    claims.site == get_env_var("SITE_ID")
}

pub(super) fn match_credentials(
    conn: &mut SqliteConnection,
    input_email: &str,
) -> Result<Option<User>, DbError> {
    use crate::db::schema::users::dsl::*;

    let user = users
        .filter(email.eq(&input_email))
        .first::<User>(conn)
        .optional()?;

    Ok(user)
}

pub(super) fn verify_password(encoded: &str, password: &str) -> bool {
    let pwd = password.as_bytes();
    argon2::verify_encoded(encoded, pwd).unwrap_or(false)
}
