# apply-patch 流程图

## 概述

`apply-patch` crate 实现了一个自定义的补丁格式解析器和应用器。它支持添加、删除和更新文件，并能处理直接调用和 bash heredoc 包装的补丁。

## 1. 补丁应用主流程

```mermaid
flowchart TD
    Start[接收命令参数] --> DetectFormat{检测调用格式}

    DetectFormat -->|"apply_patch [patch]"| DirectInvocation[直接调用]
    DetectFormat -->|bash -lc heredoc| HeredocInvocation[Heredoc 调用]
    DetectFormat -->|其他| NotApplyPatch[非补丁命令]

    DirectInvocation --> ParsePatch[解析补丁]
    HeredocInvocation --> ExtractHeredoc[提取 Heredoc 内容]
    ExtractHeredoc --> CheckCdCommand{包含 cd 命令?}

    CheckCdCommand -->|是| ExtractWorkdir[提取工作目录]
    CheckCdCommand -->|否| ParsePatch

    ExtractWorkdir --> ParsePatch

    ParsePatch --> ValidateSyntax{验证补丁语法}
    ValidateSyntax -->|有效| ParseHunks[解析补丁块]
    ValidateSyntax -->|无效| ReturnParseError[返回解析错误]

    ParseHunks --> ResolveRelativePaths[解析相对路径]
    ResolveRelativePaths --> VerifyPatches[验证补丁可应用性]

    VerifyPatches --> CheckFiles{检查文件状态}
    CheckFiles -->|文件存在且匹配| ComputeChanges[计算变更]
    CheckFiles -->|文件不存在/不匹配| ReturnVerifyError[返回验证错误]

    ComputeChanges --> ApplyToFileSystem[应用到文件系统]
    ApplyToFileSystem --> CheckSuccess{应用成功?}

    CheckSuccess -->|是| PrintSummary[打印摘要]
    CheckSuccess -->|否| ReturnApplyError[返回应用错误]

    PrintSummary --> End[完成]
```

## 2. 补丁解析流程

```mermaid
flowchart TD
    Start[开始解析补丁文本] --> CheckBoundaries{检查边界标记}

    CheckBoundaries -->|有 Begin/End| StrictMode[严格模式]
    CheckBoundaries -->|有 heredoc 包装| LenientMode[宽容模式]
    CheckBoundaries -->|都没有| ParseError[解析错误]

    LenientMode --> StripHeredoc[去除 heredoc 标记]
    StripHeredoc --> StrictMode

    StrictMode --> SplitLines[按行分割]
    SplitLines --> ParseLoop{还有行要处理?}

    ParseLoop -->|是| DetectHunkType{检测补丁块类型}
    ParseLoop -->|否| BuildResult[构建解析结果]

    DetectHunkType -->|*** Add File:| ParseAddHunk[解析添加文件块]
    DetectHunkType -->|*** Delete File:| ParseDeleteHunk[解析删除文件块]
    DetectHunkType -->|*** Update File:| ParseUpdateHunk[解析更新文件块]
    DetectHunkType -->|其他| HunkError[块格式错误]

    ParseAddHunk --> CollectAddedLines[收集添加的行]
    CollectAddedLines --> AddToHunkList[添加到块列表]

    ParseDeleteHunk --> AddToHunkList

    ParseUpdateHunk --> CheckMoveTo{有 Move to: 标记?}
    CheckMoveTo -->|是| ExtractMovePath[提取移动路径]
    CheckMoveTo -->|否| ParseChangeChunks[解析变更块]

    ExtractMovePath --> ParseChangeChunks

    ParseChangeChunks --> ParseChunkLoop{还有变更块?}
    ParseChunkLoop -->|是| ParseOneChunk[解析一个变更块]
    ParseChunkLoop -->|否| AddToHunkList

    ParseOneChunk --> CheckContext{有 @@ 上下文?}
    CheckContext -->|是| ExtractContext[提取上下文行]
    CheckContext -->|否| UseEmptyContext[使用空上下文]

    ExtractContext --> ParseDiffLines[解析差异行]
    UseEmptyContext --> ParseDiffLines

    ParseDiffLines --> CollectOldNew[收集旧行/新行]
    CollectOldNew --> CheckEOF{有 End of File 标记?}

    CheckEOF -->|是| MarkAsEOF[标记为文件末尾]
    CheckEOF -->|否| AddChunkToList[添加块到列表]

    MarkAsEOF --> AddChunkToList
    AddChunkToList --> ParseChunkLoop

    AddToHunkList --> ParseLoop

    BuildResult --> End[返回补丁结构]
```

