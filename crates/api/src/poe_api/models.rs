//! API response models

use serde::{Deserialize, Serialize};

// TODO: Add models for API responses

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct League {
    pub id: String,
    pub realm: Option<String>,
    #[serde(rename = "startAt")]
    pub start_at: Option<String>,
    #[serde(rename = "endAt")]
    pub end_at: Option<String>,
}
