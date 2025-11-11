# app-server-protocol 流程图

## 概述

`app-server-protocol` crate 定义了客户端与服务器之间的 JSON-RPC 通信协议。它支持协议的版本 v1（已废弃）和 v2（当前版本）。

## 1. JSON-RPC 消息处理流程

```mermaid
flowchart TD
    Start[接收 JSON 消息] --> Parse{解析消息类型}
    Parse -->|Request| HandleRequest[处理请求]
    Parse -->|Notification| HandleNotification[处理通知]
    Parse -->|Response| HandleResponse[处理响应]
    Parse -->|Error| HandleError[处理错误]

    HandleRequest --> ValidateRequest{验证请求参数}
    ValidateRequest -->|有效| ProcessRequest[处理业务逻辑]
    ValidateRequest -->|无效| ReturnError[返回错误响应]

    ProcessRequest --> Success{执行成功?}
    Success -->|是| ReturnResponse[返回成功响应]
    Success -->|否| ReturnError

    HandleNotification --> ValidateNotification{验证通知参数}
    ValidateNotification -->|有效| ProcessNotification[处理通知逻辑]
    ValidateNotification -->|无效| LogError[记录错误]

    HandleResponse --> MatchRequest{匹配原始请求}
    MatchRequest -->|匹配| ResolvePromise[解析 Promise]
    MatchRequest -->|不匹配| LogWarning[记录警告]

    HandleError --> MatchRequest
```

## 2. 客户端请求类型处理流程

```mermaid
flowchart LR
    ClientRequest[客户端请求] --> CheckMethod{检查方法名}

    CheckMethod -->|thread/start| ThreadStart[启动会话线程]
    CheckMethod -->|thread/resume| ThreadResume[恢复会话线程]
    CheckMethod -->|thread/archive| ThreadArchive[归档会话线程]
    CheckMethod -->|turn/start| TurnStart[启动新回合]
    CheckMethod -->|turn/interrupt| TurnInterrupt[中断回合]
    CheckMethod -->|account/login/start| Login[登录账户]
    CheckMethod -->|account/logout| Logout[登出账户]
    CheckMethod -->|model/list| ModelList[列出模型]

    ThreadStart --> ValidateParams[验证参数]
    ThreadResume --> ValidateParams
    ThreadArchive --> ValidateParams
    TurnStart --> ValidateParams
    TurnInterrupt --> ValidateParams
    Login --> ValidateParams
    Logout --> ValidateParams
    ModelList --> ValidateParams

    ValidateParams --> ExecuteAction[执行操作]
    ExecuteAction --> BuildResponse[构建响应]
    BuildResponse --> SerializeResponse[序列化响应]
    SerializeResponse --> SendResponse[发送响应]
```

## 3. 服务器通知推送流程

```mermaid
flowchart TD
    Event[系统事件发生] --> DetermineNotification{确定通知类型}

    DetermineNotification -->|线程启动| ThreadStarted[thread/started]
    DetermineNotification -->|回合启动| TurnStarted[turn/started]
    DetermineNotification -->|回合完成| TurnCompleted[turn/completed]
    DetermineNotification -->|项目启动| ItemStarted[item/started]
    DetermineNotification -->|项目完成| ItemCompleted[item/completed]
    DetermineNotification -->|消息增量| AgentMessageDelta[item/agentMessage/delta]
    DetermineNotification -->|命令输出| CommandOutputDelta[item/commandExecution/outputDelta]
    DetermineNotification -->|账户更新| AccountUpdated[account/updated]
    DetermineNotification -->|登录完成| LoginCompleted[account/login/completed]

    ThreadStarted --> BuildNotification[构建通知对象]
    TurnStarted --> BuildNotification
    TurnCompleted --> BuildNotification
    ItemStarted --> BuildNotification
    ItemCompleted --> BuildNotification
    AgentMessageDelta --> BuildNotification
    CommandOutputDelta --> BuildNotification
    AccountUpdated --> BuildNotification
    LoginCompleted --> BuildNotification

    BuildNotification --> SerializeNotification[序列化通知]
    SerializeNotification --> BroadcastToClients[广播给客户端]
    BroadcastToClients --> ClientsReceive[客户端接收]
```

## 4. 服务器请求客户端审批流程

```mermaid
sequenceDiagram
    participant Server as 服务器
    participant Protocol as 协议层
    participant Client as 客户端

    Server->>Protocol: 需要审批（补丁/命令）
    Protocol->>Protocol: 生成请求 ID
    Protocol->>Protocol: 构建请求参数

    alt 补丁审批
        Protocol->>Client: ApplyPatchApproval 请求
        Note over Client: 显示文件变更
        Client->>Protocol: 审批决策（批准/拒绝）
    else 命令执行审批
        Protocol->>Client: ExecCommandApproval 请求
        Note over Client: 显示命令和风险评估
        Client->>Protocol: 审批决策（批准/拒绝）
    end

    Protocol->>Server: 返回审批结果

    alt 批准
        Server->>Server: 执行操作
        Server->>Protocol: 发送成功通知
    else 拒绝
        Server->>Server: 取消操作
        Server->>Protocol: 发送取消通知
    end

    Protocol->>Client: 推送状态通知
```

## 5. 请求/响应匹配流程

