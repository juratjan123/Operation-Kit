use anyhow::{anyhow, Result};
use hmac::{Hmac, Mac};
use sha1::Sha1;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_LENGTH, CONTENT_TYPE, DATE, HOST};
use chrono::Utc;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

// 定义支持的渠道
pub enum Channel {
    Vivo,
    Oppo,
    Huawei,
    Xiaomi,
}

impl Channel {
    // 将字符串转换为Channel枚举
    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "vivo" => Ok(Channel::Vivo),
            "oppo" => Ok(Channel::Oppo),
            "huawei" => Ok(Channel::Huawei),
            "xiaomi" => Ok(Channel::Xiaomi),
            _ => Err(anyhow!("不支持的渠道: {}", s)),
        }
    }

    // 获取渠道名称字符串
    pub fn as_str(&self) -> &'static str {
        match self {
            Channel::Vivo => "vivo",
            Channel::Oppo => "oppo",
            Channel::Huawei => "huawei",
            Channel::Xiaomi => "xiaomi",
        }
    }
}

// 用于生成OSS签名的类型
type HmacSha1 = Hmac<Sha1>;

// 计算MD5并转为Base64格式
fn md5_base64(data: &[u8]) -> String {
    let digest = md5::compute(data);
    BASE64.encode(digest.as_ref())
}

// 上传ID列表到OSS
pub async fn upload_ids_to_oss(
    access_id: &str, 
    access_key: &str, 
    content: &str, 
    channel: Channel
) -> Result<String> {
    // 检查OSS参数
    if access_id.trim().is_empty() || access_key.trim().is_empty() {
        return Err(anyhow!("OSS Access ID或Access Key不能为空"));
    }
    
    // 验证输入内容是否符合要求 (每行一个数字ID)
    validate_content(content)?;

    // 桶名称和对象路径
    let bucket_name = "Bucket";
    let endpoint = "endpoint";
    let host = format!("{}.{}", bucket_name, endpoint);
    
    // 设置文件夹路径和文件名
    let folder_name = format!("hive2/dim/tmp_{}_ids", channel.as_str());
    let file_name = format!("{}.txt", channel.as_str());  // 使用渠道名作为文件名，添加.txt后缀
    let object_path = format!("{}/{}", folder_name, file_name);  // 完整对象路径
    
    // 转换内容为字节
    let content_bytes = content.as_bytes();
    let content_length = content_bytes.len();
    
    // 构建请求头和签名相关信息
    let date = Utc::now().format("%a, %d %b %Y %H:%M:%S GMT").to_string();
    let content_type = "text/plain";
    
    // 计算Content-MD5 - OSS API要求对内容计算MD5并Base64编码
    let content_md5 = md5_base64(content_bytes);
    
    // 构建规范资源(CanonicalizedResource)
    let canonicalized_resource = format!("/{}/{}", bucket_name, object_path);
    
    // 构建用于签名的规范字符串
    // 格式: VERB + "\n" + Content-MD5 + "\n" + Content-Type + "\n" + Date + "\n" + CanonicalizedOSSHeaders + CanonicalizedResource
    let string_to_sign = format!(
        "PUT\n{}\n{}\n{}\n{}",
        content_md5,
        content_type,
        date,
        canonicalized_resource
    );
    
    // 使用HMAC-SHA1计算签名
    let mut mac = HmacSha1::new_from_slice(access_key.as_bytes())
        .map_err(|e| anyhow!("初始化HMAC失败: {}", e))?;
    mac.update(string_to_sign.as_bytes());
    let result = mac.finalize();
    let signature = BASE64.encode(result.into_bytes());
    
    // 构建Authorization头
    let authorization = format!("OSS {}:{}", access_id, signature);
    
    // 构建完整URL
    let url = format!("https://{}/{}", host, object_path);
    
    // 创建请求头
    let mut headers = HeaderMap::new();
    headers.insert(HOST, HeaderValue::from_str(&host)?);
    headers.insert(DATE, HeaderValue::from_str(&date)?);
    headers.insert(CONTENT_TYPE, HeaderValue::from_str(content_type)?);
    headers.insert(CONTENT_LENGTH, HeaderValue::from_str(&content_length.to_string())?);
    headers.insert("Content-MD5", HeaderValue::from_str(&content_md5)?);
    headers.insert("Authorization", HeaderValue::from_str(&authorization)?);
    
    // 发送PUT请求
    let client = reqwest::Client::new();
    let response = client
        .put(&url)
        .headers(headers)
        .body(content.to_string())
        .send()
        .await
        .map_err(|e| anyhow!("发送请求失败: {}", e))?;
    
    let status = response.status();
    
    // 处理响应
    if status.is_success() {
        Ok(format!("成功上传到 OSS 路径: {}/{}", host, object_path))
    } else {
        let error_text = response.text().await
            .unwrap_or_else(|_| "无法获取错误详情".to_string());
        
        if error_text.contains("InvalidAccessKeyId") {
            Err(anyhow!("Access ID无效，请检查配置"))
        } else if error_text.contains("SignatureDoesNotMatch") {
            Err(anyhow!("Access Key无效，请检查配置或签名错误"))
        } else if error_text.contains("NetworkingError") || error_text.contains("Connection refused") {
            Err(anyhow!("网络连接错误，请检查网络并确认使用了正确的OSS接入点"))
        } else {
            Err(anyhow!("上传到OSS失败: HTTP {}, 错误: {}", status, error_text))
        }
    }
}

// 验证内容是否满足格式要求 (每行一个数字ID)
fn validate_content(content: &str) -> Result<()> {
    if content.trim().is_empty() {
        return Err(anyhow!("内容不能为空"));
    }

    for (i, line) in content.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if !line.chars().all(|c| c.is_digit(10)) {
            return Err(anyhow!("第 {} 行不是有效的数字ID: '{}'", i + 1, line));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_content_valid() {
        let content = "123\n456\n789";
        assert!(validate_content(content).is_ok());
    }

    #[test]
    fn test_validate_content_invalid() {
        let content = "123\nabc\n789";
        assert!(validate_content(content).is_err());
    }

    #[test]
    fn test_channel_conversion() {
        assert!(matches!(Channel::from_str("vivo").unwrap(), Channel::Vivo));
        assert!(matches!(Channel::from_str("OPPO").unwrap(), Channel::Oppo));
        assert!(Channel::from_str("invalid").is_err());
        
        assert_eq!(Channel::Vivo.as_str(), "vivo");
        assert_eq!(Channel::Oppo.as_str(), "oppo");
    }
} 