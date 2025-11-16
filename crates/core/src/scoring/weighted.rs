//! Weighted scoring system

use std::collections::HashMap;

use crate::items::MatchedMod;

/// Weighted scorer for calculating item scores based on matched mods
#[derive(Debug, Clone)]
pub struct WeightedScorer {
    /// Mod weights
    weights: HashMap<String, f64>,
}

impl WeightedScorer {
    /// Create a new weighted scorer
    pub fn new(weights: HashMap<String, f64>) -> Self {
        Self { weights }
    }

    /// Calculate score from matched mods
    pub fn calculate_score(&self, matched_mods: &[MatchedMod]) -> f64 {
        matched_mods
            .iter()
            .map(|m| m.weight * m.count as f64)
            .sum()
    }

    /// Get weight for a specific mod
    pub fn get_weight(&self, mod_text: &str) -> Option<f64> {
        self.weights.get(mod_text).copied()
    }

    /// Check if a mod is valuable (has a weight assigned)
    pub fn is_valuable(&self, mod_text: &str) -> bool {
        self.weights.contains_key(mod_text)
    }
}
