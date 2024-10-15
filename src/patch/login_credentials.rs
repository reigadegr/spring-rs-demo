
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginCredentials {
    pub username: String,
    pub password: String,
}
