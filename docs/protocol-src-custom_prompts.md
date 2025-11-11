# protocol/src/custom_prompts.rs

## 文件作用

定义自定义提示（Custom Prompt）的数据结构，用于 slash 命令系统。

## 主要常量

### `pub const PROMPTS_CMD_PREFIX`
- 自定义提示命令前缀："prompts"

## 主要结构体

### `pub struct CustomPrompt`
- 自定义提示
- 字段：
  - `name`: String - 提示名称
  - `path`: PathBuf - 提示文件路径
  - `content`: String - 提示内容
  - `description`: Option<String> - 描述
  - `argument_hint`: Option<String> - 参数提示

## 主要函数和方法

无公共函数。该文件主要定义数据结构。
