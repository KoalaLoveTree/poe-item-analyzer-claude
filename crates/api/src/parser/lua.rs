//! Lua file parser for PoB data files

use crate::error::DownloadError;
use mlua::{Lua, Table, Value};
use std::collections::HashMap;
use std::path::Path;

/// Parsed NodeIndexMapping.lua data
#[derive(Debug, Clone)]
pub struct NodeIndexMapping {
    pub size: usize,
    pub size_notable: usize,
    pub nodes: HashMap<u32, NodeMappingInfo>,
}

#[derive(Debug, Clone)]
pub struct NodeMappingInfo {
    pub index: usize,
    pub size: u32,
}

/// Parsed LegionPassives.lua data
#[derive(Debug, Clone)]
pub struct LegionPassives {
    pub additions: HashMap<String, PassiveAddition>,
}

#[derive(Debug, Clone)]
pub struct PassiveAddition {
    pub display_name: String,
    pub stat_descriptions: Vec<String>,
}

/// Lua file parser
pub struct LuaParser;

impl LuaParser {
    /// Parse NodeIndexMapping.lua
    pub fn parse_node_index_mapping(path: &Path) -> Result<NodeIndexMapping, DownloadError> {
        let lua_code = std::fs::read_to_string(path)
            .map_err(|e| DownloadError::IoError(e))?;

        let lua = Lua::new();

        // Execute Lua code
        lua.load(&lua_code)
            .exec()
            .map_err(|e| DownloadError::InvalidManifest(format!("Lua error: {}", e)))?;

        // Get the nodeIDList table
        let globals = lua.globals();
        let node_list: Table = globals
            .get("nodeIDList")
            .map_err(|e| DownloadError::InvalidManifest(format!("Missing nodeIDList: {}", e)))?;

        // Extract size values
        let size: usize = node_list
            .get("size")
            .map_err(|e| DownloadError::InvalidManifest(format!("Missing size: {}", e)))?;

        let size_notable: usize = node_list
            .get("sizeNotable")
            .map_err(|e| {
                DownloadError::InvalidManifest(format!("Missing sizeNotable: {}", e))
            })?;

        // Extract node mappings
        let mut nodes = HashMap::new();

        for pair in node_list.pairs::<Value, Value>() {
            let (key, value) = pair.map_err(|e| {
                DownloadError::InvalidManifest(format!("Error iterating table: {}", e))
            })?;

            // Skip string keys (size, sizeNotable)
            if let Value::Integer(node_id) = key {
                if let Value::Table(info_table) = value {
                    let index: usize = info_table.get("index").map_err(|e| {
                        DownloadError::InvalidManifest(format!("Missing index: {}", e))
                    })?;

                    let size_val: u32 = info_table.get("size").map_err(|e| {
                        DownloadError::InvalidManifest(format!("Missing size: {}", e))
                    })?;

                    nodes.insert(
                        node_id as u32,
                        NodeMappingInfo {
                            index,
                            size: size_val,
                        },
                    );
                }
            }
        }

        Ok(NodeIndexMapping {
            size,
            size_notable,
            nodes,
        })
    }

    /// Parse LegionPassives.lua
    pub fn parse_legion_passives(path: &Path) -> Result<LegionPassives, DownloadError> {
        let lua_code = std::fs::read_to_string(path)
            .map_err(|e| DownloadError::IoError(e))?;

        let lua = Lua::new();

        // Execute and get return value
        let data: Table = lua
            .load(&lua_code)
            .eval()
            .map_err(|e| DownloadError::InvalidManifest(format!("Lua error: {}", e)))?;

        // Get additions table
        let additions_table: Table = data.get("additions").map_err(|e| {
            DownloadError::InvalidManifest(format!("Missing additions: {}", e))
        })?;

        // Parse additions
        let mut additions = HashMap::new();

        for pair in additions_table.pairs::<Value, Table>() {
            let (_index, addition_table) = pair.map_err(|e| {
                DownloadError::InvalidManifest(format!("Error iterating additions: {}", e))
            })?;

            // Get required fields
            let id: String = addition_table.get("id").map_err(|e| {
                DownloadError::InvalidManifest(format!("Missing id: {}", e))
            })?;

            let display_name: String = addition_table.get("dn").map_err(|e| {
                DownloadError::InvalidManifest(format!("Missing dn: {}", e))
            })?;

            // Get stat descriptions array
            let sd_table: Table = addition_table.get("sd").unwrap_or_else(|_| {
                // Create empty table if sd is missing
                lua.create_table().unwrap()
            });

            let mut stat_descriptions = Vec::new();
            for pair in sd_table.pairs::<usize, String>() {
                if let Ok((_idx, desc)) = pair {
                    stat_descriptions.push(desc);
                }
            }

            additions.insert(
                id,
                PassiveAddition {
                    display_name,
                    stat_descriptions,
                },
            );
        }

        Ok(LegionPassives { additions })
    }
}
