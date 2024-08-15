use std::fmt::Debug;
use std::time::Duration;
use axum::http::Response;
use tower_http::trace::OnResponse;
use tracing::{info, Level, Span};

#[derive(Clone)]
pub struct CustomOnResponse {
    pub level: Level,
}

impl CustomOnResponse {
    pub fn new() -> Self {
        Self {
            level: Level::INFO,
        }
    }

    pub fn level(mut self, level: Level) -> Self {
        self.level = level;
        self
    }
}

impl<B: Debug> OnResponse<B> for CustomOnResponse {
    fn on_response(self, response: &Response<B>, latency: Duration, _: &Span) {
        info!("+ [WebServer] Response: {:?}, Latency: {} ms", response, latency.as_millis());
    }
}
