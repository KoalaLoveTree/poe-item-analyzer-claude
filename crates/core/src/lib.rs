//! Core business logic for PoE Item Analyzer
//!
//! This crate contains the core domain models, traits, and analysis logic
//! without any I/O operations. It's designed to be platform-agnostic and
//! easily testable.

pub mod items;
pub mod analyzers;
pub mod data;
pub mod scoring;
pub mod error;

// Re-export commonly used types
pub use error::{AnalysisError, DataError};
