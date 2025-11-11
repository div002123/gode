# windows-sandbox-rs/src/env.rs

## 文件作用

处理沙盒环境变量规范化和网络隔离配置。

## 主要函数和方法

### `pub fn normalize_null_device_env(env_map: &mut HashMap<String, String>)`
将 `/dev/null` 和 `\\dev\\null` 规范化为 Windows 的 `NUL`

### `pub fn ensure_non_interactive_pager(env_map: &mut HashMap<String, String>)`
确保使用非交互式分页器（GIT_PAGER、PAGER 设置为 `more.com`）

### `pub fn apply_no_network_to_env(env_map: &mut HashMap<String, String>) -> Result<()>`
应用网络隔离配置：
- 设置代理环境变量指向无效地址（127.0.0.1:9）
- 禁用包管理器的网络访问（pip, npm, cargo）
- 配置 Git 禁止网络协议
- 创建 ssh/scp 拒绝脚本（放在 PATH 前面）
- 允许 curl/wget 运行（但通过代理设置禁用网络）

### `fn prepend_path(env_map: &mut HashMap<String, String>, prefix: &str)`
将路径前置到 PATH 环境变量

### `fn reorder_pathext_for_stubs(env_map: &mut HashMap<String, String>)`
重新排序 PATHEXT，将 .BAT 和 .CMD 移到前面（确保拒绝脚本优先）

### `fn ensure_denybin(...) -> Result<PathBuf>`
创建拒绝脚本目录（默认 `~/.sbx-denybin`），生成 ssh.bat/cmd 和 scp.bat/cmd 脚本
