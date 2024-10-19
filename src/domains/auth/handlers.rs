use crate::utils::{get_jwt_secret, get_site_identifier};
use actix_web::{dev::ServiceRequest, web, Error, HttpResponse, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

use super::models::{Claims, LoginRequest, LoginResponse};

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

pub async fn login(form: web::Json<LoginRequest>) -> impl Responder {
    // Add Login logic here
    // Temp hardcoded credentials for development
    // Implement password hashing
    if form.email == "mail@mail.com" && form.password == "root" {
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(1))
            .expect("valid timestamp")
            .timestamp();

        let claims = Claims {
            sub: form.email.clone(),
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
