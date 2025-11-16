//! Error types for the API crate

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("HTTP request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),

    #[error("Invalid response: {0}")]
    InvalidResponse(String),

    #[error("Rate limited: {0}")]
    RateLimited(String),

    #[error("API error: {0}")]
    ApiError(String),
}

#[derive(Error, Debug)]
pub enum DownloadError {
    #[error("Download failed: {0}")]
    DownloadFailed(String),

    #[error("Checksum mismatch: expected {expected}, got {actual}")]
    ChecksumMismatch { expected: String, actual: String },

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("Invalid manifest: {0}")]
    InvalidManifest(String),
}

#[derive(Error, Debug)]
pub enum SourceError {
    #[error("Failed to fetch items: {0}")]
    FetchFailed(String),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("API error: {0}")]
    ApiError(#[from] ApiError),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}
