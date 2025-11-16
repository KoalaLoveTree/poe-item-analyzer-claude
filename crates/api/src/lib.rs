//! API and service layer for PoE Item Analyzer
//!
//! This crate handles I/O operations including:
//! - PoE API communication
//! - Data downloading and management
//! - File operations
//! - Service orchestration

pub mod poe_api;
pub mod sources;
pub mod downloader;
pub mod error;

#[cfg(test)]
mod tests;

pub use error::{ApiError, DownloadError, SourceError};
