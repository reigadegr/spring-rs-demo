use once_cell::sync::Lazy;
use rbatis::RBatis;
use rbdc_mysql::MysqlDriver;
pub static RB: Lazy<RBatis> = Lazy::new(RBatis::new);

pub async fn init_mysql() {
    // mysql connect info
    let mysql_uri = "mysql://root:1234@127.0.0.1:3306/bs_desktop?characterEncoding=utf-8&serverTimezone=UTC&useSSL=false&allowPublicKeyRetrieval=true";
    RB.link(MysqlDriver {}, mysql_uri).await.unwrap();
}
