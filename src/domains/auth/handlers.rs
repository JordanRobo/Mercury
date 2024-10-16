use crate::db::schema::users;
use crate::utils::get_site_identifier;
use actix_web::{dev, web, Error, HttpResponse, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use chrono::{Duration, Utc};
use diesel::prelude::*;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

use super::models::{Claims, LoginRequest, LoginResponse};

// Add a proper secret management system for JWT secret
// Add token refresh mechanisms
const JWT_SECRET: &[u8] = b"im_the_greatest";

pub async fn check_auth(req: dev::ServiceRequest, credentials: BearerAuth) -> Result<dev::ServiceRequest, (Error, dev::ServiceRequest)> {
    let token = credentials.token();

    match decode::<Claims>(token, &DecodingKey::from_secret(JWT_SECRET), &Validation::default()) {
        Ok(_claims) => Ok(req),
        Err(_) => Err((actix_web::error::ErrorUnauthorized("Invalid token"), req)),
    }
}

pub async fn login(form: web::Json<LoginRequest>) -> impl Responder {
    // Add Login logic here
    // Temp hardcoded credentials for development
    // Implement password hashing
    if form.username == "root" && form.password == "root" {
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(1))
            .expect("valid timestamp")
            .timestamp();

        let claims = Claims {
            sub: form.username.clone(),
            exp: expiration as usize,
            site: get_site_identifier(),
        };

        let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(JWT_SECRET)).unwrap();

        HttpResponse::Ok().json(LoginResponse { token })
    } else {
        HttpResponse::Unauthorized().body("Invalid username or password")
    }
}

pub fn check_admin(conn: &mut SqliteConnection) -> bool {
    let count = users::table
        .count()
        .get_result(conn)
        .unwrap_or(0);
    count == 0
}
