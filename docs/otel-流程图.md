# otel 流程图

## 概述

`otel` crate 提供了 OpenTelemetry 遥测功能的集成，支持跟踪、指标和日志记录。它可以通过 feature flag 启用或禁用。

## 1. OpenTelemetry 初始化流程

```mermaid
flowchart TD
    Start[应用启动] --> CheckFeature{otel feature 启用?}

    CheckFeature -->|否| UseStub[使用存根实现]
    CheckFeature -->|是| ReadConfig[读取 OtelSettings]

    UseStub --> ReturnNone[返回 None]

    ReadConfig --> ParseEndpoint[解析 OTLP 端点]
    ParseEndpoint --> CheckEndpoint{端点配置有效?}

    CheckEndpoint -->|否| ReturnNone
    CheckEndpoint -->|是| InitProvider[初始化 OtelProvider]

    InitProvider --> SetupTracer[设置 Tracer]
    SetupTracer --> ConfigureExporter[配置 OTLP 导出器]
    ConfigureExporter --> SetupBatch[配置批量处理器]

    SetupBatch --> InitGlobalProvider[安装全局 TracerProvider]
    InitGlobalProvider --> ReturnProvider[返回 Provider 实例]

    ReturnProvider --> End[完成]
    ReturnNone --> End
```

## 2. 事件记录流程

```mermaid
flowchart TD
    Start[事件发生] --> GetEventManager[获取 OtelEventManager]
    GetEventManager --> CheckEnabled{启用 otel?}

    CheckEnabled -->|否| SkipEvent[跳过事件]
    CheckEnabled -->|是| CreateSpan[创建 Span]

    CreateSpan --> SetAttributes[设置 Span 属性]
    SetAttributes --> AddMetadata[添加元数据]
    AddMetadata --> RecordTimestamp[记录时间戳]

    RecordTimestamp --> CheckParent{有父 Span?}
    CheckParent -->|是| LinkToParent[链接到父 Span]
    CheckParent -->|否| CreateRoot[创建根 Span]

    LinkToParent --> RecordEvent[记录事件]
    CreateRoot --> RecordEvent

    RecordEvent --> CheckBatch{达到批量阈值?}
    CheckBatch -->|是| FlushBatch[刷新批量数据]
    CheckBatch -->|否| ContinueBuffering[继续缓冲]

    FlushBatch --> ExportToCollector[导出到收集器]
    ContinueBuffering --> End[完成]
    ExportToCollector --> End

    SkipEvent --> End
```

## 3. HTTP 请求追踪流程

```mermaid
sequenceDiagram
    participant App as 应用
    participant Otel as OtelProvider
    participant Span as Span
    participant HTTP as HTTP 客户端
    participant Backend as 后端服务

    App->>Otel: 创建请求追踪
    Otel->>Span: 创建新 Span
    Span->>Span: 生成 trace_id 和 span_id

    Otel->>HTTP: 提取追踪头
    Note over HTTP: traceparent<br/>tracestate

    HTTP->>Backend: 发送请求 + 追踪头
    Backend->>Backend: 处理请求<br/>（保持追踪上下文）
    Backend->>HTTP: 返回响应

    HTTP->>Span: 记录响应信息
    Span->>Span: 设置状态和属性
    Span->>Otel: 结束 Span
    Otel->>App: 返回结果
```

## 4. Span 生命周期管理

```mermaid
stateDiagram-v2
    [*] --> 创建: 事件触发
    创建 --> 活跃: 开始记录
    活跃 --> 活跃: 添加属性/事件
    活跃 --> 结束: 操作完成
    活跃 --> 错误: 发生错误
    错误 --> 结束: 记录错误信息
    结束 --> 导出: 准备导出
    导出 --> [*]: 发送到收集器

    note right of 创建
        设置名称和类型
        生成 span_id
    end note

    note right of 活跃
        记录属性
        记录子事件
        更新状态
    end note

    note right of 导出
        批量处理
        OTLP 格式
    end note
```

## 5. 数据流图

```mermaid
flowchart LR
    subgraph Application[应用层]
        Code[应用代码]
        Events[事件生成]
    end

    subgraph OtelCrate[otel crate]
        EventManager[OtelEventManager]
        Provider[OtelProvider]
    end

    subgraph SDK[OpenTelemetry SDK]
        Tracer[Tracer]
        Processor[BatchProcessor]
        Exporter[OTLP Exporter]
    end

    subgraph Backend[后端]
        Collector[OTLP Collector]
        Storage[存储系统]
    end

    Code --> Events
    Events --> EventManager
    EventManager --> Provider
    Provider --> Tracer
    Tracer --> Processor
    Processor --> Exporter
    Exporter -->|HTTP/gRPC| Collector
    Collector --> Storage
```

