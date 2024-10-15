use crate::res::result::ResponseData;
use serde::Serialize;
use spring_web::axum;
use spring_web::axum::http::StatusCode;
use spring_web::axum::response::IntoResponse;

// 为 ResponseData 实现 IntoResponse trait
impl<T: Serialize> IntoResponse for ResponseData<T> {
    fn into_response(self) -> axum::response::Response {
        // 这里我们选择将 ResponseData 序列化为 JSON 格式的响应体
        let body =
            serde_json::to_string(&self).unwrap_or_else(|_| "Internal Server Error".to_string());
        (StatusCode::OK, body).into_response()
    }
}
