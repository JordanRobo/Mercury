use super::models::{Claims, LoginRequest, LoginResponse, Sub};
use super::utils::{
    check_token_expiry, check_token_site, decode_token, match_credentials, verify_password,
};
use crate::db::DbPool;
use crate::utils::get_env_var;

use actix_web::{dev::ServiceRequest, error, web, Error, HttpResponse, Responder, Result};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use argon2::{self, Config};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};

pub fn create_password(password: &str, salt: &[u8]) -> String {
    let config = Config::default();
    argon2::hash_encoded(password.as_bytes(), salt, &config).unwrap()
}

pub async fn check_auth(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
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

pub async fn login(
    pool: web::Data<DbPool>,
    form: web::Json<LoginRequest>,
) -> Result<impl Responder> {
    let input_pass = form.password.clone();
    let input_email = form.email.clone();

    let default_hash = get_env_var("DEFAULT_PASSWORD");
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

    let response = if user_exists && valid_pass {
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
            site: get_env_var("SITE_ID"),
        };

        match encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(get_env_var("JWT_SECRET").as_bytes()),
        ) {
            Ok(token) => HttpResponse::Ok().json(LoginResponse { token }),
            Err(_) => HttpResponse::InternalServerError().body("Could not generate token"),
        }
    } else {
        HttpResponse::Unauthorized().body("Invalid username or password")
    };

    let elapsed = start_time.elapsed();
    let target_duration = std::time::Duration::from_millis(500);
    if elapsed < target_duration {
        actix_web::rt::time::sleep(target_duration - elapsed).await;
    }

    Ok(response)
}
