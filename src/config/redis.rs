use redis::{Commands, RedisError};

pub async fn redis_write(key: &str, value: &str) -> Result<(), RedisError> {
    // connect to redis
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    // throw away the result, just make sure it does not fail
    let _: () = con.set(key, value)?;
    Ok(())
}


pub async fn redis_read(key: &str) -> Result<String, RedisError> {
    // connect to redis
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    // throw away the result, just make sure it does not fail
    let rs = con.get(key)?;
    Ok(rs)
}
