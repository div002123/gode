# rmcp-client/src/auth_status.rs

## 文件作用

确定 streamable HTTP MCP 服务器的认证状态，支持 Bearer Token 和 OAuth 两种方式。

## 主要函数和方法

### `pub async fn determine_streamable_http_auth_status(...) -> Result<McpAuthStatus>`
确定 MCP 服务器的认证状态

### `pub async fn supports_oauth_login(url: &str) -> Result<bool>`
检测服务器是否支持 OAuth 登录

### `fn discovery_paths(base_path: &str) -> Vec<String>`
实现 RFC 8414 section 3.1，生成 OAuth 发现路径

## 主要结构体

内部使用 `OAuthDiscoveryMetadata` 结构体（非公开）用于解析 OAuth 元数据。
