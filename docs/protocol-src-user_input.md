# protocol/src/user_input.rs

## 文件作用

定义用户输入类型，支持文本、图像（data URL）和本地图像路径。

## 主要枚举

### `pub enum UserInput`
- 用户输入类型
- 变体：
  - `Text { text }`: 文本输入
  - `Image { image_url }`: 预编码的 data URI 图像
  - `LocalImage { path }`: 本地图像路径（在序列化时会转换为 Image 变体）

## 主要函数和方法

无公共函数。该文件主要定义枚举类型。
