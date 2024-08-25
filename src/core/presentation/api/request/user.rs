use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub identifier: String,
    pub password: String
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub identifier: String,
    pub password: String
}
