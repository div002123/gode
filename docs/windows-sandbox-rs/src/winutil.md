# windows-sandbox-rs/src/winutil.rs

## 文件作用

Windows 工具函数，提供字符串转换和错误格式化功能。

## 主要函数和方法

### `pub fn to_wide<S: AsRef<OsStr>>(s: S) -> Vec<u16>`
将字符串转换为 UTF-16 宽字符数组（以 null 结尾）

### `pub fn format_last_error(err: i32) -> String`
格式化 Win32 错误码为可读字符串
- 使用 FormatMessageW 获取系统错误描述
- 失败时返回 "Win32 error <code>"
