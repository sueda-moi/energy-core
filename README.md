# Project Energy-Core (MVP v2.0)

> **目标**：构建一个基于 Rust 的高性能、可拓展的家庭能源管理后端系统，实现“数据注入”模式的能源规划。

## 1. 核心架构 (Architecture)

在这个新架构中，**Rust Backend** 成为指挥官，Home Assistant 退化为单纯的感知与执行层，EMHASS 退化为无状态的计算核心。

```mermaid
graph LR
    User[Next.js Frontend] <-->|gRPC/API| Core[**Energy-Core (Rust)**]
    
    subgraph "Core Infrastructure"
        Core <-->|SQLx| DB[(TimescaleDB)]
    end
    
    subgraph "Peripherals"
        Core <-->|WebSocket| HA[Home Assistant (Sensors/Switches)]
        Core -->|HTTP POST (Data Injection)| EMHASS[EMHASS (Optimization Engine)]
    end
```

## 2. 技术栈 (Tech Stack)

* **Language**: Rust (2021 Edition)
* **Web Framework**: Axum (基于 Tokio 的高性能框架)
* **Database**: TimescaleDB (PostgreSQL 的时序数据插件)
* **ORM-ish**: SQLx (编译时检查 SQL 的类型安全库)
* **Protocol**: WebSocket (连 HA), REST (连 EMHASS)

## 3. 工程结构 (Workspace)

采用 `Cargo Workspace` 模式以防止架构腐化：

* `crates/energy-models`: **[通用]** 定义数据结构 (Structs) 和数据库 Schema。
* `crates/energy-db`: **[数据层]** 负责所有数据库读写操作 (CRUD)。
* `crates/energy-ha`: **[连接层]** 负责处理 HA 的 WebSocket 通信。
* `services/api-server`: **[主程序]** 组合以上模块，提供 HTTP 接口和定时任务。

---

## 4. 实施路线图 (Roadmap)

我们目前处于 **Phase 1**。

### ✅ Phase 0: 基础设施 (Infrastructure)
- [x] 本地 Docker 运行 TimescaleDB。
- [x] Rust Workspace 目录结构搭建。
- [x] `.env` 环境变量配置。

###  Phase 1: 数据库与数据模型 (当前阶段)
> **目的**：建立数据持久化能力，让数据有家可归。
- [ ] **安装工具**: `cargo install sqlx-cli` 。
```
cargo install sqlx-cli --no-default-features --features native-tls,postgres --locked
```
- [ ] **表结构设计**: 编写 SQL Migration 脚本 (`sensor_data` & `forecast_data`)。
- [ ] **应用变更**: 运行 `sqlx migrate run`，在 DB 中创建表。
- [ ] **代码映射**: 在 `energy-models` 中编写对应的 Rust Struct。

###  Phase 2: 连接与采集 (Connectivity)
> **目的**：打通与 Home Assistant 的实时数据管道。
- [ ] 实现 WebSocket Client，连接本地/远程 HA。
- [ ] 监听 `state_changed` 事件。
- [ ] 将实时数据写入 `energy-db`。

###  Phase 3: 核心逻辑 (The Brain)
> **目的**：实现“数据注入”模式，解耦 EMHASS。
- [ ] 编写 SQL 查询，提取过去 N 天的历史数据。
- [ ] 封装 EMHASS API Client。
- [ ] 实现逻辑：`查询 DB -> 格式化 CSV -> POST 给 EMHASS -> 获取预测结果 -> 存回 DB`。

###  Phase 4: API 暴露 (For Frontend)
- [ ] 使用 Axum 编写 API 接口，供 Next.js 调用。
- [ ] 替换 Next.js 原有的 API Route。

---

## 5. 快速启动 (Quick Start)

**环境要求**
* Rust (Cargo)
* Docker & Docker Compose

**启动数据库**
```bash
cd energy-backend-dev
docker-compose up -d
```

**运行迁移 (初始化 DB)**
```bash
cd energy-core
sqlx migrate run
```

**运行后端**
```bash
cargo run --bin api-server
```