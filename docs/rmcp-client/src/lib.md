# rmcp-client/src/lib.rs

## 文件作用

MCP 客户端库的主模块，导出公共接口包括认证状态、OAuth 凭证管理和 RMCP 客户端。

## 主要导出

- `determine_streamable_http_auth_status`: 确定认证状态
- `supports_oauth_login`: 检测 OAuth 支持
- `McpAuthStatus`: 认证状态枚举
- `OAuthCredentialsStoreMode`: OAuth 凭证存储模式
- `StoredOAuthTokens`: 存储的 OAuth 令牌
- `save_oauth_tokens`: 保存 OAuth 令牌
- `delete_oauth_tokens`: 删除 OAuth 令牌
- `perform_oauth_login`: 执行 OAuth 登录
- `RmcpClient`: RMCP 客户端

## 主要模块

- `auth_status`: 认证状态检测
- `find_codex_home`: 配置目录查找
- `logging_client_handler`: 日志客户端处理器
- `oauth`: OAuth 凭证管理
- `perform_oauth_login`: OAuth 登录流程
- `rmcp_client`: RMCP 客户端实现
- `utils`: 工具函数
