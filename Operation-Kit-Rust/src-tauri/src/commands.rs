use crate::{
    crypto::{decrypt_text, encrypt_number, get_current_config, set_config, CryptoConfig, init_config, use_huawei_prefix, set_use_huawei_prefix},
    text_processor::{add_quotes, convert_format, remove_quotes, replace_chinese_commas},
    oss::{upload_ids_to_oss, Channel},
};

// 初始化配置（在应用启动时调用）
pub fn init_crypto_config() {
    init_config();
}

// 获取当前配置名称
#[tauri::command]
pub async fn get_crypto_config() -> Result<String, String> {
    let config = get_current_config();
    match config {
        CryptoConfig::General => Ok("通用".to_string()),
        CryptoConfig::Huawei => Ok("华为".to_string()),
    }
}

// 设置当前配置
#[tauri::command]
pub async fn set_crypto_config(config_name: String) -> Result<(), String> {
    let config = match config_name.as_str() {
        "通用" => CryptoConfig::General,
        "华为" => CryptoConfig::Huawei,
        _ => return Err("无效的配置名称，必须是 '通用' 或 '华为'".to_string()),
    };
    
    set_config(config);
    Ok(())
}

#[tauri::command]
pub async fn process_batch_encrypt(input: String) -> Result<String, String> {
    let items: Vec<&str> = input
        .split(|c| c == '\n' || c == ',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect();

    let uses_newlines = input.matches('\n').count() > input.matches(',').count();
    let delimiter = if uses_newlines { "\n" } else { "," };

    let mut result = Vec::with_capacity(items.len());
    for item in items {
        match encrypt_number(item) {
            Ok(encrypted) => result.push(encrypted),
            Err(e) => return Err(e.to_string()),
        }
    }

    Ok(result.join(delimiter))
}

#[tauri::command]
pub async fn process_batch_decrypt(input: String) -> Result<String, String> {
    let items: Vec<&str> = input
        .split(|c| c == '\n' || c == ',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect();

    let uses_newlines = input.matches('\n').count() > input.matches(',').count();
    let delimiter = if uses_newlines { "\n" } else { "," };

    let mut result = Vec::with_capacity(items.len());
    for item in items {
        match decrypt_text(item) {
            Ok(decrypted) => result.push(decrypted),
            Err(e) => return Err(e.to_string()),
        }
    }

    Ok(result.join(delimiter))
}

#[tauri::command]
pub async fn process_convert_format(input: String) -> Result<String, String> {
    convert_format(&input).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn process_replace_commas(input: String) -> Result<String, String> {
    replace_chinese_commas(&input).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn process_add_quotes(input: String) -> Result<String, String> {
    add_quotes(&input).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn process_remove_quotes(input: String) -> Result<String, String> {
    remove_quotes(&input).map_err(|e| e.to_string())
}

// 获取当前华为前缀配置
#[tauri::command]
pub async fn get_huawei_prefix_config() -> Result<bool, String> {
    Ok(use_huawei_prefix())
}

// 设置华为前缀配置
#[tauri::command]
pub async fn set_huawei_prefix_config(use_prefix: bool) -> Result<(), String> {
    set_use_huawei_prefix(use_prefix);
    Ok(())
}

// 将ID列表上传到OSS
#[tauri::command]
pub async fn upload_to_oss(access_id: String, access_key: String, content: String, channel: String) -> Result<String, String> {
    // 转换渠道名称为枚举类型
    let channel = Channel::from_str(&channel).map_err(|e| e.to_string())?;
    
    // 调用OSS模块上传内容
    upload_ids_to_oss(&access_id, &access_key, &content, channel)
        .await
        .map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_batch_encrypt_decrypt() {
        let input = "12345,67890";
        let encrypted = process_batch_encrypt(input.to_string()).await.unwrap();
        let decrypted = process_batch_decrypt(encrypted).await.unwrap();
        assert_eq!(input, decrypted);
    }

    #[tokio::test]
    async fn test_format_conversion() {
        let input = "1,2,3";
        let converted = process_convert_format(input.to_string()).await.unwrap();
        assert!(converted.contains('\n'));
        let back = process_convert_format(converted).await.unwrap();
        assert_eq!(input, back);
    }
} 