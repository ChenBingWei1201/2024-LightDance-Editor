use redis::Client;
use sqlx::{MySql, Pool};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct AppState {
    pub mysql_pool: Arc<Pool<MySql>>,
    pub redis_client: Arc<Client>,
}