```mermaid
flowchart TD
    SendRequest[客户端发送请求] --> GenerateID[生成唯一请求 ID]
    GenerateID --> StoreCallback[存储回调/Promise]
    StoreCallback --> SerializeRequest[序列化请求]
    SerializeRequest --> SendToServer[发送到服务器]

    ReceiveResponse[接收响应] --> ExtractID[提取请求 ID]
    ExtractID --> LookupCallback{查找回调}

    LookupCallback -->|找到| RemoveCallback[移除回调]
    LookupCallback -->|未找到| LogOrphan[记录孤立响应]

    RemoveCallback --> CheckType{检查响应类型}
    CheckType -->|成功| ResolveSuccess[解析成功结果]
    CheckType -->|错误| RejectError[拒绝错误]

    ResolveSuccess --> UpdateUI[更新 UI]
    RejectError --> ShowError[显示错误]
```

## 6. 数据流图

```mermaid
flowchart LR
    subgraph Client[客户端]
        UI[用户界面]
        ClientLogic[客户端逻辑]
    end

    subgraph Protocol[协议层]
        Serialize[序列化器]
        Deserialize[反序列化器]
        Validator[验证器]
    end

    subgraph Server[服务器]
        RequestHandler[请求处理器]
        NotificationEmitter[通知发射器]
        ServerLogic[服务器逻辑]
    end

    UI -->|用户操作| ClientLogic
    ClientLogic -->|请求对象| Serialize
    Serialize -->|JSON 字符串| RequestHandler
    RequestHandler -->|处理| ServerLogic
    ServerLogic -->|响应对象| Serialize
    Serialize -->|JSON 字符串| Deserialize
    Deserialize -->|响应对象| ClientLogic
    ClientLogic -->|更新| UI

    ServerLogic -->|事件| NotificationEmitter
    NotificationEmitter -->|通知对象| Serialize
    Serialize -->|JSON 字符串| Deserialize
    Deserialize -->|通知对象| ClientLogic
```

## 7. 协议版本切换流程

```mermaid
flowchart TD
    Start[连接建立] --> Negotiate{协商协议版本}

    Negotiate -->|客户端支持 v2| UseV2[使用 v2 协议]
    Negotiate -->|仅支持 v1| UseV1[使用 v1 协议]

    UseV2 --> V2Methods[v2 方法路由]
    V2Methods --> ThreadAPI[thread/* 方法]
    V2Methods --> TurnAPI[turn/* 方法]
    V2Methods --> AccountAPI[account/* 方法]
    V2Methods --> ModelAPI[model/* 方法]

    UseV1 --> V1Methods[v1 方法路由]
    V1Methods --> ConversationAPI[conversation 方法]
    V1Methods --> AuthAPI[认证方法]
    V1Methods --> ConfigAPI[配置方法]

    ThreadAPI --> ProcessRequest[处理请求]
    TurnAPI --> ProcessRequest
    AccountAPI --> ProcessRequest
    ModelAPI --> ProcessRequest
    ConversationAPI --> ProcessRequest
    AuthAPI --> ProcessRequest
    ConfigAPI --> ProcessRequest
```

## 8. 错误处理流程

```mermaid
flowchart TD
    Error[发生错误] --> DetermineSource{确定错误来源}

    DetermineSource -->|解析错误| ParseError[解析错误]
    DetermineSource -->|验证错误| ValidationError[验证错误]
    DetermineSource -->|业务逻辑错误| LogicError[业务逻辑错误]
    DetermineSource -->|系统错误| SystemError[系统错误]

    ParseError --> BuildErrorResponse[构建错误响应]
    ValidationError --> BuildErrorResponse
    LogicError --> BuildErrorResponse
    SystemError --> BuildErrorResponse

    BuildErrorResponse --> SetErrorCode[设置错误代码]
    SetErrorCode --> SetErrorMessage[设置错误消息]
    SetErrorMessage --> AddErrorData{需要附加数据?}

    AddErrorData -->|是| AttachData[附加错误数据]
    AddErrorData -->|否| SerializeError[序列化错误响应]
    AttachData --> SerializeError

    SerializeError --> SendError[发送错误响应]
    SendError --> LogError[记录错误日志]
```

## 关键决策点

1. **消息类型识别**：根据 JSON 字段确定是请求、通知、响应还是错误
2. **方法路由**：根据 `method` 字段路由到相应的处理器
3. **参数验证**：使用 JSON Schema 验证参数结构
4. **请求 ID 匹配**：使用请求 ID 匹配请求和响应
5. **审批决策**：客户端用户决定是否批准服务器的操作请求
6. **协议版本选择**：根据客户端能力选择使用 v1 或 v2 协议

## 类型系统关系

该 crate 使用 Rust 的类型系统和宏来定义协议：

- `JSONRPCMessage` - 所有消息的顶层枚举
- `ClientRequest` - 客户端发起的请求（使用 `client_request_definitions!` 宏生成）
- `ServerRequest` - 服务器发起的请求（使用 `server_request_definitions!` 宏生成）
- `ServerNotification` - 服务器推送的通知（使用 `server_notification_definitions!` 宏生成）
- `ClientNotification` - 客户端推送的通知（使用 `client_notification_definitions!` 宏生成）

所有类型都支持序列化/反序列化（Serde）、JSON Schema 生成（Schemars）和 TypeScript 类型导出（ts-rs）。
