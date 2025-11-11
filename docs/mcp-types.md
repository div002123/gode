# mcp-types

## 文件在整体的作用

`mcp-types` 定义 Model Context Protocol (MCP) 的核心类型和请求/响应模式。这是一个**自动生成**的 crate，包含 MCP 协议的所有标准类型定义，用于与 LLM 上下文服务器通信。

## 自动生成

```rust
// 文件顶部注释
// This file is auto-generated. Run `./generate_mcp_types.py` to regenerate.
```

- 类型定义从 MCP schema 自动生成
- 不应手动编辑此文件
- 运行 `./generate_mcp_types.py` 重新生成

## 主要 Trait

### `ModelContextProtocolRequest`
```rust
pub trait ModelContextProtocolRequest {
    type Result;
    const METHOD: &'static str;
}
```
- MCP 请求定义 trait
- 每个请求类型关联一个响应类型

### `ModelContextProtocolNotification`
```rust
pub trait ModelContextProtocolNotification {
    const METHOD: &'static str;
}
```
- 单向 MCP 消息（无响应）

## 主要结构体（示例）

### `Annotations`
```rust
pub struct Annotations {
    pub audience: Option<Vec<Role>>,
    pub priority: Option<f64>,
}
```
- 消息注释元数据

### `AudioContent`
```rust
pub struct AudioContent {
    pub annotations: Option<Annotations>,
    pub data: String,
    pub mime_type: String,
    pub r#type: String,
}
```
- 音频内容类型

### `BlobResourceContents`
```rust
pub struct BlobResourceContents {
    pub blob: String,
    pub mime_type: Option<String>,
    pub uri: String,
}
```
- 二进制资源内容

### `CallToolRequest`
```rust
pub struct CallToolRequest {
    pub name: String,
    pub arguments: Option<HashMap<String, Value>>,
}
```
- 工具调用请求

### `CallToolResult`
```rust
pub struct CallToolResult {
    pub content: Vec<AnnotatedContent>,
    pub is_error: Option<bool>,
    pub _meta: Option<HashMap<String, Value>>,
}
```
- 工具调用结果

### `ListResourcesRequest` / `ListResourcesResult`
- 列出可用资源

### `ReadResourceRequest` / `ReadResourceResult`
- 读取资源内容

## 协议版本

### `MCP_SCHEMA_VERSION`
```rust
pub const MCP_SCHEMA_VERSION: &str = "2024-11-05";
```
- MCP schema 版本标识

### `JSONRPC_VERSION`
```rust
pub const JSONRPC_VERSION: &str = "2.0";
```
- JSON-RPC 版本

## 枚举类型

### `Role`
```rust
pub enum Role {
    User,
    Assistant,
}
```

### `AnnotatedContent`
- 带注释的内容（文本、图像、音频等）

## 请求-响应映射

| 请求类型 | 响应类型 | 方法名 |
|---------|---------|--------|
| `CallToolRequest` | `CallToolResult` | `tools/call` |
| `ListResourcesRequest` | `ListResourcesResult` | `resources/list` |
| `ReadResourceRequest` | `ReadResourceResult` | `resources/read` |
| `ListToolsRequest` | `ListToolsResult` | `tools/list` |

## 使用示例

```rust
use mcp_types::{CallToolRequest, CallToolResult, ModelContextProtocolRequest};

// 创建工具调用请求
let request = CallToolRequest {
    name: "search_files".to_string(),
    arguments: Some(HashMap::from([
        ("pattern".to_string(), json!("main")),
    ])),
};

// 方法名
assert_eq!(CallToolRequest::METHOD, "tools/call");

// 发送请求并接收结果
let result: CallToolResult = send_mcp_request(request).await?;
```

## 依赖关系

- `serde` / `serde_json`: 序列化
- 被 `codex-protocol` 和 `rmcp-client` 使用

## 在项目中的位置

```
┌──────────────────────┐
│  MCP Schema          │
│  (官方协议定义)       │
└──────────┬───────────┘
           │ generate_mcp_types.py
┌──────────▼───────────┐
│  mcp-types           │ ← 此 crate (自动生成)
└──────────┬───────────┘
           │ 使用
┌──────────▼───────────┐
│  rmcp-client         │
│  codex-protocol      │
└──────────────────────┘
```

## 重要提示

⚠️ **不要手动编辑此 crate 的代码**
- 所有类型都是自动生成的
- 修改将在下次生成时丢失
- 如需更改，修改 schema 或生成脚本
