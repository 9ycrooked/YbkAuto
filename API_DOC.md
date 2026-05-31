# 云班课 API 文档

> 基于浏览器抓包（HAR）分析的 API 接口汇总

---

## 环境说明

| 环境 | Base URL | 认证方式 |
|------|----------|----------|
| 网页版 | `https://coreapi.mosoteach.cn` | `x-token` header |
| 手机版 | `https://api.mosoteach.cn/mssvc/index.php` | HMAC-SHA1 签名 |

---

## 一、网页版 API

### 通用请求头

所有请求都需要以下 headers：

```http
Origin: https://www.mosoteach.cn
Referer: https://www.mosoteach.cn/
x-client-app-id: MTWEB
x-client-version: 6.0.0
x-security-type: SECURITY_TYPE_TOKEN
x-token: {login_token}
Accept: application/json, text/plain, */*
```

**注意**：登录接口 (`/passports/account-login`) 不需要 `x-token` header。

---

### 1. 认证相关

#### 账号登录

```
POST /passports/account-login?_ts={timestamp}
```

**请求体 (JSON)**:
```json
{
  "account": "{phone_number}",
  "ciphertext": "{encrypted_password}"
}
```

- `account`: 手机号
- `ciphertext`: 密码经 WASM 加密后的 Base64 字符串

**登录成功响应**:
```json
{
  "user": {
    "accessId": "{mobile_api_access_id}",
    "accessSecret": "{mobile_api_access_secret}",
    "fullName": "示例用户",
    "userId": "{user_id}",
    "nickName": "138****0000",
    "phoneNumber": "{phone_number}",
    "bindSchool": {
      "schoolId": "{school_id}",
      "schoolName": "示例学校",
      "departmentId": "{department_id}",
      "departmentName": "示例院系"
    },
    "studentNo": "{student_no}"
  },
  "token": "{web_api_token}",  // 网页版 API 认证 token
  "status": true
}
```

**重要说明**：
- `accessId` + `accessSecret` = **手机版** API 签名凭证
- `token` = **网页版** API 认证凭证（用于 `x-token` header）

---

### 2. 班课相关

#### 获取已加入的班课列表

```
GET /ccs/joined?_ts={timestamp}
```

**响应示例**:
```json
{
  "clazzCourses": [
    {
      "id": "284A240E-BD1D-11F0-B5BC-9C63C078B890",
      "courseName": "室内设计课题3",
      "fullCoverUrl": "https://...",
      "creater": {
        "fullName": "教师姓名",
        "userId": "..."
      },
      "roleId": 2
    }
  ]
}
```

#### 班课详情

```
GET /ccs/{ccid}?_ts={timestamp}
```

**响应示例**:
```json
{
  "clazzCourse": {
    "id": "284A240E-BD1D-11F0-B5BC-9C63C078B890",
    "operater": {
      "fullName": "示例用户",
      "roleId": 2
    },
    "school": {
      "id": "{school_id}",
      "name": "示例学校"
    },
    "joinAllowedFlag": "Y",
    "leaveAllowedFlag": "Y"
  }
}
```

#### 获取用户在班课中的角色

```
GET /ccs/{ccid}/user-role?_ts={timestamp}
```

**响应**:
```json
{"roleId": 2, "status": true}
```

- `roleId`: 1=老师, 2=学生, 3=助教

#### AI 助手状态

```
GET /ccs/{ccid}/ai-agent/status?_ts={timestamp}
```

**响应**:
```json
{"result": {"aiAgentFlag": true}, "status": true}
```

#### 班课统计

```
GET /ccs/{ccid}/simple-stat?roleId={roleId}&_ts={timestamp}
```

**响应**:
```json
{
  "stat": {
    "activityCount": 4,
    "memberCount": 75,
    "noticeCount": 1,
    "resourceCount": 15
  },
  "status": true
}
```

#### 活动/作业列表

```
GET /ccs/{ccid}/activities?roleId={roleId}&_ts={timestamp}
```

**响应**:
```json
{
  "activities": [
    {
      "id": "...",
      "title": "期末作业",
      "type": "HOMEWORK",
      "purpose": "FINAL_TERM",
      "hw": {
        "deadlineTime": "2026-01-04 23:30:00",
        "allowReSubmit": "N",
        "homeworkAppraiseType": "TEACHER"
      }
    }
  ]
}
```

#### 资源文件列表

```
GET /ccs/{ccid}/resources?roleId={roleId}&_ts={timestamp}
```

