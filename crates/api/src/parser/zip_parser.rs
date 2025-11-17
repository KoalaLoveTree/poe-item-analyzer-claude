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
//! - Header section indicating data size per node
//! - Variable stat replacements for all nodes
//! - Multiple stats per notable with roll values
//!
//! TODO: Implement Glorious Vanity header parsing when needed

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

        // Parse the binary LUT data
        let lookup_table = Self::parse_binary_data(&decompressed_data, seed_range)?;

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
}
