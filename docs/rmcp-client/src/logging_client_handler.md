# rmcp-client/src/logging_client_handler.rs

## 文件作用

实现 MCP 客户端处理器，记录来自 MCP 服务器的通知和请求。

## 主要结构体

### `pub(crate) struct LoggingClientHandler`
- 日志客户端处理器
- 字段：`client_info` (ClientInfo)

## 主要函数和方法

### `pub(crate) fn new(client_info: ClientInfo) -> Self`
创建新的日志客户端处理器

### `impl ClientHandler for LoggingClientHandler`
实现 rmcp ClientHandler trait
- `async fn create_elicitation(...)`: 处理 elicitation 请求（暂不支持）
- `async fn on_cancelled(...)`: 处理取消通知
- `async fn on_progress(...)`: 处理进度通知
- `async fn on_resource_updated(...)`: 处理资源更新通知
- `async fn on_logging_message(...)`: 处理日志消息
- 其他 MCP 通知处理方法