**响应**:
```json
{
  "resources": [
    {
      "id": "0039D651-F890-45...",
      "sortableName": "1.2chengshibinshuijingguan.mp4",
      "mimeType": "video/mp4",
      "metaDuration": 625,
      "score": 2,
      "releaseTime": "2025-11-10 10:02:38"
    }
  ]
}
```

#### 班课成员列表

```
GET /ccs/{ccid}/members?_ts={timestamp}
```

**响应**:
```json
{
  "users": [
    {
      "userId": "C54183D6-FD39-4B96-AD46-EB467702A246",
      "fullName": "学生姓名",
      "roles": [2],
      "joinTime": "2025-11-10 07:59:42"
    }
  ]
}
```

---

### 3. 用户相关

#### 我的个人信息

```
GET /users/my-profile?_ts={timestamp}
```

**响应**:
```json
{
  "user": {
    "accessId": "{mobile_api_access_id}",
    "accessSecret": "{mobile_api_access_secret}",
    "fullName": "示例用户",
    "userId": "{user_id}",
    "phoneNumber": "{phone_number}",
    "bindSchool": {...}
  }
}
```

#### 消息元数据

```
GET /user-messages/meta?_ts={timestamp}
```

**响应**:
```json
{
  "message": {
    "unreadCount": 0,
    "totalCount": 1
  },
  "status": true
}
```

---

### 4. 组织相关

#### 已加入的组织

```
GET /orgs/joined?service=CCP&_ts={timestamp}
```

**响应**:
```json
{"orgs": [], "status": true}
```

---

### 5. 配置相关

#### 系统配置项

```
GET /config-items?_ts={timestamp}
```

**响应**:
```json
{
  "item": {
    "EXCEL_EXTS": [".csv", ".ods", ".xls", ".xlsb", ".xlsm", ".xlsx"],
    "SECURITY_CONFIG": {
      "password_expired_max_days": 90,
      "password_remind_silence_days": 90
    },
    "APP_FILING_NUMBER": {...}
  }
}
```

---

### 6. 成绩相关

#### 学生成绩摘要

```
GET /user_record/{userId}/student_score_summary?ccId={ccid}&_ts={timestamp}
```

**响应**:
```json
{
  "result": {
    "actJoinedCount": 4,
    "actJoinedRate": 1.0,
    "checkinCount": 11,
    "checkinJoinedRate": 1.0,
    "checkinScore": 22,
    "checkinTotalCount": 11
  }
}
```

---

## 二、手机版 API（需签名）

### 通用请求头

```http
User-Agent: Dalvik/2.1.0 (Linux; U; Android 8.1.0; ONE A2001 Build/OPM7.181205.001)
X-scheme: https
X-app-id: MTANDROID
X-app-version: 5.1.1
X-dpr: 2.7
X-app-machine: ONE A2001
X-app-system-version: 8.1.0
Host: api.mosoteach.cn
Date: {timestamp}
X-mssvc-signature: {signature}
X-mssvc-access-id: {access_id}
X-mssvc-sec-ts: {last_sec_update_ts_s}
```

### 签名算法

**签名 Key**: `526EBA802E6FCF44661DE4393A82ABDA`

**签名格式**:
```
# 无表单请求
{url}|{user_id_uppercase}|{timestamp}

# 登录表单
{url}|{user_id_uppercase}|{timestamp}|{form_md5}

# 普通表单
{url}|{user_id_uppercase}|{timestamp}|{form_md5}
```

**Form MD5 计算**:
```python
form_str = "key1=value1|key2=value2|..."
md5(form_str).upper()
```

### 1. 认证

#### 登录

```
POST /mssvc/index.php/passport/login
```

**表单参数**:
| 参数 | 值 |
|------|-----|
| account_name | 用户名 |
| app_id | MTANDROID |
| app_version_name | 5.1.1 |
| app_version_number | 111 |
| device_type | ANDROID |
| dpr | 2.7 |
| system_version | 8.1.0 |
| user_pwd | 密码 |

**登录成功返回**:
```json
{
  "result_code": 0,
  "result_msg": "OK",
  "user": {
    "user_id": "xxx",
    "access_id": "xxx",
    "access_secret": "xxx",
    "last_sec_update_ts_s": "xxx",
    "full_name": "姓名",
    "school_name": "学校"
  }
}
```

### 2. 班课相关

#### 获取已加入的班课列表

```
POST /mssvc/index.php/cc/list_joined
```

### 3. 签到相关

#### 签到任务列表

```
POST /mssvc/index.php/checkin/index
```

#### 当前签到状态

```
POST /mssvc/index.php/checkin/current_open
```

#### 签到提交

```
POST /mssvc/index.php/checkin
```

#### 签到提交(独立服务)

