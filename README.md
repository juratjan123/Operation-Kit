# 运营百宝箱

运营百宝箱是一个基于 Tauri + Vue3 + TypeScript 的跨平台桌面应用。

## 功能特性

### 1. 文本处理
- 加密/解密功能
  - 使用 hashids 算法进行数字加密
  - 批量处理
- 格式转换
  - 按行分布的文字数据和按逗号分隔的文字数据互换格式
  - 中英文标点符号转换
  - 添加/移除引号

### 2. 剪贴板集成
- 一键复制 & 一键粘贴

## 技术栈

- **前端框架**: Vue 3
- **开发语言**: TypeScript, Rust
- **构建工具**: Vite
- **桌面框架**: Tauri
- **包管理器**: npm (前端), Cargo (Rust)

## 注意事项
使用的时候请将Your_Slat替换为自己的Salt（还有oss.rs的Bucket_name等信息）
```
baibaoxiang/baibaoxiang-go/main.go
&
baibaoxiang/baibaoxiang-rust/src-tauri/src/crypto.rs
```