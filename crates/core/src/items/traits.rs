//! Core traits for items in the analyzer

use serde_json::Value;

/// Core trait that all items must implement
pub trait Item: Send + Sync {
    /// Unique identifier for this item
    fn id(&self) -> String;

    /// Type name (e.g., "Timeless Jewel", "Cluster Jewel")
    fn item_type(&self) -> &str;

    /// Display name for the item
    fn name(&self) -> &str;
}

/// Trait for items that can be analyzed
pub trait AnalyzableItem: Item {
    /// Metrics that define value for this item type
    type ValueMetrics;

    /// Raw item data from the game API
    fn raw_data(&self) -> &Value;
}
