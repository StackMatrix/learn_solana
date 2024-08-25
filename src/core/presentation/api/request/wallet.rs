use serde::Deserialize;

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct GenerationRequest {
    pub user_id: i32,
    pub pub_key: String,
    pub privy_key: String
}