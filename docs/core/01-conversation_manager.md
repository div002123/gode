# conversation_manager.rs 文档

## 文件作用

`conversation_manager.rs` 是对话管理的核心模块，负责创建、维护和管理 Codex 对话会话。它充当对话生命周期的中央协调器，提供了创建新对话、恢复现有对话的接口。

**在整体架构中的位置**：位于会话管理层的顶层，是外部调用者（如 CLI、服务器）与 Codex 引擎之间的主要接口。

## 主要结构体

### `ConversationManager`
```rust
pub struct ConversationManager {
    conversations: Arc<RwLock<HashMap<ConversationId, Arc<CodexConversation>>>>,
    auth_manager: Arc<AuthManager>,
    session_source: SessionSource,
}
```

**职责**：
- 维护所有活动对话的内存映射
- 管理对话的创建和生命周期
- 提供认证管理器的访问
- 追踪会话来源（CLI、Exec、Server 等）

**字段说明**：
- `conversations`: 线程安全的对话映射表，以 ConversationId 为键
- `auth_manager`: 共享的认证管理器实例
- `session_source`: 会话来源标识

### `NewConversation`
```rust
pub struct NewConversation {
    pub conversation_id: ConversationId,
    pub conversation: Arc<CodexConversation>,
    pub session_configured: SessionConfiguredEvent,
}
```

**职责**：封装新创建的对话及其初始化事件

**字段说明**：
- `conversation_id`: 唯一对话标识符
- `conversation`: 对话实例的 Arc 引用
- `session_configured`: 会话配置事件（首个事件）

## 主要方法

### `new(auth_manager, session_source) -> Self`
创建新的 ConversationManager 实例。

**参数**：
- `auth_manager`: 认证管理器
- `session_source`: 会话来源

### `with_auth(auth: CodexAuth) -> Self`
测试用构造函数，使用提供的认证创建虚拟 AuthManager。

### `new_conversation(&self, config: Config) -> Result<NewConversation>`
创建新对话的主要入口点。

**流程**：
1. 调用 `spawn_conversation` 生成 Codex 实例
2. 等待并验证第一个 `SessionConfigured` 事件
3. 将对话添加到内存映射
4. 返回 NewConversation 结构

### `spawn_conversation(config, auth_manager) -> Result<NewConversation>`
内部方法，负责实际的对话生成逻辑。

### `resume_conversation(config, history, rollout_path) -> Result<NewConversation>`
从保存的历史记录恢复对话。

**参数**：
- `config`: 配置对象
- `history`: 初始历史记录
- `rollout_path`: rollout 文件路径（用于持久化）

### `fork_conversation(config, history, rollout_path) -> Result<NewConversation>`
从现有对话分叉创建新对话。

### `get_conversation(&self, id) -> Option<Arc<CodexConversation>>`
根据 ID 获取对话实例。

### `conversation_count(&self) -> usize`
返回当前活动对话数量。

### `remove_conversation(&self, id) -> Option<Arc<CodexConversation>>`
从管理器中移除对话（用于清理）。

## 关键流程

### 创建新对话流程
```
用户请求
  ↓
new_conversation(config)
  ↓
spawn_conversation()
  ↓
Codex::spawn()
  ↓
等待 SessionConfigured 事件
  ↓
finalize_spawn()
  ↓
添加到 conversations 映射
  ↓
返回 NewConversation
```

### 恢复对话流程
```
恢复请求
  ↓
resume_conversation(config, history)
  ↓
Codex::spawn(InitialHistory::Resume)
  ↓
验证 SessionConfigured 事件
  ↓
恢复上下文和状态
  ↓
返回 NewConversation
```

## 错误处理

- `SessionConfiguredNotFirstEvent`: 当第一个事件不是 SessionConfigured 时抛出
- `CodexErr`: 传播 Codex 生成过程中的错误

## 线程安全

- 使用 `Arc<RwLock<HashMap>>` 确保多线程环境下的安全访问
- 所有对话实例都包装在 Arc 中，支持共享所有权

## 使用示例

```rust
// 创建管理器
let manager = ConversationManager::new(auth_manager, SessionSource::Cli);

// 创建新对话
let new_conv = manager.new_conversation(config).await?;
let conversation_id = new_conv.conversation_id;

// 获取对话
let conversation = manager.get_conversation(&conversation_id).unwrap();

// 使用对话
conversation.submit(message).await?;
```

## 相关文件

- `codex.rs` - 实际的 Codex 引擎实现
- `codex_conversation.rs` - CodexConversation 抽象
- `auth.rs` - 认证管理
- `config/mod.rs` - 配置类型
