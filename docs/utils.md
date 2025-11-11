# utils

## 文件在整体的作用

`utils/` 目录包含多个实用工具子 crate，为项目提供各种基础功能：缓存、字符串处理、Git 操作、图像处理、PTY 管理、就绪检查和 tokenizer。

## 子 Crate 列表

### 1. codex-utils-cache
### 2. codex-utils-string
### 3. codex-git
### 4. codex-utils-image
### 5. codex-utils-pty
### 6. codex-utils-readiness
### 7. codex-utils-tokenizer

---

## 1. codex-utils-cache

### 作用
提供 LRU (Least Recently Used) 缓存实现，支持异步操作。

### 主要结构体
```rust
pub struct LruCache<K, V> {
    // 内部使用 lru crate
}
```

### 主要方法
- `new(capacity: usize) -> Self` - 创建指定容量的缓存
- `get(&mut self, key: &K) -> Option<&V>` - 获取值
- `put(&mut self, key: K, value: V)` - 插入值
- `contains(&self, key: &K) -> bool` - 检查 key 是否存在

### 使用场景
- 图像缓存
- API 响应缓存
- 文件内容缓存

---

## 2. codex-utils-string

### 作用
提供字符串处理的实用函数。

### 主要函数
- 字符串规范化
- 空白处理
- 编码转换
- 字符串匹配辅助

### 依赖
最小依赖，纯字符串操作

---

## 3. codex-git

### 作用
Git 操作工具，包括 ghost commits 支持。

### 主要功能
- Git 仓库信息提取
- Ghost commit 创建（不影响工作区的临时提交）
- 分支操作
- 提交历史查询

### 主要结构体
```rust
pub struct GitInfo {
    pub branch: String,
    pub commit: String,
    pub is_dirty: bool,
}
```

### Ghost Commits
- 临时提交，用于保存状态快照
- 不改变工作目录
- 用于 Codex 会话恢复

### 依赖
- `regex`: 解析 Git 输出
- `tempfile`: 临时文件
- `walkdir`: 目录遍历

---

## 4. codex-utils-image

### 作用
图像处理工具，包括加载、缩放、格式转换和 base64 编码。

### 主要函数

#### `load_and_resize(path: &Path, max_width, max_height) -> Result<Vec<u8>>`
- 加载图像并调整大小
- 保持宽高比
- 返回 PNG 格式

#### `to_base64(image_data: &[u8]) -> String`
- 将图像数据编码为 base64
- 用于 API 传输

#### `detect_format(data: &[u8]) -> Option<ImageFormat>`
- 检测图像格式（JPEG、PNG、GIF 等）

### 缓存集成
- 使用 `codex-utils-cache` 缓存处理后的图像
- 避免重复处理

### 依赖
- `image`: 图像处理库
- `base64`: Base64 编解码
- `codex-utils-cache`: 缓存

### 使用场景
- 上传截图到 AI
- 缩略图生成
- 格式转换

---

## 5. codex-utils-pty

### 作用
伪终端 (PTY) 管理，用于交互式命令执行。

### 主要结构体
```rust
pub struct PtySession {
    // PTY 会话
}
```

### 主要方法
- `spawn(command: &str, args: &[&str]) -> Result<Self>` - 启动 PTY 会话
- `read(&mut self) -> Result<Vec<u8>>` - 读取输出
- `write(&mut self, data: &[u8]) -> Result<()>` - 写入输入
- `resize(rows, cols)` - 调整终端大小

### 特性
- 支持彩色输出
- 支持交互式程序（如 vim、less）
- 保留 ANSI 转义序列

### 依赖
- `portable-pty`: 跨平台 PTY 支持
- `tokio`: 异步 I/O

### 使用场景
- Codex Shell 工具
- 交互式命令执行
- 实时输出流

---

## 6. codex-utils-readiness

### 作用
服务就绪检查工具，等待依赖服务启动完成。

### 主要 Trait
```rust
#[async_trait]
pub trait Readiness {
    async fn wait_ready(&self, timeout: Duration) -> Result<()>;
    async fn is_ready(&self) -> bool;
}
```

### 实现示例
```rust
impl Readiness for HttpService {
    async fn wait_ready(&self, timeout: Duration) -> Result<()> {
        let start = Instant::now();
        while start.elapsed() < timeout {
            if self.is_ready().await {
                return Ok(());
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
        Err(TimeoutError)
    }
}
```

### 使用场景
- MCP 服务器启动等待
- 数据库连接就绪
- HTTP 服务健康检查

### 依赖
- `async-trait`: 异步 trait
- `tokio`: 异步运行时
- `time`: 时间处理

---

## 7. codex-utils-tokenizer

### 作用
Token 计数和编码，用于计算 LLM token 使用量。

### 主要函数

#### `count_tokens(text: &str, model: &str) -> usize`
- 计算文本的 token 数量
- 支持不同模型的 tokenizer（GPT-4、Claude 等）

#### `encode(text: &str, model: &str) -> Vec<u32>`
- 将文本编码为 token IDs

#### `decode(tokens: &[u32], model: &str) -> String`
- 将 token IDs 解码为文本

### 支持的模型
- GPT-4 / GPT-3.5 (tiktoken cl100k_base)
- Claude (tiktoken 兼容)
- 自定义 tokenizer

### 依赖
- `tiktoken-rs`: Rust tiktoken 实现

### 使用场景
- 上下文窗口管理
- 成本估算
- Token 限制检查
- 自动截断

### 使用示例
```rust
use codex_utils_tokenizer::count_tokens;

let text = "Hello, world!";
let tokens = count_tokens(text, "gpt-4");
println!("Token 数量: {}", tokens);

if tokens > 4096 {
    // 需要截断
}
```

---

## 整体架构

```
┌────────────────────────────────────┐
│          codex-core                │
└──────────────┬─────────────────────┘
               │ 使用所有工具
    ┌──────────┼──────────┬──────────┬──────────┬──────────┬──────────┐
    │          │          │          │          │          │          │
┌───▼───┐ ┌───▼───┐ ┌───▼───┐ ┌───▼───┐ ┌───▼───┐ ┌───▼───┐ ┌───▼───┐
│ cache │ │string │ │  git  │ │ image │ │  pty  │ │readiness│ │tokenizer│
└───────┘ └───────┘ └───────┘ └───────┘ └───────┘ └───────┘ └───────┘
```

## 依赖关系

```
tokenizer → (计算 token 数)
    ↓
cache ← image (缓存图像)
    ↓
pty → (执行命令)
    ↓
git → (版本控制)
    ↓
readiness → (等待服务)
    ↓
string → (字符串处理)
```

## 使用建议

### 性能
- 使用 cache 避免重复计算
- tokenizer 缓存常见字符串的 token 数
- image 缓存调整后的图像

### 错误处理
- 所有工具都返回 `Result`
- 提供清晰的错误消息
- 支持优雅降级

### 测试
- 每个工具都有完整的单元测试
- Mock 实现用于集成测试
