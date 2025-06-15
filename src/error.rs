use std::fmt;
use std::io;
use std::net::AddrParseError;

/// Error type for the Vehicle Shadow Client
#[derive(Debug)]
pub enum ClientError {
    /// IO error
    Io(io::Error),
    /// Serialization error
    Serialization(String),
    /// Database error
    Database(String),
    /// Not found error
    NotFound(String),
    /// Invalid input error
    InvalidInput(String),
    /// Configuration error
    Configuration(String),
    /// RPC error
    Rpc(String),
    /// Network error
    Network(String),
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClientError::Io(e) => write!(f, "IO error: {}", e),
            ClientError::Serialization(e) => write!(f, "Serialization error: {}", e),
            ClientError::Database(e) => write!(f, "Database error: {}", e),
            ClientError::NotFound(e) => write!(f, "Not found: {}", e),
            ClientError::InvalidInput(e) => write!(f, "Invalid input: {}", e),
            ClientError::Configuration(e) => write!(f, "Configuration error: {}", e),
            ClientError::Rpc(e) => write!(f, "RPC error: {}", e),
            ClientError::Network(e) => write!(f, "Network error: {}", e),
        }
    }
}

impl std::error::Error for ClientError {}

impl From<io::Error> for ClientError {
    fn from(err: io::Error) -> Self {
        ClientError::Io(err)
    }
}

impl From<serde_json::Error> for ClientError {
    fn from(err: serde_json::Error) -> Self {
        ClientError::Serialization(err.to_string())
    }
}

impl From<tonic::Status> for ClientError {
    fn from(err: tonic::Status) -> Self {
        ClientError::Rpc(err.to_string())
    }
}

impl From<tonic::transport::Error> for ClientError {
    fn from(err: tonic::transport::Error) -> Self {
        ClientError::Network(err.to_string())
    }
}

impl From<AddrParseError> for ClientError {
    fn from(err: AddrParseError) -> Self {
        ClientError::Configuration(err.to_string())
    }
}

impl From<Box<dyn std::error::Error>> for ClientError {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        ClientError::Configuration(err.to_string())
    }
}

impl From<http::uri::InvalidUri> for ClientError {
    fn from(err: http::uri::InvalidUri) -> Self {
        ClientError::Configuration(err.to_string())
    }
}

/// Result type for the Vehicle Shadow Client
pub type Result<T> = std::result::Result<T, ClientError>;
