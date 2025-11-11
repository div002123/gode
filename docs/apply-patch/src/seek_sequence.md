# apply-patch/src/seek_sequence.rs

## 文件作用

实现序列搜索算法，用于在文件内容中查找最佳匹配位置以应用补丁。

## 主要结构体

### `pub struct SeekMatch`
- 搜索匹配结果
- 字段：
  - `line_number`: usize - 匹配的行号
  - `score`: f64 - 匹配得分

## 主要函数和方法

### `pub fn seek_sequence(haystack: &[String], needle: &[String]) -> Option<SeekMatch>`
在文本中搜索最佳匹配序列

### `pub fn compute_similarity(a: &str, b: &str) -> f64`
计算两个字符串的相似度得分

### `pub fn fuzzy_match(lines: &[String], pattern: &[String], threshold: f64) -> Option<usize>`
模糊匹配文本行序列
