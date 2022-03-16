mod api;
mod res;

use std::net::SocketAddr;

use dotenv::{dotenv, var};
use log::info;

use crate::api::run_code;
use axum::extract::Extension;
use axum::{
    routing::{delete, get, get_service, post},
    Router,
};
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};
#[tokio::main]
async fn main() {
    dotenv().ok();

    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    let api = Router::new().route("/run", post(run_code)); // 用户接口

    let app = Router::new()
        .nest("/api", api)
        // .layer(Extension(1))
        // .layer(Extension(pool))
        .layer(
            CorsLayer::new()
                .allow_headers(Any)
                .allow_origin(Any)
                .allow_methods(Any),
        );

    let port: u16 = var("PORT").unwrap_or("8000".to_string()).parse().unwrap();
    let host: String = var("HOST").unwrap_or("127.0.0.1".to_string());

    let addr = format!("{}:{}", host, port).parse().unwrap();
    log::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
