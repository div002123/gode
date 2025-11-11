# protocol

## 文件在整体的作用

`codex-protocol` 定义 Codex 会话协议的核心数据结构，包括消息、操作、事件等所有类型定义。它是客户端与 Codex 代理之间通信的基础协议层，采用提交-事件队列（SQ/EQ）模式。

## 核心模式：SQ/EQ

### Submission Queue (SQ)
- 用户 → Codex 的请求队列
- 类型：`Submission`

### Event Queue (EQ)
- Codex → 用户的事件流
- 类型：`Event`

```
用户  ──[Submission]──→  Codex
Codex ──[Event]──→       用户
```

## 主要结构体

### 提交相关 (protocol.rs)

#### `Submission`
```rust
pub struct Submission {
    pub id: String,
    pub op: Op,
}
```
- 用户提交的请求
- `id`: 唯一提交 ID
- `op`: 操作类型

#### `Op` (enum)
```rust
pub enum Op {
    UserTurn(UserInput),
    UserInput(UserInput),
    Interrupt,
    ReviewDecision { id: String, decision: ReviewDecision },
    AbortTurn,
    // ...更多操作
}
```
- 提交操作类型枚举

### 事件相关

#### `Event`
```rust
pub struct Event {
    pub id: String,
    pub msg: EventMsg,
}
```
- 事件包装器

#### `EventMsg` (enum)
```rust
pub enum EventMsg {
    SessionConfigured(SessionConfiguredEvent),
    AgentMessageContentDelta(AgentMessageContentDeltaEvent),
    ItemStarted(ItemStartedEvent),
    ItemCompleted(ItemCompletedEvent),
    TokenCount(TokenCountEvent),
    TurnDiff(TurnDiffEvent),
    Error(ErrorEvent),
    ExecApprovalRequest(ExecApprovalRequestEvent),
    // ...更多事件
}
```
- 事件消息类型

### 响应项 (models.rs)

#### `ResponseItem`
```rust
pub enum ResponseItem {
    Message { content: Vec<ContentItem> },
    Reasoning { thinking: String },
    ToolCall { id: String, name: String, input: Value },
    ToolResult { id: String, content: Value },
}
```
- AI 响应项

#### `ContentItem`
```rust
pub enum ContentItem {
    InputText { text: String },
    InputImage { source: ImageSource },
    OutputText { text: String },
}
```
- 内容块类型

### 配置类型 (config_types.rs)

#### `ReasoningEffort`
```rust
pub enum ReasoningEffort {
    Low,
    Medium,
    High,
}
```
- 推理努力级别

#### `ReasoningSummary`
```rust
pub enum ReasoningSummary {
    Enabled,
    Disabled,
}
```
- 推理摘要配置

### 审批相关 (approvals.rs)

#### `AskForApproval`
```rust
pub enum AskForApproval {
    Always,
    Never,
    Dangerous,
}
```
- 审批策略

#### `ReviewDecision`
```rust
pub enum ReviewDecision {
    Approved,
    Denied,
    Abort,
    ApprovedForSession,
}
```
- 审批决策

### 账户类型 (account.rs)

#### `PlanType`
```rust
pub enum PlanType {
    Free,
    Pro,
    Team,
    Enterprise,
}
```
- 账户计划类型

### 消息历史 (message_history.rs)

#### `MessageHistory`
- 对话历史记录
- 包含所有消息和元数据

### 用户输入 (user_input.rs)

#### `UserInput`
```rust
pub enum UserInput {
    Text(String),
    Image(ImageData),
    // ...
}
```
- 用户输入类型

## 主要枚举

### `TurnAbortReason`
```rust
pub enum TurnAbortReason {
    UserAbort,
    StreamError,
    ToolError,
    Timeout,
}
```
- 轮次中止原因

### `SandboxPolicy`
```rust
pub enum SandboxPolicy {
    Disabled,
    Ask,
    Strict,
}
```
- 沙箱策略

### `SessionSource`
```rust
pub enum SessionSource {
    Cli,
    Exec,
    Server,
    Desktop,
}
```
- 会话来源

## 事件流示例

### 典型交互
```
[SQ] UserTurn("修改 main.rs")
  ↓
[EQ] SessionConfigured
[EQ] AgentMessageContentDelta("我会帮你...")
[EQ] ItemStarted(ToolCall: read_file)
[EQ] ItemCompleted(ToolCall: read_file)
[EQ] ItemStarted(ToolCall: apply_patch)
[EQ] ExecApprovalRequest(应用补丁)
  ↓
[SQ] ReviewDecision(Approved)
  ↓
[EQ] ItemCompleted(ToolCall: apply_patch)
[EQ] TurnDiff(文件变更)
[EQ] TokenCount(使用 1200 tokens)
```

## 协议扩展

### 自定义提示 (custom_prompts.rs)
- 支持自定义 prompt 定义

### 计划工具 (plan_tool.rs)
- 任务规划工具

### 命令解析 (parse_command.rs)
- 解析用户命令（如 /compact, /undo）

## 依赖关系

- `serde` / `serde_json`: 序列化
- `uuid`: 唯一 ID 生成
- `chrono`: 时间戳
- `mcp-types`: MCP 协议支持

## 在项目中的位置

```
┌──────────────────────────┐
│  app-server-protocol     │ (客户端协议)
└──────────┬───────────────┘
           │ 使用
┌──────────▼───────────────┐
│  protocol                │ ← 此 crate (核心协议)
└──────────┬───────────────┘
           │ 被使用
┌──────────▼───────────────┐
│  codex-core              │ (业务逻辑)
└──────────────────────────┘
```

## 重要常量

### `ConversationId`
- 类型别名：`Uuid`
- 唯一标识一个对话

### `InitialHistory`
```rust
pub enum InitialHistory {
    New,
    Resume { items: Vec<TurnItem> },
    Fork { items: Vec<TurnItem> },
}
```
- 会话初始历史

## 协议版本

当前版本通过类型系统保证兼容性，不需要显式版本号。未来如有不兼容更改，会通过新的 enum variant 处理。
