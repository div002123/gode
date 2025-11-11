# rmcp-client/src/utils.rs

## 文件作用

提供工具函数集合，包括超时执行、类型转换、环境变量处理和 HTTP 头部构建。

## 主要函数和方法

### `pub(crate) async fn run_with_timeout<F, T>(...) -> Result<T>`
执行 Future 并设置超时

### `pub(crate) fn convert_call_tool_result(...) -> Result<CallToolResult>`
转换 RMCP CallToolResult 为 MCP CallToolResult，确保 content 字段存在

### `pub(crate) fn convert_to_rmcp<T, U>(value: T) -> Result<U>`
从 mcp-types 转换为 Rust SDK 类型

### `pub(crate) fn convert_to_mcp<T, U>(value: T) -> Result<U>`
从 Rust SDK 类型转换为 mcp-types

### `pub(crate) fn create_env_for_mcp_server(...) -> HashMap<String, String>`
为 MCP 服务器创建环境变量映射，包含默认和额外的环境变量

### `pub(crate) fn build_default_headers(...) -> Result<HeaderMap>`
构建默认 HTTP 头部，支持静态头部和从环境变量读取的头部

### `pub(crate) fn apply_default_headers(...) -> ClientBuilder`
将默认头部应用到 HTTP 客户端构建器

## 主要常量

### `pub(crate) const DEFAULT_ENV_VARS`
默认环境变量列表（Unix 和 Windows 各有不同）
- Unix: HOME, PATH, SHELL, USER, LANG, TERM 等
- Windows: PATH, SYSTEMROOT, USERNAME, PROGRAMFILES, APPDATA, TEMP 等