## 6. 配置加载流程

```mermaid
flowchart TD
    Start[读取配置] --> CheckEnvVar{检查环境变量}

    CheckEnvVar -->|OTEL_EXPORTER_OTLP_ENDPOINT| UseEnvEndpoint[使用环境变量端点]
    CheckEnvVar -->|未设置| ReadConfigFile[读取配置文件]

    ReadConfigFile --> ParseConfig[解析 OtelSettings]
    ParseConfig --> ValidateConfig{验证配置}

    ValidateConfig -->|有效| MergeSettings[合并配置]
    ValidateConfig -->|无效| UseDefaults[使用默认值]

    UseEnvEndpoint --> MergeSettings
    UseDefaults --> MergeSettings

    MergeSettings --> ApplyConfig[应用配置]
    ApplyConfig --> End[返回配置]
```

## 7. 批量处理和导出流程

```mermaid
flowchart TD
    Start[Span 结束] --> AddToBuffer[添加到缓冲区]
    AddToBuffer --> CheckBuffer{检查缓冲区}

    CheckBuffer -->|未满| WaitMore[等待更多 Span]
    CheckBuffer -->|已满| TriggerExport[触发导出]
    CheckBuffer -->|超时| TriggerExport

    TriggerExport --> SerializeSpans[序列化 Span 数据]
    SerializeSpans --> CompressData{需要压缩?}

    CompressData -->|是| GzipCompress[Gzip 压缩]
    CompressData -->|否| PreparePayload[准备负载]

    GzipCompress --> PreparePayload
    PreparePayload --> SendHTTP[发送 HTTP 请求]

    SendHTTP --> CheckResponse{响应状态}
    CheckResponse -->|成功| ClearBuffer[清空缓冲区]
    CheckResponse -->|失败| RetryLogic[重试逻辑]

    RetryLogic --> CheckRetries{重试次数}
    CheckRetries -->|未超限| BackoffWait[退避等待]
    CheckRetries -->|超限| LogError[记录错误]

    BackoffWait --> SendHTTP
    LogError --> DiscardBatch[丢弃批次]

    ClearBuffer --> End[完成]
    DiscardBatch --> End
    WaitMore --> End
```

## 8. HTTP 头注入流程

```mermaid
flowchart TD
    Start[准备发送 HTTP 请求] --> GetCurrentSpan[获取当前 Span]
    GetCurrentSpan --> CheckSpan{Span 存在?}

    CheckSpan -->|否| NoHeaders[不添加追踪头]
    CheckSpan -->|是| ExtractContext[提取 SpanContext]

    ExtractContext --> BuildTraceparent[构建 traceparent 头]
    BuildTraceparent --> CheckTracestate{有 tracestate?}

    CheckTracestate -->|是| AddTracestate[添加 tracestate 头]
    CheckTracestate -->|否| AddToHeaders[添加到 HeaderMap]

    AddTracestate --> AddToHeaders
    AddToHeaders --> ReturnHeaders[返回 HeaderMap]

    NoHeaders --> ReturnEmpty[返回空 HeaderMap]

    ReturnHeaders --> End[完成]
    ReturnEmpty --> End
```

## 关键决策点

1. **Feature Flag 检查**：在编译时决定是使用真实实现还是存根实现
2. **端点配置**：从环境变量或配置文件读取 OTLP 端点
3. **批量阈值**：达到数量或时间阈值时触发导出
4. **重试策略**：失败时使用指数退避重试
5. **Span 父子关系**：自动链接父子 Span 以构建追踪树

## 配置项

### OtelSettings 结构

```rust
pub struct OtelSettings {
    pub endpoint: Option<String>,
    pub service_name: String,
    pub batch_size: usize,
    pub export_timeout: Duration,
}
```

### 环境变量

- `OTEL_EXPORTER_OTLP_ENDPOINT`: OTLP 收集器端点
- `OTEL_SERVICE_NAME`: 服务名称
- `OTEL_LOG_LEVEL`: 日志级别

## Span 属性

常见的 Span 属性：
- `service.name`: 服务名称
- `operation.name`: 操作名称
- `http.method`: HTTP 方法
- `http.url`: HTTP URL
- `http.status_code`: HTTP 状态码
- `error`: 是否发生错误
- `error.message`: 错误消息

## 性能优化

1. **批量处理**：减少网络往返次数
2. **异步导出**：不阻塞主线程
3. **背压控制**：限制内存使用
4. **采样策略**：按比例采样以减少开销
5. **Feature Flag**：禁用时零开销

## 与 Tracing 集成

`otel` crate 与 Rust 的 `tracing` crate 集成：

- 使用 `tracing::Span` 作为基础
- 自动转换 `tracing` 事件到 OpenTelemetry
- 支持结构化日志和属性
