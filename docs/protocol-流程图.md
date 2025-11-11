# protocol 流程图

## 概述

`protocol` crate (也称为 `codex-protocol`) 定义了 Codex 系统的核心协议类型和消息结构。它包含会话管理、消息历史、审批流程、工具调用等核心功能的类型定义。

## 1. 会话生命周期流程

```mermaid
flowchart TD
    Start[创建会话] --> GenerateID[生成 ConversationId]
    GenerateID --> InitConfig[初始化配置]

    InitConfig --> StartThread[启动线程]
    StartThread --> FirstTurn[第一个回合]

    FirstTurn --> ProcessInput[处理用户输入]
    ProcessInput --> CheckApproval{需要审批?}

    CheckApproval -->|是| RequestApproval[请求审批]
    CheckApproval -->|否| ExecuteAction[执行操作]

    RequestApproval --> WaitApproval[等待用户决策]
    WaitApproval --> CheckDecision{批准?}

    CheckDecision -->|是| ExecuteAction
    CheckDecision -->|否| CancelAction[取消操作]

    CancelAction --> NextTurn{更多回合?}
    ExecuteAction --> RecordHistory[记录到历史]

    RecordHistory --> CheckComplete{回合完成?}
    CheckComplete -->|否| ProcessInput
    CheckComplete -->|是| NextTurn

    NextTurn -->|是| FirstTurn
    NextTurn -->|否| ArchiveThread[归档线程]

    ArchiveThread --> End[结束]
```

## 2. 消息历史管理流程

```mermaid
flowchart TD
    Start[新消息到达] --> DetermineType{消息类型}

    DetermineType -->|UserMessage| AddUserMsg[添加用户消息]
    DetermineType -->|AssistantMessage| AddAssistantMsg[添加助手消息]
    DetermineType -->|ToolCall| AddToolCall[添加工具调用]
    DetermineType -->|ToolResult| AddToolResult[添加工具结果]

    AddUserMsg --> AppendToHistory[追加到历史]
    AddAssistantMsg --> AppendToHistory
    AddToolCall --> AppendToHistory
    AddToolResult --> AppendToHistory

    AppendToHistory --> CheckLimit{超过限制?}
    CheckLimit -->|是| TruncateOld[截断旧消息]
    CheckLimit -->|否| UpdateIndex[更新索引]

    TruncateOld --> UpdateIndex
    UpdateIndex --> PersistHistory{需要持久化?}

    PersistHistory -->|是| SaveToDisk[保存到磁盘]
    PersistHistory -->|否| ReturnHistory[返回历史]

    SaveToDisk --> ReturnHistory
    ReturnHistory --> End[完成]
```

## 3. 审批流程

```mermaid
sequenceDiagram
    participant Agent as 代理
    participant Protocol as 协议层
    participant Approval as 审批系统
    participant User as 用户

    Agent->>Protocol: 请求执行操作
    Protocol->>Protocol: 检查操作类型

    alt 需要审批
        Protocol->>Approval: 创建审批请求
        Approval->>User: 显示审批界面

        User->>Approval: 审批决策（批准/拒绝）
        Approval->>Protocol: 返回决策

        alt 批准
            Protocol->>Agent: 继续执行
            Agent->>Agent: 执行操作
            Agent->>Protocol: 返回结果
        else 拒绝
            Protocol->>Agent: 操作被拒绝
        end
    else 无需审批
        Protocol->>Agent: 直接执行
        Agent->>Agent: 执行操作
        Agent->>Protocol: 返回结果
    end
```

## 4. 命令解析流程

```mermaid
flowchart TD
    Start[接收命令字符串] --> Tokenize[词法分析]
    Tokenize --> ParseCommand[解析命令名]

    ParseCommand --> CheckKnown{已知命令?}
    CheckKnown -->|否| ReturnUnknown[返回未知命令错误]

    CheckKnown -->|是| ParseArgs[解析参数]
    ParseArgs --> ValidateArgs{验证参数}

    ValidateArgs -->|失败| ReturnArgError[返回参数错误]
    ValidateArgs -->|成功| ParseOptions[解析选项]

    ParseOptions --> ValidateOptions{验证选项}
    ValidateOptions -->|失败| ReturnOptionError[返回选项错误]
    ValidateOptions -->|成功| BuildCommand[构建命令对象]

    BuildCommand --> ReturnCommand[返回解析后的命令]

    ReturnUnknown --> End[结束]
    ReturnArgError --> End
    ReturnOptionError --> End
    ReturnCommand --> End
```

