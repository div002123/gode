# windows-sandbox-rs/src/process.rs

## 文件作用

Windows 进程创建和管理工具函数，支持使用受限令牌创建进程。

## 主要函数和方法

### `pub unsafe fn create_process_as_user(...) -> Result<(PROCESS_INFORMATION, STARTUPINFOW)>`
使用指定令牌创建进程
- 参数：
  - `h_token`: 安全令牌
  - `argv`: 命令参数
  - `cwd`: 工作目录
  - `env_map`: 环境变量
  - `logs_base_dir`: 日志目录
- 返回：进程信息和启动信息
- 注意：设置 `lpDesktop = "Winsta0\\Default"` 避免某些进程（如 PowerShell）失败

### `pub unsafe fn wait_process_and_exitcode(pi: &PROCESS_INFORMATION) -> Result<i32>`
等待进程结束并获取退出码

### `pub unsafe fn create_job_kill_on_close() -> Result<HANDLE>`
创建 Job 对象，设置 `KILL_ON_JOB_CLOSE` 标志

### `pub unsafe fn assign_to_job(h_job: HANDLE, h_process: HANDLE) -> Result<()>`
将进程分配到 Job 对象

### `pub fn make_env_block(env: &HashMap<String, String>) -> Vec<u16>`
创建 Unicode 环境块（按字母顺序排序）

### `fn quote_arg(a: &str) -> String`
引用命令行参数（处理空格、引号、反斜杠）

### `unsafe fn ensure_inheritable_stdio(si: &mut STARTUPINFOW) -> Result<()>`
确保标准 I/O 句柄可继承
