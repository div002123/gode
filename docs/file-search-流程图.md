# file-search 流程图

## 概述

`file-search` crate 实现了一个高性能的模糊文件名搜索工具。它使用 `nucleo_matcher` 进行模糊匹配，使用 `ignore` crate 实现并行文件遍历，并支持 gitignore 规则。

## 1. 文件搜索主流程

```mermaid
flowchart TD
    Start[接收搜索请求] --> ParseArgs[解析命令行参数]
    ParseArgs --> CheckPattern{提供了搜索模式?}

    CheckPattern -->|否| ShowListing[显示目录列表]
    CheckPattern -->|是| InitSearch[初始化搜索]

    ShowListing --> ExecuteLs[执行 ls 命令]
    ExecuteLs --> End[结束]

    InitSearch --> CreatePattern[创建模糊匹配模式]
    CreatePattern --> CalculateWorkers[计算工作线程数量]
    CalculateWorkers --> InitWorkerData[初始化每个工作线程的数据]

    InitWorkerData --> ConfigureWalkBuilder[配置文件遍历器]
    ConfigureWalkBuilder --> SetThreads[设置线程数]
    SetThreads --> ConfigureIgnore{尊重 gitignore?}

    ConfigureIgnore -->|是| EnableGitIgnore[启用 git 忽略规则]
    ConfigureIgnore -->|否| DisableGitIgnore[禁用 git 忽略规则]

    EnableGitIgnore --> CheckExclude{有排除模式?}
    DisableGitIgnore --> CheckExclude

    CheckExclude -->|是| BuildExcludePatterns[构建排除规则]
    CheckExclude -->|否| BuildWalker[构建并行遍历器]

    BuildExcludePatterns --> BuildWalker

    BuildWalker --> StartParallelWalk[启动并行文件遍历]
    StartParallelWalk --> WorkerLoop{遍历文件}

    WorkerLoop -->|每个文件| FilterEntry{是文件?}
    FilterEntry -->|是| MatchPattern[模糊匹配文件路径]
    FilterEntry -->|否| WorkerLoop

    MatchPattern --> CheckScore{匹配成功?}
    CheckScore -->|是| InsertToBestList[插入到最佳匹配列表]
    CheckScore -->|否| WorkerLoop

    InsertToBestList --> CheckCancel{检查取消标志}
    CheckCancel -->|已取消| StopWalk[停止遍历]
    CheckCancel -->|继续| WorkerLoop

    WorkerLoop -->|完成| MergeResults[合并各工作线程结果]
    StopWalk --> MergeResults

    MergeResults --> SortResults[排序结果]
    SortResults --> CheckComputeIndices{需要计算索引?}

    CheckComputeIndices -->|是| ComputeMatchIndices[计算匹配字符索引]
    CheckComputeIndices -->|否| FormatResults[格式化结果]

    ComputeMatchIndices --> FormatResults
    FormatResults --> OutputResults[输出结果]
    OutputResults --> CheckTruncated{结果被截断?}

    CheckTruncated -->|是| ShowTruncatedWarning[显示截断警告]
    CheckTruncated -->|否| End

    ShowTruncatedWarning --> End
```

## 2. 最佳匹配列表维护流程

```mermaid
flowchart TD
    Start[接收文件路径] --> ConvertToUTF32[转换为 UTF-32]
    ConvertToUTF32 --> ScoreMatch[使用模式计算匹配分数]

    ScoreMatch --> CheckMatch{有匹配分数?}
    CheckMatch -->|否| SkipFile[跳过文件]
    CheckMatch -->|是| IncrementCount[增加匹配计数]

    IncrementCount --> CheckHeapSize{堆已满?}
    CheckHeapSize -->|否| AddToHeap[直接添加到堆]
    CheckHeapSize -->|是| CompareWithMin{分数 > 最小分数?}

    CompareWithMin -->|是| ReplaceMin[移除最小元素]
    CompareWithMin -->|否| SkipFile

    ReplaceMin --> AddToHeap
    AddToHeap --> Done[完成]
    SkipFile --> Done
```

## 3. 工作线程并行处理流程

