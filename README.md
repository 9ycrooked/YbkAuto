# YbkAuto

云班课桌面助手 — 课程管理、资源追踪、一键完成。

## 功能特性

- **课程概览** — 查看所有已加入的云班课课程
- **资源状态追踪** — 实时显示已完成/未完成资源数量
- **一键完成资源** — 批量标记课程资源为已完成状态
- **自动更新** — 支持静默自动更新，新版本自动下载安装
- **主题模式** — 支持 Light / Dark 跟随系统切换

## 技术栈

| 层级 | 技术 |
|---|---|
| 前端框架 | Vue 3 + Composition API |
| 语言 | TypeScript |
| 构建工具 | Vite |
| 状态管理 | Pinia |
| 路由 | Vue Router |
| 桌面框架 | Tauri 2 |
| 后端语言 | Rust |

## 系统要求

- **操作系统** Windows 10/11 (64-bit)
- **运行环境** WebView2 运行时（Windows 11 已内置，Windows 10 可能需要安装）
- **内存** 建议 4GB 及以上

## 安装说明

### 安装步骤

1. 下载最新版本的安装包（.msi 或 .exe）
2. 运行安装程序，按照提示完成安装
3. 启动 YbkAuto 并登录云班课账号

## 开发指南

### 环境要求

- Node.js 18+
- Rust 1.70+
- Yarn 4+
- Windows 10/11

### 本地开发

```bash
# 安装依赖
yarn install

# 启动开发服务器
yarn tauri dev
```

### 构建发布

```bash
# 构建生产版本
yarn tauri build
```

构建产物位于 `src-tauri/target/release/bundle/` 目录下。

## 项目结构

```
YbkAuto/
├── src/                      # Vue 前端源代码
│   ├── components/           # Vue 组件
│   ├── views/                # 页面视图
│   ├── stores/               # Pinia 状态管理
│   ├── composables/          # Vue Composables
│   ├── types/                # TypeScript 类型定义
│   └── utils/                # 工具函数
├── src-tauri/                # Tauri/Rust 后端源代码
│   ├── src/
│   │   ├── commands.rs       # Tauri 命令
│   │   ├── login.rs         # 登录及 API 逻辑
│   │   └── lib.rs           # 库入口
│   ├── Cargo.toml           # Rust 依赖配置
│   └── tauri.conf.json      # Tauri 配置文件
├── public/                   # 静态资源
└── package.json             # 项目配置
```

## License

MIT License

## 联系方式

- 邮箱：qianmang1@gmail.com
- GitHub：https://github.com/9ycrooked/YbkAuto

## 打赏作者
![微信赞赏码.jpg](images/%E5%BE%AE%E4%BF%A1%E8%B5%9E%E8%B5%8F%E7%A0%81.jpg)
