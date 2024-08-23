mod bootstrap;
mod core;
mod test;

use std::sync::Arc;
use bootstrap::Bootstrap;
use color_eyre::{eyre::eyre, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // 程序初始化
    match Bootstrap::run().await {
        Ok(bootstrap) => {
            // 在 axum_shutdown 调用之前，确保 infrastructure_layer 被正确引用
            let webserver = Arc::clone(&bootstrap.infrastructure_layer.webserver);

            // 等待 Ctrl+C 信号，并在接收到信号后关闭 Web 服务器
            tokio::signal::ctrl_c().await?;
            webserver.axum_shutdown();

            Ok(())
        }
        Err(err) => Err(eyre!("Application error: {}", err))?
    }
}

// use std::error::Error;
// use std::sync::Arc;
// use axum::response::IntoResponse;
// use axum::{debug_handler, Json, Router};
// use axum::extract::State;
// use axum::handler::Handler;
// use axum::http::StatusCode;
// use axum::routing::get;
// use serde_json::json;
//
// #[tokio::main]
// async fn main() {
//     // initialize tracing
//     tracing_subscriber::fmt::init();
//
//     let te_fun = Arc::new(TeFun::new());
//
//     // build our application with a route
//     let app = Router::new()
//         .route("/test", get(test1))
//         .with_state(te_fun);
//
//     // run our app with hyper
//     let listener = tokio::net::TcpListener::bind("127.0.0.1:9000")
//         .await
//         .unwrap();
//
//     tracing::info!("listening on http://{}", listener.local_addr().unwrap());
//     axum::serve(listener, app).await.unwrap();
// }
//
//
// #[axum::debug_handler]
// async fn test1(
//     State(s): State<Arc<TeFun>>
// ) -> impl IntoResponse {
//     s.function1().await.expect("Unable to insert the record!");
//     s.function2().await.expect("Unable to insert the record!");
//     let output = json!({"info": "Record inserted!"});
//     // (axum::http::StatusCode, Json(output))
//
//     Json(json!({ "status": "success", "message": "User registered successfully" }))
// }
//
//
// pub struct TeFun;
//
// impl TeFun {
//     fn new() -> Self {
//         Self {}
//     }
//
//
//
//     async fn function1(&self) -> Result<(), Box<dyn Error>> {
//         println!("12312312");
//         Ok(())
//     }
//
//     async fn function2(&self) -> Result<(), Box<dyn Error>> {
//         println!("12312qqqq312");
//         Ok(())
//     }
// }



