# protocol/src/models.rs

## 文件作用

定义与模型交互相关的数据类型，包括响应项、内容项、工具调用等。

## 主要结构体

### `pub struct FunctionCallOutputPayload`
- 函数调用输出负载
- 字段：`content`, `content_items`, `success`

### `pub struct ShellToolCallParams`
- Shell 工具调用参数
- 字段：`command`, `workdir`, `timeout_ms`

### `pub struct LocalShellExecAction`
- 本地 Shell 执行动作
- 字段：`command`, `timeout_ms`, `working_directory`, `env`, `user`

## 主要枚举

### `pub enum ResponseItem`
- 响应项类型
- 变体：
  - `Message { id, role, content }`: 消息
  - `Reasoning { id, summary, content }`: 推理
  - `LocalShellCall { id, call_id, status, action }`: Shell 调用
  - `FunctionCall { id, name, arguments, call_id }`: 函数调用
  - `WebSearchCall { id, status, action }`: Web 搜索
  - 等

### `pub enum ContentItem`
- 内容项类型
- 变体：InputText, InputImage, OutputText

### `pub enum LocalShellStatus`
- Shell 状态
- 变体：Completed, InProgress, Incomplete

### `pub enum FunctionCallOutputContentItem`
- 函数调用输出内容项
- 变体：InputText, InputImage

## 主要函数和方法

### `impl From<&CallToolResult> for FunctionCallOutputPayload`
从 MCP CallToolResult 转换为输出负载
