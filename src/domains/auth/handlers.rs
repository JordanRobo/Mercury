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
                Err((actix_web::error::ErrorUnauthorized("Token expired"), req))
            } else if !check_token_site(&claims) {
                Err((actix_web::error::ErrorUnauthorized("Invalid site"), req))
            } else {
                Ok(req)
            }
        }
        Err(_) => Err((actix_web::error::ErrorUnauthorized("Invalid token"), req)),
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

pub fn create_password(password: &str, salt: &[u8]) -> String {
    let config = Config::default();
    argon2::hash_encoded(password.as_bytes(), salt, &config).unwrap()
}

fn verify_password(encoded: &str, password: &str) -> bool {
    let pwd = password.as_bytes();
    argon2::verify_encoded(encoded, pwd).unwrap_or(false)
}

pub async fn login(pool: web::Data<DbPool>, form: web::Json<LoginRequest>) -> Result<impl Responder> {
    let input_pass = form.password.clone();
    let input_email = form.email.clone();

    let default_hash = "$argon2id$v=19$m=4096,t=3,p=1$saltysaltysaltysalt$HashValueThatWillNeverMatchButHasCorrectLength".to_string();
    let start_time = std::time::Instant::now();

    let user_result = web::block(move || {
        let mut conn = pool.get()?;
        match_credentials(&mut conn, &input_email)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    let (hash_to_verify, user_exists, user) = match user_result {
        Some(ref user) => (&user.pass_hash, true, Some(user)),
        None => (&default_hash, false, None),
    };

    let valid_pass = verify_password(hash_to_verify, &input_pass);

    let token_result = if user_exists && valid_pass {
        let user = user.unwrap().clone(); 

        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(1))
            .expect("valid timestamp")
            .timestamp();

        let claims = Claims {
            sub: Sub {
                id: user.id,
                name: user.name,
                email: user.email,
                role: user.role,
            },
            exp: expiration as usize,
            site: get_site_identifier(),
        };

        encode(&Header::default(), &claims, &EncodingKey::from_secret(&get_jwt_secret()))
    } else {
        let dummy_claims = Claims {
            sub: Sub {
                id: "0".to_string(),
                name: "dummy".to_string(),
                email: "dummy@example.com".to_string(),
                role: "none".to_string(),
            },
            exp: (Utc::now().timestamp() + 3600) as usize,
            site: get_site_identifier(),
        };
        
        encode(&Header::default(), &dummy_claims, &EncodingKey::from_secret(&get_jwt_secret()))
    };

    let elapsed = start_time.elapsed();
    let target_duration = std::time::Duration::from_millis(400);
    if elapsed < target_duration {
        actix_web::rt::time::sleep(target_duration - elapsed).await;
    }

    Ok(if user_exists && valid_pass {
        match token_result {
            Ok(token) => HttpResponse::Ok().json(LoginResponse { token }),
            Err(_) => HttpResponse::InternalServerError().body("Could not generate token"),
        }
    } else {
        HttpResponse::Unauthorized().body("Invalid username or password")
    })
}
