# codex.rs 文档

## 文件作用

`codex.rs` 是 Codex 系统的核心引擎，实现了整个 AI 代码助手的主要交互逻辑。它负责协调会话生命周期、处理用户提交、管理 AI 响应、编排工具执行、维护上下文等核心功能。这是整个系统最重要的文件之一。

**在整体架构中的位置**：位于会话管理层的核心，作为各个子系统（客户端、工具、上下文、状态管理）的协调中心。

## 主要结构体

### `Codex`
```rust
pub struct Codex {
    pub(crate) next_id: AtomicU64,
    pub(crate) tx_sub: Sender<Submission>,
    pub(crate) rx_event: Receiver<Event>,
}
```

**职责**：
- 提供高层次的 Codex 系统接口
- 实现提交-事件队列模式
- 管理提交 ID 生成
- 协调内部会话处理

**字段说明**：
- `next_id`: 原子计数器，生成唯一提交 ID
- `tx_sub`: 提交通道发送端（用户输入 → 会话处理器）
- `rx_event`: 事件通道接收端（会话处理器 → 用户）

**设计模式**：Actor 模式 - 通过消息传递实现解耦

### `CodexSpawnOk`
```rust
pub struct CodexSpawnOk {
    pub codex: Codex,
    pub conversation_id: ConversationId,
}
```

**职责**：封装 Codex::spawn 的返回值

### `ProcessedResponseItem`
```rust
pub struct ProcessedResponseItem {
    // 处理后的响应项数据
}
```

**职责**：表示经过处理的 AI 响应项

## 主要方法

### 生命周期管理

#### `spawn(config, auth_manager, conversation_history, session_source) -> Result<CodexSpawnOk>`
生成新的 Codex 实例的主要入口点。

**流程**：
1. 创建提交和事件通道
2. 加载用户指令（从 `.codex/instructions.md` 等）
3. 构建会话配置 (SessionConfiguration)
4. 创建 Session 实例
5. 生成对话 ID
6. 发送初始 SessionConfigured 事件
7. 启动后台会话任务 (Session::run)
8. 返回 Codex 实例

**参数**：
- `config`: 配置对象（模型、路径、策略等）
- `auth_manager`: 认证管理器
- `conversation_history`: 初始历史记录（New、Resume 或 Fork）
- `session_source`: 会话来源（CLI、Exec、Server）

### 用户交互

#### `submit(&self, submission: Submission) -> Result<String>`
提交用户请求到 Codex。

**流程**：
1. 生成唯一提交 ID
2. 将提交发送到内部通道
3. 返回提交 ID（用于追踪）

**支持的提交类型**：
- `UserMessage`: 用户消息
- `ReviewDecision`: 审查决策
- `ApprovalResponse`: 批准响应
- `AbortTurn`: 中止当前轮次
- `FunctionCallOutput`: 函数调用输出

#### `next_event(&self) -> Result<Event>`
获取下一个事件（阻塞）。

**事件类型**：
- `SessionConfigured`: 会话配置完成
- `AgentMessageContentDelta`: AI 消息内容增量
- `ItemStarted`: 工具/任务开始
- `ItemCompleted`: 工具/任务完成
- `TokenCount`: Token 使用统计
- `TurnDiff`: 文件变更差异
- `Error`: 错误事件
- 等等...

#### `try_next_event(&self) -> Result<Option<Event>>`
非阻塞地尝试获取事件。

#### `poll_next_event(&mut self, cx: &mut Context) -> Poll<Option<Event>>`
Future 轮询接口，用于异步流。

## 内部架构

### Session (内部结构)
Session 是 Codex 的内部工作引擎，运行在独立的异步任务中。

**主要职责**：
- 维护会话状态 (SessionState)
- 管理上下文 (ContextManager)
- 处理用户提交
- 协调 AI 交互
- 编排工具执行
- 持久化会话数据 (RolloutRecorder)

**核心方法**：
- `run()`: 主事件循环
- `process_turn()`: 处理一个完整的交互轮次
- `handle_submission()`: 处理用户提交
- `stream_model_response()`: 流式处理 AI 响应
- `execute_tools()`: 执行工具调用

### SessionConfiguration
会话配置信息，包含：
- 模型提供商和模型名称
- 推理努力级别和摘要配置
- 开发者和用户指令
- 批准和沙箱策略
- 工作目录和功能开关

### 核心组件集成

#### ContextManager
管理对话上下文：
- 维护消息历史
- 实现上下文截断
- 规范化消息格式
- 计算 token 使用

#### ModelClient
与 AI API 通信：
- 发送 prompt
- 接收流式响应
- 处理认证和重试

