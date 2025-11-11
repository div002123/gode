# Codex 项目文档（除 core 外）

本目录包含除 `core` crate 之外所有代码文件的详细中文文档。

## 📚 文档组织

### 代码文件文档（按文件一一对应）
每个 `.rs` 源文件都有对应的 `.md` 文档，说明：
- 文件在整体中的作用
- 主要结构体和枚举
- 主要函数和方法

### 流程图文档
每个 crate 都有对应的运行机制流程图，使用 Mermaid 绘制。

---

## 🗂️ 文档索引

### app-server-protocol (8 个文件)

**代码文档：**
- [export.rs](./app-server-protocol-src-export.md) - 类型导出工具
- [jsonrpc_lite.rs](./app-server-protocol-src-jsonrpc_lite.md) - JSON-RPC 轻量实现
- [lib.rs](./app-server-protocol-src-lib.md) - 库入口
- [protocol/common.rs](./app-server-protocol-src-protocol-common.md) - 公共协议类型
- [protocol/mod.rs](./app-server-protocol-src-protocol-mod.md) - 协议模块
- [protocol/v1.rs](./app-server-protocol-src-protocol-v1.md) - V1 协议定义
- [protocol/v2.rs](./app-server-protocol-src-protocol-v2.md) - V2 协议定义

**流程图：**
- [app-server-protocol 流程图](./app-server-protocol-流程图.md)

---

### apply-patch (5 个文件)

**代码文档：**
- [lib.rs](./apply-patch-src-lib.md) - 补丁应用核心逻辑
- [main.rs](./apply-patch-src-main.md) - CLI 入口
- [parser.rs](./apply-patch-src-parser.md) - 补丁解析器
- [seek_sequence.rs](./apply-patch-src-seek_sequence.md) - 序列查找
- [standalone_executable.rs](./apply-patch-src-standalone_executable.md) - 独立可执行文件

**流程图：**
- [apply-patch 流程图](./apply-patch-流程图.md)

---

### async-utils (1 个文件)

**代码文档：**
- [lib.rs](./async-utils-src-lib.md) - 异步工具（可取消 Future）

**流程图：**
- [async-utils 流程图](./async-utils-流程图.md)

---

### file-search (3 个文件)

**代码文档：**
- [cli.rs](./file-search-src-cli.md) - CLI 参数定义
- [lib.rs](./file-search-src-lib.md) - 文件搜索核心
- [main.rs](./file-search-src-main.md) - 主入口

**流程图：**
- [file-search 流程图](./file-search-流程图.md)

---

### keyring-store (1 个文件)

**代码文档：**
- [lib.rs](./keyring-store-src-lib.md) - 跨平台凭据存储

**流程图：**
- [keyring-store 流程图](./keyring-store-流程图.md)

---

### mcp-types (1 个文件)

**代码文档：**
- [lib.rs](./mcp-types-src-lib.md) - MCP 协议类型定义（自动生成）

**流程图：**
- [mcp-types 流程图](./mcp-types-流程图.md)

---

### otel (4 个文件)

**代码文档：**
- [config.rs](./otel-src-config.md) - 配置类型
- [lib.rs](./otel-src-lib.md) - 库入口
- [otel_event_manager.rs](./otel-src-otel_event_manager.md) - 事件管理器
- [otel_provider.rs](./otel-src-otel_provider.md) - OpenTelemetry 提供者

**流程图：**
- [otel 流程图](./otel-流程图.md)

---

### protocol (14 个文件)

**代码文档：**
- [account.rs](./protocol-src-account.md) - 账户相关类型
- [approvals.rs](./protocol-src-approvals.md) - 审批类型
- [config_types.rs](./protocol-src-config_types.md) - 配置类型
- [conversation_id.rs](./protocol-src-conversation_id.md) - 对话 ID
- [custom_prompts.rs](./protocol-src-custom_prompts.md) - 自定义提示
- [items.rs](./protocol-src-items.md) - 项目类型
- [lib.rs](./protocol-src-lib.md) - 库入口
- [message_history.rs](./protocol-src-message_history.md) - 消息历史
- [models.rs](./protocol-src-models.md) - 模型相关
- [num_format.rs](./protocol-src-num_format.md) - 数字格式化
- [parse_command.rs](./protocol-src-parse_command.md) - 命令解析
- [plan_tool.rs](./protocol-src-plan_tool.md) - 计划工具
- [protocol.rs](./protocol-src-protocol.md) - 核心协议定义
- [user_input.rs](./protocol-src-user_input.md) - 用户输入

