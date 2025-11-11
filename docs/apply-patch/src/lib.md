# apply-patch/src/lib.rs

## 文件作用

提供统一的代码补丁应用库，支持解析和应用 unified diff 格式的补丁。

## 主要结构体

### `pub struct PatchApplication`
- 补丁应用结果
- 字段：
  - `succeeded`: Vec<PathBuf> - 成功应用的文件
  - `failed`: Vec<(PathBuf, String)> - 失败的文件及原因

## 主要函数和方法

### `pub fn apply_patch(content: &str, base_dir: &Path) -> Result<PatchApplication>`
应用 unified diff 格式的补丁到指定目录

### `pub fn parse_diff(content: &str) -> Result<Vec<FileDiff>>`
解析 diff 内容为文件差异列表

### `pub fn apply_hunks(file: &Path, hunks: &[Hunk]) -> Result<()>`
将代码块应用到指定文件
