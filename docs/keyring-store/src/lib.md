# keyring-store/src/lib.rs

## 文件作用

提供跨平台的凭证存储抽象，使用操作系统原生的密钥环服务（macOS Keychain、Windows Credential Manager、Linux Secret Service）。

## 主要结构体

### `pub struct DefaultKeyringStore`
- 默认密钥环存储实现

## 主要 Trait

### `pub trait KeyringStore`
- 密钥环存储接口
- 方法：
  - `load(service: &str, key: &str) -> Result<Option<String>>`
  - `save(service: &str, key: &str, value: &str) -> Result<()>`
  - `delete(service: &str, key: &str) -> Result<bool>`

## 主要函数和方法

### `impl KeyringStore for DefaultKeyringStore`
为 DefaultKeyringStore 实现密钥环存储接口
