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
pub mod manifest;
pub mod github;
pub mod update_checker;
pub mod checksum;
pub mod parser;
pub mod error;

#[cfg(test)]
mod tests;

pub use error::{ApiError, DownloadError, SourceError};
pub use manifest::{DataFile, DataManifest, DataSource};
pub use github::GitHubClient;
pub use update_checker::{UpdateChecker, UpdateInfo};
pub use parser::{LutData, NodeModifier, PobDataParser};
pub use downloader::DataDownloader;
