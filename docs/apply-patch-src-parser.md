# apply-patch/src/parser.rs

## 文件作用

解析 unified diff 格式的补丁文件，提取文件差异和代码块信息。

## 主要结构体

### `pub struct FileDiff`
- 文件差异
- 字段：
  - `old_path`: PathBuf
  - `new_path`: PathBuf
  - `hunks`: Vec<Hunk>

### `pub struct Hunk`
- 代码块（连续的修改区域）
- 字段：
  - `old_start`: usize
  - `old_lines`: usize
  - `new_start`: usize
  - `new_lines`: usize
  - `lines`: Vec<DiffLine>

### `pub enum DiffLine`
- 差异行类型
- 变体：Added, Removed, Context

## 主要函数和方法

### `pub fn parse_unified_diff(content: &str) -> Result<Vec<FileDiff>>`
解析 unified diff 格式字符串

### `pub fn parse_hunk_header(line: &str) -> Result<(usize, usize, usize, usize)>`
解析代码块头部信息
