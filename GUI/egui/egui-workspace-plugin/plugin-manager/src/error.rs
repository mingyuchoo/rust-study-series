use thiserror::Error;

/// Errors that can occur during file conversion operations
#[derive(Error, Debug)]
pub enum ConversionError {
    /// Plugin with the specified name was not found in the registry
    #[error("플러그인을 찾을 수 없습니다: {0}")]
    PluginNotFound(String),
    
    /// The file format is not supported by any available plugin
    #[error("지원하지 않는 파일 형식: {0}")]
    UnsupportedFormat(String),
    
    /// Failed to read or write a file
    #[error("파일 읽기 실패: {0}")]
    FileReadError(#[from] std::io::Error),
    
    /// The conversion operation failed
    #[error("변환 실패: {0}")]
    ConversionFailed(String),
    
    /// Database operation failed
    #[error("데이터베이스 오류: {0}")]
    DatabaseError(String),
    
    /// Plugin initialization failed
    #[error("플러그인 초기화 실패: {0}")]
    PluginInitError(String),
    
    /// Invalid input provided
    #[error("잘못된 입력: {0}")]
    InvalidInput(String),
    
    /// No plugin available for the requested conversion
    #[error("변환을 수행할 수 있는 플러그인이 없습니다: {0} -> {1}")]
    NoPluginAvailable(String, String),
}

/// Result type for conversion operations
pub type ConversionResult<T> = Result<T, ConversionError>;
