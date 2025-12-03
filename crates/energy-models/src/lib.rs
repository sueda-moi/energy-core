use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

// --------------------------------------------------------
// 1. 传感器数据点 (对应 sensor_data 表)
// --------------------------------------------------------
// Derive 宏的解释：
// - Debug: 允许用 println! 打印它
// - Serialize/Deserialize: 允许转成 JSON
// - FromRow: 允许 sqlx 直接把 SQL 查询结果变成这个 struct
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SensorData {
    pub time: DateTime<Utc>,
    pub sensor_id: String,
    // 在 SQL 里 value 是 DOUBLE PRECISION，但在 Rust 里可能是 NULL，所以用 Option
    pub value: Option<f64>,
    pub site_id: Option<String>,
}

// --------------------------------------------------------
// 2. 预测数据点 (对应 forecast_data 表)
// --------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ForecastData {
    pub created_at: Option<DateTime<Utc>>, // SQL 里有 DEFAULT NOW()，所以读取时可能有值
    pub target_time: DateTime<Utc>,
    pub model_type: String,
    pub sensor_id: String,
    pub value: Option<f64>,
}