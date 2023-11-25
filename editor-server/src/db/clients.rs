use dotenv::var;
use redis::Client;
use sqlx::{MySql, MySqlPool, Pool};

pub async fn build_mysql_pool() -> Pool<MySql> {
    let mysql_host = var("DATABASE_URL").expect("DATABASE_URL is not set");

    MySqlPool::connect(mysql_host.as_str())
        .await
        .expect("Failed to create mysql pool")
}

pub async fn build_redis_client() -> Client {
    let redis_host = var("REDIS_HOST").expect("REDIS_HOST is not set");
    let redis_port = var("REDIS_PORT").expect("REDIS_PORT is not set");

    Client::open(format!("redis://{}:{}", redis_host, redis_port))
        .expect("Failed to create redis client")
}
