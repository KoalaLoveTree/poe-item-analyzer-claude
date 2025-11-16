//! Tests for parser module

use super::*;
use tempfile::TempDir;

#[test]
fn test_pob_data_parser_creation() {
    // Just ensure the struct can be created
    let _parser = PobDataParser;
}

#[test]
fn test_lut_data_creation() {
    use super::lua::{NodeIndexMapping, LegionPassives};
    use std::collections::HashMap;

    let node_mapping = NodeIndexMapping {
        size: 100,
        size_notable: 10,
        nodes: HashMap::new(),
    };

    let legion_passives = LegionPassives {
        additions: HashMap::new(),
    };

    let result = LutData::from_pob_data(node_mapping, legion_passives);
    assert!(result.is_ok());

    let lut_data = result.unwrap();
    assert_eq!(lut_data.version, "1.0.0");
    assert_eq!(lut_data.node_indices.len(), 0);
    assert_eq!(lut_data.modifiers.len(), 0);
}

#[test]
fn test_node_modifier_search_text() {
    use super::lut::NodeModifier;

    let modifier = NodeModifier {
        id: "test_id".to_string(),
        display_name: "Fire Damage".to_string(),
        stat_descriptions: vec!["10% increased Fire Damage".to_string()],
        search_text: "fire damage 10% increased fire damage".to_string(),
    };

    assert!(modifier.search_text.contains("fire"));
    assert!(modifier.search_text.contains("damage"));
}

#[test]
fn test_save_and_load_json() {
    use super::lua::{NodeIndexMapping, LegionPassives};
    use std::collections::HashMap;

    let temp_dir = TempDir::new().unwrap();
    let json_path = temp_dir.path().join("lut_data.json");

    // Create test data
    let node_mapping = NodeIndexMapping {
        size: 100,
        size_notable: 10,
        nodes: HashMap::new(),
    };

    let legion_passives = LegionPassives {
        additions: HashMap::new(),
    };

    let lut_data = LutData::from_pob_data(node_mapping, legion_passives).unwrap();

    // Save to JSON
    let save_result = PobDataParser::save_to_json(&lut_data, &json_path);
    assert!(save_result.is_ok());
    assert!(json_path.exists());

    // Load from JSON
    let load_result = PobDataParser::load_from_json(&json_path);
    assert!(load_result.is_ok());

    let loaded_data = load_result.unwrap();
    assert_eq!(loaded_data.version, lut_data.version);
}

#[test]
fn test_get_modifier_not_found() {
    use super::lua::{NodeIndexMapping, LegionPassives};
    use std::collections::HashMap;

    let node_mapping = NodeIndexMapping {
        size: 100,
        size_notable: 10,
        nodes: HashMap::new(),
    };

    let legion_passives = LegionPassives {
        additions: HashMap::new(),
    };

    let lut_data = LutData::from_pob_data(node_mapping, legion_passives).unwrap();

    // Query non-existent data
    let result = lut_data.get_modifier("LethalPride", 12345, 6);
    assert!(result.is_none());
}

#[test]
fn test_zip_parser_seed_ranges() {
    use super::zip_parser::ZipParser;
    use std::fs::File;
    use std::io::Write;
    use zip::write::FileOptions;
    use zip::ZipWriter;

    let temp_dir = TempDir::new().unwrap();
    let zip_path = temp_dir.path().join("LethalPride.zip");

    // Create a dummy ZIP file
    {
        let file = File::create(&zip_path).unwrap();
        let mut zip = ZipWriter::new(file);

        zip.start_file("data.bin", FileOptions::default()).unwrap();
        zip.write_all(&[0u8; 100]).unwrap();
        zip.finish().unwrap();
    }

    // Parse it
    let result = ZipParser::parse_jewel_zip(&zip_path, "LethalPride");
    assert!(result.is_ok());

    let jewel_data = result.unwrap();
    assert_eq!(jewel_data.jewel_type, "LethalPride");
    assert_eq!(jewel_data.seed_range, (10000, 18000));
}
