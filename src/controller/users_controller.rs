use crate::entity_patch::login_credentials::LoginCredentials;
use crate::service::r#impl::users_services_impl::UsersServicesImpl;
use crate::service::users_service::UsersService;
use spring_web::axum::response::IntoResponse;
use spring_web::extractor::Json;
use spring_web::{error::Result, route};

#[route("/users/login", method = "POST")]
async fn login(Json(credentials): Json<LoginCredentials>) -> Result<impl IntoResponse> {
    <UsersServicesImpl as UsersService>::login(Json(credentials)).await
}

#[route("/users/info", method = "GET")]
async fn info() -> Result<impl IntoResponse> {
    <UsersServicesImpl as UsersService>::info().await
}
