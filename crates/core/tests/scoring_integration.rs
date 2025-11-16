//! Integration test: Scoring system integration
//!
//! Tests how the scoring system works with matched mods

use poe_item_analyzer_core::{items::MatchedMod, scoring::WeightedScorer};
use std::collections::HashMap;

#[test]
fn test_realistic_scoring_scenario() {
    // User defines valuable mods with weights
    let mut weights = HashMap::new();
    weights.insert("5% chance to deal Double Damage".to_string(), 10.0);
    weights.insert("Onslaught on Hit".to_string(), 8.0);
    weights.insert("+20 to Strength".to_string(), 2.0);
    weights.insert("+20 to Dexterity".to_string(), 2.0);
    weights.insert("Endurance Charge on Kill".to_string(), 5.0);

    let scorer = WeightedScorer::new(weights);

    // Scenario 1: Jewel with multiple valuable mods
    let matched_mods_1 = vec![
        MatchedMod {
            mod_text: "5% chance to deal Double Damage".to_string(),
            weight: 10.0,
            count: 2, // Found 2 nodes with this mod
        },
        MatchedMod {
            mod_text: "Onslaught on Hit".to_string(),
            weight: 8.0,
            count: 1,
        },
        MatchedMod {
            mod_text: "+20 to Strength".to_string(),
            weight: 2.0,
            count: 5,
        },
    ];

    let score_1 = scorer.calculate_score(&matched_mods_1);
    // (10.0 * 2) + (8.0 * 1) + (2.0 * 5) = 20 + 8 + 10 = 38.0
    assert_eq!(score_1, 38.0);

    // Scenario 2: Jewel with fewer valuable mods
    let matched_mods_2 = vec![MatchedMod {
        mod_text: "Endurance Charge on Kill".to_string(),
        weight: 5.0,
        count: 1,
    }];

    let score_2 = scorer.calculate_score(&matched_mods_2);
    assert_eq!(score_2, 5.0);

    // First jewel should be valued higher
    assert!(score_1 > score_2);
}

#[test]
fn test_scoring_comparison() {
    let mut weights = HashMap::new();
    weights.insert("High Value Mod".to_string(), 100.0);
    weights.insert("Low Value Mod".to_string(), 1.0);

    let scorer = WeightedScorer::new(weights);

    // One high-value mod
    let high_value = vec![MatchedMod {
        mod_text: "High Value Mod".to_string(),
        weight: 100.0,
        count: 1,
    }];

    // Many low-value mods
    let many_low_value = vec![MatchedMod {
        mod_text: "Low Value Mod".to_string(),
        weight: 1.0,
        count: 50,
    }];

    let high_score = scorer.calculate_score(&high_value);
    let low_score = scorer.calculate_score(&many_low_value);

    assert_eq!(high_score, 100.0);
    assert_eq!(low_score, 50.0);
    assert!(high_score > low_score);
}

#[test]
fn test_empty_weights_empty_mods() {
    let weights = HashMap::new();
    let scorer = WeightedScorer::new(weights);

    let mods = vec![];
    let score = scorer.calculate_score(&mods);

    assert_eq!(score, 0.0);
}
