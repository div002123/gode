# codex-core 文档索引

欢迎查阅 codex-core 的详细中文文档。本文档集全面介绍了 Codex 核心库的架构、模块和运行机制。

## 📚 文档列表

### 概览和架构

1. **[00-概览.md](./00-概览.md)**
   - 项目简介
   - 整体架构
   - 核心模块概述
   - 主要数据流
   - 关键特性

### 核心模块详解

2. **[01-conversation_manager.md](./01-conversation_manager.md)**
   - 对话管理器
   - 会话创建和恢复
   - 对话生命周期管理

3. **[02-client.md](./02-client.md)**
   - ModelClient 详解
   - AI API 通信
   - 流式响应处理
   - 重试和错误处理

4. **[03-codex.md](./03-codex.md)**
   - Codex 核心引擎
   - Session 内部架构
   - 提交-事件模式
   - 完整交互轮次

### 认证和安全

5. **[04-auth.md](./04-auth.md)**
   - 认证系统
   - OAuth 和 API Key
   - 令牌刷新机制
   - 凭证存储

### 工具系统

6. **[05-工具系统概览.md](./05-工具系统概览.md)**
   - 工具编排器
   - 工具注册和路由
   - 工具处理器
   - 并行执行
   - 沙箱和批准

### 其他重要模块

7. **[06-其他核心模块.md](./06-其他核心模块.md)**
   - 命令执行 (exec.rs)
   - 错误类型 (error.rs)
   - 命令解析 (parse_command.rs)
   - Git 信息 (git_info.rs)
   - 上下文管理 (context_manager/)
   - 状态管理 (state/)
   - 会话持久化 (rollout/)
   - MCP 连接 (mcp_connection_manager.rs)
   - 统一执行 (unified_exec/)
   - 沙箱 (sandboxing/)
   - 命令安全 (command_safety/)
   - 任务系统 (tasks/)
   - 配置管理 (config/)
   - 其他工具模块

### 运行机制

8. **[99-运行机制流程图.md](./99-运行机制流程图.md)**
   - 系统整体架构图
   - 完整交互流程
   - 会话生命周期
   - 工具执行流程
   - 认证和令牌管理
   - 上下文管理和压缩
   - MCP 集成流程
   - 数据流图
   - 并发模型
   - 错误处理流程

## 🚀 快速导航

### 我想了解...

- **整体架构** → 从 [00-概览.md](./00-概览.md) 开始
- **如何与 AI 通信** → 查看 [02-client.md](./02-client.md)
- **核心引擎如何工作** → 阅读 [03-codex.md](./03-codex.md)
- **认证系统** → 参考 [04-auth.md](./04-auth.md)
- **工具系统** → 浏览 [05-工具系统概览.md](./05-工具系统概览.md)
- **运行流程** → 查看 [99-运行机制流程图.md](./99-运行机制流程图.md)

### 按功能分类

#### 会话管理
- conversation_manager.rs - 对话管理
- codex.rs - 核心引擎
- state/ - 状态管理
- rollout/ - 持久化

#### 通信和认证
- client.rs - AI 客户端
- auth.rs - 认证管理
- model_provider_info.rs - 提供商配置

#### 工具和执行
- tools/ - 工具系统
- exec.rs - 命令执行
- unified_exec/ - 统一执行
- shell.rs - Shell 集成

#### 上下文和处理
- context_manager/ - 上下文管理
- parse_command.rs - 命令解析
- response_processing.rs - 响应处理

#### 安全
- sandboxing/ - 沙箱
- command_safety/ - 命令安全
- auth.rs - 认证

#### 集成
- mcp_connection_manager.rs - MCP 集成
- git_info.rs - Git 集成

## 📊 关键概念

### 核心抽象

- **Codex**: 整个系统的高层接口，提供提交-事件模型
- **Session**: 运行在后台的会话处理引擎
- **ModelClient**: 与 AI API 通信的客户端
- **ToolOrchestrator**: 协调工具执行的编排器
- **ContextManager**: 管理对话上下文和历史

### 设计模式

- **Actor 模式**: Codex 通过消息通道通信
- **流式处理**: AI 响应增量传输
- **编排器模式**: ToolOrchestrator 协调复杂流程
- **策略模式**: 沙箱和批准策略可配置
- **观察者模式**: 事件流通知外部

### 关键流程

1. **用户提交** → Codex → Session → ContextManager
2. **AI 请求** → ModelClient → Auth → API
3. **AI 响应** → 解析 → 工具调用 → ToolOrchestrator
4. **工具执行** → 批准检查 → 沙箱 → 执行 → 结果
5. **持久化** → RolloutRecorder → 磁盘

## 🔍 深入阅读

### 推荐学习路径

#### 初学者
1. 阅读概览了解整体架构
2. 查看流程图理解数据流
3. 学习 ConversationManager 了解如何创建会话
4. 研究 Codex 了解核心引擎

#### 进阶
1. 深入 ModelClient 了解 AI 通信细节
2. 研究工具系统了解扩展机制
3. 学习认证系统了解安全实现
4. 探索上下文管理了解 token 优化

#### 高级
1. 分析并发模型了解性能优化
2. 研究沙箱机制了解安全沙箱
3. 学习 MCP 集成了解协议实现
4. 探索错误处理了解容错设计

## 🛠️ 开发指南

### 添加新工具
参考 [05-工具系统概览.md](./05-工具系统概览.md) 的"扩展工具系统"章节。

### 自定义认证
参考 [04-auth.md](./04-auth.md) 了解认证接口。

### 配置选项
查看 [06-其他核心模块.md](./06-其他核心模块.md) 的配置管理部分。

## 📝 文档维护

本文档对应 codex-core 代码库的当前状态。如发现文档与代码不符，请：

1. 检查代码最新变更
2. 更新对应文档
3. 提交 Pull Request

## 🤝 贡献

欢迎改进文档！贡献方式：

- 修正错误
- 补充细节
- 添加示例
- 改进流程图
- 翻译文档

## 📄 许可

本文档随 Codex 项目一起发布，遵循项目的许可协议。

---

**最后更新**: 2025-11-11
**文档版本**: 1.0
**对应代码**: codex-core (latest)
