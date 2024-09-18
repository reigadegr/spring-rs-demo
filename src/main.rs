use std::sync::Arc;
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
use nacos_sdk::api::config::{
    ConfigChangeListener, ConfigResponse, ConfigService, ConfigServiceBuilder,
};
use nacos_sdk::api::constants;
use nacos_sdk::api::naming::{
    NamingChangeEvent, NamingEventListener, NamingService, NamingServiceBuilder, ServiceInstance,
};
use nacos_sdk::api::props::ClientProps;

struct MyConfigChangeListener;

impl ConfigChangeListener for MyConfigChangeListener {
    fn notify(&self, config_resp: ConfigResponse) {
        tracing::info!("listen the config={}", config_resp);
    }
}

pub struct MyNamingEventListener;

impl NamingEventListener for MyNamingEventListener {
    fn event(&self, event: Arc<NamingChangeEvent>) {
        tracing::info!("subscriber notify: {:?}", event);
    }
}
#[auto_config(WebConfigurator)]   // 自动扫描web router
#[tokio::main]
async fn main() -> Result<()> {
    // tracing_subscriber::fmt()
    //     // all spans/events with a level higher than TRACE (e.g, info, warn, etc.)
    //     // will be written to stdout.
    //     .with_max_level(tracing::Level::INFO)
    //     .with_thread_names(true)
    //     .with_thread_ids(true)
    //     // sets this to be the default, global collector for this application.
    //     .init();
    let client_props = ClientProps::new()
        .server_addr("127.0.0.1:8848")
        .namespace("")
        .auth_username("admin")
        .auth_password("admin");

    let naming_service = NamingServiceBuilder::new(client_props).enable_auth_plugin_http().build();
    let listener = std::sync::Arc::new(MyNamingEventListener);
    let _subscribe_ret = naming_service.expect("REASON")
        .subscribe(
            "test-service".to_string(),
            Some(constants::DEFAULT_GROUP.to_string()),
            Vec::default(),
            listener,
        ).await;
    let service_instance1 = ServiceInstance {
        ip: "127.0.0.1".to_string(),
        port: 9090,
        ..Default::default()
    };

    let client_props = ClientProps::new()
        .server_addr("127.0.0.1:8848")
        .namespace("")
        .auth_username("admin")
        .auth_password("admin");
    let naming_service = NamingServiceBuilder::new(client_props).enable_auth_plugin_http().build();

    let _register_instance_ret = naming_service.expect("REASON")
        .batch_register_instance(
            "test-service".to_string(),
            Some(constants::DEFAULT_GROUP.to_string()),
            vec![service_instance1],
        )
        .await;
    App::new()
        .add_plugin(SqlxPlugin)  // 添加插件
        .add_plugin(WebPlugin)
        .run()
        .await;
    Ok(())
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
