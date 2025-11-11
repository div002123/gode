# otel/src/otel_provider.rs

## 文件作用

提供 OpenTelemetry 日志导出器的初始化和管理，支持 gRPC 和 HTTP 两种导出方式。

## 主要结构体

### `pub struct OtelProvider`
- OTLP 提供者
- 字段：`logger` (SdkLoggerProvider)

## 主要函数和方法

### `pub fn from(settings: &OtelSettings) -> Result<Option<Self>>`
从配置创建 OTLP 提供者

### `pub fn shutdown(&self)`
关闭 OTLP 提供者并清理资源