## 5. 用户输入处理流程

```mermaid
flowchart TD
    Start[接收用户输入] --> CheckType{输入类型}

    CheckType -->|纯文本| ProcessText[处理文本]
    CheckType -->|命令| ProcessCommand[处理命令]
    CheckType -->|多模态| ProcessMultimodal[处理多模态]

    ProcessText --> NormalizeText[标准化文本]
    NormalizeText --> DetectIntent[检测意图]

    ProcessCommand --> ParseCommand[解析命令]
    ParseCommand --> ValidateCommand{验证命令}

    ValidateCommand -->|有效| ExecuteCommand[执行命令]
    ValidateCommand -->|无效| ShowError[显示错误]

    ProcessMultimodal --> ExtractComponents[提取组件]
    ExtractComponents --> ProcessText

    DetectIntent --> BuildMessage[构建消息]
    ExecuteCommand --> BuildMessage
    ShowError --> BuildMessage

    BuildMessage --> AddToHistory[添加到历史]
    AddToHistory --> End[完成]
```

## 6. 工具调用流程

```mermaid
flowchart TD
    Start[代理决定调用工具] --> SelectTool[选择工具]
    SelectTool --> PrepareArgs[准备参数]

    PrepareArgs --> CheckApproval{需要审批?}
    CheckApproval -->|是| RequestApproval[请求审批]
    CheckApproval -->|否| InvokeTool[调用工具]

    RequestApproval --> WaitDecision[等待决策]
    WaitDecision --> CheckApproved{批准?}

    CheckApproved -->|否| RecordRejection[记录拒绝]
    CheckApproved -->|是| InvokeTool

    InvokeTool --> ExecuteTool[执行工具逻辑]
    ExecuteTool --> CheckResult{执行成功?}

    CheckResult -->|是| FormatResult[格式化结果]
    CheckResult -->|否| FormatError[格式化错误]

    FormatResult --> RecordSuccess[记录成功]
    FormatError --> RecordFailure[记录失败]

    RecordSuccess --> ReturnToAgent[返回给代理]
    RecordFailure --> ReturnToAgent
    RecordRejection --> ReturnToAgent

    ReturnToAgent --> End[完成]
```

## 7. 配置类型层次

```mermaid
graph TD
    Config[Config Types] --> AccountConfig[AccountConfig<br/>账户配置]
    Config --> ModelConfig[ModelConfig<br/>模型配置]
    Config --> ApprovalConfig[ApprovalConfig<br/>审批配置]
    Config --> CustomPrompts[CustomPrompts<br/>自定义提示]

    AccountConfig --> UserId[user_id]
    AccountConfig --> ApiKey[api_key]
    AccountConfig --> Preferences[preferences]

    ModelConfig --> ModelName[model_name]
    ModelConfig --> Parameters[parameters]
    ModelConfig --> Limits[limits]

    ApprovalConfig --> AutoApprove[auto_approve_list]
    ApprovalConfig --> RequireApproval[require_approval_list]

    CustomPrompts --> SystemPrompt[system_prompt]
    CustomPrompts --> UserPrompts[user_prompts]

    style Config fill:#bbf,stroke:#333,stroke-width:3px
```

## 8. 消息项类型

```mermaid
graph TD
    Item[Item 枚举] --> UserMessage[UserMessage<br/>用户消息]
    Item --> AssistantMessage[AssistantMessage<br/>助手消息]
    Item --> ToolCall[ToolCall<br/>工具调用]
    Item --> ToolResult[ToolResult<br/>工具结果]
    Item --> SystemMessage[SystemMessage<br/>系统消息]
    Item --> ErrorMessage[ErrorMessage<br/>错误消息]

    UserMessage --> Text[text: String]
    UserMessage --> Attachments[attachments: Vec]

    AssistantMessage --> Content[content: String]
    AssistantMessage --> Metadata[metadata: Map]

    ToolCall --> ToolName[tool_name: String]
    ToolCall --> Arguments[arguments: JSON]

    ToolResult --> Output[output: String]
    ToolResult --> Success[success: bool]

    style Item fill:#fbb,stroke:#333,stroke-width:3px
```

