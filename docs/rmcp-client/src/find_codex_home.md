# rmcp-client/src/find_codex_home.rs

## 文件作用

查找 Codex 配置目录路径，优先使用 `CODEX_HOME` 环境变量，否则默认为 `~/.codex`。

## 主要函数和方法

### `pub(crate) fn find_codex_home() -> std::io::Result<PathBuf>`
返回 Codex 配置目录路径
- 如果设置了 `CODEX_HOME`，使用该值并规范化路径
- 否则返回 `~/.codex`（不验证目录是否存在）

## 主要结构体

无公共结构体。
