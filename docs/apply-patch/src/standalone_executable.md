# apply-patch/src/standalone_executable.rs

## 文件作用

提供独立可执行文件相关的工具函数，支持补丁工具的独立运行。

## 主要结构体

无公共结构体。

## 主要函数和方法

### `pub fn is_standalone_executable() -> bool`
检查当前是否作为独立可执行文件运行

### `pub fn get_executable_path() -> Result<PathBuf>`
获取当前可执行文件的路径

### `pub fn setup_panic_handler()`
设置 panic 处理器以改善错误报告
