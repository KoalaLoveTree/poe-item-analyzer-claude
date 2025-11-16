//! Traits for data sources

use crate::error::DataError;

/// Trait for data sources that provide game data
pub trait DataSource: Send + Sync {
    /// Load data from the source
    fn load(&self) -> Result<(), DataError>;

    /// Check if data is available
    fn is_available(&self) -> bool;

    /// Validate data integrity
    fn validate(&self) -> Result<(), DataError>;
}
