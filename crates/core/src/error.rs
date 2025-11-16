//! Error types for the core crate

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AnalysisError {
    #[error("Invalid item data: {0}")]
    InvalidItemData(String),

    #[error("Missing required field: {0}")]
    MissingField(String),

    #[error("Data error: {0}")]
    DataError(#[from] DataError),

    #[error("Analysis failed: {0}")]
    AnalysisFailed(String),
}

#[derive(Error, Debug)]
pub enum DataError {
    #[error("Failed to load data: {0}")]
    LoadError(String),

    #[error("Invalid data format: {0}")]
    InvalidFormat(String),

    #[error("Missing data file: {0}")]
    MissingFile(String),

    #[error("Data corruption detected: {0}")]
    CorruptedData(String),
}
