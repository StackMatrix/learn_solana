use std::sync::Arc;
use chrono::{Duration, Utc};
use color_eyre::Report;
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
struct CustomClaims {
    exp: usize,
    iat: usize,
    iss: String,
    nbf: usize,
    sub: String,
}

#[derive(Debug, Serialize)]
pub struct TokenOutPut {
    access_token: String,
    expires_in: u64,
    token_type: String,
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

    /// Description
    ///     生成 JWT
    /// Params
    ///     user_id: &str - 用户id
    /// Return
    ///     Result<TokenOutPut, Report>
    ///         - TokenOutPut jwt 信息输出
    ///         - Report 错误报告
    pub fn create_jwt(&self, user_id: &str) -> Result<TokenOutPut, Report> {
        let now = Utc::now();

        // 自定义 claims
        let claims = CustomClaims {
            exp: (now + Duration::seconds(self.jwt_config.jwt_ttl as i64)).timestamp() as usize,
            iat: now.timestamp() as usize,
            iss: "GuardName".to_string(),
            nbf: (now.timestamp() - 1000) as usize,
            sub: user_id.to_string(),
        };

        // 生成 access_token
        let access_token = encode(
            &Header::new(Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(self.jwt_config.secret.as_ref()),
        )?;

        // 输出 token 信息
        let token_data = TokenOutPut {
            access_token,
            expires_in: self.jwt_config.jwt_ttl,
            token_type: "Bearer".to_string(),
        };

        Ok(token_data)
    }

    /// Description
    ///     验证 JWT
    /// Params
    ///     token: &str - token
    /// Return
    ///     Result<CustomClaims, Report>
    ///         - CustomClaims 自定义 Claims
    ///         - Report 错误报告
    pub fn validate_jwt(&self, token: &str) -> Result<CustomClaims, Report> {
        // 校验 jwt 是否正确
        let get_jwt = decode::<CustomClaims>(
            token,
            &DecodingKey::from_secret(self.jwt_config.secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        ).map(|data| data.claims);

        Ok(get_jwt?)
    }
}
