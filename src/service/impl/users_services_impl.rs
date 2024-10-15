use crate::config::mysql::RB;
use crate::config::redis::redis_read;
use crate::config::redis::redis_write;
use crate::entity_patch::login_credentials::LoginCredentials;
use crate::pojo::token::Token;
use crate::pojo::users::Users;
use crate::res::result::ResponseData;
use crate::service::users_service::UsersService;
use redis::RedisError;
use serde::Deserialize;
use serde::Serialize;
use spring_web::axum::response::IntoResponse;
use spring_web::axum::Json;

// 确保这个结构体实现了 Serialize 特性
#[derive(Serialize, Default, Deserialize)]
struct UserData {
    username: String,
    roles: Vec<String>,
}
pub struct UsersServicesImpl;
impl UsersService for UsersServicesImpl {
    async fn login(
        Json(credentials): Json<LoginCredentials>,
    ) -> spring_web::error::Result<impl IntoResponse> {
        let LoginCredentials { username, password } = credentials;
        //必须把模块附加到main.rs 否则无法使用
        let data = Users::login(&RB.clone(), username.to_string(), password.to_string())
            .await
            .unwrap();

        if data.is_none() {
            return Ok(ResponseData::error("用户名密码错误"));
        }

        let rs = redis_write(
            "now_user_role",
            &<Option<Users> as Clone>::clone(&data).unwrap()._type,
        )
        .await;

        if let Err(e) = rs {
            println!("! {:?}", e);
            return Ok(ResponseData::error("Redis连接错误"));
        }

        let rs = redis_write(
            "now_user_name",
            &<Option<Users> as Clone>::clone(&data).unwrap().username,
        )
        .await;

        if let Err(e) = rs {
            println!("! {:?}", e);
            return Ok(ResponseData::error("Redis连接错误"));
        }
        let now_token = Token {
            token: format!(
                "token-{}",
                &<Option<Users> as Clone>::clone(&data).unwrap()._type
            ),
        };
        Ok(ResponseData::success(now_token, "登录成功"))
    }

    async fn info() -> spring_web::error::Result<impl IntoResponse> {
        let roles = redis_read("now_user_role").await.unwrap_or_default();
        let roles = vec![roles];
        let username = redis_read("now_user_name").await.unwrap_or_default();
        let rs_data = UserData {
            username,
            roles: roles.clone(),
        };
        Ok(ResponseData::success(
            rs_data,
            &("获取".to_owned() + &roles[1] + "类型用户成功"),
        ))
    }
}
