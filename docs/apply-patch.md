# apply-patch

## 文件在整体的作用

`codex-apply-patch` 实现了补丁应用工具，能够解析和应用统一格式（unified diff）的补丁到文件系统。这允许 AI 助手通过生成补丁的方式修改代码文件，支持精确的多文件编辑。

## 主要结构体

### `ApplyPatchAction` (lib.rs)
```rust
pub struct ApplyPatchAction {
    pub changes: Vec<ApplyPatchFileChange>,
}
```
- 补丁应用后的结果
- 包含所有文件的变更列表

### `ApplyPatchFileChange`
```rust
pub enum ApplyPatchFileChange {
    Add { path: PathBuf, new_content: String },
    Delete { path: PathBuf },
    Update {
        path: PathBuf,
        old_content: String,
        new_content: String,
        diff: String,
    },
}
```
- 文件变更类型枚举
- 支持添加、删除、更新操作

### `Hunk` (parser.rs)
```rust
pub struct Hunk {
    pub old_start: usize,
    pub old_count: usize,
    pub new_start: usize,
    pub new_count: usize,
    pub lines: Vec<String>,
}
```
- 补丁块（hunk）表示
- 包含行号和具体变更

## 主要函数

### `maybe_parse_apply_patch(text: &str) -> Option<(String, String)>`
- 检测文本是否包含 apply_patch 命令
- 返回：`(patch_id, patch_content)`

### `maybe_parse_apply_patch_verified(text: &str, cwd: &Path) -> Result<ApplyPatchAction>`
- 解析并验证补丁
- 检查所有目标文件是否存在
- 生成文件变更预览

### `apply_patch(action: &ApplyPatchAction, cwd: &Path) -> Result<()>`
- 实际应用补丁到文件系统
- 创建/删除/更新文件

### `unified_diff_from_chunks(old_lines, new_lines, context) -> String`
- 从文本行生成统一 diff 格式
- 支持上下文行数配置

## 补丁格式示例

```diff
apply_patch abc123 <<'PATCH_EOF'
--- src/main.rs
+++ src/main.rs
@@ -10,6 +10,7 @@
 fn main() {
     println!("Hello");
+    println!("World");
 }
PATCH_EOF
```

## 工作流程

```mermaid
graph LR
    A[AI 生成补丁] --> B[maybe_parse_apply_patch]
    B --> C[maybe_parse_apply_patch_verified]
    C --> D[验证文件存在]
    D --> E[生成 ApplyPatchAction]
    E --> F[apply_patch]
    F --> G[写入文件系统]
```

## 特性

- ✅ 支持 Bash heredoc 格式
- ✅ 多文件同时修改
- ✅ 精确的行号匹配
- ✅ 模糊匹配（查找最佳位置）
- ✅ 冲突检测
- ✅ 预览变更（dry-run）

## 依赖关系

- `similar`: 文本差异算法
- `dunce`: 路径规范化
- `regex`: 模式匹配

## 在项目中的位置

```
Codex AI
  ↓ 生成补丁命令
codex-core
  ↓ 调用 apply_patch
apply-patch crate ← 此 crate
  ↓ 写入
文件系统
```
