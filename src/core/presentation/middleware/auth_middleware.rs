use axum::{
    response::Response,
    middleware::{Next},
    extract::Request,
};
use tracing::info;

/// # Description
///     认证中间件
pub struct AuthMiddleware;

impl AuthMiddleware {
    /// # Description
    ///     认证
    /// # Param
    ///     None
    /// # Return
    ///     Router: 路由
    pub async fn auth() {
        info!("+ [Router] Register handle");
    }

    pub async fn my_middleware(
        request: Request,
        next: Next,
    ) -> Response {
        // do something with `request`...
        info!("+ [Router] Register handle");

        let response = next.run(request).await;

        // do something with `response`...

        response
    }
}


// use axum::body::{box_body, BoxBody};
// use axum::extract::Extension;
// use axum::handler::Handler;
// use axum::http::{header, Request, Response, Uri};
// use axum::response::{IntoResponse, Redirect};
// use axum::{routing::get, Router};
// use headers::{HeaderName, HeaderValue};
// use tower_http::auth::{AsyncAuthorizeRequest, RequireAuthorizationLayer};
// use tower_http::set_header::SetResponseHeaderLayer;
//
// #[derive(Clone)]
// struct MyAuth {
//     n: i32,
// }
// impl AsyncAuthorizeRequest<B> for MyAuth {
//     type Output = i32;
//     type ResponseBody = BoxBody;
//     fn authorize<B>(&mut self, request: &Request<B>) -> Option<Self::Output> {
//         #[rustfmt::skip]
//         let i=request.headers().get(header::AUTHORIZATION).map(|header|{
//             header.to_str().ok().map(|s|s.parse::<i32>().ok()).flatten()
//         }).flatten()?;
//         (i % self.n == 0).then(|| i)
//     }
//     fn on_authorized<B>(&mut self, request: &mut Request<B>, output: Self::Output) {
//         request.extensions_mut().insert(output);
//     }
//     fn unauthorized_response<B>(&mut self, _request: &Request<B>) -> Response<Self::ResponseBody> {
//         // use axum::body::{Bytes, Full};
//         // let buf = Bytes::from("unauthorized!");
//         // Response::builder()
//         //     .status(StatusCode::UNAUTHORIZED)
//         //     .body(box_body(Full::new(buf)))
//         //     .unwrap()
//         Redirect::to(Uri::from_static("/login"))
//             .into_response()
//             .map(|b| box_body(b))
//     }
// }