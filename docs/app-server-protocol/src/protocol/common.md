# app-server-protocol/src/protocol_message.rs

## 文件作用

提供协议消息的解析和序列化功能，支持 JSON-RPC 消息与协议类型之间的转换。

## 主要枚举

### `pub enum ProtocolMessage`
- 协议消息统一类型
- 变体：
  - `ClientRequest`: 客户端请求
  - `ClientNotification`: 客户端通知
  - `ServerNotification`: 服务端通知

## 主要函数和方法

### `pub fn from_json(value: Value) -> Result<Self>`
从 JSON 值解析协议消息

### `pub fn to_json(&self) -> Result<Value>`
将协议消息序列化为 JSON 值

### `pub fn parse_method(method: &str) -> ProtocolMessageType`
根据方法名解析消息类型
