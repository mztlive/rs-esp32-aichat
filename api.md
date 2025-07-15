# AI 聊天系统 Web API 文档

## 概述

这是一个基于 Rust 开发的 AI 聊天系统的 Web API 文档。该系统提供了完整的聊天会话管理、消息流式传输、文件上传等功能。

### 基础信息

- **API 版本**: v1
- **基础 URL**: `http://localhost:3000/api`
- **协议**: HTTP/HTTPS
- **数据格式**: JSON
- **字符编码**: UTF-8

### 认证方式

所有 API 请求都需要在请求头中包含指纹认证：

```http
X-Fingerprint: <用户指纹标识>
```

## 响应格式

所有 API 响应都采用统一的 JSON 格式：

```json
{
  "status": 200, // 状态码：200(成功), 400(请求错误), 401(未授权), 403(禁止访问), 500(服务器错误)
  "data": {}, // 响应数据
  "message": null // 错误消息（成功时为空）
}
```

### 错误响应示例

```json
{
  "status": 404,
  "data": {},
  "message": "Session not found"
}
```

## API 端点

### 1. 聊天相关接口

#### 1.1 创建新会话

**请求**

```http
GET /api/chat/create?model=<模型名称>
```

**查询参数**

- `model` (可选): 指定使用的 AI 模型

**响应**

```json
{
  "status": 200,
  "data": {
    "session_id": "uuid-string"
  },
  "message": null
}
```

#### 1.2 流式发送消息

**请求**

```http
POST /api/chat/message/{session_id}
Content-Type: application/json
```

**路径参数**

- `session_id`: 会话 ID

**请求体**

```json
{
  "message": "用户消息内容",
  "files": ["file1.md", "file2.md"] // 可选：引用的文件列表
}
```

**响应**

```json
{
  "status": 200,
  "data": {},
  "message": null
}
```

> 注意：此接口触发流式响应，实际的 AI 回复通过 SSE 接口获取。

#### 1.3 提示消息（同步）

**请求**

```http
POST /api/chat/prompt/{session_id}
Content-Type: application/json
```

**路径参数**

- `session_id`: 会话 ID

**请求体**

```json
{
  "message": "用户消息内容",
  "files": ["file1.md", "file2.md"] // 可选：引用的文件列表
}
```

**响应**

```json
{
  "status": 200,
  "data": "AI 的回复内容",
  "message": null
}
```

#### 1.4 SSE 流式接收响应

**请求**

```http
GET /api/chat/sse/{session_id}
```

**路径参数**

- `session_id`: 会话 ID

**响应格式**

```
Content-Type: text/event-stream
Cache-Control: no-cache
Connection: keep-alive

data: {"type": "message", "content": "AI回复的部分内容"}

data: {"type": "complete", "message_id": "uuid"}
```

#### 1.5 获取会话历史

**请求**

```http
GET /api/session/history
```

**响应**

```json
{
  "status": 200,
  "data": [
    {
      "session_id": "uuid-string",
      "title": "会话标题",
      "created_at": "2024-01-01T00:00:00Z",
      "updated_at": "2024-01-01T00:00:00Z"
    }
  ],
  "message": null
}
```

#### 1.6 获取消息历史

**请求**

```http
GET /api/message/history/{session_id}
```

**路径参数**

- `session_id`: 会话 ID

**响应**

```json
{
  "status": 200,
  "data": [
    {
      "role": "user",
      "content": "用户消息"
    },
    {
      "role": "assistant",
      "content": "AI回复"
    }
  ],
  "message": null
}
```

#### 1.7 删除会话

**请求**

```http
DELETE /api/session/{session_id}
```

**路径参数**

- `session_id`: 会话 ID

**响应**

```json
{
  "status": 200,
  "data": {},
  "message": null
}
```

### 2. 文档相关接口

#### 2.1 获取所有文档类别

**请求**

```http
GET /api/all/document/category
```

**响应**

```json
{
  "status": 200,
  "data": ["知识库", "FAQ", "技术文档"],
  "message": null
}
```

### 3. 文件上传接口

#### 3.1 上传文件

**请求**

```http
POST /api/upload
Content-Type: multipart/form-data
```

**请求体**

- 支持多文件上传
- 当前支持的文件类型：PDF (`.pdf`)
- 文件大小限制：300MB

**响应**

```json
{
  "status": 200,
  "data": [
    {
      "original_name": "document.pdf",
      "server_name": "uuid.md",
      "content_type": "application/pdf",
      "size": 1024000,
      "path": "./uploads/uuid.md",
      "upload_time": "2024-01-01T00:00:00Z"
    }
  ],
  "message": null
}
```

