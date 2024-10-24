use crate::{
    users::models::User,
    utils::{get_jwt_secret, get_site_identifier},
};
use actix_web::{dev::ServiceRequest, error, web, Error, HttpResponse, Responder, Result};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use argon2::{self, Config};
use chrono::{DateTime, Duration, Utc};
use diesel::prelude::*;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

use super::models::{Claims, LoginRequest, LoginResponse, Sub};
use crate::db::{DbError, DbPool};

fn decode_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    decode::<Claims>(token, &DecodingKey::from_secret(&get_jwt_secret()), &Validation::default()).map(|token_data| token_data.claims)
}

fn check_token_expiry(claims: &Claims) -> bool {
    let exp = DateTime::from_timestamp(claims.exp as i64, 0);
    match exp {
        Some(exp_time) => Utc::now() <= exp_time,
        None => false,
    }
}

fn check_token_site(claims: &Claims) -> bool {
    claims.site == get_site_identifier()
}

pub async fn check_auth(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    // Add logging for failed attempts
    // Might add caching for valid tokens
    let token = credentials.token();

    match decode_token(token) {
        Ok(claims) => {
            if !check_token_expiry(&claims) {
                Err((Error::from(actix_web::error::ErrorUnauthorized("Token expired")), req))
            } else if !check_token_site(&claims) {
                Err((Error::from(actix_web::error::ErrorUnauthorized("Invalid site")), req))
            } else {
                Ok(req)
            }
        }
        Err(_) => Err((Error::from(actix_web::error::ErrorUnauthorized("Invalid token")), req)),
    }
}

fn match_credentials(conn: &mut SqliteConnection, input_email: &str) -> Result<Option<User>, DbError> {
    use crate::db::schema::users::dsl::*;

    let user = users
        .filter(email.eq(&input_email))
        .first::<User>(conn)
        .optional()?;

    Ok(user)
}

pub fn hash_password(password: &str, salt: &[u8]) -> String {
    let config = Config::default();
    argon2::hash_encoded(password.as_bytes(), salt, &config).unwrap()
}

pub async fn login(pool: web::Data<DbPool>, form: web::Json<LoginRequest>) -> Result<impl Responder> {
    // Add rate limiting check here
    // Log failed login attempts
    // Add password complexity validation

    let input_pass = form.password.clone();
    let input_email = form.email.clone();

    let user = web::block(move || {
        let mut conn = pool.get()?;
        match_credentials(&mut conn, &input_email)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(match user {
        Some(user) => {
            if hash_password(&input_pass, &user.pass_salt.clone()) == user.pass_hash {
                let expiration = Utc::now()
                    .checked_add_signed(Duration::hours(1))
                    .expect("valid timestamp")
                    .timestamp();

                let sub = Sub {
                    id: user.id.to_string(),
                    name: user.name,
                    email: user.email,
                    role: user.role,
                };

                let claims = Claims {
                    sub,
                    exp: expiration as usize,
                    site: get_site_identifier(),
                };

                match encode(&Header::default(), &claims, &EncodingKey::from_secret(&get_jwt_secret())) {
                    Ok(token) => HttpResponse::Ok().json(LoginResponse { token }),
                    Err(_) => HttpResponse::InternalServerError().body("Could not generate token"),
                }
            } else {
                HttpResponse::Unauthorized().body("Invalid username or password")
            }
        }
        None => HttpResponse::Unauthorized().body("Invalid username or password"),
    })
}
