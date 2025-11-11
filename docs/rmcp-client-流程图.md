# rmcp-client 流程图

## 概述

`rmcp-client` crate 实现了 MCP (Model Context Protocol) 客户端，支持通过 stdio 和 HTTP 两种传输方式连接到 MCP 服务器，并包含 OAuth 认证功能。

## 1. MCP 客户端连接流程

```mermaid
flowchart TD
    Start[创建 RmcpClient] --> DetectTransport{检测传输类型}

    DetectTransport -->|stdio| SetupStdio[设置 stdio 传输]
    DetectTransport -->|HTTP| SetupHTTP[设置 HTTP 传输]

    SetupStdio --> SpawnProcess[启动子进程]
    SpawnProcess --> ConnectStdio[连接 stdin/stdout]

    SetupHTTP --> CheckAuth{需要认证?}
    CheckAuth -->|是| LoadAuth[加载认证信息]
    CheckAuth -->|否| CreateHTTPClient[创建 HTTP 客户端]

    LoadAuth --> CheckAuthStatus{认证状态}
    CheckAuthStatus -->|已认证| CreateHTTPClient
    CheckAuthStatus -->|未认证| InitiateOAuth[启动 OAuth 流程]

    ConnectStdio --> InitializeProtocol[初始化协议]
    CreateHTTPClient --> InitializeProtocol
    InitiateOAuth --> InitializeProtocol

    InitializeProtocol --> SendInitialize[发送 initialize 请求]
    SendInitialize --> WaitResponse[等待响应]
    WaitResponse --> CheckResponse{响应有效?}

    CheckResponse -->|是| SendInitialized[发送 initialized 通知]
    CheckResponse -->|否| ReturnError[返回错误]

    SendInitialized --> ClientReady[客户端就绪]
    ClientReady --> End[完成]
    ReturnError --> End
```

## 2. OAuth 登录流程

```mermaid
sequenceDiagram
    participant User as 用户
    participant Client as RmcpClient
    participant Browser as 浏览器
    participant AuthServer as 认证服务器
    participant KeyringStore as 密钥环

    Client->>Client: 检测认证状态
    Client->>User: 提示需要登录
    Client->>Client: 生成 PKCE 挑战
    Client->>Client: 启动本地回调服务器

    Client->>Browser: 打开认证 URL
    Note over Browser: 用户登录<br/>授权应用

    Browser->>AuthServer: 用户授权
    AuthServer->>Browser: 重定向 + 授权码
    Browser->>Client: 回调本地服务器

    Client->>AuthServer: 交换令牌<br/>（授权码 + PKCE verifier）
    AuthServer->>Client: 返回访问令牌和刷新令牌

    Client->>KeyringStore: 保存令牌
    KeyringStore->>Client: 确认保存

    Client->>User: 登录成功
```

## 3. 请求/响应处理流程

```mermaid
flowchart TD
    Start[发送请求] --> GenerateID[生成请求 ID]
    GenerateID --> RegisterCallback[注册回调]

    RegisterCallback --> CheckTransport{传输类型}

    CheckTransport -->|stdio| WriteStdio[写入 stdin]
    CheckTransport -->|HTTP| SendHTTP[发送 HTTP POST]

    WriteStdio --> WaitCallback[等待回调]
    SendHTTP --> AddHeaders[添加认证头]
    AddHeaders --> WaitCallback

    WaitCallback --> SetTimeout[设置超时]
    SetTimeout --> Race{竞争}

    Race -->|响应到达| ProcessResponse[处理响应]
    Race -->|超时| CancelRequest[取消请求]

    ProcessResponse --> CheckError{是错误?}
    CheckError -->|是| ReturnError[返回错误]
    CheckError -->|否| DeserializeResult[反序列化结果]

    DeserializeResult --> RemoveCallback[移除回调]
    RemoveCallback --> ReturnResult[返回结果]

    CancelRequest --> RemoveCallback
    ReturnError --> End[完成]
    ReturnResult --> End
```

## 4. 令牌刷新流程

```mermaid
flowchart TD
    Start[HTTP 请求失败] --> CheckStatus{检查状态码}

    CheckStatus -->|401 Unauthorized| LoadTokens[加载刷新令牌]
    CheckStatus -->|其他| ReturnError[返回错误]

    LoadTokens --> CheckRefreshToken{有刷新令牌?}

    CheckRefreshToken -->|否| RequireLogin[需要重新登录]
    CheckRefreshToken -->|是| SendRefreshRequest[发送刷新请求]

    SendRefreshRequest --> CheckRefreshResponse{刷新成功?}

    CheckRefreshResponse -->|是| SaveNewTokens[保存新令牌]
    CheckRefreshResponse -->|否| RequireLogin

    SaveNewTokens --> RetryOriginalRequest[重试原请求]
    RetryOriginalRequest --> CheckRetry{重试成功?}

    CheckRetry -->|是| ReturnSuccess[返回成功]
    CheckRetry -->|否| ReturnError

    RequireLogin --> End[完成]
    ReturnSuccess --> End
    ReturnError --> End
```

## 5. stdio 传输数据流

