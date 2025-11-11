# otel/src/otel_event_manager.rs

## 文件作用

OpenTelemetry 事件管理器，记录和追踪 Codex 会话的各种事件（API 请求、SSE 事件、工具调用等）。

## 主要结构体

### `pub struct OtelEventManager`
- OTLP 事件管理器
- 封装了会话元数据（conversation_id, model, account_id 等）

### `pub struct OtelEventMetadata`
- 事件元数据
- 字段：conversation_id, auth_mode, account_id, model, slug 等

## 主要枚举

### `pub enum ToolDecisionSource`
- 工具决策来源
- 变体：Config, User

## 主要函数和方法

### `pub fn new(...) -> OtelEventManager`
创建事件管理器

### `pub fn conversation_starts(...)`
记录会话开始事件

### `pub async fn log_request(...) -> Result<Response, Error>`
记录 API 请求事件

### `pub fn log_sse_event(...)`
记录 SSE 事件

### `pub async fn log_tool_result(...) -> Result<(String, bool), E>`
记录工具调用结果

### `pub fn tool_decision(...)`
记录工具决策事件
