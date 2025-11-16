//! Unit tests for items module

use super::*;
use serde_json::Value;

#[test]
fn test_jewel_type_as_str() {
    assert_eq!(JewelType::LethalPride.as_str(), "Lethal Pride");
    assert_eq!(JewelType::BrutalRestraint.as_str(), "Brutal Restraint");
    assert_eq!(JewelType::GloriousVanity.as_str(), "Glorious Vanity");
    assert_eq!(JewelType::ElegantHubris.as_str(), "Elegant Hubris");
    assert_eq!(JewelType::MilitantFaith.as_str(), "Militant Faith");
}

#[test]
fn test_jewel_type_from_str() {
    assert_eq!(
        JewelType::from_str("Lethal Pride"),
        Some(JewelType::LethalPride)
    );
    assert_eq!(
        JewelType::from_str("Brutal Restraint"),
        Some(JewelType::BrutalRestraint)
    );
    assert_eq!(JewelType::from_str("Unknown"), None);
    assert_eq!(JewelType::from_str(""), None);
}

#[test]
fn test_timeless_jewel_creation() {
    let jewel = TimelessJewel::new(
        "test-id".to_string(),
        JewelType::LethalPride,
        12345,
        "Kaom".to_string(),
        Value::Null,
    );

    assert_eq!(jewel.id(), "test-id");
    assert_eq!(jewel.item_type(), "Lethal Pride");
    assert_eq!(jewel.name(), "Lethal Pride");
    assert_eq!(jewel.seed(), 12345);
    assert_eq!(jewel.conqueror(), "Kaom");
}

#[test]
fn test_timeless_jewel_different_types() {
    let types = vec![
        (JewelType::LethalPride, "Kaom"),
        (JewelType::BrutalRestraint, "Balbala"),
        (JewelType::GloriousVanity, "Doryani"),
        (JewelType::ElegantHubris, "Caspiro"),
        (JewelType::MilitantFaith, "Avarius"),
    ];

    for (jewel_type, conqueror) in types {
        let jewel = TimelessJewel::new(
            "id".to_string(),
            jewel_type,
            1000,
            conqueror.to_string(),
            Value::Null,
        );

        assert_eq!(jewel.jewel_type, jewel_type);
        assert_eq!(jewel.conqueror(), conqueror);
    }
}
