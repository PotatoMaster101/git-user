use thiserror::Error;

/// Represents the application errors.
#[derive(Debug, Error)]
pub enum AppError {
    /// Config file error.
    #[error("Config error: {0}")]
    Config(String),

    /// File IO error.
    #[error("File error: {0}")]
    File(String),

    /// Git error.
    #[error("Git error: {0}")]
    Git(String),

    /// JSON error.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}
