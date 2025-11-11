# windows-sandbox-rs/src/audit.rs

## 文件作用

审计文件系统，查找 Everyone 可写的目录，用于沙盒环境的安全预检。

## 主要函数和方法

### `pub fn audit_everyone_writable(...) -> Result<Vec<PathBuf>>`
扫描文件系统，查找 Everyone 可写的目录
- 参数：
  - `cwd`: 当前工作目录
  - `env`: 环境变量
  - `logs_base_dir`: 日志目录
- 返回：world-writable 目录列表
- 限制：
  - 最多 2 秒扫描时间
  - 最多检查 50000 个目录
  - 每个目录最多 1000 个子项
- 优先扫描 CWD 的直接子目录以快速发现工作区问题

### `unsafe fn path_has_world_write_allow(path: &Path) -> Result<bool>`
检查路径是否允许 Everyone 写入

### `unsafe fn dacl_quick_world_write_mask_allows(...) -> bool`
快速检查 DACL 是否包含 Everyone 的写入权限（基于掩码）

### `fn gather_candidates(...) -> Vec<PathBuf>`
收集要审计的候选路径（CWD、TEMP、PATH、系统根目录等）

## 主要常量

- `MAX_ITEMS_PER_DIR`: 1000
- `AUDIT_TIME_LIMIT_SECS`: 2
- `MAX_CHECKED_LIMIT`: 50000
- `SKIP_DIR_SUFFIXES`: ["/windows/installer", "/windows/registration", "/programdata"]
