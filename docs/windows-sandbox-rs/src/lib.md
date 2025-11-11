# windows-sandbox-rs/src/lib.rs

## 文件作用

Windows 沙盒库的主模块，提供受限权限执行命令的功能（仅在 Windows 上可用）。

## 主要结构体

### `pub struct CaptureResult`
- 捕获结果
- 字段：
  - `exit_code`: i32
  - `stdout`: Vec<u8>
  - `stderr`: Vec<u8>
  - `timed_out`: bool

## 主要函数和方法

### `pub fn run_windows_sandbox_capture(...) -> Result<CaptureResult>`
在沙盒中执行命令并捕获输出（仅 Windows）
- 参数：
  - `policy_json_or_preset`: 策略 JSON 或预设名称
  - `sandbox_policy_cwd`: 策略工作目录
  - `command`: 要执行的命令
  - `cwd`: 命令工作目录
  - `env_map`: 环境变量
  - `timeout_ms`: 超时（毫秒）
  - `logs_base_dir`: 日志目录
- 流程：
  1. 解析沙盒策略
  2. 规范化环境变量，应用网络隔离
  3. 创建受限令牌（ReadOnly 或 WorkspaceWrite）
  4. 为允许的路径添加 ACE
  5. 使用 CreateProcessAsUserW 启动进程
  6. 捕获 stdout/stderr
  7. 等待进程结束或超时
  8. 清理临时 ACE（非持久化模式）

### `pub fn preflight_audit_everyone_writable(...) -> Result<Vec<PathBuf>>`
预检审计，查找 world-writable 目录

## 平台支持

- Windows: 完整实现
- 非 Windows: 提供 stub 实现，调用时返回错误
