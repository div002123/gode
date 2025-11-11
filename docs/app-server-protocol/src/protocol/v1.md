# app-server-protocol/src/protocol_upgrade.rs

## 文件作用

处理协议版本升级，确保向后兼容性，特别是处理增量补丁相关的协议升级。

## 主要结构体

无公共结构体。

## 主要函数和方法

### `pub fn upgrade_initialize_request(req: &mut ExportInitializeRequest)`
升级初始化请求以支持增量补丁功能

### `pub fn needs_incremental_patch_upgrade(caps: &ClientCapabilities) -> bool`
检查客户端能力是否需要增量补丁升级

### `pub fn apply_incremental_patch_defaults(caps: &mut ClientCapabilities)`
为客户端能力应用增量补丁的默认值
