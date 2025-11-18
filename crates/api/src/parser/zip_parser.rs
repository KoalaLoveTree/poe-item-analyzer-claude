//! ZIP file parser for timeless jewel LUT data
//!
//! This parser handles the binary LUT format from Path of Building.
//!
//! # Binary Format
//!
//! For most jewel types (Lethal Pride, Brutal Restraint, Elegant Hubris, Militant Faith):
//! - Data is a flat array of u8 values
//! - Layout: `data[node_index * seed_range_size + (seed - min_seed)] = modifier_index`
//! - modifier_index 0 = no change
//! - modifier_index > 0 = maps to a modifier in LegionPassives.lua
//!
//! # Glorious Vanity Special Case
//!
//! Glorious Vanity uses a more complex format with:
//! - Header section (nodeCount × seedRange bytes) indicating data size per node
//! - Variable-length data section with stat IDs and roll values
//! - Format: All stats first, then all rolls (not interleaved)
//! - Valid patterns: 1+1, 1+2, 3+3, or 4+4 (stats+rolls)

use crate::error::DownloadError;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use flate2::read::ZlibDecoder;

use super::lut::JewelLutData;

/// ZIP file parser for jewel LUT data
pub struct ZipParser;

impl ZipParser {
    /// Extract and parse a jewel ZIP file
    ///
    /// The "ZIP" files are actually zlib-compressed binary data, not ZIP archives
    pub fn parse_jewel_zip(
        zip_path: &Path,
        jewel_type: &str,
    ) -> Result<JewelLutData, DownloadError> {
        eprintln!("Parsing jewel file: {}", zip_path.display());

        // Open the file
        let file = File::open(zip_path).map_err(|e| DownloadError::IoError(e))?;

        // Decompress with zlib
        let mut decoder = ZlibDecoder::new(file);
        let mut decompressed_data = Vec::new();
        decoder
            .read_to_end(&mut decompressed_data)
            .map_err(|e| DownloadError::DownloadFailed(format!("Failed to decompress: {}", e)))?;

        eprintln!(
            "Decompressed {} bytes from {}",
            decompressed_data.len(),
            zip_path.file_name().unwrap().to_string_lossy()
        );

        // Get the seed range for this jewel type
        let seed_range = Self::get_seed_range(jewel_type);

        // Parse the binary LUT data based on jewel type
        let lookup_table = if jewel_type == "GloriousVanity" {
            Self::parse_glorious_vanity(&decompressed_data, seed_range)?
        } else {
            Self::parse_binary_data(&decompressed_data, seed_range)?
        };

        Ok(JewelLutData {
            jewel_type: jewel_type.to_string(),
            seed_range,
            lookup_table,
        })
    }

    /// Get seed range for a jewel type
    fn get_seed_range(jewel_type: &str) -> (u32, u32) {
        match jewel_type {
            "LethalPride" => (10000, 18000),
            "BrutalRestraint" => (500, 8000),
            "GloriousVanity" => (100, 8000),
            "ElegantHubris" => (2000, 160000),
            "MilitantFaith" => (2000, 10000),
            _ => (0, 0),
        }
    }

    /// Parse binary LUT data from decompressed buffer
    ///
    /// The binary format from PoB is:
    /// - Array of bytes representing modifier indices
    /// - Formula: array[node_index * seed_range_size + (seed - min_seed)] = modifier_index
    /// - Where modifier_index 0 means "no change"
    /// - Non-zero modifier_index maps to a modifier ID (string representation)
    fn parse_binary_data(
        buffer: &[u8],
        seed_range: (u32, u32),
    ) -> Result<HashMap<u32, HashMap<usize, String>>, DownloadError> {
        let mut lookup_table: HashMap<u32, HashMap<usize, String>> = HashMap::new();

        if buffer.is_empty() {
            return Ok(lookup_table);
        }

        eprintln!(
            "Parsing {} bytes (seed range: {:?})",
            buffer.len(),
            seed_range
        );

        // Calculate seed range size
        let min_seed = seed_range.0;
        let max_seed = seed_range.1;
        let seed_size = (max_seed - min_seed + 1) as usize;

        // The buffer is organized as:
        // For each node (node_index 0..N):
        //   For each seed (seed - min_seed = 0..seed_size):
        //     modifier_index: u8
        //
        // To determine number of nodes: buffer.len() / seed_size
        if buffer.len() % seed_size != 0 {
            eprintln!(
                "Warning: Buffer size {} is not evenly divisible by seed_size {}",
                buffer.len(),
                seed_size
            );
        }

        let num_nodes = buffer.len() / seed_size;

        eprintln!(
            "Detected {} nodes with {} seeds each",
            num_nodes, seed_size
        );

        // Parse the binary data
        // We iterate by seed to build the lookup table structure: seed -> node_index -> modifier
        for seed_offset in 0..seed_size {
            let seed = min_seed + seed_offset as u32;
            let mut node_modifiers: HashMap<usize, String> = HashMap::new();

            for node_index in 0..num_nodes {
                let byte_offset = node_index * seed_size + seed_offset;

                if byte_offset >= buffer.len() {
                    break;
                }

                let modifier_index = buffer[byte_offset];

                // modifier_index 0 typically means "no change" - we skip these
                if modifier_index != 0 {
                    // Convert modifier index to string ID
                    // The modifier index maps to entries in LegionPassives.lua
                    // For now, use the index as a string; this will be resolved
                    // against the actual modifier data later
                    let modifier_id = modifier_index.to_string();
                    node_modifiers.insert(node_index, modifier_id);
                }
            }

            // Only store if there are actual modifiers for this seed
            if !node_modifiers.is_empty() {
                lookup_table.insert(seed, node_modifiers);
            }
        }

        eprintln!(
            "Parsed {} seeds with modifier data",
            lookup_table.len()
        );

        Ok(lookup_table)
    }

