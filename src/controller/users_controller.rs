use crate::res::result::ResponseData;
use serde::{Deserialize, Serialize};
use spring_web::axum::http::StatusCode;
use spring_web::extractor::Json;
use spring_web::{axum, error::Result, route};
use spring_web::axum::response::IntoResponse;

#[derive(Deserialize)]
struct LoginCredentials {
    username: String,
    password: String,
}


#[route("/users/login", method = "POST")]
async fn login(Json(credentials): Json<LoginCredentials>) -> Result<impl IntoResponse> {
    let LoginCredentials { username, password } = credentials;
    Ok(ResponseData::success("aaa", "bbb"))
}
