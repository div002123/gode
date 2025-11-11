# app-server-protocol

## 文件在整体的作用

`codex-app-server-protocol` 定义应用程序与 Codex 服务器之间的 JSON-RPC 通信协议。它是客户端（CLI、Web UI 等）与 Codex AI 助手服务器通信的标准接口，支持会话初始化、对话管理等核心功能。

## 主要结构体

### 协议版本 V1 (protocol/v1.rs)

#### `InitializeParams`
- 初始化请求参数
- 包含客户端信息和配置

#### `InitializeResponse`
- 服务器能力和配置响应

#### `NewConversationParams`
- 创建新对话的参数
- 字段：工作目录、模型配置、审批策略等

#### `NewConversationResponse`
- 新对话创建后的响应
- 包含：`conversation_id`, `session_configured` 事件

#### `ResumeConversationResponse`
- 恢复已有对话的响应

### 协议版本 V2 (protocol/v2.rs)
- V2 版本的协议扩展

### 公共类型 (protocol/common.rs)

#### `AuthMode`
```rust
pub enum AuthMode {
    None,
    ApiKey,
    OAuth,
}
```
- 认证模式枚举

## 主要函数

### 类型导出 (export.rs)

#### `generate_json() -> Result<String>`
- 生成 JSON Schema 定义
- 用于文档生成和验证

#### `generate_ts() -> Result<String>`
- 生成 TypeScript 类型定义
- 用于前端开发

#### `generate_types()`
- 综合类型导出工具

### JSON-RPC Lite (jsonrpc_lite.rs)
- 轻量级 JSON-RPC 2.0 实现
- 请求/响应/错误消息封装

## 数据流

```
客户端
  ↓ InitializeParams
服务器
  ↓ InitializeResponse
客户端
  ↓ NewConversationParams
服务器
  ↓ NewConversationResponse (包含 conversation_id)
客户端
  ↓ 后续请求（使用 conversation_id）
```

## 依赖关系

- `codex-protocol`: 核心协议类型
- `mcp-types`: MCP 协议支持
- `serde/serde_json`: 序列化
- `ts-rs`: TypeScript 生成
- `schemars`: JSON Schema 生成

## 在项目中的位置

```
┌──────────────────────┐
│   客户端 (CLI/Web)    │
└──────────┬───────────┘
           │ JSON-RPC
┌──────────▼───────────┐
│ app-server-protocol  │ ← 此 crate
└──────────┬───────────┘
           │
┌──────────▼───────────┐
│   Codex 服务器核心    │
└──────────────────────┘
```
