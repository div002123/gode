# mcp-types 类型关系图

## 概述

`mcp-types` crate 定义了 Model Context Protocol (MCP) 的所有类型定义。这是一个自动生成的 crate，包含协议的请求、响应、通知和各种数据结构。

## 1. MCP 协议类型层次结构

```mermaid
graph TD
    JSONRPCMessage[JSONRPCMessage] --> JSONRPCRequest[JSONRPCRequest]
    JSONRPCMessage --> JSONRPCNotification[JSONRPCNotification]
    JSONRPCMessage --> JSONRPCResponse[JSONRPCResponse]
    JSONRPCMessage --> JSONRPCError[JSONRPCError]

    JSONRPCRequest --> ClientRequest[ClientRequest]
    JSONRPCRequest --> ServerRequest[ServerRequest]

    JSONRPCNotification --> ClientNotification[ClientNotification]
    JSONRPCNotification --> ServerNotification[ServerNotification]

    JSONRPCResponse --> ClientResult[ClientResult]
    JSONRPCResponse --> ServerResult[ServerResult]

    style JSONRPCMessage fill:#f9f,stroke:#333,stroke-width:4px
    style ClientRequest fill:#bbf,stroke:#333,stroke-width:2px
    style ServerRequest fill:#bbf,stroke:#333,stroke-width:2px
    style ClientNotification fill:#bfb,stroke:#333,stroke-width:2px
    style ServerNotification fill:#bfb,stroke:#333,stroke-width:2px
```

## 2. 客户端请求类型

```mermaid
graph LR
    ClientRequest[ClientRequest] --> Initialize[InitializeRequest]
    ClientRequest --> Ping[PingRequest]
    ClientRequest --> Resources[资源相关]
    ClientRequest --> Prompts[提示相关]
    ClientRequest --> Tools[工具相关]
    ClientRequest --> Logging[日志相关]
    ClientRequest --> Completion[补全相关]

    Resources --> ListResources[ListResourcesRequest]
    Resources --> ListResourceTemplates[ListResourceTemplatesRequest]
    Resources --> ReadResource[ReadResourceRequest]
    Resources --> Subscribe[SubscribeRequest]
    Resources --> Unsubscribe[UnsubscribeRequest]

    Prompts --> ListPrompts[ListPromptsRequest]
    Prompts --> GetPrompt[GetPromptRequest]

    Tools --> ListTools[ListToolsRequest]
    Tools --> CallTool[CallToolRequest]

    Logging --> SetLevel[SetLevelRequest]

    Completion --> Complete[CompleteRequest]

    style ClientRequest fill:#bbf,stroke:#333,stroke-width:3px
```

## 3. 服务器通知类型

```mermaid
graph TD
    ServerNotification[ServerNotification] --> Progress[ProgressNotification<br/>进度通知]
    ServerNotification --> Cancelled[CancelledNotification<br/>取消通知]
    ServerNotification --> ResourceUpdated[ResourceUpdatedNotification<br/>资源更新]
    ServerNotification --> ResourceListChanged[ResourceListChangedNotification<br/>资源列表变更]
    ServerNotification --> PromptListChanged[PromptListChangedNotification<br/>提示列表变更]
    ServerNotification --> ToolListChanged[ToolListChangedNotification<br/>工具列表变更]
    ServerNotification --> LoggingMessage[LoggingMessageNotification<br/>日志消息]

    style ServerNotification fill:#bfb,stroke:#333,stroke-width:3px
```

## 4. 内容块类型层次

