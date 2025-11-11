# keyring-store

## 文件在整体的作用

`codex-keyring-store` 提供跨平台的安全凭据存储抽象，使用系统 keyring（密钥链）来存储敏感信息如 OAuth tokens。它确保凭据以加密方式存储在系统级安全存储中。

## 主要 Trait

### `KeyringStore`
```rust
pub trait KeyringStore: Send + Sync {
    fn load(&self, key: &str) -> Result<Option<String>, CredentialStoreError>;
    fn save(&self, key: &str, value: &str) -> Result<(), CredentialStoreError>;
    fn delete(&self, key: &str) -> Result<(), CredentialStoreError>;
}
```
- 凭据存储接口
- 支持加载、保存、删除操作

## 主要结构体

### `DefaultKeyringStore`
```rust
pub struct DefaultKeyringStore {
    service_name: String,
}
```
- 系统 keyring 实现
- 使用 `keyring` crate 访问系统密钥链

### `MockKeyringStore`
```rust
pub struct MockKeyringStore {
    store: Arc<Mutex<HashMap<String, String>>>,
}
```
- 内存版本，用于测试
- 不实际访问系统 keyring

### `CredentialStoreError`
```rust
pub enum CredentialStoreError {
    NotFound,
    AccessDenied,
    ParseError(String),
    Other(String),
}
```
- 凭据存储错误类型

## 主要方法

### DefaultKeyringStore

#### `new(service_name: &str) -> Self`
- 创建新的 keyring 存储实例
- `service_name`: 服务标识（如 "codex"）

#### `load(&self, key: &str) -> Result<Option<String>>`
- 从 keyring 加载凭据
- 返回 `None` 如果 key 不存在

#### `save(&self, key: &str, value: &str) -> Result<()>`
- 保存凭据到 keyring
- 覆盖已存在的值

#### `delete(&self, key: &str) -> Result<()>`
- 从 keyring 删除凭据

## 平台支持

### macOS
- 使用 **Keychain** (Apple 原生)
- 存储在用户登录钥匙串中

### Windows
- 使用 **Credential Manager** (Windows 原生)
- 存储在 Windows 凭据管理器中

### Linux
- 使用 **Secret Service** (async persistent)
- 通过 D-Bus 访问 `libsecret`
- 需要 GNOME Keyring 或 KWallet

## 使用示例

```rust
use codex_keyring_store::{DefaultKeyringStore, KeyringStore};

// 创建存储
let store = DefaultKeyringStore::new("codex");

// 保存令牌
store.save("access_token", "eyJhbGciOi...")?;

// 加载令牌
if let Some(token) = store.load("access_token")? {
    println!("找到令牌: {}", token);
}

// 删除令牌
store.delete("access_token")?;
```

## 错误处理

```rust
match store.load("key") {
    Ok(Some(value)) => println!("值: {}", value),
    Ok(None) => println!("Key 不存在"),
    Err(CredentialStoreError::AccessDenied) => {
        eprintln!("访问被拒绝");
    }
    Err(e) => eprintln!("错误: {}", e),
}
```

## 安全特性

- ✅ 系统级加密存储
- ✅ 需要用户认证（某些平台）
- ✅ 进程隔离
- ✅ 不在文件系统中明文存储

## 依赖关系

- `keyring`: 跨平台 keyring 访问
  - Feature flags: `apple-native`, `windows-native`, `linux-native-async-persistent`

## 在项目中的位置

```
┌─────────────────────┐
│  codex-core (auth)  │
│  OAuth tokens       │
└──────────┬──────────┘
           │
┌──────────▼──────────┐
│  keyring-store      │ ← 此 crate
└──────────┬──────────┘
           │
┌──────────▼──────────┐
│  系统 Keyring       │
│  (加密存储)         │
└─────────────────────┘
```

## MockKeyringStore (测试用)

```rust
use codex_keyring_store::MockKeyringStore;

let mock = MockKeyringStore::new();
mock.save("test", "value")?;
assert_eq!(mock.load("test")?, Some("value".to_string()));
```

- 不需要系统 keyring
- 用于单元测试
- 数据存储在内存中
