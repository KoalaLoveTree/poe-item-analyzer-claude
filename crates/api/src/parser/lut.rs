//! LUT (Lookup Table) data structures
//!
//! Our optimized JSON format for timeless jewel data

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::lua::{LegionPassives, NodeIndexMapping};
use crate::error::DownloadError;

/// Complete LUT data for all timeless jewels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LutData {
    /// Version identifier
    pub version: String,

    /// Node ID to index mapping
    pub node_indices: HashMap<u32, NodeInfo>,

    /// Available modifiers by ID
    pub modifiers: HashMap<String, NodeModifier>,

    /// Jewel-specific data
    pub jewels: HashMap<String, JewelLutData>,
}

/// Node information from passive tree
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfo {
    /// Sequential index for array lookups
    pub index: usize,

    /// Size value (purpose TBD - from PoB data)
    pub size: u32,

    /// Node display name
    pub name: Option<String>,

    /// Whether this is a notable passive
    pub is_notable: bool,
}

/// Modifier that can be applied to a node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeModifier {
    /// Unique identifier
    pub id: String,

    /// Display name
    pub display_name: String,

    /// Stat descriptions (e.g., "10% increased Fire Damage")
    pub stat_descriptions: Vec<String>,

    /// Searchable text (lowercase, for searching)
    pub search_text: String,
}

/// LUT data for a specific jewel type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JewelLutData {
    /// Jewel type name (e.g., "LethalPride", "BrutalRestraint")
    pub jewel_type: String,

    /// Seed range (min, max)
    pub seed_range: (u32, u32),

    /// Raw LUT data: seed -> node_index -> modifier_id
    /// Format: HashMap<seed, HashMap<node_index, modifier_id>>
    pub lookup_table: HashMap<u32, HashMap<usize, String>>,
}

/// Passive skill node on the tree
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassiveNode {
    pub id: u32,
    pub name: String,
    pub is_notable: bool,
}

impl LutData {
    /// Create LUT data from PoB's parsed Lua files
    pub fn from_pob_data(
        node_mapping: NodeIndexMapping,
        legion_passives: LegionPassives,
    ) -> Result<Self, DownloadError> {
        // Convert node mapping
        let mut node_indices = HashMap::new();
        for (node_id, info) in node_mapping.nodes {
            node_indices.insert(
                node_id,
                NodeInfo {
                    index: info.index,
                    size: info.size,
                    name: None, // Will be populated from other data
                    is_notable: false, // Will be determined later
                },
            );
        }

        // Convert modifiers from additions
        let mut modifiers = HashMap::new();
        for (id, addition) in legion_passives.additions {
            let search_text = format!(
                "{} {}",
                addition.display_name,
                addition.stat_descriptions.join(" ")
            )
            .to_lowercase();

            modifiers.insert(
                id.clone(),
                NodeModifier {
                    id: id.clone(),
                    display_name: addition.display_name,
                    stat_descriptions: addition.stat_descriptions,
                    search_text,
                },
            );
        }

        Ok(LutData {
            version: "1.0.0".to_string(),
            node_indices,
            modifiers,
            jewels: HashMap::new(), // Will be populated from ZIP files
        })
    }

    /// Get modifier for a specific jewel, seed, and node
    pub fn get_modifier(
        &self,
        jewel_type: &str,
        seed: u32,
        node_id: u32,
    ) -> Option<&NodeModifier> {
        // Get jewel data
        let jewel_data = self.jewels.get(jewel_type)?;

        // Get node index
        let node_info = self.node_indices.get(&node_id)?;

        // Lookup modifier ID
        let seed_data = jewel_data.lookup_table.get(&seed)?;
        let modifier_id = seed_data.get(&node_info.index)?;

        // Get modifier
        self.modifiers.get(modifier_id)
    }
}
