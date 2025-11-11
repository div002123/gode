# file-search/src/cli.rs

## 文件作用

定义文件搜索工具的命令行接口和参数解析。

## 主要结构体

### `pub struct CliArgs`
- 命令行参数
- 字段：
  - `pattern`: String - 搜索模式
  - `path`: Option<PathBuf> - 搜索路径
  - `ignore_case`: bool - 是否忽略大小写
  - `max_results`: Option<usize> - 最大结果数

## 主要函数和方法

### `pub fn parse_args() -> CliArgs`
解析命令行参数

### `pub fn build_cli() -> Command`
构建 CLI 命令定义
