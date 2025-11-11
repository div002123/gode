# file-search/src/lib.rs

## 文件作用

文件搜索库的主模块，提供文件内容搜索和模式匹配功能。

## 主要结构体

### `pub struct SearchConfig`
- 搜索配置
- 字段：
  - `pattern`: String
  - `case_sensitive`: bool
  - `max_depth`: Option<usize>

### `pub struct SearchResult`
- 搜索结果
- 字段：
  - `file_path`: PathBuf
  - `line_number`: usize
  - `line_content`: String
  - `match_start`: usize
  - `match_end`: usize

## 主要函数和方法

### `pub fn search(config: &SearchConfig, root: &Path) -> Result<Vec<SearchResult>>`
在指定目录中搜索匹配的文件内容

### `pub fn search_file(path: &Path, pattern: &str) -> Result<Vec<SearchResult>>`
在单个文件中搜索匹配内容
