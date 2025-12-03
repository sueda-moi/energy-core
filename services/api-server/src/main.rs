use dotenvy::dotenv;
use std::env;
use energy_db::init_pool;
use energy_ha::HaClient; // 引入新写的 Client
use tracing::{info, error};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenv().ok();
    
    // 1. 读取配置
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL missing");
    let ha_url = env::var("HA_URL").expect("HA_URL missing (e.g., http://localhost:8123)");
    let ha_token = env::var("HA_TOKEN").expect("HA_TOKEN missing");

    // 2. 初始化数据库 (依然保留)
    let _pool = init_pool(&database_url).await.expect("DB failed");
    info!("✅ DB Connected.");

    // 3. 启动 HA 监听 (Spawn 一个新任务去跑，不要阻塞主线程)
    let ha_client = HaClient::new(ha_url, ha_token);
    tokio::spawn(async move {
        ha_client.start_listening().await;
    });

    // 4. 阻塞主线程，防止程序退出
    // (以后这里会运行 Web Server，现在先用这一行占位)
    tokio::signal::ctrl_c().await.unwrap();
    info!("Shutdown signal received.");
}