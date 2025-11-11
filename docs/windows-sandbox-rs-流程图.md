# windows-sandbox-rs 流程图

## 概述

`windows-sandbox-rs` crate 提供了 Windows 沙箱执行环境，使用 Windows 安全特性（如受限令牌、完整性级别、ACL 等）来隔离和限制进程权限。

## 1. 沙箱执行主流程

```mermaid
flowchart TD
    Start[接收执行请求] --> CheckPlatform{检查操作系统}

    CheckPlatform -->|Windows| WindowsImpl[使用 Windows 实现]
    CheckPlatform -->|非 Windows| StubImpl[使用存根实现]

    StubImpl --> ReturnUnsupported[返回不支持错误]

    WindowsImpl --> ParsePolicy[解析沙箱策略]
    ParsePolicy --> PreflightAudit[预检审计]

    PreflightAudit --> CheckAudit{审计通过?}
    CheckAudit -->|否| ReturnAuditError[返回审计错误]
    CheckAudit -->|是| LoadCapSids[加载或创建 CAP SID]

    LoadCapSids --> CreateToken[创建受限令牌]
    CreateToken --> SetupACL[设置文件 ACL]

    SetupACL --> ComputeAllowPaths[计算允许路径]
    ComputeAllowPaths --> ApplyACL[应用 ACL 规则]

    ApplyACL --> PrepareEnv[准备环境变量]
    PrepareEnv --> NormalizeNullDevice[标准化空设备]
    NormalizeNullDevice --> ApplyNoNetwork{禁用网络?}

    ApplyNoNetwork -->|是| RemoveNetworkEnv[移除网络相关环境变量]
    ApplyNoNetwork -->|否| SetupPager[设置非交互式分页器]

    RemoveNetworkEnv --> SetupPager
    SetupPager --> CreatePipes[创建管道]

    CreatePipes --> SetupStdHandles[设置标准句柄]
    SetupStdHandles --> BuildCommandLine[构建命令行]

    BuildCommandLine --> CreateProcessAsUser[调用 CreateProcessAsUserW]
    CreateProcessAsUser --> CheckCreation{创建成功?}

    CheckCreation -->|否| CleanupAndError[清理并返回错误]
    CheckCreation -->|是| CaptureOutput[捕获输出]

    CaptureOutput --> WaitProcess[等待进程完成]
    WaitProcess --> GetExitCode[获取退出码]

    GetExitCode --> CleanupHandles[清理句柄]
    CleanupHandles --> RestoreACL[恢复 ACL]
    RestoreACL --> ReturnResult[返回结果]

    ReturnAuditError --> End[结束]
    ReturnUnsupported --> End
    CleanupAndError --> End
    ReturnResult --> End
```

## 2. 受限令牌创建流程

```mermaid
flowchart TD
    Start[创建受限令牌] --> GetCurrentToken[获取当前进程令牌]
    GetCurrentToken --> DuplicateToken[复制令牌]

    DuplicateToken --> RemovePrivileges[移除特权]
    RemovePrivileges --> ListPrivileges[列举要移除的特权]

    ListPrivileges --> RemoveAdmin[移除管理员特权]
    RemoveAdmin --> RemoveNetwork[移除网络特权]
    RemoveNetwork --> RemoveDevice[移除设备访问特权]

    RemoveDevice --> SetIntegrityLevel[设置完整性级别]
    SetIntegrityLevel --> SetLowIntegrity[设置为 Low Integrity]

    SetLowIntegrity --> AddCapabilities[添加能力 SID]
    AddCapabilities --> RestrictSIDs[限制 SID]

    RestrictSIDs --> ReturnToken[返回受限令牌]
    ReturnToken --> End[完成]
```

## 3. ACL 权限管理流程

```mermaid
flowchart TD
    Start[设置文件 ACL] --> ReadCurrentACL[读取当前 ACL]
    ReadCurrentACL --> ParsePolicy[解析策略]

    ParsePolicy --> DetermineMode{确定沙箱模式}

    DetermineMode -->|ReadOnly| ComputeReadPaths[计算只读路径]
    DetermineMode -->|ReadWrite| ComputeRWPaths[计算读写路径]
    DetermineMode -->|NoAccess| DenyAll[拒绝所有访问]

    ComputeReadPaths --> AddReadACE[添加读取 ACE]
    ComputeRWPaths --> AddWriteACE[添加读写 ACE]

    AddReadACE --> ApplyCapSID[应用 CAP SID]
    AddWriteACE --> ApplyCapSID
    DenyAll --> ApplyCapSID

    ApplyCapSID --> SetFileSecurity[设置文件安全性]
    SetFileSecurity --> VerifyACL{验证 ACL}

    VerifyACL -->|成功| StoreOriginal[存储原始 ACL]
    VerifyACL -->|失败| RestoreACL[恢复原始 ACL]

    StoreOriginal --> End[完成]
    RestoreACL --> ReturnError[返回错误]
    ReturnError --> End
```

