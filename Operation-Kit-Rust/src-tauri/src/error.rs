use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("加密错误: {0}")]
    EncryptError(String),
    
    #[error("解密错误: {0}")]
    DecryptError(String),
    
    #[error("格式转换错误: {0}")]
    FormatError(String),
    
    #[error("参数错误: {0}")]
    InvalidInput(String),
}

impl From<std::string::FromUtf8Error> for AppError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        AppError::DecryptError(err.to_string())
    }
}

impl From<base64::DecodeError> for AppError {
    fn from(err: base64::DecodeError) -> Self {
        AppError::DecryptError(err.to_string())
    }
}

pub type AppResult<T> = Result<T, AppError>; 