```
POST https://checkin.mosoteach.cn:19528/checkin
```

### 4. 任务相关

#### 任务列表

```
POST /mssvc/index.php/interaction/mylist_v2
```

### 5. 成员相关

#### 成员列表

```
POST /mssvc/index.php/member/list_member
```

---

## 三、签名示例

### Python 实现

```python
from datetime import datetime
from hashlib import sha1, md5
import hmac

GMT_FORMAT = '%a, %d %b %Y %H:%M:%S GMT+00:00'
KEY = '526EBA802E6FCF44661DE4393A82ABDA'

def get_time():
    return datetime.utcnow().strftime(GMT_FORMAT)

def form_sign(form_str):
    return md5(form_str.encode('utf-8')).hexdigest().upper()

def sign_request(url, form=None, user_id=None, access_secret=None, login_form=False):
    time_str = get_time()

    if user_id:
        result = f"{url}|{user_id.upper()}|{time_str}"
    else:
        result = f"{url}|{time_str}"

    if form:
        form_str = "|".join(f"{k}={v}" for k, v in form.items())
        if login_form:
            result += "|" + form_str
        else:
            result += "|" + form_sign(form_str)

    secret = access_secret or KEY
    signature = hmac.new(
        secret.encode('utf-8'),
        result.encode('utf-8'),
        sha1
    ).hexdigest()

    return time_str, signature
```

---

## 四、WASM 密码加密

### 加密文件

- `encrypt.js`: WASM 加载器胶水代码
- `encrypt_bg.wasm`: Rust 编译的 WASM 加密模块

### CDN 地址

```
https://static-cdn-oss.mosoteach.cn/third-party/wasm/1.0.0/no-modules/encrypt.js
https://static-cdn-oss.mosoteach.cn/third-party/wasm/1.0.0/no-modules/encrypt_bg.wasm
```

### 前端调用方式

```javascript
import init, { encrypt_password, Password } from './encrypt.js';

await init();

// 加密密码
const phone = "{phone_number}";
const password = "{plain_password}";
const encrypted = encrypt_password(phone, password);
const ciphertext = encrypted.ciphertext();

// 登录请求
const response = await fetch('https://coreapi.mosoteach.cn/passports/account-login', {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json',
    'x-client-app-id': 'MTWEB',
    'x-client-version': '6.0.0',
    'x-security-type': 'SECURITY_TYPE_TOKEN'
  },
  body: JSON.stringify({
    account: phone,
    ciphertext: ciphertext
  })
});
```

### Tauri 项目集成建议

由于 WASM 加密需要在浏览器/前端环境执行，建议架构如下：

```
┌─────────────────────────────────────────────────────────────┐
│  Tauri 桌面应用                                              │
│  ┌─────────────────────┐    ┌────────────────────────────┐│
│  │  Vue3 前端 (TS)    │    │  Rust Backend (本地)        ││
│  │                      │    │                            ││
│  │  encrypt.js +        │    │  Tauri Command:            ││
│  │  encrypt_bg.wasm     │────│  login(ciphertext)         ││
│  │  (WASM 加密)        │    │        │                    ││
│  │                      │    │        ▼                   ││
│  │                      │    │  HTTP POST                  ││
│  └─────────────────────┘    │  coreapi.mosoteach.cn      ││
│                              └────────────────────────────┘│
└─────────────────────────────────────────────────────────────┘
```

**优势**：
- 复用了官方加密逻辑，无需逆向
- Rust 后端本地发请求，无 CORS 问题
- Token 安全存储在本地

---

## 五、测试账号

本文档不保存真实账号、密码、token、access secret 或其他可复用凭据。

如需联调，请在本地安全位置保存测试凭据，并在示例中使用以下占位符：

| 字段 | 占位符 |
|------|--------|
| 用户名 | `{phone_number}` |
| 密码 | `{plain_password}` |
| User ID | `{user_id}` |
| 网页版 Token (x-token) | `{web_api_token}` |
| 手机版 AccessId | `{mobile_api_access_id}` |
| 手机版 AccessSecret | `{mobile_api_access_secret}` |
| 学校 | `{school_name}` |
| 班级ID | `{clazz_course_id}` |

---

## 更新日志

| 日期 | 更新内容 |
|------|----------|
| 2026-04-30 | 初始文档，汇总网页版和手机版 API |
| 2026-04-30 | 新增网页版 API：登录、消息、组织、配置 |
| 2026-04-30 | 整理班课相关 API（详情/活动/资源/成员/统计） |
| 2026-04-30 | 新增 WASM 加密文件信息和 Tauri 集成建议 |