## 3. Bash Heredoc 提取流程

```mermaid
flowchart TD
    Start[接收 bash -lc 脚本] --> ParseWithTreeSitter[使用 Tree-sitter 解析]

    ParseWithTreeSitter --> CheckAST{AST 解析成功?}
    CheckAST -->|失败| ReturnParseError[返回解析错误]
    CheckAST -->|成功| QueryAST[查询 AST]

    QueryAST --> MatchPattern{匹配模式}

    MatchPattern -->|"apply_patch <<'EOF'..."| DirectPattern[直接模式]
    MatchPattern -->|"cd path; apply_patch <<'EOF'..."| CdPattern[cd + 补丁模式]
    MatchPattern -->|不匹配| NotApplyPatchCommand[非 apply_patch 命令]

    DirectPattern --> ExtractHeredoc[提取 heredoc 正文]
    CdPattern --> ExtractCdPath[提取 cd 路径]
    ExtractCdPath --> ExtractHeredoc

    ExtractHeredoc --> ValidateUTF8{验证 UTF-8}
    ValidateUTF8 -->|有效| TrimNewlines[修剪尾部换行符]
    ValidateUTF8 -->|无效| UTF8Error[UTF-8 错误]

    TrimNewlines --> ReturnResult[返回 heredoc 内容和工作目录]
```

## 4. 补丁应用到文件系统流程

```mermaid
flowchart TD
    Start[接收补丁块列表] --> ProcessLoop{遍历补丁块}

    ProcessLoop -->|AddFile| HandleAdd[处理添加文件]
    ProcessLoop -->|DeleteFile| HandleDelete[处理删除文件]
    ProcessLoop -->|UpdateFile| HandleUpdate[处理更新文件]
    ProcessLoop -->|完成| CollectResults[收集结果]

    HandleAdd --> CreateParentDirs[创建父目录]
    CreateParentDirs --> WriteNewFile[写入新文件]
    WriteNewFile --> RecordAdded[记录已添加]
    RecordAdded --> ProcessLoop

    HandleDelete --> RemoveFile[删除文件]
    RemoveFile --> RecordDeleted[记录已删除]
    RecordDeleted --> ProcessLoop

    HandleUpdate --> ReadOriginalFile[读取原文件]
    ReadOriginalFile --> SplitLines[按行分割]
    SplitLines --> ComputeReplacements[计算替换]

    ComputeReplacements --> ApplyReplacements[应用替换]
    ApplyReplacements --> CheckMovePath{有移动路径?}

    CheckMovePath -->|是| WriteToDest[写入目标位置]
    CheckMovePath -->|否| WriteToOriginal[写入原位置]

    WriteToDest --> RemoveOriginal[删除原文件]
    RemoveOriginal --> RecordModified[记录已修改]

    WriteToOriginal --> RecordModified
    RecordModified --> ProcessLoop

    CollectResults --> BuildAffectedPaths[构建受影响路径]
    BuildAffectedPaths --> End[返回结果]
```

## 5. 变更块匹配和应用流程