## 4. 管道和输出捕获流程

```mermaid
sequenceDiagram
    participant Parent as 父进程
    participant Pipe as 管道
    participant Child as 子进程（沙箱）

    Parent->>Pipe: 创建 stdin 管道
    Parent->>Pipe: 创建 stdout 管道
    Parent->>Pipe: 创建 stderr 管道

    Parent->>Pipe: 设置句柄继承标志
    Note over Pipe: 子进程端可继承<br/>父进程端不可继承

    Parent->>Child: CreateProcessAsUserW<br/>传递管道句柄

    par 并行读取
        Parent->>Pipe: 读取 stdout
        Parent->>Pipe: 读取 stderr
    end

    Child->>Pipe: 写入 stdout
    Child->>Pipe: 写入 stderr

    Pipe-->>Parent: stdout 数据
    Pipe-->>Parent: stderr 数据

    Child->>Parent: 进程退出
    Parent->>Pipe: 关闭所有管道
    Parent->>Parent: 返回捕获的输出
```

## 5. 环境变量处理流程

```mermaid
flowchart TD
    Start[准备环境变量] --> CopyEnv[复制当前环境]
    CopyEnv --> NormalizeNull[标准化空设备路径]

    NormalizeNull --> SetNUL[设置 NUL 设备变量]
    SetNUL --> CheckNoNetwork{禁用网络?}

    CheckNoNetwork -->|是| RemoveHTTP[移除 HTTP_PROXY]
    CheckNoNetwork -->|否| SetPager[设置 PAGER]

    RemoveHTTP --> RemoveHTTPS[移除 HTTPS_PROXY]
    RemoveHTTPS --> RemoveAllProxy[移除 ALL_PROXY]
    RemoveAllProxy --> SetPager

    SetPager --> SetPagerToLess[PAGER=less]
    SetPagerToLess --> SetLessOpts[LESS=-FRX]

    SetLessOpts --> SortEnv[按字母排序环境变量]
    SortEnv --> CreateEnvBlock[创建 UTF-16 环境块]

    CreateEnvBlock --> FormatBlock[格式化：KEY=VALUE\0...]
    FormatBlock --> AddDoubleNull[添加双空终止符]
    AddDoubleNull --> ReturnBlock[返回环境块]
```

## 6. 沙箱策略类型

```mermaid
graph TD
    SandboxPolicy[SandboxPolicy] --> Mode[mode: SandboxMode]
    SandboxPolicy --> AllowPaths[allow_paths: Vec PathBuf]
    SandboxPolicy --> DenyPaths[deny_paths: Vec PathBuf]
    SandboxPolicy --> NoNetwork[no_network: bool]

    Mode --> ReadOnly[ReadOnly<br/>只允许读取]
    Mode --> ReadWrite[ReadWrite<br/>允许读写]
    Mode --> NoAccess[NoAccess<br/>拒绝所有访问]

    AllowPaths --> AllowList[允许访问的路径列表]
    DenyPaths --> DenyList[明确拒绝的路径列表]

    style SandboxPolicy fill:#bbf,stroke:#333,stroke-width:3px
    style Mode fill:#fbb,stroke:#333,stroke-width:2px
```

## 7. CAP SID 管理流程

```mermaid
flowchart TD
    Start[需要 CAP SID] --> CheckCache{检查缓存}

    CheckCache -->|存在| LoadFromCache[从缓存加载]
    CheckCache -->|不存在| GenerateNew[生成新 SID]

    LoadFromCache --> VerifySID{验证 SID}
    VerifySID -->|有效| UseSID[使用 SID]
    VerifySID -->|无效| GenerateNew

    GenerateNew --> CreateRandomSID[创建随机 SID]
    CreateRandomSID --> SaveToFile[保存到文件]

    SaveToFile --> ApplyToFiles[应用到文件]
    ApplyToFiles --> UseSID

    UseSID --> ReturnSID[返回 SID]
    ReturnSID --> End[完成]
```

## 8. 审计流程