```mermaid
graph TD
    ContentBlock[ContentBlock] --> TextContent[TextContent<br/>文本内容]
    ContentBlock --> ImageContent[ImageContent<br/>图片内容]
    ContentBlock --> AudioContent[AudioContent<br/>音频内容]
    ContentBlock --> ResourceLink[ResourceLink<br/>资源链接]
    ContentBlock --> EmbeddedResource[EmbeddedResource<br/>嵌入资源]

    EmbeddedResource --> EmbeddedResourceResource[resource]
    EmbeddedResourceResource --> TextResourceContents[TextResourceContents]
    EmbeddedResourceResource --> BlobResourceContents[BlobResourceContents]

    TextContent --> Annotations1[Annotations]
    ImageContent --> Annotations2[Annotations]
    AudioContent --> Annotations3[Annotations]
    ResourceLink --> Annotations4[Annotations]
    EmbeddedResource --> Annotations5[Annotations]

    style ContentBlock fill:#fbb,stroke:#333,stroke-width:3px
    style Annotations1 fill:#ffd,stroke:#333,stroke-width:1px
    style Annotations2 fill:#ffd,stroke:#333,stroke-width:1px
    style Annotations3 fill:#ffd,stroke:#333,stroke-width:1px
    style Annotations4 fill:#ffd,stroke:#333,stroke-width:1px
    style Annotations5 fill:#ffd,stroke:#333,stroke-width:1px
```

## 5. 协议握手流程

```mermaid
sequenceDiagram
    participant Client as 客户端
    participant Server as 服务器

    Client->>Server: InitializeRequest<br/>(clientInfo, capabilities)
    Server->>Client: InitializeResult<br/>(serverInfo, capabilities)
    Client->>Server: InitializedNotification
    Note over Client,Server: 连接已建立，可以进行通信
```

## 6. Trait 关系

```mermaid
classDiagram
    class ModelContextProtocolRequest {
        <<trait>>
        +METHOD: &'static str
        +Params: Type
        +Result: Type
    }

    class ModelContextProtocolNotification {
        <<trait>>
        +METHOD: &'static str
        +Params: Type
    }

    InitializeRequest ..|> ModelContextProtocolRequest
    PingRequest ..|> ModelContextProtocolRequest
    ListResourcesRequest ..|> ModelContextProtocolRequest
    CallToolRequest ..|> ModelContextProtocolRequest

    ProgressNotification ..|> ModelContextProtocolNotification
    CancelledNotification ..|> ModelContextProtocolNotification
    ResourceUpdatedNotification ..|> ModelContextProtocolNotification
```

## 7. 工具定义结构

```mermaid
graph TD
    Tool[Tool] --> Name[name: String]
    Tool --> Description[description: Option String]
    Tool --> InputSchema[inputSchema: ToolInputSchema]
    Tool --> OutputSchema[outputSchema: Option ToolOutputSchema]
    Tool --> Annotations[annotations: Option ToolAnnotations]

    ToolAnnotations --> ReadOnlyHint[readOnlyHint]
    ToolAnnotations --> DestructiveHint[destructiveHint]
    ToolAnnotations --> IdempotentHint[idempotentHint]
    ToolAnnotations --> OpenWorldHint[openWorldHint]

    ToolInputSchema --> Properties[properties: JSON]
    ToolInputSchema --> Required[required: Vec String]

    style Tool fill:#fbf,stroke:#333,stroke-width:3px
```

## 8. 资源类型关系

```mermaid
graph TD
    Resource[Resource] --> ResourceMeta[元数据]
    Resource --> URI[uri: String]

    ResourceMeta --> Name[name: String]
    ResourceMeta --> Description[description]
    ResourceMeta --> MimeType[mimeType]
    ResourceMeta --> Size[size]
    ResourceMeta --> Annotations[annotations]

    ResourceTemplate[ResourceTemplate] --> TemplateURI[uriTemplate: String]
    ResourceTemplate --> TemplateMeta[元数据]

    ReadResourceResult[ReadResourceResult] --> Contents[contents: Vec]
    Contents --> TextResourceContents[TextResourceContents]
    Contents --> BlobResourceContents[BlobResourceContents]

    style Resource fill:#bff,stroke:#333,stroke-width:2px
    style ResourceTemplate fill:#bff,stroke:#333,stroke-width:2px
```

## 9. Schema 类型定义

```mermaid
graph TD
    PrimitiveSchemaDefinition[PrimitiveSchemaDefinition] --> StringSchema[StringSchema]
    PrimitiveSchemaDefinition --> NumberSchema[NumberSchema]
    PrimitiveSchemaDefinition --> BooleanSchema[BooleanSchema]
    PrimitiveSchemaDefinition --> EnumSchema[EnumSchema]

    StringSchema --> MaxLength[maxLength]
    StringSchema --> MinLength[minLength]
    StringSchema --> Format[format]

    NumberSchema --> Maximum[maximum]
    NumberSchema --> Minimum[minimum]

    EnumSchema --> EnumValues[enum: Vec String]
    EnumSchema --> EnumNames[enumNames: Option]

    style PrimitiveSchemaDefinition fill:#ffe,stroke:#333,stroke-width:3px
```

