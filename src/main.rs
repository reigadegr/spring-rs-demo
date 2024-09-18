use std::sync::{Arc, LazyLock};
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
use nacos_sdk::api::constants;
use nacos_sdk::api::naming::{
    NamingChangeEvent, NamingEventListener, NamingService, NamingServiceBuilder, ServiceInstance,
};
use nacos_sdk::api::props::ClientProps;

pub struct MyNamingEventListener;

impl NamingEventListener for MyNamingEventListener {
    fn event(&self, event: Arc<NamingChangeEvent>) {
        tracing::info!("subscriber notify: {:?}", event);
    }
}

static CLIENT_PROPS: LazyLock<ClientProps> = LazyLock::new(|| {
    ClientProps::new()
        .server_addr(constants::DEFAULT_SERVER_ADDR)
        .namespace("")
        .app_name("lazy_app")
        .auth_username("admin")
        .auth_password("admin")
});
static NAMING_SERVICE: LazyLock<Box<dyn NamingService>> = LazyLock::new(|| {
    let naming_service = NamingServiceBuilder::new(CLIENT_PROPS.clone())
        .enable_auth_plugin_http()
        .build()
        .unwrap();
    Box::new(naming_service)
});
#[auto_config(WebConfigurator)]   // 自动扫描web router
#[tokio::main]
async fn main() -> Result<()> {
    let listener = std::sync::Arc::new(MyNamingEventListener);
    let _subscribe_ret = NAMING_SERVICE
        .subscribe(
            "spring-rs-service".to_string(),
            Some(constants::DEFAULT_GROUP.to_string()),
            Vec::default(),
            listener,
        ).await;

    let service_instance1 = ServiceInstance {
        ip: "127.0.0.1".to_string(),
        port: 8123,
        ..Default::default()
    };

    let _register_instance_ret = NAMING_SERVICE
        .batch_register_instance(
            "spring-rs-service".to_string(),
            Some(constants::DEFAULT_GROUP.to_string()),
            vec![service_instance1],
        ).await;

    App::new()
        .add_plugin(SqlxPlugin)  // 添加插件
        .add_plugin(WebPlugin)
        .run()
        .await;
    Ok(())
}

// Component可以抽取由Sqlx插件在AppState中的注册的连接池
#[get("/vvv")]
async fn sqlx_request_handler(Component(pool): Component<ConnectPool>) -> Result<String> {
    let version = sqlx::query("SELECT email FROM users")
        .fetch_one(&pool)
        .await
        .context("sqlx query failed")?
        .get("email");
    println!("version: {}", version);
    Ok(version)
}
