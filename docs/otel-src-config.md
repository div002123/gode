# otel/src/config.rs

## 文件作用

OpenTelemetry 配置管理，定义 OTLP 导出器的配置选项。

## 主要结构体

### `pub struct OtelSettings`
- OTLP 设置
- 字段：
  - `service_name`: String
  - `service_version`: String
  - `environment`: String
  - `exporter`: OtelExporter

## 主要枚举

### `pub enum OtelExporter`
- OTLP 导出器类型
- 变体：
  - `None`: 不导出
  - `OtlpGrpc { endpoint, headers }`: gRPC 导出
  - `OtlpHttp { endpoint, headers, protocol }`: HTTP 导出

### `pub enum OtelHttpProtocol`
- HTTP 协议类型
- 变体：Binary, Json

## 主要函数和方法

无公共函数。该文件主要定义配置结构。
