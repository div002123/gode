# protocol/src/config_types.rs

## 文件作用

定义配置相关的类型，包括推理强度、推理摘要、详细程度、沙盒模式等。

## 主要枚举

### `pub enum ReasoningEffort`
- 推理强度
- 变体：Minimal, Low, Medium (默认), High

### `pub enum ReasoningSummary`
- 推理摘要模式
- 变体：Auto (默认), Concise, Detailed, None

### `pub enum Verbosity`
- 详细程度（GPT-5 模型）
- 变体：Low, Medium (默认), High

### `pub enum SandboxMode`
- 沙盒模式
- 变体：
  - `ReadOnly` (默认): 只读
  - `WorkspaceWrite`: 工作区可写
  - `DangerFullAccess`: 完全访问

### `pub enum ForcedLoginMethod`
- 强制登录方式
- 变体：Chatgpt, Api

## 主要函数和方法

无公共函数。该文件主要定义枚举类型。