    /// Parse Glorious Vanity binary data (special format with header)
    ///
    /// Glorious Vanity uses a two-part format:
    /// 1. Header: nodeCount × seedRange bytes, each indicating data length for that node/seed
    /// 2. Data: Variable-length byte arrays with stat IDs and roll values
    ///
    /// Format: All stats first, then all rolls (not interleaved)
    /// Valid patterns: 1+1, 1+2, 3+3, or 4+4 (stats+rolls)
    fn parse_glorious_vanity(
        buffer: &[u8],
        seed_range: (u32, u32),
    ) -> Result<HashMap<u32, HashMap<usize, String>>, DownloadError> {
        let mut lookup_table: HashMap<u32, HashMap<usize, String>> = HashMap::new();

        if buffer.is_empty() {
            return Ok(lookup_table);
        }

        eprintln!(
            "Parsing Glorious Vanity: {} bytes (seed range: {:?})",
            buffer.len(),
            seed_range
        );

        let min_seed = seed_range.0;
        let max_seed = seed_range.1;
        let seed_size = (max_seed - min_seed + 1) as usize;

        // Glorious Vanity has fixed node count (1678 nodes)
        const GV_NODE_COUNT: usize = 1678;

        // Header size: nodeCount × seedRange
        let header_size = GV_NODE_COUNT * seed_size;

        if buffer.len() < header_size {
            return Err(DownloadError::DownloadFailed(format!(
                "Buffer too small for Glorious Vanity header: {} < {}",
                buffer.len(),
                header_size
            )));
        }

        // Split buffer into header and data sections
        let header = &buffer[0..header_size];
        let data = &buffer[header_size..];

        eprintln!(
            "Header: {} bytes ({} nodes × {} seeds), Data: {} bytes",
            header_size,
            GV_NODE_COUNT,
            seed_size,
            data.len()
        );

        // Parse data section using header as index
        let mut data_offset = 0;

        for seed_offset in 0..seed_size {
            let seed = min_seed + seed_offset as u32;
            let mut node_modifiers: HashMap<usize, String> = HashMap::new();

            for node_index in 0..GV_NODE_COUNT {
                // Get data length from header
                let header_index = node_index * seed_size + seed_offset;
                let data_length = header[header_index] as usize;

                if data_length > 0 {
                    // Extract the data bytes for this node/seed
                    if data_offset + data_length > data.len() {
                        eprintln!(
                            "Warning: Data offset {} + length {} exceeds buffer size {}",
                            data_offset,
                            data_length,
                            data.len()
                        );
                        break;
                    }

                    let node_data = &data[data_offset..data_offset + data_length];

                    // Parse the variable-length data
                    // Format: [stat1, stat2, ...] [roll1, roll2, ...]
                    // Valid patterns: 1+1, 1+2, 3+3, or 4+4
                    let modifier_str = Self::parse_gv_node_data(node_data, data_length);

                    if !modifier_str.is_empty() {
                        node_modifiers.insert(node_index, modifier_str);
                    }

                    data_offset += data_length;
                }
            }

            // Only store seeds that have modifiers
            if !node_modifiers.is_empty() {
                lookup_table.insert(seed, node_modifiers);
            }
        }

        eprintln!(
            "Parsed {} Glorious Vanity seeds with data",
            lookup_table.len()
        );

        Ok(lookup_table)
    }

    /// Parse Glorious Vanity node data (variable-length byte array)
    ///
    /// Returns a string representation of the stats and rolls
    fn parse_gv_node_data(data: &[u8], length: usize) -> String {
        if length == 0 {
            return String::new();
        }

        // Determine pattern based on length
        // 1+1 = 2 bytes, 1+2 = 3 bytes, 3+3 = 6 bytes, 4+4 = 8 bytes
        let (num_stats, num_rolls) = match length {
            2 => (1, 1),
            3 => (1, 2),
            6 => (3, 3),
            8 => (4, 4),
            _ => {
                eprintln!("Warning: Unexpected GV data length: {}", length);
                // Try to infer from length (assume equal stats and rolls)
                if length % 2 == 0 {
                    let half = length / 2;
                    (half, half)
                } else {
                    // Odd length, might be 1 stat with multiple rolls
                    (1, length - 1)
                }
            }
        };

        // Extract stats and rolls
        let mut parts = Vec::new();

        // Stats come first
        for i in 0..num_stats {
            if i < data.len() {
                parts.push(format!("s{}", data[i]));
            }
        }

        // Rolls come after stats
        for i in 0..num_rolls {
            let idx = num_stats + i;
            if idx < data.len() {
                parts.push(format!("r{}", data[idx]));
            }
        }

        parts.join("|")
    }
}
