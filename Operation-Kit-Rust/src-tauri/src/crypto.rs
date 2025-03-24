use crate::error::{AppError, AppResult};
use hashids::HashIds;
use std::sync::OnceLock;

static HASHIDS: OnceLock<HashIds> = OnceLock::new();

fn get_hashids() -> &'static HashIds {
    HASHIDS.get_or_init(|| {
        HashIds::new_with_salt_and_min_length("Your_Salt".to_string(), 12)
            .expect("Failed to create HashIds instance")
    })
}

pub fn encrypt_number(text: &str) -> AppResult<String> {
    if text.is_empty() {
        return Err(AppError::EncryptError("输入不能为空".to_string()));
    }
    
    match text.parse::<i64>() {
        Ok(num) => {
            let numbers = vec![num];
            Ok(get_hashids().encode(&numbers))
        }
        Err(e) => {
            if !text.chars().all(|c| c.is_ascii_digit()) {
                Err(AppError::InvalidInput(format!("输入必须为数字: {}", e)))
            } else {
                Ok(text.to_string())
            }
        }
    }
}

pub fn decrypt_text(text: &str) -> AppResult<String> {
    if text.is_empty() {
        return Err(AppError::DecryptError("输入不能为空".to_string()));
    }
    
    let result = std::panic::catch_unwind(|| {
        get_hashids().decode(text.to_string())
    });

    match result {
        Ok(numbers) => {
            if numbers.is_empty() {
                if text.chars().all(|c| c.is_ascii_alphanumeric()) {
                    Err(AppError::DecryptError("无效的加密字符串".to_string()))
                } else {
                    Ok(text.to_string())
                }
            } else {
                Ok(numbers[0].to_string())
            }
        }
        Err(_) => Err(AppError::DecryptError("解密失败，无效的加密字符串".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let original = "12345";
        let encrypted = encrypt_number(original).unwrap();
        let decrypted = decrypt_text(&encrypted).unwrap();
        assert_eq!(original, decrypted);
    }

    #[test]
    fn test_invalid_input() {
        let invalid = "not_a_number";
        assert!(encrypt_number(invalid).is_err());
    }

    #[test]
    fn test_empty_input() {
        assert!(encrypt_number("").is_err());
        assert!(decrypt_text("").is_err());
    }

    #[test]
    fn test_multiple_numbers() {
        let numbers = ["123", "456", "789", "1000", "99999"];
        for &num in numbers.iter() {
            let encrypted = encrypt_number(num).unwrap();
            let decrypted = decrypt_text(&encrypted).unwrap();
            assert_eq!(num, decrypted);
        }
    }

    #[test]
    fn test_invalid_hash() {
        assert!(decrypt_text("invalid_hash").is_err());
    }
} 