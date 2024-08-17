use std::sync::Arc;
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use jsonwebtoken::{Algorithm, decode, DecodingKey, encode, EncodingKey, Header, Validation};
use crate::core::infrastructure::config::{Config, JwtConfig };

/// Description
///     JWT
/// Fields
///     pub jwt_config: Arc<JwtConfig>, 配置实例
pub struct Jwt {
    pub jwt_config: Arc<JwtConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

impl Jwt {
    /// # Description
    ///     初始化 JWT 信息
    /// # Param
    ///     config: Arc<Config>, 配置文件
    /// # Return
    ///     Self: 初始化后的 JWT 实例
    pub fn new(config: Arc<Config>) -> Self {
        let jwt_config = Arc::new(
            JwtConfig {
                secret: config.jwt.secret.clone(),
                jwt_ttl: config.jwt.jwt_ttl.clone(),
                jwt_blacklist_grace_period: config.jwt.jwt_blacklist_grace_period.clone(),
                refresh_grace_period: config.jwt.refresh_grace_period.clone(),
            }
        );

        Self { jwt_config }
    }

    /// 生成 JWT
    pub fn create_jwt(&self, user_id: &str) -> String {
        let expiration = Utc::now()
            .checked_add_signed(Duration::seconds(self.jwt_config.jwt_ttl as i64))
            .expect("valid timestamp")
            .timestamp() as usize;


        let claims = Claims {
            sub: user_id.to_owned(),
            exp: expiration,
        };

        encode(
            &Header::new(Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(self.jwt_config.secret.as_ref()),
        ).expect("JWT Token creation failed")
    }

    /// 验证 JWT
    pub fn validate_jwt(&self, token: &str) -> Result<Claims, String> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_config.secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        )
            .map(|data| data.claims)
            .map_err(|err| err.to_string())
    }
}