```mermaid
flowchart TD
    Start[接收原文件行和变更块] --> InitLineIndex[初始化行索引 = 0]

    InitLineIndex --> ChunkLoop{遍历变更块}

    ChunkLoop -->|还有块| CheckContext{块有上下文行?}
    ChunkLoop -->|完成| SortReplacements[排序替换列表]

    CheckContext -->|是| SeekContext[在文件中查找上下文]
    CheckContext -->|否| ProcessOldLines[处理旧行]

    SeekContext --> ContextFound{找到上下文?}
    ContextFound -->|是| UpdateLineIndex[更新行索引]
    ContextFound -->|否| ContextError[上下文未找到错误]

    UpdateLineIndex --> ProcessOldLines

    ProcessOldLines --> CheckEmpty{旧行为空?}
    CheckEmpty -->|是| PureAddition[纯添加操作]
    CheckEmpty -->|否| SeekOldLines[查找旧行匹配]

    PureAddition --> DetermineInsertPoint[确定插入点]
    DetermineInsertPoint --> AddReplacement["添加替换 (idx, 0, new_lines)"]
    AddReplacement --> ChunkLoop

    SeekOldLines --> MatchFound{找到匹配?}
    MatchFound -->|是| RecordReplacement[记录替换]
    MatchFound -->|否| TryWithoutTrailing{旧行末尾是空行?}

    TryWithoutTrailing -->|是| RetryWithoutEmpty[去除空行重试]
    TryWithoutTrailing -->|否| MatchError[匹配失败错误]

    RetryWithoutEmpty --> SecondMatch{找到匹配?}
    SecondMatch -->|是| RecordReplacement
    SecondMatch -->|否| MatchError

    RecordReplacement --> UpdateLineIndex2[更新行索引]
    UpdateLineIndex2 --> ChunkLoop

    SortReplacements --> ApplyInReverse[逆序应用替换]
    ApplyInReverse --> ReplaceLoop{遍历替换}

    ReplaceLoop -->|还有| RemoveOldLines[删除旧行]
    ReplaceLoop -->|完成| ReturnNewLines[返回新行列表]

    RemoveOldLines --> InsertNewLines[插入新行]
    InsertNewLines --> ReplaceLoop
```

## 6. 统一差异生成流程

```mermaid
flowchart TD
    Start[接收文件路径和变更块] --> DeriveNewContent[派生新内容]

    DeriveNewContent --> ApplyChunksToOriginal[将变更块应用到原文件]
    ApplyChunksToOriginal --> GetNewContent[获取新内容]

    GetNewContent --> CreateTextDiff[创建文本差异对象]
    CreateTextDiff --> ConfigureContext[配置上下文半径]

    ConfigureContext --> GenerateUnifiedDiff[生成统一差异格式]
    GenerateUnifiedDiff --> FormatDiff[格式化差异输出]

    FormatDiff --> BuildResult[构建结果]
    BuildResult --> ReturnResult[返回统一差异和新内容]
```

## 7. 数据流图

```mermaid
flowchart LR
    subgraph Input[输入]
        CommandLine[命令行参数]
        PatchText[补丁文本]
    end

    subgraph Parsing[解析层]
        FormatDetector[格式检测器]
        HeredocExtractor[Heredoc 提取器]
        PatchParser[补丁解析器]
        HunkParser[补丁块解析器]
    end

    subgraph Validation[验证层]
        SyntaxValidator[语法验证器]
        FileChecker[文件检查器]
        PathResolver[路径解析器]
    end

    subgraph Application[应用层]
        ReplacementComputer[替换计算器]
        LineSeeker[行查找器]
        FileWriter[文件写入器]
    end

    subgraph Output[输出]
        AffectedPaths[受影响路径]
        Summary[操作摘要]
        Errors[错误信息]
    end

    CommandLine --> FormatDetector
    FormatDetector -->|直接| PatchParser
    FormatDetector -->|heredoc| HeredocExtractor
    HeredocExtractor --> PatchParser
    PatchParser --> HunkParser

    HunkParser --> SyntaxValidator
    SyntaxValidator --> PathResolver
    PathResolver --> FileChecker

    FileChecker --> ReplacementComputer
    ReplacementComputer --> LineSeeker
    LineSeeker --> FileWriter

    FileWriter --> AffectedPaths
    AffectedPaths --> Summary
    SyntaxValidator -.错误.-> Errors
    FileChecker -.错误.-> Errors
    FileWriter -.错误.-> Errors
```

