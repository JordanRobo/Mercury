use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
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
    pub role: String,
}
