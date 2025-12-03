use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use std::time::Duration;
use tokio::time::sleep;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;
use tracing::{info, error, warn};

// 定义一个结构体来管理连接
pub struct HaClient {
    base_url: String,
    token: String,
}

impl HaClient {
    pub fn new(url: String, token: String) -> Self {
        Self { base_url: url, token }
    }

    // 启动监听的主循环
    pub async fn start_listening(&self) {
        let ws_url = format!("{}/api/websocket", self.base_url.replace("http", "ws"));
        
        // 自动重连机制
        loop {
            info!("🔌 Connecting to Home Assistant at {}...", ws_url);
            
            match connect_async(Url::parse(&ws_url).unwrap()).await {
                Ok((ws_stream, _)) => {
                    info!("✅ Connected via WebSocket!");
                    let (mut write, mut read) = ws_stream.split();

                    // 1. 认证阶段 (Auth)
                    // HA 连上后会发个 "auth_required"，要回发 token
                    let auth_msg = json!({
                        "type": "auth",
                        "access_token": self.token
                    });
                    
                    if let Err(e) = write.send(Message::Text(auth_msg.to_string())).await {
                        error!("Failed to send auth: {:?}", e);
                        continue;
                    }

                    // 2. 消息循环
                    while let Some(msg) = read.next().await {
                        match msg {
                            Ok(Message::Text(text)) => {
                                // 解析收到的 JSON
                                let data: serde_json::Value = serde_json::from_str(&text).unwrap_or_default();
                                
                                // 如果认证成功，就开始订阅事件
                                if data["type"] == "auth_ok" {
                                    info!("🔓 Auth successful! Subscribing to events...");
                                    let sub_msg = json!({
                                        "id": 1,
                                        "type": "subscribe_events",
                                        "event_type": "state_changed"
                                    });
                                    write.send(Message::Text(sub_msg.to_string())).await.unwrap();
                                }
                                
                                // 打印具体的事件 (这就是要的数据！)
                                if data["type"] == "event" {
                                    if let Some(event) = data.get("event") {
                                        // 咱们只打印 entity_id 看看效果
                                        let entity_id = event["data"]["entity_id"].as_str().unwrap_or("unknown");
                                        let new_state = event["data"]["new_state"]["state"].as_str().unwrap_or("unknown");
                                        info!("📡 Event: {} -> {}", entity_id, new_state);
                                    }
                                }
                            }
                            Err(e) => {
                                error!("WebSocket error: {:?}", e);
                                break; 
                            }
                            _ => {}
                        }
                    }
                },
                Err(e) => {
                    error!("❌ Connection failed: {:?}. Retrying in 5s...", e);
                }
            }
            
            // 如果断开了，等5秒再重连
            sleep(Duration::from_secs(5)).await;
        }
    }
}