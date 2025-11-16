//! Integration test: Full timeless jewel analysis workflow
//!
//! This tests the public API as a user would interact with it.

use poe_item_analyzer_core::{
    analyzers::{Analyzer, TimelessJewelAnalyzer, TimelessJewelConfig},
    items::{JewelType, TimelessJewel},
};
use serde_json::Value;

#[test]
fn test_create_jewel_and_config() {
    // Create a jewel like a user would
    let jewel = TimelessJewel::new(
        "my-jewel-1".to_string(),
        JewelType::LethalPride,
        12345,
        "Kaom".to_string(),
        Value::Null,
    );

    // Create analysis config
    let mut config = TimelessJewelConfig::new();
    config.add_mod("Double Damage".to_string(), 5.0);
    config.add_mod("Onslaught".to_string(), 3.0);

    // Create analyzer
    let analyzer = TimelessJewelAnalyzer::new();

    // Analyze the jewel
    let result = analyzer.analyze(&jewel, &config);

    // Should succeed (even if placeholder implementation)
    assert!(result.is_ok());
}

#[test]
fn test_batch_analysis() {
    // Create multiple jewels
    let jewels = vec![
        TimelessJewel::new(
            "jewel-1".to_string(),
            JewelType::LethalPride,
            10000,
            "Kaom".to_string(),
            Value::Null,
        ),
        TimelessJewel::new(
            "jewel-2".to_string(),
            JewelType::BrutalRestraint,
            20000,
            "Balbala".to_string(),
            Value::Null,
        ),
        TimelessJewel::new(
            "jewel-3".to_string(),
            JewelType::GloriousVanity,
            30000,
            "Doryani".to_string(),
            Value::Null,
        ),
    ];

    // Create config
    let config = TimelessJewelConfig::new();

    // Create analyzer
    let analyzer = TimelessJewelAnalyzer::new();

    // Analyze batch
    let results = analyzer.analyze_batch(&jewels, &config);

    // Should succeed
    assert!(results.is_ok());

    // Should return ranked results
    let ranked = results.unwrap();
    assert_eq!(ranked.len(), jewels.len());

    // Results should be ranked (1, 2, 3, ...)
    for (i, result) in ranked.iter().enumerate() {
        assert_eq!(result.rank, i + 1);
    }
}

#[test]
fn test_all_jewel_types_can_be_analyzed() {
    let jewel_types = vec![
        (JewelType::LethalPride, "Kaom"),
        (JewelType::BrutalRestraint, "Balbala"),
        (JewelType::GloriousVanity, "Doryani"),
        (JewelType::ElegantHubris, "Caspiro"),
        (JewelType::MilitantFaith, "Avarius"),
    ];

    let config = TimelessJewelConfig::new();
    let analyzer = TimelessJewelAnalyzer::new();

    for (jewel_type, conqueror) in jewel_types {
        let jewel = TimelessJewel::new(
            format!("{:?}", jewel_type),
            jewel_type,
            1000,
            conqueror.to_string(),
            Value::Null,
        );

        let result = analyzer.analyze(&jewel, &config);
        assert!(
            result.is_ok(),
            "Failed to analyze {:?} jewel",
            jewel_type
        );
    }
}
