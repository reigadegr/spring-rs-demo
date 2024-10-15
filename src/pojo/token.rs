use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Token {
    pub token: String,
}
impl Token {
    pub fn merge_token(token: &str) -> String {
        let rs = "token-".to_owned() + &*token;
        println!("结果：{}", rs);
        rs
    }
}
