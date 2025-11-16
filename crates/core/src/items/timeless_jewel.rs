//! Timeless Jewel item model

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::traits::{AnalyzableItem, Item};

/// Types of timeless jewels in Path of Exile
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum JewelType {
    LethalPride,
    BrutalRestraint,
    GloriousVanity,
    ElegantHubris,
    MilitantFaith,
}

impl JewelType {
    /// Get the string representation of the jewel type
    pub fn as_str(&self) -> &'static str {
        match self {
            JewelType::LethalPride => "Lethal Pride",
            JewelType::BrutalRestraint => "Brutal Restraint",
            JewelType::GloriousVanity => "Glorious Vanity",
            JewelType::ElegantHubris => "Elegant Hubris",
            JewelType::MilitantFaith => "Militant Faith",
        }
    }

    /// Parse jewel type from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Lethal Pride" => Some(JewelType::LethalPride),
            "Brutal Restraint" => Some(JewelType::BrutalRestraint),
            "Glorious Vanity" => Some(JewelType::GloriousVanity),
            "Elegant Hubris" => Some(JewelType::ElegantHubris),
            "Militant Faith" => Some(JewelType::MilitantFaith),
            _ => None,
        }
    }
}

/// A timeless jewel item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelessJewel {
    /// Unique identifier
    id: String,

    /// Type of jewel
    pub jewel_type: JewelType,

    /// Seed number (determines mods)
    pub seed: u32,

    /// Conqueror/variant name (e.g., "Kaom", "Balbala")
    pub conqueror: String,

    /// Raw JSON data from the game
    #[serde(skip)]
    raw_data: Value,
}

impl TimelessJewel {
    /// Create a new timeless jewel
    pub fn new(
        id: String,
        jewel_type: JewelType,
        seed: u32,
        conqueror: String,
        raw_data: Value,
    ) -> Self {
        Self {
            id,
            jewel_type,
            seed,
            conqueror,
            raw_data,
        }
    }

    /// Get the seed number
    pub fn seed(&self) -> u32 {
        self.seed
    }

    /// Get the conqueror name
    pub fn conqueror(&self) -> &str {
        &self.conqueror
    }
}

impl Item for TimelessJewel {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn item_type(&self) -> &str {
        self.jewel_type.as_str()
    }

    fn name(&self) -> &str {
        self.jewel_type.as_str()
    }
}

/// Value metrics for a timeless jewel analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelessJewelMetrics {
    /// Results per socket location
    pub socket_results: Vec<SocketResult>,
}

/// Analysis result for a specific socket location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocketResult {
    /// Socket identifier
    pub socket_id: String,

    /// Human-readable socket name (e.g., "Far-Left (near Marauder)")
    pub socket_name: String,

    /// Calculated score for this socket
    pub score: f64,

    /// Mods that matched the user's valuable mod list
    pub matched_mods: Vec<MatchedMod>,

    /// All mods this jewel provides at this socket
    pub all_mods: Vec<String>,
}

/// A mod that matched the user's valuable mod criteria
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchedMod {
    /// The mod text
    pub mod_text: String,

    /// Weight assigned by user
    pub weight: f64,

    /// How many times this mod appears
    pub count: usize,
}

impl AnalyzableItem for TimelessJewel {
    type ValueMetrics = TimelessJewelMetrics;

    fn raw_data(&self) -> &Value {
        &self.raw_data
    }
}
