// 防止在Windows发布版本中出现额外的控制台窗口，请勿删除此配置！
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// 导入自定义模块
mod commands;     // 包含所有Tauri命令处理函数
mod crypto;       // 加密解密相关功能
mod error;        // 错误处理模块
mod text_processor;   // 文本处理相关功能

// 导入commands模块中的所有公开项
use commands::*;

/// 主函数 - 应用程序入口点
fn main() {
    // 创建并配置Tauri应用程序
    tauri::Builder::default()
        // 注册所有可以从前端调用的命令处理函数
        .invoke_handler(tauri::generate_handler![
            process_batch_encrypt,    // 批量加密处理
            process_batch_decrypt,    // 批量解密处理
            process_convert_format,   // 格式转换处理
            process_replace_commas,   // 替换逗号处理
            process_add_quotes,       // 添加引号处理
            process_remove_quotes,    // 移除引号处理
        ])
        // 运行Tauri应用，使用默认上下文配置
        .run(tauri::generate_context!())
        // 处理运行时可能出现的错误
        .expect("error while running tauri application");
}
