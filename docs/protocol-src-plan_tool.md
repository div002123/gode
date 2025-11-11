# protocol/src/plan_tool.rs

## 文件作用

定义 TODO 工具的参数类型，用于管理任务计划和步骤状态。

## 主要结构体

### `pub struct UpdatePlanArgs`
- 更新计划参数
- 字段：
  - `explanation`: Option<String>
  - `plan`: Vec<PlanItemArg>

### `pub struct PlanItemArg`
- 计划项参数
- 字段：
  - `step`: String
  - `status`: StepStatus

## 主要枚举

### `pub enum StepStatus`
- 步骤状态
- 变体：Pending, InProgress, Completed

## 主要函数和方法

无公共函数。该文件主要定义数据结构。
