use crate::entity_patch::login_credentials::LoginCredentials;
use spring_web::axum::response::IntoResponse;
use spring_web::extractor::Json;

// 定义 UserService trait 作为接口
pub trait UsersService {
    async fn login(
        credentials: Json<LoginCredentials>,
    ) -> spring_web::error::Result<impl IntoResponse>;

    async fn info() -> spring_web::error::Result<impl IntoResponse>;
}
