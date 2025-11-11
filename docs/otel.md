# otel

## 文件在整体的作用

`codex-otel` 提供 OpenTelemetry 集成，支持可选的遥测和性能追踪功能。通过 feature gate 控制，可以在生产环境启用详细的性能监控，而在开发环境使用轻量级 stub 实现。

## Feature Gates

### `otel` feature
- 启用完整的 OpenTelemetry 支持
- 编译 `otel_provider.rs`
- 依赖：`opentelemetry`, `opentelemetry-otlp`

### 默认（无 feature）
- 使用 stub 实现
- 最小开销
- 不发送遥测数据

## 主要结构体

### `OtelSettings`
```rust
pub struct OtelSettings {
    pub enabled: bool,
    pub service_name: String,
    pub exporter: OtelExporter,
}
```
- OpenTelemetry 配置
- 控制是否启用和导出方式

### `OtelExporter`
```rust
pub enum OtelExporter {
    None,
    OtlpGrpc {
        endpoint: String,
    },
    OtlpHttp {
        endpoint: String,
        protocol: OtelHttpProtocol,
    },
}
```
- 导出器类型枚举
- 支持 gRPC 和 HTTP

### `OtelHttpProtocol`
```rust
pub enum OtelHttpProtocol {
    Binary,  // protobuf
    Json,    // JSON
}
```
- HTTP 导出协议选择

### `OtelProvider`
```rust
pub struct OtelProvider {
    // 内部字段（feature-gated）
}
```
- OpenTelemetry 提供者
- 管理追踪器和导出器

### `OtelEventManager`
```rust
pub struct OtelEventManager {
    // ...
}
```
- 事件管理器
- 记录各种操作事件

## 主要方法

### OtelProvider

#### `new(settings: OtelSettings) -> Result<Self>`
- 创建新的 OTEL 提供者
- 初始化追踪器和导出器

#### `shutdown(&self)`
- 关闭提供者
- 刷新所有待处理的遥测数据

### OtelEventManager

#### `tool_decision(tool_name, call_id, decision, source)`
- 记录工具决策事件
- 参数：工具名、调用 ID、决策、来源

#### `api_request(model, provider, session_source)`
- 记录 API 请求
- 参数：模型、提供商、会话来源

#### `tool_execution(tool_name, duration, success)`
- 记录工具执行
- 参数：工具名、耗时、是否成功

## 导出配置示例

### gRPC 导出
```rust
use codex_otel::{OtelSettings, OtelExporter};

let settings = OtelSettings {
    enabled: true,
    service_name: "codex".to_string(),
    exporter: OtelExporter::OtlpGrpc {
        endpoint: "http://localhost:4317".to_string(),
    },
};

let provider = OtelProvider::new(settings)?;
```

### HTTP (JSON) 导出
```rust
let settings = OtelSettings {
    enabled: true,
    service_name: "codex".to_string(),
    exporter: OtelExporter::OtlpHttp {
        endpoint: "http://localhost:4318".to_string(),
        protocol: OtelHttpProtocol::Json,
    },
};
```

### 禁用遥测
```rust
let settings = OtelSettings {
    enabled: false,
    service_name: "codex".to_string(),
    exporter: OtelExporter::None,
};
```

## Stub 实现（无 feature）

当未启用 `otel` feature 时：
```rust
impl OtelProvider {
    pub fn new(_settings: OtelSettings) -> Result<Self> {
        Ok(Self {})  // 空实现
    }
    pub fn shutdown(&self) {
        // 无操作
    }
}
```

## 依赖关系

### With `otel` feature
- `opentelemetry`: 核心 API
- `opentelemetry-otlp`: OTLP 导出器
- `opentelemetry-sdk`: SDK 实现
- `tonic`: gRPC 支持

### 基础依赖
- `serde`: 配置序列化
- `codex-protocol`: 协议类型

## 在项目中的位置

```
┌────────────────────────┐
│  codex-core            │
│  (业务逻辑)            │
└──────────┬─────────────┘
           │
┌──────────▼─────────────┐
│  otel                  │ ← 此 crate
│  (可选遥测)            │
└──────────┬─────────────┘
           │ (如果启用)
┌──────────▼─────────────┐
│  OTLP Collector        │
│  (Jaeger/Prometheus)   │
└────────────────────────┘
```

## 可观测性数据

### Traces（追踪）
- API 请求持续时间
- 工具执行时间
- MCP 调用延迟

### Events（事件）
- 用户批准决策
- 工具调用
- 错误和异常

### Metrics（指标）
- 请求速率
- 错误率
- Token 使用量

## 环境变量（可选）

```bash
# 启用 OTEL
OTEL_ENABLED=true
OTEL_SERVICE_NAME=codex
OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317
```

## 与 Jaeger/Prometheus 集成

Codex 可以将遥测数据发送到：
- **Jaeger**: 分布式追踪
- **Prometheus**: 指标监控
- **Grafana**: 可视化仪表板
