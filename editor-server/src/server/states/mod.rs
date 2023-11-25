use redis::Client;
use sqlx::{MySql, Pool};

#[derive(Clone, Debug)]
pub struct AppState {
    pub mysql_pool: Pool<MySql>,
    pub redis_client: Client,
}

impl AppState {
    pub fn new(mysql_pool: Pool<MySql>, redis_client: Client) -> Self {
        Self {
            mysql_pool,
            redis_client,
        }
    }

    pub fn mysql_pool(&self) -> &Pool<MySql> {
        &self.mysql_pool
    }

    pub fn redis_client(&self) -> &Client {
        &self.redis_client
    }
}
