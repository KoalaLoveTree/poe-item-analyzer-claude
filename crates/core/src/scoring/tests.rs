//! Unit tests for scoring module

use super::*;
use crate::items::MatchedMod;
use std::collections::HashMap;

#[test]
fn test_weighted_scorer_creation() {
    let mut weights = HashMap::new();
    weights.insert("Double Damage".to_string(), 5.0);
    weights.insert("Onslaught".to_string(), 3.0);

    let scorer = WeightedScorer::new(weights);

    assert_eq!(scorer.get_weight("Double Damage"), Some(5.0));
    assert_eq!(scorer.get_weight("Onslaught"), Some(3.0));
    assert_eq!(scorer.get_weight("Unknown"), None);
}

#[test]
fn test_weighted_scorer_is_valuable() {
    let mut weights = HashMap::new();
    weights.insert("Double Damage".to_string(), 5.0);
    weights.insert("Onslaught".to_string(), 3.0);

    let scorer = WeightedScorer::new(weights);

    assert!(scorer.is_valuable("Double Damage"));
    assert!(scorer.is_valuable("Onslaught"));
    assert!(!scorer.is_valuable("Unknown"));
    assert!(!scorer.is_valuable(""));
}

#[test]
fn test_calculate_score_single_mod() {
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

#[test]
fn test_calculate_score_multiple_mods() {
    let mut weights = HashMap::new();
    weights.insert("Double Damage".to_string(), 5.0);
    weights.insert("Onslaught".to_string(), 3.0);

    let scorer = WeightedScorer::new(weights);

    let matched_mods = vec![
        MatchedMod {
            mod_text: "Double Damage".to_string(),
            weight: 5.0,
            count: 2,
        },
        MatchedMod {
            mod_text: "Onslaught".to_string(),
            weight: 3.0,
            count: 1,
        },
    ];

    // (5.0 * 2) + (3.0 * 1) = 10.0 + 3.0 = 13.0
    assert_eq!(scorer.calculate_score(&matched_mods), 13.0);
}

#[test]
fn test_calculate_score_empty() {
    let weights = HashMap::new();
    let scorer = WeightedScorer::new(weights);

    let matched_mods = vec![];
    assert_eq!(scorer.calculate_score(&matched_mods), 0.0);
}

#[test]
fn test_calculate_score_zero_weight() {
    let mut weights = HashMap::new();
    weights.insert("Zero Weight Mod".to_string(), 0.0);

    let scorer = WeightedScorer::new(weights);

    let matched_mods = vec![MatchedMod {
        mod_text: "Zero Weight Mod".to_string(),
        weight: 0.0,
        count: 100,
    }];

    assert_eq!(scorer.calculate_score(&matched_mods), 0.0);
}