## 10. 能力协商

```mermaid
graph LR
    ClientCapabilities[ClientCapabilities] --> Roots[roots]
    ClientCapabilities --> Sampling[sampling]
    ClientCapabilities --> Elicitation[elicitation]
    ClientCapabilities --> Experimental[experimental]

    ServerCapabilities[ServerCapabilities] --> Resources[resources]
    ServerCapabilities --> Prompts[prompts]
    ServerCapabilities --> Tools[tools]
    ServerCapabilities --> Logging[logging]
    ServerCapabilities --> Completions[completions]
    ServerCapabilities --> ServerExperimental[experimental]

    Resources --> ListChanged[listChanged]
    Resources --> Subscribe[subscribe]

    style ClientCapabilities fill:#ddf,stroke:#333,stroke-width:2px
    style ServerCapabilities fill:#dfd,stroke:#333,stroke-width:2px
```

## 11. 消息 ID 类型

```mermaid
graph TD
    RequestId[RequestId] --> StringId[String 类型 ID]
    RequestId --> IntegerId[Integer 类型 ID]

    ProgressToken[ProgressToken] --> StringToken[String 类型 Token]
    ProgressToken --> IntegerToken[Integer 类型 Token]

    style RequestId fill:#ffc,stroke:#333,stroke-width:2px
    style ProgressToken fill:#ffc,stroke:#333,stroke-width:2px
```

## 12. 角色枚举

```mermaid
graph TD
    Role[Role 枚举] --> Assistant[Assistant<br/>助手]
    Role --> User[User<br/>用户]

    LoggingLevel[LoggingLevel 枚举] --> Emergency[emergency]
    LoggingLevel --> Alert[alert]
    LoggingLevel --> Critical[critical]
    LoggingLevel --> Error[error]
    LoggingLevel --> Warning[warning]
    LoggingLevel --> Notice[notice]
    LoggingLevel --> Info[info]
    LoggingLevel --> Debug[debug]

    style Role fill:#fcf,stroke:#333,stroke-width:2px
    style LoggingLevel fill:#cff,stroke:#333,stroke-width:2px
```

## 核心概念

### 1. 请求/响应模式

每个请求类型实现 `ModelContextProtocolRequest` trait，定义：
- `METHOD`: 方法名常量
- `Params`: 请求参数类型
- `Result`: 响应结果类型

### 2. 通知模式

单向消息，实现 `ModelContextProtocolNotification` trait，定义：
- `METHOD`: 方法名常量
- `Params`: 通知参数类型

### 3. 内容类型

支持多种内容类型：
- **Text**: 纯文本
- **Image**: Base64 编码的图片
- **Audio**: Base64 编码的音频
- **Resource**: 资源链接或嵌入资源

### 4. 注解系统

`Annotations` 提供元数据：
- `audience`: 目标受众（角色列表）
- `priority`: 优先级
- `lastModified`: 最后修改时间

## 类型转换

### JSONRPCRequest → ClientRequest

```rust
impl TryFrom<JSONRPCRequest> for ClientRequest {
    // 根据 method 字段路由到具体的请求类型
}
```

### JSONRPCNotification → ServerNotification

```rust
impl TryFrom<JSONRPCNotification> for ServerNotification {
    // 根据 method 字段路由到具体的通知类型
}
```

## 序列化特性

所有类型都支持：
- **Serde**: JSON 序列化/反序列化
- **JsonSchema**: JSON Schema 生成
- **TS**: TypeScript 类型定义生成

## 协议版本

- **Schema Version**: `2025-06-18`
- **JSON-RPC Version**: `2.0`

## 扩展性

协议支持实验性功能：
- `ClientCapabilities.experimental`
- `ServerCapabilities.experimental`

允许各方定义自定义能力而不破坏兼容性。
