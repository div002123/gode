# app-server-protocol/src/lib.rs

## 文件作用

App Server 协议库的主模块，导出所有公共接口，包括 JSON-RPC、协议消息和客户端认证模式。

## 主要枚举

### `pub enum AuthMode`
- 认证模式枚举
- 变体：
  - `ApiKey`: API 密钥认证
  - `ChatGPT`: ChatGPT 会话认证

## 主要模块导出

- `jsonrpc_lite`: JSON-RPC 协议实现
- `protocol`: 协议消息定义
- `export`: 导出类型定义

## 主要函数和方法

无公共函数。该文件主要作为模块导出入口。