```mermaid
sequenceDiagram
    participant Main as 主线程
    participant Walker as 文件遍历器
    participant Worker1 as 工作线程 1
    participant Worker2 as 工作线程 2
    participant WorkerN as 工作线程 N

    Main->>Walker: 配置并启动并行遍历
    Walker->>Worker1: 分配文件子集 1
    Walker->>Worker2: 分配文件子集 2
    Walker->>WorkerN: 分配文件子集 N

    par 并行处理
        Worker1->>Worker1: 遍历并匹配文件
        Worker2->>Worker2: 遍历并匹配文件
        WorkerN->>WorkerN: 遍历并匹配文件
    end

    Note over Worker1,WorkerN: 每个工作线程维护自己的<br/>BestMatchesList 和匹配计数

    Worker1-->>Main: 返回局部最佳匹配列表
    Worker2-->>Main: 返回局部最佳匹配列表
    WorkerN-->>Main: 返回局部最佳匹配列表

    Main->>Main: 合并所有局部结果
    Main->>Main: 全局排序和截断
    Main->>Main: 输出最终结果
```

## 4. 结果合并和排序流程

```mermaid
flowchart TD
    Start[开始合并] --> InitGlobalHeap[初始化全局堆]
    InitGlobalHeap --> InitCount[初始化总匹配计数 = 0]

    InitCount --> WorkerLoop{遍历工作线程列表}
    WorkerLoop -->|每个工作线程| GetWorkerHeap[获取工作线程的堆]
    GetWorkerHeap --> AddCount[累加匹配计数]

    AddCount --> ItemLoop{遍历堆中的项}
    ItemLoop -->|每个项| CheckGlobalHeapSize{全局堆已满?}

    CheckGlobalHeapSize -->|否| AddToGlobal[添加到全局堆]
    CheckGlobalHeapSize -->|是| CompareScore{分数 > 全局最小?}

    CompareScore -->|是| ReplaceInGlobal[替换全局最小元素]
    CompareScore -->|否| SkipItem[跳过项]

    AddToGlobal --> ItemLoop
    ReplaceInGlobal --> ItemLoop
    SkipItem --> ItemLoop

    ItemLoop -->|完成| WorkerLoop

    WorkerLoop -->|完成| ExtractFromHeap[从堆提取为向量]
    ExtractFromHeap --> SortVector[排序向量]

    SortVector --> SortByScore[按分数降序]
    SortByScore --> SortByPath[分数相同时按路径升序]
    SortByPath --> ReturnResults[返回排序结果]
```

## 5. 模糊匹配索引计算流程

```mermaid
flowchart TD
    Start[接收匹配结果] --> InitMatcher[初始化 Matcher]
    InitMatcher --> ResultLoop{遍历结果}

    ResultLoop -->|每个结果| ConvertToUTF32[转换路径为 UTF-32]
    ConvertToUTF32 --> InitIndexVec[初始化索引向量]

    InitIndexVec --> CallIndices[调用 pattern.indices]
    CallIndices --> GetMatchIndices[获取匹配字符位置]

    GetMatchIndices --> SortIndices[排序索引]
    SortIndices --> DedupIndices[去重索引]

    DedupIndices --> AttachToResult[附加到结果对象]
    AttachToResult --> ResultLoop

    ResultLoop -->|完成| ReturnResults[返回带索引的结果]
```

## 6. 数据流图

```mermaid
flowchart LR
    subgraph Input[输入]
        Pattern[搜索模式]
        SearchDir[搜索目录]
        Options[选项参数]
    end

    subgraph Initialization[初始化层]
        PatternParser[模式解析器]
        WorkerAllocator[工作线程分配器]
        WalkBuilder[遍历器构建器]
    end

    subgraph Parallel[并行处理层]
        Worker1[工作线程 1<br/>BestMatchesList]
        Worker2[工作线程 2<br/>BestMatchesList]
        WorkerN[工作线程 N<br/>BestMatchesList]
    end

    subgraph Aggregation[聚合层]
        ResultMerger[结果合并器]
        Sorter[排序器]
        IndexComputer[索引计算器]
    end

    subgraph Output[输出]
        FileMatches[文件匹配列表]
        MatchCount[总匹配计数]
    end

    Pattern --> PatternParser
    SearchDir --> WalkBuilder
    Options --> WorkerAllocator
    Options --> WalkBuilder

    PatternParser --> Worker1
    PatternParser --> Worker2
    PatternParser --> WorkerN

    WalkBuilder --> Worker1
    WalkBuilder --> Worker2
    WalkBuilder --> WorkerN

    Worker1 --> ResultMerger
    Worker2 --> ResultMerger
    WorkerN --> ResultMerger

    ResultMerger --> Sorter
    Sorter --> IndexComputer
    IndexComputer --> FileMatches

    Worker1 -.计数.-> MatchCount
    Worker2 -.计数.-> MatchCount
    WorkerN -.计数.-> MatchCount
```

