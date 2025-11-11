# rmcp-client/src/oauth.rs

## 文件作用

管理 MCP OAuth 凭证，使用 keyring crate 存储（支持 macOS Keychain、Windows Credential Manager、Linux Secret Service），失败时回退到 `CODEX_HOME/.credentials.json` 文件。

## 主要结构体

### `pub struct StoredOAuthTokens`
- 存储的 OAuth 令牌
- 字段：
  - `server_name`: String
  - `url`: String
  - `client_id`: String
  - `token_response`: WrappedOAuthTokenResponse

### `pub struct WrappedOAuthTokenResponse`
- OAuth Token 响应包装器
- 包装 `OAuthTokenResponse` 并实现 PartialEq

### `pub(crate) struct OAuthPersistor`
- OAuth 凭证持久化管理器
- 用于自动保存和删除凭证

## 主要枚举

### `pub enum OAuthCredentialsStoreMode`
- 凭证存储模式
- 变体：
  - `Auto` (默认): 优先 Keyring，失败时回退到 File
  - `File`: `CODEX_HOME/.credentials.json`
  - `Keyring`: 仅 Keyring，失败则报错

## 主要函数和方法

### `pub(crate) fn load_oauth_tokens(...) -> Result<Option<StoredOAuthTokens>>`
加载 OAuth 令牌

### `pub fn save_oauth_tokens(...) -> Result<()>`
保存 OAuth 令牌

### `pub fn delete_oauth_tokens(...) -> Result<bool>`
删除 OAuth 令牌

### `impl OAuthPersistor`
- `pub(crate) async fn persist_if_needed(&self) -> Result<()>`: 按需持久化凭证
