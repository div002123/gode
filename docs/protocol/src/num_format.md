# protocol/src/num_format.rs

## 文件作用

提供数字格式化功能，支持区域感知的千位分隔符和 SI 后缀（K, M, G）。

## 主要函数和方法

### `pub fn format_with_separators(n: i64) -> String`
使用区域感知的千位分隔符格式化数字（如 "12,345" for en-US）

### `pub fn format_si_suffix(n: i64) -> String`
使用 SI 后缀格式化数字，保留 3 位有效数字
- 示例：999 -> "999", 1200 -> "1.20K", 123456789 -> "123M"

## 主要结构体

无公共结构体。内部使用 ICU DecimalFormatter 实现区域感知格式化。