## 7. 取消机制流程

```mermaid
flowchart TD
    Start[搜索开始] --> SetupCancelFlag[设置取消标志]
    SetupCancelFlag --> DistributeFlag[分发标志给所有工作线程]

    DistributeFlag --> WorkerStart[工作线程开始处理]
    WorkerStart --> ProcessFiles[处理文件]

    ProcessFiles --> IncrementCounter[增加处理计数]
    IncrementCounter --> CheckInterval{达到检查间隔?}

    CheckInterval -->|是| ReadCancelFlag[读取取消标志]
    CheckInterval -->|否| ProcessFiles

    ReadCancelFlag --> IsCancelled{已取消?}
    IsCancelled -->|是| StopWorker[停止工作线程]
    IsCancelled -->|否| ProcessFiles

    StopWorker --> ReturnPartialResults[返回部分结果]

    ProcessFiles -->|完成所有文件| ReturnFullResults[返回完整结果]

    ReturnPartialResults --> CheckMainCancel[主线程检查取消标志]
    ReturnFullResults --> CheckMainCancel

    CheckMainCancel --> CancelledMain{主线程检测到取消?}
    CancelledMain -->|是| ReturnEmptyResults[返回空结果]
    CancelledMain -->|否| ProceedWithResults[继续处理结果]

    ReturnEmptyResults --> End[结束]
    ProceedWithResults --> End
```

## 8. 排除模式处理流程

```mermaid
flowchart TD
    Start[接收排除模式列表] --> CheckEmpty{列表为空?}
    CheckEmpty -->|是| SkipExclude[跳过排除配置]
    CheckEmpty -->|否| CreateBuilder[创建 OverrideBuilder]

    SkipExclude --> ReturnWalker[返回遍历器]

    CreateBuilder --> PatternLoop{遍历排除模式}
    PatternLoop -->|每个模式| AddPrefix[添加 ! 前缀]
    AddPrefix --> AddToBuilder[添加到构建器]

    AddToBuilder --> PatternLoop

    PatternLoop -->|完成| BuildOverrides[构建排除规则]
    BuildOverrides --> AttachToWalker[附加到遍历器]
    AttachToWalker --> ReturnWalker
```

## 关键决策点

1. **模式提供检查**：如果没有提供搜索模式，显示目录列表而不是搜索
2. **文件类型过滤**：只处理文件，跳过目录
3. **匹配分数评估**：只保留有匹配分数的文件（非 None）
4. **堆容量检查**：当堆未满时直接插入；已满时比较分数
5. **取消检查间隔**：每 1024 个文件检查一次取消标志以平衡性能和响应性
6. **索引计算选项**：仅在明确请求时计算匹配字符索引
7. **gitignore 尊重**：可配置是否遵循 git 忽略规则

## 性能优化

### 并行化策略

- 使用 `ignore` crate 的并行文件遍历
- 每个工作线程维护独立的 BestMatchesList
- 工作线程数 = 指定线程数 + 1（遍历器实现细节）

### 内存管理

- 使用二叉堆维护 top-N 结果
- 限制每个工作线程的内存使用
- UTF-32 转换缓冲区重用

### 取消检查优化

- 批量处理减少原子操作开销
- 检查间隔为 1024 个文件

## 匹配算法特性

### 模糊匹配配置

- **大小写匹配**：Smart（智能大小写）
- **标准化**：Smart（智能标准化）
- **原子类型**：Fuzzy（模糊匹配）

### 排序规则

1. 首先按匹配分数降序排列（分数高的在前）
2. 分数相同时按文件路径字母顺序排列（字典序）