## 8. 错误处理流程

```mermaid
flowchart TD
    Start[检测到错误] --> ClassifyError{分类错误}

    ClassifyError -->|解析错误| ParseError[ParseError]
    ClassifyError -->|I/O 错误| IoError[IoError]
    ClassifyError -->|计算错误| ComputeError[ComputeReplacements 错误]
    ClassifyError -->|隐式调用| ImplicitError[ImplicitInvocation 错误]

    ParseError --> DetermineType{确定解析错误类型}
    DetermineType -->|补丁无效| InvalidPatchError[InvalidPatchError]
    DetermineType -->|块无效| InvalidHunkError[InvalidHunkError]

    InvalidPatchError --> FormatErrorMessage[格式化错误消息]
    InvalidHunkError --> IncludeLineNumber[包含行号]
    IncludeLineNumber --> FormatErrorMessage

    IoError --> AddContext[添加上下文信息]
    AddContext --> FormatErrorMessage

    ComputeError --> ExplainMismatch[解释不匹配原因]
    ExplainMismatch --> FormatErrorMessage

    ImplicitError --> ProvideGuidance[提供修正指导]
    ProvideGuidance --> FormatErrorMessage

    FormatErrorMessage --> WriteToStderr[写入标准错误]
    WriteToStderr --> ReturnErrorCode[返回错误码]
```

## 9. 相对路径解析流程

```mermaid
flowchart TD
    Start[接收补丁和当前工作目录] --> CheckWorkdir{补丁中指定工作目录?}

    CheckWorkdir -->|是| ExtractWorkdir[提取工作目录]
    CheckWorkdir -->|否| UseCwd[使用当前工作目录]

    ExtractWorkdir --> CheckAbsolute{工作目录是绝对路径?}
    CheckAbsolute -->|是| UseWorkdirDirect[直接使用工作目录]
    CheckAbsolute -->|否| JoinWithCwd[与 cwd 拼接]

    JoinWithCwd --> SetEffectiveCwd[设置有效 cwd]
    UseWorkdirDirect --> SetEffectiveCwd
    UseCwd --> SetEffectiveCwd

    SetEffectiveCwd --> ProcessPaths{遍历补丁中的路径}

    ProcessPaths -->|还有路径| CheckPathType{路径是绝对还是相对?}
    ProcessPaths -->|完成| ReturnResolvedPaths[返回解析后的路径]

    CheckPathType -->|绝对| KeepAbsolute[保持绝对路径]
    CheckPathType -->|相对| ResolveRelative[用有效 cwd 解析]

    KeepAbsolute --> ProcessPaths
    ResolveRelative --> MakeAbsolute[生成绝对路径]
    MakeAbsolute --> ProcessPaths
```

## 关键决策点

1. **调用格式检测**：判断是直接调用、bash heredoc 还是其他命令
2. **工作目录确定**：从 heredoc 的 `cd` 命令或当前目录获取
3. **补丁语法验证**：检查 Begin/End 标记、块标记格式
4. **文件存在性检查**：对于更新和删除，文件必须存在；对于添加，文件不应存在
5. **上下文行匹配**：使用 `seek_sequence` 查找上下文行的位置
6. **旧行匹配**：精确匹配旧行内容，支持模糊匹配 Unicode 标点符号
7. **替换顺序**：必须逆序应用替换以避免索引偏移
8. **文件末尾处理**：特殊处理标记为 EOF 的块

## 特殊处理

### Unicode 标点符号模糊匹配

为了提高兼容性，查找器支持将某些 Unicode 标点符号（如 EN DASH、NON-BREAKING HYPHEN）与 ASCII 对应字符匹配。

### Heredoc 验证

使用 Tree-sitter Bash 解析器严格验证 heredoc 语法，仅接受以下模式：
- `apply_patch <<'EOF'\n...\nEOF`
- `cd <path> && apply_patch <<'EOF'\n...\nEOF`

### 移动操作

`Move to:` 标记支持在应用补丁的同时移动文件到新位置。
