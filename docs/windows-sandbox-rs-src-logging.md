# windows-sandbox-rs/src/logging.rs

## 文件作用

记录沙盒命令执行日志到 `sandbox_commands.rust.log` 文件。

## 主要常量

### `pub const LOG_FILE_NAME`
日志文件名："sandbox_commands.rust.log"

### `const LOG_COMMAND_PREVIEW_LIMIT`
命令预览长度限制：200 字符

## 主要函数和方法

### `pub fn log_start(command: &[String], base_dir: Option<&Path>)`
记录命令开始执行

### `pub fn log_success(command: &[String], base_dir: Option<&Path>)`
记录命令成功完成

### `pub fn log_failure(command: &[String], detail: &str, base_dir: Option<&Path>)`
记录命令执行失败及详情

### `pub fn debug_log(msg: &str, base_dir: Option<&Path>)`
调试日志（仅当 `SBX_DEBUG=1` 时输出）

### `pub fn log_note(msg: &str, base_dir: Option<&Path>)`
无条件记录注释信息

### `fn preview(command: &[String]) -> String`
生成命令预览（截断到 200 字符）

### `fn log_file_path(base_dir: &Path) -> Option<PathBuf>`
返回日志文件路径
