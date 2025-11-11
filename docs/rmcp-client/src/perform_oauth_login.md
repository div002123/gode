# rmcp-client/src/perform_oauth_login.rs

## 文件作用

执行 MCP 服务器的 OAuth 登录流程，启动本地回调服务器接收授权码。

## 主要结构体

内部结构体（非公开）：
- `CallbackServerGuard`: 回调服务器守卫，在 drop 时自动 unblock 服务器
- `OauthCallbackResult`: OAuth 回调结果，包含 `code` 和 `state`

## 主要函数和方法

### `pub async fn perform_oauth_login(...) -> Result<()>`
执行完整的 OAuth 登录流程：
1. 启动本地 HTTP 回调服务器（127.0.0.1:0）
2. 构造授权 URL 并在浏览器中打开
3. 等待授权回调（超时 300 秒）
4. 处理授权码并获取凭证
5. 保存凭证到存储

### `fn spawn_callback_server(...)`
在后台线程中启动回调服务器，监听 OAuth 回调

### `fn parse_oauth_callback(path: &str) -> Option<OauthCallbackResult>`
解析 OAuth 回调 URL，提取 `code` 和 `state` 参数
