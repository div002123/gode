# auth.rs 文档

## 文件作用

`auth.rs` 实现了 Codex 的身份验证系统，负责管理用户认证、令牌刷新、凭证存储等功能。支持多种认证模式（OAuth、API Key），并提供跨平台的安全凭证存储。

**在整体架构中的位置**：位于客户端层，为 ModelClient 提供认证服务，确保 API 请求的安全性。

## 主要结构体

### `CodexAuth`
```rust
pub struct CodexAuth {
    pub mode: AuthMode,
    pub(crate) api_key: Option<String>,
    pub(crate) auth_dot_json: Arc<Mutex<Option<AuthDotJson>>>,
    storage: Arc<dyn AuthStorageBackend>,
    pub(crate) client: CodexHttpClient,
}
```

**职责**：
- 管理用户认证状态
- 处理令牌刷新
- 提供访问令牌
- 存储和加载凭证

**字段说明**：
- `mode`: 认证模式（OAuth、ApiKey、None）
- `api_key`: API 密钥（当使用 API Key 模式时）
- `auth_dot_json`: OAuth 令牌数据（内存缓存）
- `storage`: 凭证存储后端（keyring 或文件系统）
- `client`: HTTP 客户端（用于令牌刷新）

### `AuthManager`
```rust
pub struct AuthManager {
    codex_home: PathBuf,
    auth_credentials_store_mode: AuthCredentialsStoreMode,
    auth: Arc<Mutex<Option<CodexAuth>>>,
}
```

**职责**：
- 提供线程安全的认证访问
- 支持认证重载
- 管理认证生命周期

**字段说明**：
- `codex_home`: Codex 主目录路径
- `auth_credentials_store_mode`: 凭证存储模式
- `auth`: 共享的 CodexAuth 实例

### `RefreshTokenError`
```rust
pub enum RefreshTokenError {
    Permanent(RefreshTokenFailedError),
    Transient(std::io::Error),
}
```

**职责**：区分永久性和临时性令牌刷新错误

**变体说明**：
- `Permanent`: 永久性错误（如 refresh token 过期），需要重新登录
- `Transient`: 临时性错误（如网络问题），可以重试

## 主要方法

### CodexAuth 方法

#### `refresh_token(&self) -> Result<String, RefreshTokenError>`
刷新访问令牌。

**流程**：
1. 获取当前的 refresh token
2. 调用 OAuth 刷新端点
3. 更新存储中的令牌
4. 更新内存缓存
5. 返回新的 access token

**错误处理**：
- Refresh token 过期 → Permanent 错误
- Refresh token 被重用 → Permanent 错误
- Refresh token 被撤销 → Permanent 错误
- 网络错误 → Transient 错误

#### `get_token(&self) -> Result<String, std::io::Error>`
获取有效的访问令牌。

**流程**：
1. 检查认证模式
2. API Key 模式：直接返回 API key
3. OAuth 模式：获取 access token（必要时自动刷新）

#### `get_token_data(&self) -> Result<TokenData, std::io::Error>`
获取完整的令牌数据，包括用户信息。

**功能**：
- 自动检查令牌是否过期（8天刷新间隔）
- 自动刷新过期令牌
- 解析 ID token 提取用户信息

#### `get_account_id() -> Option<String>`
获取账户 ID。

#### `get_account_email() -> Option<String>`
获取账户邮箱。

#### `account_plan_type() -> Option<AccountPlanType>`
获取账户计划类型（Free、Pro、Team、Enterprise）。

#### `from_auth_storage(codex_home, store_mode) -> Result<Option<Self>>`
从存储加载认证信息。

**存储位置**：
- Keyring 模式：系统密钥链（macOS Keychain、Windows Credential Manager、Linux Secret Service）
- File 模式：`~/.codex/auth.json`

#### `from_api_key(api_key: &str) -> Self`
使用 API Key 创建认证实例。

### AuthManager 方法

#### `new(codex_home, store_mode) -> Self`
创建新的 AuthManager。

#### `from_auth_for_testing(auth: CodexAuth) -> Arc<Self>`
测试用构造函数。

#### `auth() -> Option<CodexAuth>`
获取当前认证实例的副本。

#### `reload() -> bool`
从存储重新加载认证信息。

**使用场景**：
- 用户在其他终端登录后同步状态
- 凭证被外部更新

#### `refresh_token() -> Result<Option<String>, RefreshTokenError>`
刷新令牌（线程安全）。

