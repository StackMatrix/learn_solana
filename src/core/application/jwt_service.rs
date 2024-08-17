// use std::sync::Arc;
// use chrono::{Duration, Utc};
// use jsonwebtoken::{Algorithm, decode, DecodingKey, encode, EncodingKey, Header, Validation};
// use serde::{Deserialize, Serialize};
// use crate::core::infrastructure::jwt::Jwt;
//
// pub struct JwtService {
//     pub config: Arc<Jwt>,
// }
//
// #[derive(Debug, Serialize, Deserialize)]
// struct Claims {
//     sub: String,
//     exp: usize,
// }
//
// impl JwtService {
//     pub fn new(config: Arc<Jwt>) -> Self {
//         Self { config }
//     }
//
//     /// 生成 JWT
//     pub fn create_jwt(&self, user_id: &str) -> String {
//         let expiration = Utc::now()
//             .checked_add_signed(Duration::seconds(self.config.jwt_config.jwt_ttl as i64))
//             .expect("valid timestamp")
//             .timestamp() as usize;
//
//         let claims = Claims {
//             sub: user_id.to_owned(),
//             exp: expiration,
//         };
//
//         encode(
//             &Header::new(Algorithm::HS256),
//             &claims,
//             &EncodingKey::from_secret(self.config.jwt_config.secret.as_ref()),
//         ).expect("JWT Token creation failed")
//     }
//
//     /// 验证 JWT
//     pub fn validate_jwt(&self, token: &str) -> Result<Claims, String> {
//         decode::<Claims>(
//             token,
//             &DecodingKey::from_secret(self.config.jwt_config.secret.as_ref()),
//             &Validation::new(Algorithm::HS256),
//         )
//             .map(|data| data.claims)
//             .map_err(|err| err.to_string())
//     }
// }