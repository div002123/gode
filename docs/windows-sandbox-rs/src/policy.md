# windows-sandbox-rs/src/policy.rs

## 文件作用

定义沙盒策略类型，支持只读和工作区可写两种模式。

## 主要结构体

### `pub struct SandboxPolicy`
- 沙盒策略包装器
- 字段：`0` (SandboxMode)

### `pub struct PolicyJson`
- 策略 JSON 表示
- 字段：
  - `mode`: String
  - `workspace_roots`: Vec<String> (默认为空)

## 主要枚举

### `pub enum SandboxMode`
- 沙盒模式
- 变体：
  - `ReadOnly`: 只读模式
  - `WorkspaceWrite`: 工作区可写模式

## 主要函数和方法

### `impl SandboxPolicy`
- `pub fn parse(value: &str) -> Result<Self>`
  - 解析策略字符串（预设名称或 JSON）
  - 支持 "read-only" 和 "workspace-write" 预设
  - 支持 JSON 格式策略
