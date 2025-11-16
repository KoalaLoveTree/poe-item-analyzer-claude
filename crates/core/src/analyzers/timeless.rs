//! Timeless jewel analyzer

use std::cmp::Ordering;
use std::collections::HashMap;

use crate::error::AnalysisError;
use crate::items::{SocketResult, TimelessJewel, TimelessJewelMetrics};

use super::traits::Analyzer;

/// Configuration for timeless jewel analysis
#[derive(Debug, Clone)]
pub struct TimelessJewelConfig {
    /// Valuable mods with their weights
    pub valuable_mods: HashMap<String, f64>,
}

impl TimelessJewelConfig {
    /// Create a new configuration
    pub fn new() -> Self {
        Self {
            valuable_mods: HashMap::new(),
        }
    }

    /// Add a valuable mod with a weight
    pub fn add_mod(&mut self, mod_text: String, weight: f64) {
        self.valuable_mods.insert(mod_text, weight);
    }

    /// Get all valuable mods
    pub fn valuable_mods(&self) -> &HashMap<String, f64> {
        &self.valuable_mods
    }
}

impl Default for TimelessJewelConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of analyzing a timeless jewel
#[derive(Debug, Clone)]
pub struct TimelessJewelAnalysisResult {
    /// The jewel that was analyzed
    pub jewel: TimelessJewel,

    /// Analysis metrics
    pub metrics: TimelessJewelMetrics,

    /// Best socket score (for easy ranking)
    pub best_score: f64,

    /// Best socket ID
    pub best_socket_id: String,
}

/// Analyzer for timeless jewels
pub struct TimelessJewelAnalyzer {
    // TODO: Add LUT data reference
}

impl TimelessJewelAnalyzer {
    /// Create a new timeless jewel analyzer
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for TimelessJewelAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl Analyzer<TimelessJewel> for TimelessJewelAnalyzer {
    type Config = TimelessJewelConfig;
    type Result = TimelessJewelAnalysisResult;

    fn analyze(
        &self,
        item: &TimelessJewel,
        _config: &Self::Config,
    ) -> Result<Self::Result, AnalysisError> {
        // TODO: Implement actual analysis logic
        // For now, return a placeholder result

        let socket_results: Vec<SocketResult> = vec![];

        let best_score = socket_results
            .iter()
            .map(|r| r.score)
            .fold(0.0, f64::max);

        let best_socket_id = socket_results
            .iter()
            .max_by(|a, b| a.score.partial_cmp(&b.score).unwrap_or(Ordering::Equal))
            .map(|r| r.socket_id.clone())
            .unwrap_or_default();

        Ok(TimelessJewelAnalysisResult {
            jewel: item.clone(),
            metrics: TimelessJewelMetrics { socket_results },
            best_score,
            best_socket_id,
        })
    }

    fn compare_results(&self, a: &Self::Result, b: &Self::Result) -> Ordering {
        // Higher score is better, so reverse the comparison
        b.best_score
            .partial_cmp(&a.best_score)
            .unwrap_or(Ordering::Equal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_creation() {
        let mut config = TimelessJewelConfig::new();
        config.add_mod("Double Damage".to_string(), 5.0);
        config.add_mod("Onslaught".to_string(), 4.5);

        assert_eq!(config.valuable_mods().len(), 2);
        assert_eq!(config.valuable_mods().get("Double Damage"), Some(&5.0));
    }

    #[test]
    fn test_analyzer_creation() {
        let _analyzer = TimelessJewelAnalyzer::new();
    }
}
