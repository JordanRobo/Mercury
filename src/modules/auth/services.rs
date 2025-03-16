use super::models::{AuthReq, AuthRes, Claims, Sub};
use crate::db::DbPool;
use crate::utils::Utils;

use actix_web::{dev::ServiceRequest, error, web, Error, HttpResponse, Responder, Result};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use argon2::{self, Config};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};

pub struct AuthService;

impl AuthService {
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

        match Claims::decode_token(token) {
            Ok(claims) => {
                if !Claims::verify_expiry(&claims) {
                    Err((actix_web::error::ErrorUnauthorized("Token expired"), req))
                } else if !Claims::verify_site(&claims) {
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
        form: web::Json<AuthReq>,
    ) -> Result<impl Responder> {
        let auth_req = form.into_inner();
        let default_hash = Utils::get_env_var("DEFAULT_PASSWORD");
        let start_time = std::time::Instant::now();

        let auth_req_clone = auth_req.clone();

        let user_result = web::block(move || {
            let mut conn = pool.get()?;
            auth_req_clone.match_credentials(&mut conn)
        })
        .await?
        .map_err(error::ErrorInternalServerError)?;

        let (hash_to_verify, user_exists, user) = match user_result {
            Some(ref user) => (&user.pass_hash, true, Some(user)),
            None => (&default_hash, false, None),
        };

        let valid_pass = auth_req.verify_password(hash_to_verify);

        let response = if user_exists && valid_pass {
            let user = user.unwrap().clone();

            let expiration = Utc::now()
                .checked_add_signed(Duration::hours(1))
                .expect("valid timestamp")
                .timestamp();

            let claims = Claims {
                sub: Sub {
                    id: user.user_id,
                    name: user.first_name,
                    email: user.email,
                },
                exp: expiration as usize,
                site: Utils::get_env_var("SITE_ID"),
            };

            match encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(Utils::get_env_var("JWT_SECRET").as_bytes()),
            ) {
                Ok(token) => HttpResponse::Ok().json(AuthRes { token }),
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
}
