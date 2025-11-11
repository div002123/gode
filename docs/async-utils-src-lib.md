# async-utils/src/lib.rs

## 文件作用

提供异步工具函数，特别是为 Future 添加取消支持。

## 主要 Trait

### `pub trait OrCancelExt`
- 为 Future 添加取消功能
- 关联类型：`Output`
- 方法：
  - `async fn or_cancel(self, token: &CancellationToken) -> Result<Self::Output, CancelErr>`

## 主要枚举

### `pub enum CancelErr`
- 取消错误
- 变体：`Cancelled`

## 主要实现

### `impl<F> OrCancelExt for F where F: Future + Send`
为所有满足条件的 Future 实现 OrCancelExt
- 使用 `tokio::select!` 等待 Future 或取消令牌
- 如果取消令牌触发，返回 `Err(CancelErr::Cancelled)`
- 否则返回 Future 的结果

## 使用示例

```rust
let token = CancellationToken::new();
let result = async_operation().or_cancel(&token).await;
```