#### `logout() -> Result<bool>`
登出并清除凭证。

### 全局函数

#### `login_with_api_key(codex_home, api_key, store_mode) -> Result<()>`
使用 API Key 登录。

#### `logout(codex_home, store_mode) -> Result<bool>`
登出并删除存储的凭证。

#### `save_auth(auth_dot_json, codex_home, store_mode) -> Result<()>`
保存认证信息到存储。

#### `load_auth_dot_json(codex_home, store_mode) -> Result<Option<AuthDotJson>>`
从存储加载认证 JSON。

#### `read_openai_api_key_from_env() -> Option<String>`
从 `OPENAI_API_KEY` 环境变量读取 API Key。

#### `read_codex_api_key_from_env() -> Option<String>`
从 `CODEX_API_KEY` 环境变量读取 API Key。

## 认证流程

### OAuth 登录流程（外部完成）
```
用户在浏览器完成 OAuth 流程
  ↓
获得 tokens (access_token, refresh_token, id_token)
  ↓
save_auth() 保存到存储
  ↓
CodexAuth::from_auth_storage() 加载
```

### 令牌刷新流程
```
需要访问令牌
  ↓
get_token()
  ↓
检查令牌是否过期（8天）
  ↓ 已过期
refresh_token()
  ↓
POST https://auth.openai.com/oauth/token
  ↓
更新存储和内存缓存
  ↓
返回新的 access_token
```

### API 请求认证流程
```
ModelClient 准备请求
  ↓
auth_manager.auth().get_token()
  ↓
添加 Authorization: Bearer <token> 头
  ↓
发送请求
  ↓
401 Unauthorized？
  ↓ Yes
refresh_token() 并重试
```

## 存储后端

### Keyring 模式（推荐）
- **macOS**: Keychain
- **Windows**: Credential Manager
- **Linux**: Secret Service (libsecret)

**优点**：
- 系统级加密
- 更安全
- 与系统集成

### File 模式
- **位置**: `~/.codex/auth.json`
- **格式**: JSON
- **权限**: 0600（仅用户可读写）

**用途**：
- Keyring 不可用时的后备方案
- CI/CD 环境
- 容器环境

## 令牌数据结构

### AuthDotJson
```rust
pub struct AuthDotJson {
    pub tokens: Option<Tokens>,
    pub last_refresh: Option<DateTime<Utc>>,
}
```

### Tokens
```rust
pub struct Tokens {
    pub access_token: String,
    pub refresh_token: String,
    pub id_token: String,
}
```

### TokenData
```rust
pub struct TokenData {
    pub access_token: String,
    pub refresh_token: String,
    pub account_id: String,
    pub email: Option<String>,
    pub plan_type: InternalPlanType,
}
```

## 错误处理

### 永久性错误（需要重新登录）
- Refresh token 过期
- Refresh token 被撤销
- Refresh token 被重用（检测到安全问题）

### 临时性错误（可重试）
- 网络连接失败
- 超时
- 服务器暂时不可用

## 安全考虑

### 凭证保护
- 使用系统 keyring 加密存储
- 文件模式使用受限权限（0600）
- 内存中使用 `Mutex` 保护

### 令牌刷新
- 自动刷新避免过期
- 8天刷新间隔（保守策略）
- 60秒超时防止挂起

### 环境变量
- 支持 `OPENAI_API_KEY` 用于开发
- 支持 `CODEX_API_KEY` 作为备选
- 环境变量优先级低于存储的凭证

## 配置选项

### AuthCredentialsStoreMode
```rust
pub enum AuthCredentialsStoreMode {
    Keyring,  // 使用系统密钥链
    File,     // 使用文件存储
}
```

### ForcedLoginMethod
```rust
pub enum ForcedLoginMethod {
    OAuth,
    ApiKey,
}
```

## 测试支持

### 测试工具
- `create_dummy_chatgpt_auth_for_testing()`: 创建虚拟 OAuth 认证
- `from_auth_for_testing()`: 创建测试用 AuthManager
- Mock HTTP 客户端支持

### 串行测试
使用 `#[serial_test::serial]` 避免并发访问 keyring 冲突。

## 相关文件

- `auth/storage.rs` - 存储后端实现
- `token_data.rs` - 令牌数据解析
- `client.rs` - 使用认证的客户端
- `default_client.rs` - HTTP 客户端创建
- `error.rs` - 认证相关错误类型
