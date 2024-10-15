use serde::{Deserialize, Serialize};
use spring_web::axum;
use spring_web::axum::http::StatusCode;
use spring_web::axum::response::IntoResponse;

// 定义响应数据结构体
#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseData<T> {
    pub code: i32,
    pub message: String,
    pub data: T,
}

// 定义统一响应代码
const SUCCESS_CODE: i32 = 0;
const ERROR_CODE: i32 = -1;

// 统一响应结构体的实现
impl<T: Default> ResponseData<T> {
    pub fn success(data: T, message: &str) -> Self {
        ResponseData {
            code: SUCCESS_CODE,
            message: message.to_string(),
            data,
        }
    }

    pub fn error(message: &str) -> Self {
        ResponseData {
            code: ERROR_CODE,
            message: message.to_string(),
            data: Default::default(),
        }
    }
}
