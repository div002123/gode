# protocol/src/message_history.rs

## 文件作用

定义消息历史条目的数据结构，用于持久化会话历史。

## 主要结构体

### `pub struct HistoryEntry`
- 历史条目
- 字段：
  - `conversation_id`: String - 会话 ID
  - `ts`: u64 - 时间戳
  - `text`: String - 文本内容

## 主要函数和方法

无公共函数。该文件主要定义数据结构。
