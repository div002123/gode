# windows-sandbox-rs/src/allow.rs

## 文件作用

计算沙盒模式下允许写入的路径列表，包括工作区目录和临时目录。

## 主要函数和方法

### `pub fn compute_allow_paths(...) -> Vec<PathBuf>`
根据沙盒策略、工作目录和环境变量计算允许写入的路径列表
- 参数：
  - `policy`: 沙盒策略
  - `_policy_cwd`: 策略工作目录（未使用）
  - `command_cwd`: 命令工作目录
  - `env_map`: 环境变量映射
- 返回：允许写入的路径列表（已去重）
- 逻辑：
  - `WorkspaceWrite` 模式：包含 command_cwd
  - 非 `ReadOnly` 模式：包含 TEMP/TMP 目录

## 主要结构体

无公共结构体。
