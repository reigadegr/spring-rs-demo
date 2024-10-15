use crate::pojo::users::Users;
use rbatis::impl_select;
// 带有->Option为返回单个元素，删除则返回vec数组
impl_select!(Users{login(username:String,password:String) -> Option => "`where username = #{username} AND password = #{password} limit 1`"});

impl_select!(Users{gl(tp:String)  => "`where type = #{tp}`"});