```mermaid
flowchart LR
    subgraph ClientProcess[客户端进程]
        Client[RmcpClient]
        StdinWriter[Stdin Writer]
        StdoutReader[Stdout Reader]
    end

    subgraph ServerProcess[服务器进程]
        ServerStdin[Server Stdin]
        ServerStdout[Server Stdout]
        ServerLogic[服务器逻辑]
    end

    Client -->|请求| StdinWriter
    StdinWriter -->|JSON-RPC| ServerStdin
    ServerStdin --> ServerLogic

    ServerLogic --> ServerStdout
    ServerStdout -->|JSON-RPC| StdoutReader
    StdoutReader -->|响应| Client

    style ClientProcess fill:#bbf
    style ServerProcess fill:#bfb
```

## 6. HTTP 传输数据流

```mermaid
flowchart LR
    subgraph Client[客户端]
        RmcpClient[RmcpClient]
        HTTPClient[HTTP 客户端]
        TokenManager[令牌管理器]
    end

    subgraph Server[服务器]
        HTTPServer[HTTP 服务器]
        AuthMiddleware[认证中间件]
        MCPHandler[MCP 处理器]
    end

    RmcpClient --> TokenManager
    TokenManager -->|Bearer Token| HTTPClient
    HTTPClient -->|HTTPS Request| HTTPServer

    HTTPServer --> AuthMiddleware
    AuthMiddleware -->|验证令牌| MCPHandler
    MCPHandler -->|响应| HTTPClient
    HTTPClient -->|响应| RmcpClient

    style Client fill:#bbf
    style Server fill:#bfb
```

## 7. 认证状态管理

```mermaid
stateDiagram-v2
    [*] --> 未认证: 初始化
    未认证 --> OAuth流程中: 开始 OAuth
    OAuth流程中 --> 已认证: 获得令牌
    OAuth流程中 --> 未认证: 取消/失败
    已认证 --> 已认证: 请求成功
    已认证 --> 令牌过期: 401 错误
    令牌过期 --> 刷新中: 使用刷新令牌
    刷新中 --> 已认证: 刷新成功
    刷新中 --> 未认证: 刷新失败
    已认证 --> 未认证: 用户登出
    未认证 --> [*]: 关闭连接

    note right of OAuth流程中
        等待用户在浏览器中授权
    end note

    note right of 刷新中
        自动刷新访问令牌
    end note
```

## 8. 事件监听器模式

```mermaid
flowchart TD
    Start[服务器发送通知] --> ReceiveNotif[接收通知]
    ReceiveNotif --> ParseNotif[解析通知]

    ParseNotif --> CheckType{通知类型}

    CheckType -->|progress| HandleProgress[处理进度通知]
    CheckType -->|resources/updated| HandleResourceUpdate[处理资源更新]
    CheckType -->|tools/list_changed| HandleToolsChange[处理工具列表变更]
    CheckType -->|其他| HandleGeneric[通用处理]

    HandleProgress --> InvokeListeners[调用注册的监听器]
    HandleResourceUpdate --> InvokeListeners
    HandleToolsChange --> InvokeListeners
    HandleGeneric --> InvokeListeners

    InvokeListeners --> ListenerLoop{遍历监听器}
    ListenerLoop -->|每个监听器| CallListener[调用监听器回调]
    CallListener --> ListenerLoop

    ListenerLoop -->|完成| End[完成]
```

## 9. 工具调用流程

```mermaid
sequenceDiagram
    participant App as 应用
    participant Client as RmcpClient
    participant Server as MCP 服务器
    participant Tool as 工具实现

    App->>Client: call_tool(name, args)
    Client->>Client: 生成请求 ID
    Client->>Server: tools/call 请求

    Server->>Tool: 执行工具
    Tool->>Tool: 处理参数
    Tool->>Tool: 执行操作
    Tool->>Server: 返回结果

    Server->>Client: CallToolResult
    Client->>Client: 解析结果
    Client->>App: 返回内容

    alt 工具执行失败
        Tool->>Server: 返回错误
        Server->>Client: isError: true
        Client->>App: 返回错误结果
    end
```

## 关键决策点

1. **传输方式选择**：根据配置选择 stdio 或 HTTP 传输
2. **认证检测**：HTTP 传输需要检查并处理认证
3. **令牌刷新时机**：收到 401 错误时自动刷新
4. **超时处理**：每个请求都有超时保护
5. **回调管理**：使用请求 ID 匹配请求和响应

## 错误处理策略

### 连接错误
- stdio: 进程启动失败或崩溃
- HTTP: 网络错误或服务器不可达

### 认证错误
- 令牌过期：自动刷新
- 令牌无效：要求重新登录
- OAuth 失败：提示用户重试

### 协议错误
- 初始化失败：版本不兼容
- 请求超时：取消并清理
- 解析错误：记录并返回错误

## 安全特性

1. **PKCE 流程**：防止授权码拦截攻击
2. **令牌存储**：使用系统密钥环安全存储
3. **HTTPS 传输**：加密网络通信
4. **令牌刷新**：最小化令牌暴露时间

## 性能优化

1. **连接复用**：HTTP 客户端复用连接
2. **并发请求**：支持多个请求同时进行
3. **异步 I/O**：非阻塞通信
4. **回调索引**：快速查找待处理请求

## 使用模式

### 基本请求
```rust
let result = client.request(method, params).await?;
```

### 订阅通知
```rust
client.subscribe_notifications(|notif| {
    // 处理通知
}).await?;
```

### 工具调用
```rust
let result = client.call_tool("tool_name", args).await?;
```
