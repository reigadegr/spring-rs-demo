mod config;
mod res;
mod pojo;
mod controller;
mod mapper;
mod service;
mod patch;

use spring_web::WebConfigurator;

use crate::config::mysql::{init_mysql};
use crate::config::nacos::init_nacos_service;
use spring::{auto_config, App};
use spring_web::{axum::response::IntoResponse, extractor::Path, WebPlugin};
use spring_web::route;

#[auto_config(WebConfigurator)]
#[tokio::main]
async fn main() {
    init_nacos_service().await;
    init_mysql().await;
    tracing::info!("Listening on http://127.0.0.1:5800");
    App::new()
        .add_plugin(WebPlugin)
        .run()
        .await
}

#[route("/", method = "GET")]
async fn hello_world() -> impl IntoResponse {
    println!("hello world");
    "hello world"
}

#[route("/hello/:name", method = "GET", method = "POST")]
async fn hello(Path(name): Path<String>) -> impl IntoResponse {
    format!("hello {name}")
}
