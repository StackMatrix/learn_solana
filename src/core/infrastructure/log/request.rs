use std::fmt::Debug;
use std::time::Duration;
use http::{Request, Response};
use tower_http::trace::{OnRequest, OnResponse};
use tracing::{event, info, Level, Span};

#[derive(Clone)]
pub struct CustomOnRequest {
    pub level: Level,
}

impl CustomOnRequest {
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

impl<B: Debug> OnRequest<B> for CustomOnRequest {
    fn on_request(&mut self, request: &Request<B>, _: &Span) {
        info!("+ [WebServer] Request: {:?}", request,);
    }
}




