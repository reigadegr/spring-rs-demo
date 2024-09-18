use anyhow::Context;
use spring::{auto_config, App};
use spring_sqlx::{
    sqlx::{self, Row},
    ConnectPool, SqlxPlugin,
};

#[allow(unused)]
use spring_web::{
    axum::response::IntoResponse,
    error::Result,
    extractor::{Component, Path},
    WebConfigurator, WebPlugin,
};
//允许未使用
#[allow(unused)]
use spring_web::{get, route};

// 主函数入口
#[auto_config(WebConfigurator)]   // 自动扫描web router
#[tokio::main]
async fn main() {
    App::new()
        .add_plugin(SqlxPlugin)  // 添加插件
        .add_plugin(WebPlugin)
        .run()
        .await
}

// Component可以抽取由Sqlx插件在AppState中的注册的连接池
#[get("/version")]
async fn sqlx_request_handler(Component(pool): Component<ConnectPool>) -> Result<String> {
    let version = sqlx::query("SELECT email FROM users")
        .fetch_one(&pool)
        .await
        .context("sqlx query failed")?
        .get("email");
    println!("version: {}", version);
    Ok(version)
}
