-- Add migration script here
-- 1. 开启 TimescaleDB 插件 (这是处理时间序列的引擎)
CREATE EXTENSION IF NOT EXISTS timescaledb CASCADE;

-- 2. 创建 uuid 插件 (用于生成唯一 ID)
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- ==========================================
-- 表 1: 传感器历史数据 (Sensor Data)
-- 核心设计: 窄表模式 (Time + ID + Value)
-- ==========================================
CREATE TABLE sensor_data (
    time        TIMESTAMPTZ NOT NULL,
    sensor_id   TEXT NOT NULL,          -- 例如: 'sensor.solar_power'
    value       DOUBLE PRECISION,       -- 具体数值
    site_id     TEXT DEFAULT 'default'  -- 支持多站点 (Parents Home, My Home)
);

-- 将其转换为超表 (Hypertable)，按时间自动分片
-- chunk_time_interval: '1 day' 表示每一天的数据存一个物理文件
SELECT create_hypertable('sensor_data', 'time', chunk_time_interval => INTERVAL '1 day');

-- 创建复合索引，保证 "查某个传感器在某段时间的数据" 
CREATE INDEX ix_sensor_time ON sensor_data (sensor_id, time DESC);


-- ==========================================
-- 表 2: 预测结果数据 (Forecast Data)
-- 用于存储 EMHASS 算出来的未来计划
-- ==========================================
CREATE TABLE forecast_data (
    created_at  TIMESTAMPTZ DEFAULT NOW(), -- 这次预测是什么时候算的
    target_time TIMESTAMPTZ NOT NULL,      -- 预测的是未来的哪个时间点
    model_type  TEXT NOT NULL,             -- 例如: 'mpc' (实时纠偏) 或 'day-ahead' (日前规划)
    sensor_id   TEXT NOT NULL,             -- 例如: 'p_pv_forecast'
    value       DOUBLE PRECISION
);

-- 预测表也转为超表
SELECT create_hypertable('forecast_data', 'target_time', chunk_time_interval => INTERVAL '1 day');