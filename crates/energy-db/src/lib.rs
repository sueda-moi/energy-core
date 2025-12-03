use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;
use thiserror::Error;

// 定义错误类型
#[derive(Error, Debug)]
pub enum DbError {
    #[error("Failed to connect to database")]
    ConnectionError(#[from] sqlx::Error),
}

// 这是一个 helper 函数，用来初始化连接池
pub async fn init_pool(database_url: &str) -> Result<PgPool, DbError> {
    let pool = PgPoolOptions::new()
        .max_connections(5) // 连接池最大连接数，t2.micro 不要设太大
        .acquire_timeout(Duration::from_secs(3)) // 3秒连不上就报错
        .connect(database_url)
        .await?;

    Ok(pool)
}