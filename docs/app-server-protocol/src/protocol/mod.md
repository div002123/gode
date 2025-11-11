# app-server-protocol/src/protocol.rs

## 文件作用

定义客户端和服务端之间的协议消息类型，包括请求、通知和响应。

## 主要枚举

### `pub enum ClientRequest`
- 客户端请求消息
- 变体：Initialize (初始化请求)

### `pub enum ClientNotification`
- 客户端通知消息
- 变体：Initialized (初始化完成通知)

### `pub enum ServerNotification`
- 服务端通知消息
- 变体：Noop (空操作)

## 主要结构体

### `pub struct ClientCapabilities`
- 客户端能力声明
- 字段：
  - `incremental_patch`: Option<IncrementalPatchCapability>
  - `file_encoding`: Option<FileEncodingCapability>

## 主要函数和方法

无公共函数。该文件主要定义协议消息类型。
