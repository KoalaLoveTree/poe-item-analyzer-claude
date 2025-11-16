//! Unit tests for analyzers module

use super::*;
use crate::items::{JewelType, TimelessJewel};
use serde_json::Value;

#[test]
fn test_config_creation() {
    let mut config = TimelessJewelConfig::new();
    config.add_mod("Double Damage".to_string(), 5.0);
    config.add_mod("Onslaught".to_string(), 4.5);

    assert_eq!(config.valuable_mods().len(), 2);
    assert_eq!(config.valuable_mods().get("Double Damage"), Some(&5.0));
    assert_eq!(config.valuable_mods().get("Onslaught"), Some(&4.5));
}

#[test]
fn test_config_default() {
    let config = TimelessJewelConfig::default();
    assert_eq!(config.valuable_mods().len(), 0);
}

#[test]
fn test_analyzer_creation() {
    let analyzer = TimelessJewelAnalyzer::new();
    // Just verify it can be created
    let _ = analyzer;
}

#[test]
fn test_analyzer_default() {
    let _analyzer = TimelessJewelAnalyzer::default();
}

#[test]
fn test_analyzer_placeholder_behavior() {
    let analyzer = TimelessJewelAnalyzer::new();
    let config = TimelessJewelConfig::new();

    let jewel = TimelessJewel::new(
        "test".to_string(),
        JewelType::LethalPride,
        12345,
        "Kaom".to_string(),
        Value::Null,
    );

    // Analyze should not panic even with empty config
    let result = analyzer.analyze(&jewel, &config);
    assert!(result.is_ok());

    // Result should have empty socket results (placeholder implementation)
    let analysis = result.unwrap();
    assert_eq!(analysis.metrics.socket_results.len(), 0);
    assert_eq!(analysis.best_score, 0.0);
}
