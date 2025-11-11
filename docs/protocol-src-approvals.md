# protocol/src/approvals.rs

## 文件作用

定义审批请求事件和风险评估相关的数据结构。

## 主要结构体

### `pub struct ExecApprovalRequestEvent`
- 执行审批请求事件
- 字段：
  - `call_id`: String
  - `command`: Vec<String>
  - `cwd`: PathBuf
  - `reason`: Option<String>
  - `risk`: Option<SandboxCommandAssessment>
  - `parsed_cmd`: Vec<ParsedCommand>

### `pub struct ApplyPatchApprovalRequestEvent`
- 补丁应用审批请求事件
- 字段：
  - `call_id`: String
  - `changes`: HashMap<PathBuf, FileChange>
  - `reason`: Option<String>
  - `grant_root`: Option<PathBuf>

### `pub struct SandboxCommandAssessment`
- 沙盒命令风险评估
- 字段：
  - `description`: String
  - `risk_level`: SandboxRiskLevel

## 主要枚举

### `pub enum SandboxRiskLevel`
- 风险等级
- 变体：Low, Medium, High

## 主要函数和方法

### `impl SandboxRiskLevel`
- `pub fn as_str(&self) -> &'static str`