**流程图：**
- [protocol 流程图](./protocol-流程图.md)

---

### rmcp-client (8 个文件)

**代码文档：**
- [auth_status.rs](./rmcp-client-src-auth_status.md) - 认证状态
- [find_codex_home.rs](./rmcp-client-src-find_codex_home.md) - 配置目录查找
- [lib.rs](./rmcp-client-src-lib.md) - 库入口
- [logging_client_handler.rs](./rmcp-client-src-logging_client_handler.md) - 日志处理器
- [oauth.rs](./rmcp-client-src-oauth.md) - OAuth 令牌管理
- [perform_oauth_login.rs](./rmcp-client-src-perform_oauth_login.md) - OAuth 登录流程
- [rmcp_client.rs](./rmcp-client-src-rmcp_client.md) - MCP 客户端核心
- [utils.rs](./rmcp-client-src-utils.md) - 工具函数

**流程图：**
- [rmcp-client 流程图](./rmcp-client-流程图.md)

---

### windows-sandbox-rs (11 个文件)

**代码文档：**
- [acl.rs](./windows-sandbox-rs-src-acl.md) - ACL 管理
- [allow.rs](./windows-sandbox-rs-src-allow.md) - 允许路径
- [audit.rs](./windows-sandbox-rs-src-audit.md) - 审计日志
- [cap.rs](./windows-sandbox-rs-src-cap.md) - 能力管理
- [env.rs](./windows-sandbox-rs-src-env.md) - 环境变量
- [lib.rs](./windows-sandbox-rs-src-lib.md) - 库入口
- [logging.rs](./windows-sandbox-rs-src-logging.md) - 日志记录
- [policy.rs](./windows-sandbox-rs-src-policy.md) - 沙箱策略
- [process.rs](./windows-sandbox-rs-src-process.md) - 进程管理
- [token.rs](./windows-sandbox-rs-src-token.md) - 令牌处理
- [winutil.rs](./windows-sandbox-rs-src-winutil.md) - Windows 工具

**流程图：**
- [windows-sandbox-rs 流程图](./windows-sandbox-rs-流程图.md)

---

## 📊 统计

- **总文档数**：65 个
  - 代码文件文档：54 个
  - 流程图文档：10 个
  - 索引文档：1 个（流程图索引）

## 🔍 快速查找

### 按功能分类

#### 协议和通信
- app-server-protocol - 客户端服务器协议
- protocol - 核心协议定义
- mcp-types - MCP 协议类型

#### 工具
- apply-patch - 补丁应用
- file-search - 文件搜索

#### MCP 集成
- rmcp-client - MCP 客户端

#### 基础设施
- keyring-store - 凭据存储
- async-utils - 异步工具
- otel - 遥测

#### 平台特定
- windows-sandbox-rs - Windows 沙箱

## 📖 使用说明

### 查看代码文档
每个源文件都有对应的文档，文件名格式：
```
{crate-name}-src-{path}.md
```

例如：
- `apply-patch/src/lib.rs` → `apply-patch-src-lib.md`
- `protocol/src/protocol.rs` → `protocol-src-protocol.md`

### 查看流程图
每个 crate 都有运行机制流程图：
```
{crate-name}-流程图.md
```

流程图使用 Mermaid 绘制，在 GitHub 上可以直接查看，或使用支持 Mermaid 的 Markdown 查看器。

### 流程图索引
查看 [流程图索引](./流程图索引.md) 了解所有流程图的概述。

## 🛠️ 文档特点

- ✅ **中文编写**：所有文档使用中文
- ✅ **一对一映射**：每个代码文件对应一个文档
- ✅ **简洁明了**：重点突出，易于理解
- ✅ **结构清晰**：统一的文档结构
- ✅ **可视化**：包含流程图和关系图

## 📝 文档内容

每个代码文档包含：
1. **文件作用**：在整体项目中的角色
2. **主要结构体**：公共数据结构
3. **主要函数**：公共 API 和方法

每个流程图文档包含：
1. **主要流程**：核心运行机制
2. **数据流**：数据在组件间的流动
3. **状态转换**：关键状态变化
4. **特殊说明**：性能、安全等注意事项

## 🤝 贡献

欢迎改进文档：
- 修正错误
- 补充细节
- 添加示例
- 改进流程图

## 📄 许可

本文档随 Codex 项目发布。

---

**最后更新**：2025-11-11
**文档版本**：1.0
**覆盖范围**：除 core 外所有 crate
