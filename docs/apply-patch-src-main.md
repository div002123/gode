# apply-patch/src/main.rs

## 文件作用

命令行工具入口，用于应用代码补丁到文件系统。

## 主要结构体

### `pub struct Args`
- 命令行参数
- 字段：
  - `patch_file`: PathBuf - 补丁文件路径
  - `target_dir`: PathBuf - 目标目录

## 主要函数和方法

### `pub fn main() -> Result<()>`
主函数，解析参数并应用补丁

### `pub async fn run(args: Args) -> Result<()>`
执行补丁应用的核心逻辑
