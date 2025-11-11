# client.rs 文档

## 文件作用

`client.rs` 实现了与 AI 模型 API 的通信客户端 `ModelClient`。它负责构建请求、处理流式响应、管理认证令牌、重试逻辑和错误处理。这是 Codex 与 Claude API（或其他兼容 API）交互的核心组件。

**在整体架构中的位置**：位于客户端层，是 Codex 引擎与外部 AI 服务之间的桥梁。

## 主要结构体

### `ModelClient`
```rust
pub struct ModelClient {
    config: Arc<Config>,
    auth_manager: Option<Arc<AuthManager>>,
    otel_event_manager: OtelEventManager,
    client: CodexHttpClient,
    provider: ModelProviderInfo,
    conversation_id: ConversationId,
    effort: Option<ReasoningEffortConfig>,
    summary: ReasoningSummaryConfig,
    session_source: SessionSource,
}
```

**职责**：
- 与 AI 模型 API 进行 HTTP 通信
- 管理请求认证和令牌刷新
- 处理流式 SSE (Server-Sent Events) 响应
- 实现重试和错误处理逻辑
- 发送遥测数据

**字段说明**：
- `config`: 应用配置
- `auth_manager`: 认证管理器（可选，用于需要认证的提供商）
- `otel_event_manager`: OpenTelemetry 事件管理器
- `client`: HTTP 客户端实例
- `provider`: 模型提供商信息（API 端点、模型名称等）
- `conversation_id`: 当前对话 ID
- `effort`: 推理努力级别配置
- `summary`: 推理摘要配置
- `session_source`: 会话来源

### `ErrorResponse`
```rust
struct ErrorResponse {
    error: Error,
}

struct Error {
    type: Option<String>,
    code: Option<String>,
    message: Option<String>,
    plan_type: Option<PlanType>,
    resets_at: Option<i64>,
}
```

**职责**：解析 API 错误响应

## 主要方法

### 构造和配置

#### `new(...) -> Self`
创建新的 ModelClient 实例。

**参数**：
- `config`: 配置对象
- `auth_manager`: 认证管理器
- `otel_event_manager`: 遥测管理器
- `provider`: 模型提供商信息
- `conversation_id`: 对话 ID
- `effort`: 推理努力级别
- `summary`: 推理摘要配置
- `session_source`: 会话来源

#### `get_model_context_window() -> Option<i64>`
获取模型的上下文窗口大小。

#### `get_auto_compact_token_limit() -> Option<i64>`
获取自动压缩的 token 限制。

#### `config() -> Arc<Config>`
获取配置引用。

#### `provider() -> &ModelProviderInfo`
获取提供商信息。

### 核心通信方法

#### `stream(&self, prompt: &Prompt) -> Result<ResponseStream>`
发送流式请求的主要入口点。

**流程**：
1. 准备请求参数
2. 获取认证令牌
3. 发送 HTTP 请求
4. 处理流式响应
5. 实现重试逻辑

**返回**：`ResponseStream` - 异步事件流

#### `stream_responses_api(prompt, token) -> Result<ResponseStream>`
调用 `/v1/responses` API 的内部实现。

**特性**：
- 支持流式 SSE 响应
- 自动处理 token 刷新
- 重试失败的请求
- 发送遥测事件

#### `stream_with_retry(prompt, token, attempt) -> Result<ResponseStream>`
带重试逻辑的流式请求实现。

**重试策略**：
- 最多重试 3 次
- 使用指数退避
- 只对特定错误类型重试（连接失败、超时等）

#### `send_responses_api_request(request, headers, attempt) -> Result<Response>`
发送实际的 HTTP 请求。

**特性**：
- 添加认证头
- 添加自定义请求头
- 处理测试 fixture（通过 `CODEX_RS_SSE_FIXTURE`）
- 超时控制（180 秒）

#### `handle_status_codes(response, attempt) -> Result<Response>`
处理 HTTP 状态码和错误。

**处理的状态码**：
- `200 OK`: 成功
- `401 Unauthorized`: 未授权
- `402 Payment Required`: 用量限制
- `429 Too Many Requests`: 速率限制
- `500-599`: 服务器错误（可重试）

### 错误处理

#### `handle_402_payment_required(response) -> Result<Never>`
处理 402 错误（用量限制）。

**错误类型**：
- `usage_limit_reached`: 用量已达上限
- `usage_not_included`: 用量不包含在计划中

#### `can_retry(attempt, response) -> bool`
判断请求是否可以重试。

**可重试的情况**：
- 5xx 服务器错误
- 429 速率限制（带 Retry-After 头）
- 连接失败
- 超时

### 辅助方法

#### `get_provider() -> ModelProviderInfo`
获取提供商信息副本。

#### `get_otel_event_manager() -> OtelEventManager`
获取遥测管理器。

#### `get_session_source() -> SessionSource`
获取会话来源。

#### `get_model() -> String`
获取模型名称。

#### `get_model_family() -> ModelFamily`
获取模型族（Claude、GPT 等）。

#### `get_reasoning_effort() -> Option<ReasoningEffortConfig>`
获取推理努力配置。

#### `get_reasoning_summary() -> ReasoningSummaryConfig`
获取推理摘要配置。

#### `get_auth_manager() -> Option<Arc<AuthManager>>`
获取认证管理器。

## 关键流程

### 流式请求流程
```
stream(prompt)
  ↓
获取认证 token (auth_manager.get_token())
  ↓
stream_responses_api(prompt, token)
  ↓
构建请求体 (ResponsesApiRequest)
  ↓
stream_with_retry(prompt, token, attempt=1)
  ↓
send_responses_api_request()
  ↓
处理响应状态码
  ↓
stream_chat_completions() - 解析 SSE 流
  ↓
aggregate_stream() - 聚合响应片段
  ↓
返回 ResponseStream
```

### 错误重试流程
```
请求失败
  ↓
can_retry(attempt, error)?
  ↓ Yes
等待退避时间 (backoff)
  ↓
attempt += 1
  ↓
重新尝试 stream_with_retry()
  ↓ 达到最大重试次数
返回 RetryLimitReachedError
```

### Token 刷新流程
```
API 返回 401
  ↓
尝试刷新 token
  ↓
auth_manager.refresh_token()
  ↓ 成功
使用新 token 重试请求
  ↓ 失败
返回 RefreshTokenError
```

## 错误类型

- `ConnectionFailedError`: 连接失败
- `UnexpectedResponseError`: 意外的响应格式
- `UsageLimitReachedError`: 用量限制
- `RetryLimitReachedError`: 重试次数超限
- `ResponseStreamFailed`: 流处理失败
- `RefreshTokenError`: Token 刷新失败

## 配置选项

通过 `Config` 和 `ModelProviderInfo` 配置：
- API 端点 URL
- 模型名称
- 超时时间
- 重试策略
- 自定义请求头

## 测试支持

- `CODEX_RS_SSE_FIXTURE` 环境变量：使用本地 fixture 文件代替实际 API 请求
- Mock HTTP 客户端支持

## 相关文件

- `client_common.rs` - 公共类型定义（Prompt、ResponseEvent 等）
- `chat_completions.rs` - SSE 流处理
- `auth.rs` - 认证管理
- `default_client.rs` - HTTP 客户端创建
- `model_provider_info.rs` - 提供商信息
- `error.rs` - 错误类型定义
