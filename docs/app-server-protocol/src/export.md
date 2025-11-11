# app-server-protocol/src/export.rs

## 文件作用

定义 App Server 协议导出类型，包括客户端初始化请求和响应的数据结构。

## 主要结构体

### `pub struct ExportInitializeRequest`
- 客户端初始化请求
- 字段：`capabilities` (ClientCapabilities)

### `pub struct ExportInitializeResponse`
- 服务端初始化响应
- 字段：
  - `model`: String
  - `slug`: String
  - `context_window`: Option<i64>
  - `max_output_tokens`: Option<i64>
  - `prefill_enabled`: bool

## 主要函数和方法

无公共函数。该文件仅包含数据结构定义。
