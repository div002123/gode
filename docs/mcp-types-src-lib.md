# mcp-types/src/lib.rs

## 文件作用

定义 Model Context Protocol (MCP) 的核心类型定义，包括工具、资源、内容块等。

## 主要结构体

### `pub struct Tool`
- MCP 工具定义
- 字段：`name`, `description`, `input_schema`

### `pub struct Resource`
- MCP 资源定义
- 字段：`uri`, `name`, `description`, `mime_type`

### `pub struct ResourceTemplate`
- MCP 资源模板
- 字段：`uri_template`, `name`, `description`

### `pub struct CallToolResult`
- 工具调用结果
- 字段：`content`, `is_error`, `structured_content`

## 主要枚举

### `pub enum ContentBlock`
- 内容块类型
- 变体：TextContent, ImageContent, ResourceContent, EmbeddedResourceContent
