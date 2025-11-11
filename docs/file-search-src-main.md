# file-search/src/main.rs

## 文件作用

文件搜索工具的主入口，集成 CLI 参数解析和搜索执行。

## 主要结构体

无公共结构体。

## 主要函数和方法

### `pub fn main() -> Result<()>`
主函数，执行文件搜索

### `pub async fn run(args: CliArgs) -> Result<()>`
执行搜索的核心逻辑

### `pub fn print_results(results: &[SearchResult])`
格式化输出搜索结果
