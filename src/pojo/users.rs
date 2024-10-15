use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Users {
    pub id: i64,
    #[serde(rename = "type")]
    pub _type: String,
    pub username: String,
    pub password: String,
    pub phone: String,
    pub email: String,
    pub status: String,
    pub create_time: String,
    pub update_time: String,
}
