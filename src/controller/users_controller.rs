use crate::patch::login_credentials::LoginCredentials;
use crate::service::users_service::UsersService;
use spring_web::axum::response::IntoResponse;
use spring_web::extractor::Json;
use spring_web::{error::Result, route};
use crate::service::r#impl::users_services_impl::UsersServicesImpl;

#[route("/users/login", method = "POST")]
async fn login(Json(credentials): Json<LoginCredentials>) -> Result<impl IntoResponse> {
    <UsersServicesImpl as UsersService>::login(Json(credentials)).await
}
