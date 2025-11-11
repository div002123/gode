# windows-sandbox-rs/src/cap.rs

## 文件作用

管理沙盒能力 SID（Capability SID），用于区分只读和可写沙盒令牌。

## 主要结构体

### `pub struct CapSids`
- 能力 SID
- 字段：
  - `workspace`: String - 工作区写入能力 SID
  - `readonly`: String - 只读能力 SID

## 主要函数和方法

### `pub fn load_or_create_cap_sids(policy_cwd: &Path) -> CapSids`
从 `policy_cwd/.codex/cap_sid` 加载或创建能力 SID
- 如果文件存在且为 JSON 格式，解析并返回
- 如果文件存在但为纯文本（旧格式），将其作为 workspace SID，生成新的 readonly SID
- 否则生成两个随机 SID

### `pub fn cap_sid_file(policy_cwd: &Path) -> PathBuf`
返回能力 SID 文件路径：`policy_cwd/.codex/cap_sid`

### `fn make_random_cap_sid_string() -> String`
生成随机能力 SID 字符串，格式：`S-1-5-21-<random>-<random>-<random>-<random>`
