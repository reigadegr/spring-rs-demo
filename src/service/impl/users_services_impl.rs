use crate::config::init::RB;
use crate::pojo::users::Users;
use crate::res::result::ResponseData;
use crate::services::users_service::UsersService;
use salvo::prelude::*;
use salvo::{Request, Response};
use serde::{Deserialize, Serialize};
use crate::config::redis::{redis_read, redis_write};
use crate::pojo::token::Token;

#[derive(Debug, Serialize, Deserialize, Extractible, Clone)]
struct UserInfo {
    username: String,
    password: String,
}
pub struct UsersServicesImpl;
impl UsersService for UsersServicesImpl {
    async fn login(req: &mut Request, res: &mut Response) {
        //示例：http://127.0.0.1:5800/login/?username=admin&password=123456
        println!("{:?}", req);
        let username = req.query::<&str>("username").unwrap();
        let password = req.query::<&str>("password").unwrap();
        let data = Users::login(&RB.clone(), username.to_string(), password.to_string())
            .await
            .unwrap();
        if data.is_none() {
            let data: ResponseData<()> = ResponseData::error("用户名或密码错误");
            println!("{:?}", data);
            return res.render(serde_json::to_string(&data).unwrap());
        }
        let data = ResponseData::success(data, "登录成功");
        println!("{:?}", data);
        res.render(serde_json::to_string(&data).unwrap())
    }
    async fn login_post(req: &mut Request, res: &mut Response) {
        //示例：http://127.0.0.1:5800/login/?username=admin&password=123456
        println!("这是login_post。请求体：{:?}", req);
        let user_info = req.parse_json::<UserInfo>().await;
        println!("{:?}", &user_info);
        let user_info = user_info.unwrap();
        let username = user_info.username;
        let password = user_info.password;
        let data = Box::new(Users::login(&RB.clone(), username.to_string(), password.to_string())
            .await
            .unwrap());

        if data.is_none() {
            let data: ResponseData<()> = ResponseData::error("用户名或密码错误");
            return res.render(serde_json::to_string(&data).unwrap());
        }

        let rs = redis_write("now_user_role", &*<Option<Users> as Clone>::clone(&data).unwrap()._type).await;

        if let Err(e) = rs {
            println!("! {:?}", e);
            let data: ResponseData<()> = ResponseData::error("Redis连接错误");
            return res.render(serde_json::to_string(&data).unwrap());
        }

        let rs = redis_write("now_user_name", &*<Option<Users> as Clone>::clone(&data).unwrap().username).await;

        if let Err(e) = rs {
            println!("! {:?}", e);
            let data: ResponseData<()> = ResponseData::error("Redis连接错误");
            return res.render(serde_json::to_string(&data).unwrap());
        }
        let now_token = Token {
            token: format!("token-{}", &*<Option<Users> as Clone>::clone(&data).unwrap()._type),
        };
        let data = ResponseData::success(now_token, "登录成功qq");

        println!("{:?}", &data);
        println!("登陆成功");
        res.render(serde_json::to_string(&data).unwrap());
        return;
    }

    async fn users_info(req: &mut Request, res: &mut Response) {
        let roles = redis_read("now_user_role");
        println!("----------------");
        match roles.await {
            Ok(data) => {
                println!("{:?}", data);
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
        println!("----------------");
        let username = redis_read("now_user_name");
    }

    async fn get_list(req: &mut Request, res: &mut Response) {
        println!("{:?}", req);
        let data = Users::gl(&mut RB.clone(), "admin".to_string())
            .await
            .unwrap();
        let data = ResponseData::success(data, "查询管理员成功");
        res.render(serde_json::to_string(&data).unwrap())
    }
}
