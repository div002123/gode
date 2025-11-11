# protocol/src/parse_command.rs

## 文件作用

解析命令字符串，识别命令类型（读取、列表、搜索等）。

## 主要枚举

### `pub enum ParsedCommand`
- 已解析的命令类型
- 变体：
  - `Read { cmd, name, path }`: 读取文件命令
  - `ListFiles { cmd, path }`: 列出文件命令
  - `Search { cmd, query, path }`: 搜索命令
  - `Unknown { cmd }`: 未知命令

## 主要函数和方法

无公共函数。该文件主要定义枚举类型。
