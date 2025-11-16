//! Parser module for converting PoB data to our optimized format

mod lua;
mod lut;
mod zip_parser;

#[cfg(test)]
mod tests;

pub use lut::{LutData, NodeModifier, PassiveNode, NodeInfo, JewelLutData};
pub use lua::{LuaParser, NodeIndexMapping, LegionPassives};
pub use zip_parser::ZipParser;

use crate::error::DownloadError;
use std::path::Path;

/// Main parser for converting PoB data to our format
pub struct PobDataParser;

impl PobDataParser {
    /// Parse PoB data directory and convert to optimized format
    pub fn parse_directory(data_dir: &Path) -> Result<LutData, DownloadError> {
        // Parse Lua metadata files
        let node_mapping = LuaParser::parse_node_index_mapping(
            &data_dir.join("NodeIndexMapping.lua")
        )?;

        let legion_passives = LuaParser::parse_legion_passives(
            &data_dir.join("LegionPassives.lua")
        )?;

        // Convert to our LUT format (without jewel data yet)
        let mut lut_data = LutData::from_pob_data(node_mapping, legion_passives)?;

        // Extract and parse ZIP files for each jewel type
        let jewel_types = vec![
            "LethalPride",
            "BrutalRestraint",
            "GloriousVanity",
            "ElegantHubris",
            "MilitantFaith",
        ];

        for jewel_type in jewel_types {
            let zip_path = data_dir.join(format!("{}.zip", jewel_type));

            if zip_path.exists() {
                let jewel_data = ZipParser::parse_jewel_zip(&zip_path, jewel_type)?;
                lut_data.jewels.insert(jewel_type.to_string(), jewel_data);
            } else {
                eprintln!("Warning: {} not found, skipping", zip_path.display());
            }
        }

        Ok(lut_data)
    }

    /// Save parsed data to JSON file
    pub fn save_to_json(lut_data: &LutData, output_path: &Path) -> Result<(), DownloadError> {
        let json = serde_json::to_string_pretty(lut_data)
            .map_err(|e| DownloadError::DownloadFailed(e.to_string()))?;

        std::fs::write(output_path, json)
            .map_err(|e| DownloadError::IoError(e))?;

        Ok(())
    }

    /// Load parsed data from JSON file
    pub fn load_from_json(input_path: &Path) -> Result<LutData, DownloadError> {
        let json = std::fs::read_to_string(input_path)
            .map_err(|e| DownloadError::IoError(e))?;

        serde_json::from_str(&json)
            .map_err(|e| DownloadError::InvalidManifest(e.to_string()))
    }
}
