# protocol/src/protocol.rs

## 文件作用

定义 Codex 会话的核心协议，包括提交队列（Submission Queue）和事件队列（Event Queue）模式，用于客户端和代理之间的异步通信。

## 主要结构体

### `pub struct Submission`
- 提交队列条目（用户请求）
- 字段：`id`, `op`

### `pub struct Event`
- 事件队列条目（代理响应）
- 字段：`id`, `msg`

### `pub struct TokenUsage`
- Token 使用统计
- 字段：input_tokens, cached_input_tokens, output_tokens, reasoning_output_tokens

### `pub struct SandboxPolicy`
- 沙盒策略
- 变体：DangerFullAccess, ReadOnly, WorkspaceWrite

### `pub struct WritableRoot`
- 可写根路径配置
- 字段：`root`, `read_only_subpaths`

## 主要枚举

### `pub enum Op`
- 提交操作类型
- 变体：Interrupt, UserInput, UserTurn, ExecApproval, PatchApproval, Compact, Undo, Review, Shutdown 等

### `pub enum EventMsg`
- 事件消息类型
- 变体：Error, Warning, TaskStarted, TaskComplete, AgentMessage, UserMessage, ExecCommandBegin 等 40+ 种事件类型

### `pub enum AskForApproval`
- 命令审批策略
- 变体：UnlessTrusted, OnFailure, OnRequest (默认), Never

### `pub enum ReviewDecision`
- 审查决策
- 变体：Approved, ApprovedForSession, Denied (默认), Abort

## 主要函数和方法

### `impl SandboxPolicy`
- `pub fn new_read_only_policy() -> Self`
- `pub fn new_workspace_write_policy() -> Self`
- `pub fn get_writable_roots_with_cwd(&self, cwd: &Path) -> Vec<WritableRoot>`

### `impl TokenUsage`
- `pub fn blended_total(&self) -> i64`
- `pub fn percent_of_context_window_remaining(&self, context_window: i64) -> i64`