```mermaid
flowchart TD
    Start[预检审计] --> ScanPaths[扫描路径]
    ScanPaths --> CheckWritable{检查可写性}

    CheckWritable -->|Everyone 可写| LogWarning[记录警告]
    CheckWritable -->|安全| Continue[继续]

    LogWarning --> CheckCritical{关键路径?}
    CheckCritical -->|是| FailAudit[审计失败]
    CheckCritical -->|否| Continue

    Continue --> CheckNext{还有路径?}
    CheckNext -->|是| ScanPaths
    CheckNext -->|否| PassAudit[审计通过]

    FailAudit --> End[结束]
    PassAudit --> End
```

## 9. 数据流图

```mermaid
flowchart LR
    subgraph Caller[调用者]
        App[应用程序]
        Policy[沙箱策略]
    end

    subgraph Sandbox[沙箱层]
        PolicyParser[策略解析器]
        TokenCreator[令牌创建器]
        ACLManager[ACL 管理器]
        EnvBuilder[环境构建器]
    end

    subgraph Windows[Windows API]
        CreateProcess[CreateProcessAsUserW]
        SetSecurity[SetFileSecurity]
        PipeAPI[Pipe API]
    end

    subgraph SandboxedProcess[沙箱进程]
        ChildProcess[受限子进程]
        Output[输出捕获]
    end

    App --> Policy
    Policy --> PolicyParser
    PolicyParser --> TokenCreator
    PolicyParser --> ACLManager
    PolicyParser --> EnvBuilder

    TokenCreator --> CreateProcess
    ACLManager --> SetSecurity
    EnvBuilder --> CreateProcess

    CreateProcess --> ChildProcess
    ChildProcess --> PipeAPI
    PipeAPI --> Output
    Output --> App

    style Caller fill:#bbf
    style Sandbox fill:#bfb
    style Windows fill:#fbb
    style SandboxedProcess fill:#ffb
```

## 10. 错误处理流程

```mermaid
flowchart TD
    Start[检测错误] --> ClassifyError{错误类型}

    ClassifyError -->|创建令牌失败| LogTokenError[记录令牌错误]
    ClassifyError -->|ACL 操作失败| LogACLError[记录 ACL 错误]
    ClassifyError -->|进程创建失败| LogProcessError[记录进程创建错误]
    ClassifyError -->|其他| LogGenericError[记录通用错误]

    LogTokenError --> Cleanup[清理资源]
    LogACLError --> Cleanup
    LogProcessError --> Cleanup
    LogGenericError --> Cleanup

    Cleanup --> CloseHandles[关闭句柄]
    CloseHandles --> RestoreACLs[恢复 ACL]
    RestoreACLs --> FormatError[格式化错误消息]

    FormatError --> GetLastError[获取 GetLastError()]
    GetLastError --> ReturnError[返回错误]
```

## 关键决策点

1. **平台检查**：仅在 Windows 上启用，其他平台返回不支持
2. **沙箱模式**：根据策略决定文件访问权限级别
3. **网络限制**：可选择性禁用网络访问
4. **ACL 保护**：在操作前备份原始 ACL，失败时恢复
5. **审计门槛**：检测高风险配置并可选择性失败

## Windows 安全机制

### 1. 受限令牌（Restricted Token）
- 移除管理员特权
- 限制可访问的 SID
- 降低完整性级别

### 2. 完整性级别（Integrity Level）
- **Low**: 沙箱进程使用低完整性级别
- 无法访问高完整性级别的资源
- 符合 Windows UAC 和 UIPI 机制

### 3. ACL（访问控制列表）
- 控制文件和目录访问
- 使用 CAP SID 标识沙箱进程
- 支持读、写、执行权限

### 4. 能力 SID（Capability SID）
- 唯一标识沙箱会话
- 用于 ACL 规则
- 持久化以支持多次执行

## 安全特性

1. **最小权限原则**：只授予必要的权限
2. **默认拒绝**：除非明确允许，否则拒绝访问
3. **审计日志**：记录所有安全操作
4. **清理保证**：确保退出时恢复原始状态
5. **错误处理**：失败时安全地清理资源

## 性能考虑

1. **SID 缓存**：重用 CAP SID 避免重复生成
2. **ACL 批量操作**：减少系统调用次数
3. **异步输出捕获**：并行读取 stdout 和 stderr
4. **环境变量预处理**：一次性构建环境块

## 限制和注意事项

1. **仅限 Windows**：不支持其他操作系统
2. **权限要求**：需要足够权限来操作令牌和 ACL
3. **性能开销**：安全检查会增加启动时间
4. **兼容性**：依赖 Windows Vista 及以上版本的 API
