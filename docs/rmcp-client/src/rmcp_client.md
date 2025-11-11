# rmcp-client/src/rmcp_client.rs

## 文件作用

MCP 客户端的主要实现，基于官方 `rmcp` SDK，支持 stdio 和 HTTP 传输，包含 OAuth 认证支持。

## 主要结构体

### `pub struct RmcpClient`
- RMCP 客户端
- 内部使用状态机管理连接状态

内部枚举（非公开）：
- `ClientState`: Connecting, Ready
- `PendingTransport`: ChildProcess, StreamableHttp, StreamableHttpWithOAuth

## 主要函数和方法

### `pub async fn new_stdio_client(...) -> io::Result<Self>`
创建 stdio MCP 客户端（通过子进程通信）

### `pub async fn new_streamable_http_client(...) -> Result<Self>`
创建 streamable HTTP MCP 客户端，支持 Bearer Token 和 OAuth

### `pub async fn initialize(...) -> Result<InitializeResult>`
执行 MCP 初始化握手

### `pub async fn list_tools(...) -> Result<ListToolsResult>`
列出可用的 MCP 工具

### `pub async fn list_resources(...) -> Result<ListResourcesResult>`
列出可用的 MCP 资源

### `pub async fn read_resource(...) -> Result<ReadResourceResult>`
读取 MCP 资源内容

### `pub async fn call_tool(...) -> Result<CallToolResult>`
调用 MCP 工具

### `async fn persist_oauth_tokens(&self)`
工具调用后持久化 OAuth 令牌（如果刷新了）
