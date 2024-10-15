use crate::config::mysql::RB;
use crate::config::redis::redis_write;
use crate::patch::login_credentials::LoginCredentials;
use crate::pojo::token::Token;
use crate::pojo::users::Users;
use crate::res::result::ResponseData;
use crate::service::users_service::UsersService;
use spring_web::axum::response::IntoResponse;
use spring_web::axum::Json;

pub struct UsersServicesImpl;
impl UsersService for UsersServicesImpl {
    async fn login(Json(credentials): Json<LoginCredentials>) -> spring_web::error::Result<impl IntoResponse> {
        let LoginCredentials { username, password } = credentials;
        //必须把模块附加到main.rs 否则无法使用
        let data = Users::login(&RB.clone(), username.to_string(), password.to_string())
            .await
            .unwrap();

        if data.is_none() {
            return Ok(ResponseData::error("用户名密码错误"));
        }

        let rs = redis_write("now_user_role", &*<Option<Users> as Clone>::clone(&data).unwrap()._type).await;

        if let Err(e) = rs {
            println!("! {:?}", e);
            return Ok(ResponseData::error("Redis连接错误"));
        }

        let rs = redis_write("now_user_name", &*<Option<Users> as Clone>::clone(&data).unwrap().username).await;

        if let Err(e) = rs {
            println!("! {:?}", e);
            return Ok(ResponseData::error("Redis连接错误"));
        }
        let now_token = Token {
            token: format!("token-{}", &*<Option<Users> as Clone>::clone(&data).unwrap()._type),
        };
        Ok(ResponseData::success(now_token, "登录成功"))
    }
}
