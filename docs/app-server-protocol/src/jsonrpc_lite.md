# app-server-protocol/src/jsonrpc_lite.rs

## 文件作用

实现轻量级 JSON-RPC 2.0 协议处理，支持请求、响应、通知和错误的序列化/反序列化。

## 主要结构体

### `pub struct JsonRpcRequest`
- JSON-RPC 请求消息
- 字段：`jsonrpc`, `id`, `method`, `params`

### `pub struct JsonRpcResponse`
- JSON-RPC 响应消息
- 字段：`jsonrpc`, `id`, `result`, `error`

### `pub struct JsonRpcNotification`
- JSON-RPC 通知消息（无需响应）
- 字段：`jsonrpc`, `method`, `params`

### `pub struct JsonRpcError`
- JSON-RPC 错误对象
- 字段：`code`, `message`, `data`

### `pub enum JsonRpcMessage`
- JSON-RPC 消息类型枚举
- 变体：Request, Response, Notification

## 主要函数和方法

### `pub fn from_value(value: Value) -> Result<Self>`
从 JSON 值解析 JsonRpcMessage

### `pub fn to_value(&self) -> Result<Value>`
将 JsonRpcMessage 序列化为 JSON 值
