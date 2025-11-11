# windows-sandbox-rs/src/token.rs

## 文件作用

Windows 安全令牌操作，创建受限令牌用于沙盒执行。

## 主要函数和方法

### `pub unsafe fn world_sid() -> Result<Vec<u8>>`
获取 Everyone (World) SID

### `pub unsafe fn convert_string_sid_to_sid(s: &str) -> Option<*mut c_void>`
将 SID 字符串转换为 SID 指针

### `pub unsafe fn get_current_token_for_restriction() -> Result<HANDLE>`
获取当前进程令牌用于创建受限令牌

### `pub unsafe fn get_logon_sid_bytes(h_token: HANDLE) -> Result<Vec<u8>>`
从令牌获取 Logon SID 字节

### `pub unsafe fn create_workspace_write_token_with_cap(...) -> Result<(HANDLE, *mut c_void)>`
创建工作区可写受限令牌
- 使用 CreateRestrictedToken 创建
- 设置标志：DISABLE_MAX_PRIVILEGE | LUA_TOKEN | WRITE_RESTRICTED
- 受限 SID 集：Capability SID, Logon SID, Everyone
- 启用 SeChangeNotifyPrivilege 特权

### `pub unsafe fn create_readonly_token_with_cap(...) -> Result<(HANDLE, *mut c_void)>`
创建只读受限令牌
- 配置与 create_workspace_write_token_with_cap 相同
- 区别在于使用不同的 Capability SID

### `unsafe fn enable_single_privilege(h_token: HANDLE, name: &str) -> Result<()>`
为令牌启用单个特权

## 主要常量

- `DISABLE_MAX_PRIVILEGE`: 0x01
- `LUA_TOKEN`: 0x04
- `WRITE_RESTRICTED`: 0x08
- `SE_GROUP_LOGON_ID`: 0xC0000000
