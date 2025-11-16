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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weighted_scorer() {
        let mut weights = HashMap::new();
        weights.insert("Double Damage".to_string(), 5.0);
        weights.insert("Onslaught".to_string(), 3.0);

        let scorer = WeightedScorer::new(weights);

        assert_eq!(scorer.get_weight("Double Damage"), Some(5.0));
        assert_eq!(scorer.get_weight("Unknown"), None);
        assert!(scorer.is_valuable("Onslaught"));
        assert!(!scorer.is_valuable("Unknown"));
    }

    #[test]
    fn test_calculate_score() {
        let mut weights = HashMap::new();
        weights.insert("Double Damage".to_string(), 5.0);

        let scorer = WeightedScorer::new(weights);

        let matched_mods = vec![MatchedMod {
            mod_text: "Double Damage".to_string(),
            weight: 5.0,
            count: 2,
        }];

        assert_eq!(scorer.calculate_score(&matched_mods), 10.0);
    }
}
