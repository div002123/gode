# protocol/src/items.rs

## 文件作用

定义会话轮次中的项（Item）类型，包括用户消息、代理消息、推理和 Web 搜索。

## 主要结构体

### `pub struct UserMessageItem`
- 用户消息项
- 字段：`id`, `content` (Vec<UserInput>)

### `pub struct AgentMessageItem`
- 代理消息项
- 字段：`id`, `content` (Vec<AgentMessageContent>)

### `pub struct ReasoningItem`
- 推理项
- 字段：`id`, `summary_text`, `raw_content`

### `pub struct WebSearchItem`
- Web 搜索项
- 字段：`id`, `query`

## 主要枚举

### `pub enum TurnItem`
- 轮次项类型
- 变体：UserMessage, AgentMessage, Reasoning, WebSearch

### `pub enum AgentMessageContent`
- 代理消息内容
- 变体：Text { text }

## 主要函数和方法

### `impl UserMessageItem`
- `pub fn new(content: &[UserInput]) -> Self`
- `pub fn message(&self) -> String`
- `pub fn image_urls(&self) -> Vec<String>`

### `impl TurnItem`
- `pub fn as_legacy_events(&self, show_raw_agent_reasoning: bool) -> Vec<EventMsg>`
