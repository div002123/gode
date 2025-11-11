# Codex 项目文档

欢迎查阅 Codex 项目的详细中文文档。本文档集覆盖了除 `core` 之外的所有 crate，全面介绍了各个组件的功能和使用方法。

## 📚 文档列表

### 协议和通信

1. **[app-server-protocol](./app-server-protocol.md)**
   - JSON-RPC 协议定义
   - 客户端-服务器通信接口
   - TypeScript 类型导出

2. **[protocol](./protocol.md)**
   - 核心协议定义
   - 提交-事件队列（SQ/EQ）
   - 消息和事件类型

3. **[mcp-types](./mcp-types.md)**
   - Model Context Protocol 类型
   - 自动生成的协议定义
   - 请求-响应模式

### 工具和功能

4. **[apply-patch](./apply-patch.md)**
   - 补丁应用工具
   - Unified diff 解析
   - 文件修改引擎

5. **[file-search](./file-search.md)**
   - 高性能文件搜索
   - 模糊匹配算法
   - 并行文件遍历

### MCP 集成

6. **[rmcp-client](./rmcp-client.md)**
   - MCP 客户端实现
   - 多种传输方式支持
   - OAuth 认证集成

### 安全和存储

7. **[keyring-store](./keyring-store.md)**
   - 跨平台凭据存储
   - 系统 Keyring 集成
   - OAuth 令牌管理

8. **[windows-sandbox-rs](./windows-sandbox-rs.md)**
   - Windows 沙箱实现
   - 进程隔离和限制
   - ACL 和令牌管理

### 基础设施

9. **[async-utils](./async-utils.md)**
   - 异步工具集
   - 可取消 Future
   - CancellationToken 扩展

10. **[otel](./otel.md)**
    - OpenTelemetry 集成
    - 可选遥测功能
    - 性能追踪

11. **[utils](./utils.md)**
    - 通用工具库集合
    - cache, git, image, pty
    - readiness, string, tokenizer

### 流程图和架构

12. **[运行机制流程图](./运行机制流程图.md)**
    - 10 张详细流程图
    - 数据流和架构图
    - 各组件交互流程

## 🗂️ 按功能分类

### 协议层
- **app-server-protocol** - 应用协议
- **protocol** - 核心协议
- **mcp-types** - MCP 协议

### 工具层
- **apply-patch** - 补丁应用
- **file-search** - 文件搜索

### 集成层
- **rmcp-client** - MCP 客户端
- **otel** - 遥测集成

### 基础设施层
- **keyring-store** - 安全存储
- **async-utils** - 异步工具
- **utils/** - 通用工具
- **windows-sandbox-rs** - 沙箱（Windows）

## 🚀 快速导航

### 我想了解...

- **项目整体架构** → 查看 [运行机制流程图](./运行机制流程图.md#1-项目整体架构)
- **协议定义** → 阅读 [protocol](./protocol.md) 和 [app-server-protocol](./app-server-protocol.md)
- **如何修改文件** → 参考 [apply-patch](./apply-patch.md)
- **如何搜索文件** → 浏览 [file-search](./file-search.md)
- **MCP 集成** → 查看 [rmcp-client](./rmcp-client.md) 和 [mcp-types](./mcp-types.md)
- **安全存储** → 参考 [keyring-store](./keyring-store.md)
- **工具库** → 阅读 [utils](./utils.md)

## 📊 Crate 关系图

```
┌─────────────────────────────────────────┐
│        应用层 (CLI/Server/Desktop)       │
└──────────────────┬──────────────────────┘
                   │
┌──────────────────▼──────────────────────┐
│      app-server-protocol (JSON-RPC)     │
└──────────────────┬──────────────────────┘
                   │
┌──────────────────▼──────────────────────┐
│      protocol (核心协议 SQ/EQ)          │
└──────────────────┬──────────────────────┘
                   │
┌──────────────────▼──────────────────────┐
│          codex-core (核心引擎)          │
└────┬────┬────┬────┬────┬────┬────┬─────┘
     │    │    │    │    │    │    │
     ▼    ▼    ▼    ▼    ▼    ▼    ▼
  apply file rmcp otel key async utils
  patch search client    ring utils
                         store
```

## 🔍 核心概念

### 协议模式

#### SQ/EQ (Submission Queue / Event Queue)
- **Submission**: 用户 → Codex
- **Event**: Codex → 用户
- 单向消息流，解耦通信

#### JSON-RPC
- 标准远程过程调用
- TypeScript 类型安全
- 易于集成

### 工具系统

#### 内置工具
- **apply_patch**: 应用代码补丁
- **file_search**: 搜索文件
- **shell**: 执行命令

#### MCP 工具
- 动态加载外部工具
- 支持子进程和 HTTP
- OAuth 认证支持

### 安全机制

#### 凭据存储
- 系统 Keyring 集成
- 加密存储敏感数据
- 跨平台支持

#### 沙箱
- Windows: ACL + 令牌限制
- Linux: Landlock (在 core 中)
- macOS: Seatbelt (在 core 中)

## 📖 使用场景

### 开发者集成

```rust
// 使用 file-search 查找文件
use codex_file_search::run;
let results = run("main", ".", vec![], 20, Some(4))?;

// 应用补丁
use codex_apply_patch::apply_patch;
apply_patch(&action, &cwd)?;

// 连接 MCP 服务器
use codex_rmcp_client::*;
let client = RmcpClient::new(transport);
let connected = client.connect().await?;
```

### 客户端开发

```typescript
// 使用 app-server-protocol 类型
import { NewConversationParams, InitializeParams } from './generated_types';

const params: NewConversationParams = {
  cwd: "/project",
  model: "claude-3-5-sonnet-20241022",
  // ...
};
```

## 🛠️ 开发指南

### 添加新的 MCP 工具
参考 [rmcp-client](./rmcp-client.md) 了解如何连接自定义 MCP 服务器。

### 扩展协议
查看 [protocol](./protocol.md) 和 [app-server-protocol](./app-server-protocol.md) 了解协议扩展方式。

### 贡献工具
参考 [apply-patch](./apply-patch.md) 和 [file-search](./file-search.md) 了解工具实现模式。

## 📝 文档约定

### 结构
每个 crate 文档包含：
1. **文件在整体的作用** - 在项目中的角色
2. **主要结构体** - 核心数据类型
3. **主要函数/方法** - API 接口
4. **使用示例** - 实际代码示例
5. **依赖关系** - 与其他 crate 的关系

### 代码示例
- 使用真实的 Rust 代码
- 包含错误处理
- 展示典型用法

### 流程图
- 使用 Mermaid 格式
- 清晰的节点标注
- 完整的流程覆盖

## 🤝 贡献

欢迎改进文档！贡献方式：
- 修正错误和不准确之处
- 补充更多使用示例
- 添加新的流程图
- 改进现有说明

## 📄 许可

本文档随 Codex 项目一起发布，遵循项目的许可协议。

---

**最后更新**: 2025-11-11
**文档版本**: 1.0
**覆盖范围**: 除 core 之外的所有 crate

## 🔗 相关链接

- **Core 文档**: 核心引擎文档（单独维护）
- **API 文档**: 运行 `cargo doc --open` 查看 Rustdoc
- **项目主页**: GitHub 仓库