> 注意：上传的 PDF 文件会自动转换为 Markdown 格式存储。

## 错误码说明

| 状态码 | 说明           | 常见原因                 |
| ------ | -------------- | ------------------------ |
| 200    | 成功           | 请求处理成功             |
| 400    | 请求错误       | 请求参数错误、格式不正确 |
| 401    | 未授权         | 缺少或无效的认证信息     |
| 403    | 禁止访问       | 权限不足                 |
| 404    | 资源不存在     | 会话不存在、文件不存在   |
| 500    | 服务器内部错误 | 系统异常、AI 服务异常    |

## 常见错误信息

- `"Session not found"`: 指定的会话 ID 不存在
- `"指纹为空"`: 请求头中缺少 X-Fingerprint
- `"不支持的文件类型"`: 上传了不支持的文件格式
- `"转换文档错误"`: PDF 转换 Markdown 失败

## 使用示例

### JavaScript/TypeScript 示例

```javascript
// 创建会话
async function createSession() {
  const response = await fetch("/api/chat/create", {
    method: "GET",
    headers: {
      "X-Fingerprint": "user-fingerprint",
    },
  });
  const result = await response.json();
  return result.data.session_id;
}

// 发送消息
async function sendMessage(sessionId, message) {
  const response = await fetch(`/api/chat/message/${sessionId}`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      "X-Fingerprint": "user-fingerprint",
    },
    body: JSON.stringify({ message }),
  });
  return await response.json();
}

// 监听 SSE 响应
function listenToSSE(sessionId) {
  const eventSource = new EventSource(`/api/chat/sse/${sessionId}`);

  eventSource.onmessage = function (event) {
    const data = JSON.parse(event.data);
    console.log("AI 回复:", data);
  };

  eventSource.onerror = function (event) {
    console.error("SSE 连接错误:", event);
  };
}

// 上传文件
async function uploadFile(file) {
  const formData = new FormData();
  formData.append("file", file);

  const response = await fetch("/api/upload", {
    method: "POST",
    headers: {
      "X-Fingerprint": "user-fingerprint",
    },
    body: formData,
  });

  return await response.json();
}
```

### curl 示例

```bash
# 创建会话
curl -X GET "http://localhost:3000/api/chat/create" \
  -H "X-Fingerprint: user-123"

# 发送消息
curl -X POST "http://localhost:3000/api/chat/message/session-id" \
  -H "Content-Type: application/json" \
  -H "X-Fingerprint: user-123" \
  -d '{"message": "你好，AI助手！"}'

# 获取会话历史
curl -X GET "http://localhost:3000/api/session/history" \
  -H "X-Fingerprint: user-123"

# 上传文件
curl -X POST "http://localhost:3000/api/upload" \
  -H "X-Fingerprint: user-123" \
  -F "file=@document.pdf"
```

## 配置说明

系统通过 `config.toml` 文件进行配置：

```toml
[app]
upload_dir = "./uploads"

[client]
api_key = "your-api-key"
chat_model = "deepseek-r1"

[embedding]
api_key = "your-embedding-api-key"
model = "text-embedding-v2"
dimensions = 1536

[document]
static_docs = []

[[document.categories]]
name = "知识库"
directory = "./docs"
```

## 部署说明

### 环境要求

- Rust 1.70+
- 支持的操作系统：Linux、macOS、Windows

### 启动参数

```bash
# 使用默认配置
./dyeing-hair-ai-server

# 指定配置文件和端口
./dyeing-hair-ai-server --config custom-config.toml --port 8080
```

### Docker 部署

```bash
# 构建镜像
docker build -f backend.Dockerfile -t ai-chat-server .

# 运行容器
docker run -p 3000:3000 -v ./config.toml:/app/config.toml ai-chat-server
```

## 注意事项

1. **并发限制**: 系统支持多并发请求，但建议控制并发数以保证性能
2. **超时设置**: API 请求超时时间为 300 秒
3. **文件大小**: 上传文件最大支持 300MB
4. **会话持久化**: 会话数据每 30 秒自动保存到磁盘
5. **CORS 支持**: 系统已配置 CORS，支持跨域请求

## 更新日志

### v1.0.0

- 基础聊天功能
- 文件上传支持
- SSE 流式响应
- 会话管理
- PDF 文档转换

---

如有问题或建议，请联系开发团队。
