use crate::{
    crypto::{decrypt_text, encrypt_number},
    text_processor::{add_quotes, convert_format, remove_quotes, replace_chinese_commas},
};

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