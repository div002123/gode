# protocol/src/conversation_id.rs

## 文件作用

定义会话 ID 类型，基于 UUID v7 实现，支持序列化和反序列化。

## 主要结构体

### `pub struct ConversationId`
- 会话 ID
- 内部使用 UUID v7（时间排序的 UUID）

## 主要函数和方法

### `pub fn new() -> Self`
创建新的会话 ID（使用 UUID v7）

### `pub fn from_string(s: &str) -> Result<Self, uuid::Error>`
从字符串解析会话 ID

### `impl Display for ConversationId`
实现 Display trait 用于格式化输出

### `impl Serialize/Deserialize for ConversationId`
实现 JSON 序列化/反序列化
