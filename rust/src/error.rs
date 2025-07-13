use thiserror::Error;

#[derive(Error, Debug)]
pub enum BbcprError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Network error: {0}")]
    Network(String),
    
    #[error("SSH error: {0}")]
    Ssh(String),
    
    #[error("Checksum mismatch: expected {expected:?}, got {actual:?}")]
    ChecksumMismatch {
        expected: Vec<u8>,
        actual: Vec<u8>,
    },
    
    #[error("Transfer error: {0}")]
    Transfer(String),
    
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("Platform error: {0}")]
    Platform(String),
    
    #[error("Protocol error: {0}")]
    Protocol(String),
    
    #[error("Authentication failed")]
    AuthenticationFailed,
    
    #[error("Connection timeout")]
    ConnectionTimeout,
    
    #[error("File not found: {0}")]
    FileNotFound(String),
    
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    
    #[error("Unsupported operation: {0}")]
    Unsupported(String),
}

pub type Result<T> = std::result::Result<T, BbcprError>;