#### ToolRouter
工具系统路由：
- 注册可用工具
- 解析工具调用
- 分发到对应处理器
- 支持并行执行

#### RolloutRecorder
会话持久化：
- 记录所有交互
- 支持会话恢复
- 保存快照
- 管理会话元数据

#### UnifiedExecSessionManager
统一执行管理：
- 管理 Shell 会话
- 处理命令执行
- 维护执行环境

#### McpConnectionManager
MCP (Model Context Protocol) 集成：
- 管理 MCP 服务器连接
- 提供额外工具和资源
- 处理 MCP 认证

## 关键流程

### 完整交互轮次流程
```
用户提交消息
  ↓
submit() → tx_sub
  ↓
Session.run() 接收提交
  ↓
handle_submission()
  ↓
构建 Prompt (上下文 + 工具定义)
  ↓
ModelClient.stream()
  ↓
流式接收 AI 响应
  ↓
parse_turn_item() 解析响应项
  ↓
发出 ItemStarted 事件
  ↓
process_items() 处理响应
  ↓
ToolRouter 执行工具调用（可能并行）
  ↓
发出 ItemCompleted 事件
  ↓
更新上下文和状态
  ↓
RolloutRecorder 持久化
  ↓
等待下一次提交
```

### 工具执行流程
```
AI 请求工具调用
  ↓
ToolRouter.route(tool_name)
  ↓
获取 ToolHandler
  ↓
检查批准策略
  ↓ 需要批准
发出 ExecApprovalRequest 事件
  ↓
等待用户批准
  ↓ 批准
ToolHandler.execute()
  ↓
UnifiedExec / Shell / MCP
  ↓
返回执行结果
  ↓
将结果加入上下文
```

### 上下文管理流程
```
新消息到达
  ↓
ContextManager.add_message()
  ↓
计算当前 token 使用
  ↓
超过限制？
  ↓ Yes
触发自动压缩 (compact)
  ↓
build_compacted_history()
  ↓
调用 AI 生成摘要
  ↓
替换旧历史为摘要
  ↓
继续处理
```

### 会话恢复流程
```
spawn(InitialHistory::Resume)
  ↓
加载 rollout 文件
  ↓
解析历史消息和状态
  ↓
重建 ContextManager
  ↓
恢复工具状态
  ↓
重新连接 MCP 服务器
  ↓
发出 SessionConfigured 事件
  ↓
准备接收新提交
```

## 并发和线程安全

- **消息传递**：使用 `async-channel` 实现无锁并发
- **状态隔离**：Session 运行在独立任务中
- **共享状态**：使用 `Arc` 和 `Mutex/RwLock` 保护共享数据
- **取消机制**：使用 `CancellationToken` 支持优雅取消

## 错误处理

主要错误类型：
- `InternalAgentDied`: 内部代理崩溃
- `SessionConfiguredNotFirstEvent`: 初始化失败
- `TurnAborted`: 轮次被中止
- `ToolExecutionFailed`: 工具执行失败
- `ModelRequestFailed`: 模型请求失败

错误处理策略：
- 关键错误：发出 Error 事件并终止会话
- 可恢复错误：发出错误事件但继续运行
- 工具错误：记录并将错误返回给 AI

## 可观测性

### 日志
使用 `tracing` crate：
- `debug!`: 详细调试信息
- `info!`: 关键操作
- `warn!`: 警告
- `error!`: 错误

### 遥测
通过 `OtelEventManager` 发送：
- API 请求指标
- 工具执行时间
- Token 使用统计
- 错误率

### 事件流
所有操作都通过事件暴露给外部：
- 实时进度追踪
- 调试和监控
- 用户界面更新

## 配置和特性开关

通过 `Config` 和 `Features` 控制：
- 启用/禁用特定工具
- 调整上下文窗口大小
- 配置批准策略
- 设置沙箱策略
- 自定义指令

## 性能优化

- **流式处理**：增量处理 AI 响应，降低延迟
- **并行工具执行**：同时执行多个独立工具
- **上下文压缩**：自动压缩历史以节省 token
- **连接池**：复用 HTTP 连接
- **异步 I/O**：非阻塞文件和网络操作

## 测试支持

提供的测试工具：
- Mock 客户端
- 固定响应 (fixtures)
- 测试工具注册表
- 内存 Rollout 记录器

## 相关文件

- `codex_conversation.rs` - 对话抽象接口
- `codex_delegate.rs` - 代理模式实现
- `state/` - 状态管理模块
- `context_manager/` - 上下文管理
- `tools/` - 工具系统
- `tasks/` - 任务系统
- `client.rs` - AI 客户端
- `rollout/` - 会话持久化