## 9. 计划工具流程

```mermaid
flowchart TD
    Start[激活计划工具] --> AnalyzeTask[分析任务]
    AnalyzeTask --> DecomposeTask[分解任务]

    DecomposeTask --> CreateSteps[创建步骤列表]
    CreateSteps --> EstimateComplexity[估算复杂度]

    EstimateComplexity --> AssignTools[分配工具]
    AssignTools --> BuildPlan[构建执行计划]

    BuildPlan --> ValidatePlan{验证计划}
    ValidatePlan -->|失败| RefineP lan[优化计划]
    ValidatePlan -->|成功| PresentPlan[呈现计划]

    RefinePlan --> BuildPlan

    PresentPlan --> UserReview{用户审查}
    UserReview -->|修改| UpdatePlan[更新计划]
    UserReview -->|批准| ExecutePlan[执行计划]

    UpdatePlan --> PresentPlan

    ExecutePlan --> StepLoop{遍历步骤}
    StepLoop -->|每个步骤| ExecuteStep[执行步骤]
    ExecuteStep --> RecordProgress[记录进度]
    RecordProgress --> StepLoop

    StepLoop -->|完成| ReturnResults[返回结果]
    ReturnResults --> End[完成]
```

## 10. 数据流图

```mermaid
flowchart LR
    subgraph Input[输入层]
        UserInput[用户输入]
        Commands[命令]
    end

    subgraph Protocol[协议层]
        Parser[解析器]
        Validator[验证器]
        History[历史管理器]
        Approval[审批管理器]
    end

    subgraph Core[核心层]
        Agent[代理逻辑]
        Tools[工具系统]
        Models[模型接口]
    end

    subgraph Storage[存储层]
        ConvStorage[会话存储]
        ConfigStorage[配置存储]
    end

    UserInput --> Parser
    Commands --> Parser
    Parser --> Validator
    Validator --> History

    History --> Agent
    Agent --> Approval
    Approval --> Tools
    Agent --> Models

    History --> ConvStorage
    Validator --> ConfigStorage

    Tools --> Agent
    Models --> Agent
    Agent --> History
```

## 11. 会话ID生成流程

```mermaid
flowchart TD
    Start[需要新会话ID] --> GenerateUUID[生成 UUID v4]
    GenerateUUID --> AddPrefix[添加前缀]

    AddPrefix --> FormatID[格式化 ID]
    FormatID --> ValidateFormat{验证格式}

    ValidateFormat -->|有效| CheckUnique{检查唯一性}
    ValidateFormat -->|无效| GenerateUUID

    CheckUnique -->|唯一| ReturnID[返回 ConversationId]
    CheckUnique -->|冲突| GenerateUUID

    ReturnID --> End[完成]
```

## 关键决策点

1. **消息类型识别**：根据消息内容确定类型（用户、助手、工具等）
2. **审批门槛**：检查操作是否需要用户审批
3. **命令验证**：解析并验证命令参数和选项
4. **历史限制**：控制消息历史的长度
5. **持久化策略**：决定何时保存会话数据

## 核心类型

### ConversationId
- 唯一标识一个会话
- 格式：UUID 字符串
- 用于跟踪和检索会话

### MessageHistory
- 存储完整的消息序列
- 支持分页和截断
- 可序列化和持久化

### ApprovalRequest
- 描述需要审批的操作
- 包含操作详情和风险评估
- 关联用户决策

### CustomPrompts
- 用户定义的提示模板
- 支持变量替换
- 可按场景分类

## 协议扩展点

1. **自定义工具**：添加新工具实现
2. **审批策略**：自定义审批规则
3. **消息格式**：扩展消息类型
4. **存储后端**：可插拔的存储实现

## 安全考虑

1. **输入验证**：所有用户输入都经过验证
2. **审批机制**：危险操作需要明确批准
3. **历史审计**：完整记录操作历史
4. **权限检查**：根据用户角色限制功能

## 性能优化

1. **懒加载**：按需加载历史消息
2. **缓存**：缓存常用配置
3. **批量操作**：批量处理消息
4. **索引**：为快速检索建立索引
